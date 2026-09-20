//! In-app Whisper transcription backend (Phase 5 core).
//!
//! Same engine as the verified sidecar approach (whisper.cpp, MIT), bound
//! in-process via `whisper-rs`: no Python, no subprocess, crash-isolated
//! behind the job system (Phase 2+), cancellable between segments.
//!
//! Pipeline: 16 kHz mono WAV (`avid-media::extract_audio_command`) →
//! [`transcribe_wav`] → [`Transcript`](crate::transcript::Transcript).
//! Model files live in the app cache (downloaded once, never committed).

use std::path::Path;

use thiserror::Error;
use whisper_rs::{
    FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters, WhisperError,
};

use crate::transcript::{Segment, Transcript, Word};

/// Transcription backend failures.
#[derive(Debug, Error)]
pub enum TranscribeError {
    /// Model file missing or unreadable.
    #[error("transcription model unavailable: {0}")]
    ModelUnavailable(String),
    /// WAV parsing failed (need 16 kHz mono PCM for the engine).
    #[error("unreadable audio input: {0}")]
    BadAudio(String),
    /// Engine failure (wraps `whisper-rs`).
    #[error("transcription engine failed: {0}")]
    Engine(String),
}

impl TranscribeError {
    /// Stable error code for UI mapping and logs.
    #[must_use]
    pub fn code(&self) -> &'static str {
        match self {
            Self::ModelUnavailable(_) => "AVID_TRANSCRIBE_001",
            Self::BadAudio(_) => "AVID_TRANSCRIBE_002",
            Self::Engine(_) => "AVID_TRANSCRIBE_003",
        }
    }
}

impl From<WhisperError> for TranscribeError {
    fn from(error: WhisperError) -> Self {
        Self::Engine(error.to_string())
    }
}

/// Load 16 kHz mono f32 samples from a WAV file.
/// Rejects anything else with a clear error (resample upstream via
/// `MediaEngine::extract_audio_command`, which emits exactly this format).
fn load_mono_16k_wav(path: &Path) -> Result<Vec<f32>, TranscribeError> {
    let mut reader =
        hound::WavReader::open(path).map_err(|e| TranscribeError::BadAudio(e.to_string()))?;
    let spec = reader.spec();
    if spec.sample_rate != 16_000 || spec.channels != 1 {
        return Err(TranscribeError::BadAudio(format!(
            "need 16kHz mono WAV, got {}Hz x{}ch",
            spec.sample_rate, spec.channels
        )));
    }
    reader
        .samples::<i16>()
        .map(|sample| {
            sample
                .map(|s| f32::from(s) / f32::from(i16::MAX))
                .map_err(|e| TranscribeError::BadAudio(e.to_string()))
        })
        .collect()
}

/// Transcribe a 16 kHz mono WAV file with a ggml model.
/// `language`: `"en"` etc. or `"auto"`. Deterministic settings (greedy,
/// single best) — the transcript must be reproducible take after take.
pub fn transcribe_wav(
    model_path: &Path,
    wav_path: &Path,
    language: &str,
    provider: &str,
) -> Result<Transcript, TranscribeError> {
    if !model_path.is_file() {
        return Err(TranscribeError::ModelUnavailable(
            model_path.display().to_string(),
        ));
    }
    let samples = load_mono_16k_wav(wav_path)?;
    if samples.is_empty() {
        return Err(TranscribeError::BadAudio("empty audio".to_owned()));
    }

    let context = WhisperContext::new_with_params(model_path, WhisperContextParameters::default())?;
    let mut state = context.create_state()?;
    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    params.set_n_threads(4);
    params.set_print_special(false);
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);
    if language != "auto" {
        params.set_language(Some(language));
    }
    state.full(params, &samples)?;

    // Timestamps arrive as integer 10 ms units; magnitudes stay tiny, so the
    // float conversion below cannot lose meaningful precision.
    #[allow(clippy::cast_precision_loss)]
    let to_seconds = |units: i64| units as f64 / 100.0;
    let mut segments = vec![];
    let count = state.full_n_segments();
    for index in 0..count {
        let segment = state
            .get_segment(index)
            .ok_or_else(|| TranscribeError::Engine(format!("missing segment {index}")))?;
        let text = segment.to_str_lossy()?.trim().to_owned();
        let start = to_seconds(segment.start_timestamp());
        let end = to_seconds(segment.end_timestamp());
        segments.push(Segment {
            start,
            end,
            text: text.clone(),
            speaker: None,
            words: vec![Word {
                word: text,
                start,
                end,
                confidence: None,
            }],
        });
    }
    Ok(Transcript {
        language: language.to_owned(),
        segments,
        provider: provider.to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn model_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_owned());
        Path::new(&home).join(".cache/avid/models/ggml-tiny.en.bin")
    }

    #[test]
    fn missing_model_fails_cleanly() {
        let err = transcribe_wav(
            Path::new("definitely-no-model.bin"),
            Path::new("definitely-no-audio.wav"),
            "en",
            "local-whispercpp",
        )
        .unwrap_err();
        assert!(matches!(err, TranscribeError::ModelUnavailable(_)));
        assert_eq!(err.code(), "AVID_TRANSCRIBE_001");
    }

    #[test]
    fn rejects_non_16k_audio() {
        let dir = std::env::temp_dir().join("avid-transcribe-test");
        std::fs::create_dir_all(&dir).unwrap();
        let wav = dir.join("wrong-rate.wav");
        let spec = hound::WavSpec {
            channels: 2,
            sample_rate: 48_000,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer = hound::WavWriter::create(&wav, spec).unwrap();
        for _ in 0..100 {
            writer.write_sample(0i16).unwrap();
        }
        writer.finalize().unwrap();
        let err = transcribe_wav(&model_path(), &wav, "en", "local-whispercpp").unwrap_err();
        assert!(matches!(err, TranscribeError::BadAudio(_)));
        std::fs::remove_dir_all(&dir).ok();
    }

    /// REAL test: synthesizes speech (`say`), converts via ffmpeg, and
    /// transcribes in-process. Ignored by default (needs macOS `say` +
    /// cached model); run explicitly. This is the Phase 5 core loop.
    #[test]
    #[ignore]
    fn transcribes_synthesized_speech_in_process() {
        let model = model_path();
        if !model.is_file() {
            return;
        }
        let dir = std::env::temp_dir().join("avid-transcribe-live");
        std::fs::create_dir_all(&dir).unwrap();
        let aiff = dir.join("speech.aiff");
        let wav = dir.join("speech.wav");
        let say = std::process::Command::new("say")
            .arg("-o")
            .arg(&aiff)
            .arg("Kafka has three partitions.")
            .status();
        if say.map(|s| !s.success()).unwrap_or(true) {
            std::fs::remove_dir_all(&dir).ok();
            return;
        }
        let converted = std::process::Command::new("ffmpeg")
            .args(["-y", "-v", "error", "-i"])
            .arg(&aiff)
            .args(["-ac", "1", "-ar", "16000"])
            .arg(&wav)
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        assert!(converted, "ffmpeg conversion");
        let transcript = transcribe_wav(&model, &wav, "en", "local-whispercpp").unwrap();
        std::fs::remove_dir_all(&dir).ok();
        let text = transcript
            .segments
            .iter()
            .map(|s| s.text.as_str())
            .collect::<Vec<_>>()
            .join(" ");
        assert!(text.to_lowercase().contains("kafka"), "got: {text}");
        assert!(text.to_lowercase().contains("partitions"), "got: {text}");
        assert!(!transcript.segments.is_empty());
    }
}
