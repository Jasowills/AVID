//! `avid-media`: FFmpeg abstraction — the `MediaEngine` (AGENTS §11, ADR-002).
//!
//! INVARIANT: raw FFmpeg command construction lives ONLY in this crate.
//! Everything else calls [`MediaEngine`]. MVP spawns version-pinned
//! sidecar binaries (one CLI per operation); the trait hides that so a
//! linked or streaming implementation can replace it without touching callers.
//!
//! Path safety: every operation rejects absolute paths and `..` traversal
//! before spawning anything (AGENTS §106).

use std::path::{Path, PathBuf};
use std::process::Command;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

/// Media engine failures.
#[derive(Debug, Error)]
pub enum MediaError {
    /// Input path is absolute or escapes its directory.
    #[error("unsafe media path: {0}")]
    UnsafePath(String),
    /// The ffmpeg/ffprobe binary is missing or not executable.
    #[error("media binary unavailable: {0}")]
    BinaryUnavailable(String),
    /// The subprocess failed; stderr is captured for the expandable
    /// technical-details disclosure (never shown raw in the UI).
    #[error("media operation '{op}' failed (exit {exit}): {stderr}")]
    ProcessFailed {
        /// Operation name, e.g. `"probe"`.
        op: &'static str,
        /// Exit code, if any.
        exit: String,
        /// Captured stderr (truncated).
        stderr: String,
    },
    /// The operation was cancelled by the user (job center).
    #[error("media operation '{0}' cancelled")]
    Cancelled(&'static str),
    /// Output parsing failed (e.g. unexpected ffprobe JSON).
    #[error("could not parse media output: {0}")]
    Parse(String),
}

impl MediaError {
    /// Stable error code for UI mapping and logs.
    #[must_use]
    pub fn code(&self) -> &'static str {
        match self {
            Self::UnsafePath(_) => "AVID_MEDIA_001",
            Self::BinaryUnavailable(_) => "AVID_MEDIA_002",
            Self::ProcessFailed { .. } => "AVID_MEDIA_003",
            Self::Cancelled(_) => "AVID_MEDIA_005",
            Self::Parse(_) => "AVID_MEDIA_004",
        }
    }
}

/// Probed stream metadata (subset of ffprobe JSON we depend on).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamInfo {
    /// Stream index within the file.
    pub index: u32,
    /// `video`, `audio`, `subtitle`, …
    pub codec_type: String,
    /// e.g. `h264`, `aac`.
    pub codec_name: String,
    /// Video width, if applicable.
    pub width: Option<u32>,
    /// Video height, if applicable.
    pub height: Option<u32>,
    /// Audio sample rate, if applicable.
    pub sample_rate: Option<u32>,
    /// Audio channel count, if applicable.
    pub channels: Option<u32>,
}

/// Probed file metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MediaInfo {
    /// Duration in seconds, if reported.
    pub duration: Option<f64>,
    /// Container format, e.g. `mov,mp4,m4a,3gp,3g2,mj2`.
    pub format: String,
    /// File size in bytes, if reported.
    pub size: Option<u64>,
    pub streams: Vec<StreamInfo>,
}

impl MediaInfo {
    /// Primary video stream, if any.
    #[must_use]
    pub fn video_stream(&self) -> Option<&StreamInfo> {
        self.streams.iter().find(|s| s.codec_type == "video")
    }

    /// Primary audio stream, if any.
    #[must_use]
    pub fn audio_stream(&self) -> Option<&StreamInfo> {
        self.streams.iter().find(|s| s.codec_type == "audio")
    }
}

/// ffprobe JSON shape (relevant subset). `serde` ignores the rest.
#[derive(Debug, Deserialize)]
struct FfprobeOutput {
    #[serde(default)]
    format: FfprobeFormat,
    #[serde(default)]
    streams: Vec<FfprobeStream>,
}

#[derive(Debug, Default, Deserialize)]
struct FfprobeFormat {
    #[serde(default)]
    duration: Option<String>,
    #[serde(default)]
    format_name: Option<String>,
    #[serde(default)]
    size: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FfprobeStream {
    #[serde(default)]
    index: u32,
    #[serde(default)]
    codec_type: Option<String>,
    #[serde(default)]
    codec_name: Option<String>,
    #[serde(default)]
    width: Option<u32>,
    #[serde(default)]
    height: Option<u32>,
    #[serde(default)]
    sample_rate: Option<String>,
    #[serde(default)]
    channels: Option<u32>,
}

/// Parse `ffprobe -print_format json` output into [`MediaInfo`].
pub fn parse_probe_output(json: &str) -> Result<MediaInfo, MediaError> {
    let output: FfprobeOutput =
        serde_json::from_str(json).map_err(|e| MediaError::Parse(e.to_string()))?;
    Ok(MediaInfo {
        duration: output.format.duration.as_deref().and_then(parse_f64),
        format: output.format.format_name.unwrap_or_default(),
        size: output.format.size.as_deref().and_then(|s| s.parse().ok()),
        streams: output
            .streams
            .into_iter()
            .map(|s| StreamInfo {
                index: s.index,
                codec_type: s.codec_type.unwrap_or_default(),
                codec_name: s.codec_name.unwrap_or_default(),
                width: s.width,
                height: s.height,
                sample_rate: s.sample_rate.as_deref().and_then(|s| s.parse().ok()),
                channels: s.channels,
            })
            .collect(),
    })
}

fn parse_f64(s: &str) -> Option<f64> {
    s.parse::<f64>().ok().filter(|v| v.is_finite() && *v >= 0.0)
}

/// Reject absolute paths and `..` traversal before any subprocess spawn.
fn check_input_path(path: &Path) -> Result<(), MediaError> {
    if path.is_absolute() {
        return Err(MediaError::UnsafePath(path.display().to_string()));
    }
    if path
        .components()
        .any(|c| matches!(c, std::path::Component::ParentDir))
    {
        return Err(MediaError::UnsafePath(path.display().to_string()));
    }
    Ok(())
}

/// Parse `-progress pipe:1` key=value output into `(out_time_us, speed)`.
/// Returns `None` for non-progress lines (headers, blank lines).
pub fn parse_progress_line(line: &str) -> Option<(u64, String)> {
    let mut time_us: Option<u64> = None;
    let mut speed: Option<&str> = None;
    for part in line.split_whitespace() {
        let (key, value) = part.split_once('=')?;
        match key {
            "out_time_us" => time_us = value.parse().ok(),
            "speed" => speed = Some(value),
            _ => {}
        }
    }
    Some((time_us?, speed.unwrap_or("").to_owned()))
}

/// A detected silence span in seconds (rough-cut proposals).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SilenceSpan {
    pub start: f64,
    pub end: f64,
}

/// Parse `silencedetect` stderr into spans. Pairs `silence_start` with the
/// next `silence_end`; a trailing unpaired start is dropped (duration
/// unknown) and documented in the proposal as unverified.
pub fn parse_silence_output(stderr: &str) -> Vec<SilenceSpan> {
    let mut spans = vec![];
    let mut open: Option<f64> = None;
    for line in stderr.lines() {
        let line = line.trim();
        if let Some(value) = line.split_once("silence_start:") {
            open = value.1.trim().parse().ok();
        } else if let Some(value) = line.split_once("silence_end:") {
            let end: Option<f64> = value.1.split('|').next().unwrap_or("").trim().parse().ok();
            if let (Some(start), Some(end)) = (open.take(), end) {
                if end > start {
                    spans.push(SilenceSpan { start, end });
                }
            }
        }
    }
    spans
}

/// Stateful `-progress` tracker: `-progress pipe:1` emits one `key=value`
/// per line (`out_time_us=…`, then `progress=continue|end`). Feed every
/// stdout line; returns the 0–1 fraction on `progress=` lines.
pub struct ProgressTracker {
    total_us: u64,
    last_us: u64,
}

impl ProgressTracker {
    /// Create a tracker for a job of `total_seconds` output.
    #[must_use]
    pub fn new(total_seconds: f64) -> Self {
        Self {
            total_us: (total_seconds.max(0.0) * 1_000_000.0) as u64,
            last_us: 0,
        }
    }

    /// Feed one stdout line; `Some(fraction)` on `progress=` markers.
    pub fn feed(&mut self, line: &str) -> Option<f64> {
        let line = line.trim();
        if let Some(value) = line.strip_prefix("out_time_us=") {
            if let Ok(parsed) = value.trim().parse::<u64>() {
                self.last_us = parsed;
            }
            return None;
        }
        if line.starts_with("progress=") {
            if self.total_us == 0 {
                return Some(1.0);
            }
            // Microsecond magnitudes: float conversion cannot lose meaningful
            // precision for a 0–1 progress fraction.
            #[allow(clippy::cast_precision_loss)]
            let fraction = (self.last_us.min(self.total_us) as f64) / (self.total_us as f64);
            return Some(fraction.min(1.0));
        }
        None
    }
}

/// The media engine: probe, proxy, thumbnails, waveform, audio extraction,
/// frame extraction, and render — all behind sidecar binaries.
pub struct MediaEngine {
    ffmpeg: PathBuf,
    ffprobe: PathBuf,
}

impl MediaEngine {
    /// Create an engine against explicit binary paths. Fails if either is
    /// missing or not executable — callers surface `AVID_MEDIA_002` with
    /// install guidance instead of a raw spawn error.
    pub fn new(ffmpeg: PathBuf, ffprobe: PathBuf) -> Result<Self, MediaError> {
        for binary in [&ffmpeg, &ffprobe] {
            if !binary.is_file() {
                return Err(MediaError::BinaryUnavailable(binary.display().to_string()));
            }
        }
        Ok(Self { ffmpeg, ffprobe })
    }

    /// Locate system binaries by name (`ffmpeg`, `ffprobe` on PATH).
    /// Sidecar-per-triple resolution (ADR-002) replaces this once bundled.
    pub fn system() -> Result<Self, MediaError> {
        fn find(name: &str) -> Result<PathBuf, MediaError> {
            let path = PathBuf::from(name);
            // Probe executability with `--version`; PATH lookup happens in the child.
            let ok = Command::new(&path)
                .arg("-version")
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false);
            if ok {
                Ok(path)
            } else {
                Err(MediaError::BinaryUnavailable(name.to_owned()))
            }
        }
        Ok(Self {
            ffmpeg: find("ffmpeg")?,
            ffprobe: find("ffprobe")?,
        })
    }

    /// Probe a media file, returning structured metadata.
    /// Input must be project-relative (traversal rejected).
    pub fn probe(&self, input: &Path) -> Result<MediaInfo, MediaError> {
        check_input_path(input)?;
        self.probe_any(input)
    }

    /// Probe by explicit path (user-chosen import sources). The caller owns
    /// validation — used for files outside the project before import copies
    /// them in. Never called with AI-generated paths.
    pub fn probe_file(&self, input: &Path) -> Result<MediaInfo, MediaError> {
        if !input.is_file() {
            return Err(MediaError::ProcessFailed {
                op: "probe",
                exit: "missing".to_owned(),
                stderr: format!("no such file: {}", input.display()),
            });
        }
        self.probe_any(input)
    }

    fn probe_any(&self, input: &Path) -> Result<MediaInfo, MediaError> {
        let output = Command::new(&self.ffprobe)
            .args([
                "-v",
                "quiet",
                "-print_format",
                "json",
                "-show_format",
                "-show_streams",
            ])
            .arg(input)
            .output()
            .map_err(|e| MediaError::BinaryUnavailable(e.to_string()))?;
        if !output.status.success() {
            return Err(MediaError::ProcessFailed {
                op: "probe",
                exit: output
                    .status
                    .code()
                    .map_or("signal".to_owned(), |c| c.to_string()),
                stderr: truncate(&String::from_utf8_lossy(&output.stderr)),
            });
        }
        parse_probe_output(&String::from_utf8_lossy(&output.stdout))
    }

    /// Run silence detection (`silencedetect` filter) on a project file.
    /// `noise_db`: threshold like -30.0; `min_seconds`: minimum span length.
    /// Spans shorter than the minimum are filtered here (the filter's own
    /// `d=` already gates, this is a second pass for float safety).
    pub fn detect_silence(
        &self,
        input: &Path,
        noise_db: f32,
        min_seconds: f64,
    ) -> Result<Vec<SilenceSpan>, MediaError> {
        check_input_path(input)?;
        self.detect_silence_any(input, noise_db, min_seconds)
    }

    /// Detection by explicit path (project-joined absolute paths from jobs).
    /// Same contract as [`MediaEngine::probe_file`]: the caller owns validity.
    pub fn detect_silence_file(
        &self,
        input: &Path,
        noise_db: f32,
        min_seconds: f64,
    ) -> Result<Vec<SilenceSpan>, MediaError> {
        if !input.is_file() {
            return Err(MediaError::ProcessFailed {
                op: "detectSilence",
                exit: "missing".to_owned(),
                stderr: format!("no such file: {}", input.display()),
            });
        }
        self.detect_silence_any(input, noise_db, min_seconds)
    }

    fn detect_silence_any(
        &self,
        input: &Path,
        noise_db: f32,
        min_seconds: f64,
    ) -> Result<Vec<SilenceSpan>, MediaError> {
        check_input_path(input)?;
        if !(-80.0..=-10.0).contains(&noise_db) || min_seconds <= 0.0 || !min_seconds.is_finite() {
            return Err(MediaError::ProcessFailed {
                op: "detectSilence",
                exit: "params".to_owned(),
                stderr: "noise_db must be -80..-10 and min_seconds positive".to_owned(),
            });
        }
        let output = Command::new(&self.ffmpeg)
            .args(["-hide_banner", "-nostats", "-i"])
            .arg(input)
            .args([
                "-af",
                &format!("silencedetect=noise={noise_db}dB:d={min_seconds}"),
                "-f",
                "null",
                "-",
            ])
            .output()
            .map_err(|e| MediaError::BinaryUnavailable(e.to_string()))?;
        if !output.status.success() {
            return Err(MediaError::ProcessFailed {
                op: "detectSilence",
                exit: output
                    .status
                    .code()
                    .map_or("signal".to_owned(), |c| c.to_string()),
                stderr: truncate(&String::from_utf8_lossy(&output.stderr)),
            });
        }
        Ok(
            parse_silence_output(&String::from_utf8_lossy(&output.stderr))
                .into_iter()
                .filter(|span| span.end - span.start >= min_seconds)
                .collect(),
        )
    }

    /// Build (not run) the proxy transcode command for inspection/testing.
    /// 540p H.264 + AAC with faststart — proxy-first preview (ADR-002).
    #[must_use]
    pub fn proxy_command(&self, input: &Path, output: &Path) -> Vec<String> {
        vec![
            self.ffmpeg.display().to_string(),
            "-y".to_owned(),
            "-hwaccel".to_owned(),
            "auto".to_owned(),
            "-i".to_owned(),
            input.display().to_string(),
            "-vf".to_owned(),
            "scale=-2:540".to_owned(),
            "-c:v".to_owned(),
            "libx264".to_owned(),
            "-preset".to_owned(),
            "veryfast".to_owned(),
            "-c:a".to_owned(),
            "aac".to_owned(),
            "-movflags".to_owned(),
            "+faststart".to_owned(),
            output.display().to_string(),
        ]
    }

    /// Build (not run) the 16 kHz mono WAV extraction command (Whisper input).
    #[must_use]
    pub fn extract_audio_command(&self, input: &Path, output: &Path) -> Vec<String> {
        vec![
            self.ffmpeg.display().to_string(),
            "-y".to_owned(),
            "-i".to_owned(),
            input.display().to_string(),
            "-vn".to_owned(),
            "-ac".to_owned(),
            "1".to_owned(),
            "-ar".to_owned(),
            "16000".to_owned(),
            "-c:a".to_owned(),
            "pcm_s16le".to_owned(),
            output.display().to_string(),
        ]
    }

    /// Run the 540p proxy transcode. Paths are explicit (project `media/`
    /// source, project `proxies/` output); the caller owns their validity.
    pub fn generate_proxy(&self, input: &Path, output: &Path) -> Result<(), MediaError> {
        self.run_ffmpeg("proxy", &self.proxy_command(input, output))
    }

    /// Build (not run) the single-frame extraction command.
    #[must_use]
    pub fn extract_frame_command(&self, input: &Path, seconds: f64, output: &Path) -> Vec<String> {
        vec![
            self.ffmpeg.display().to_string(),
            "-y".to_owned(),
            "-ss".to_owned(),
            seconds.to_string(),
            "-i".to_owned(),
            input.display().to_string(),
            "-frames:v".to_owned(),
            "1".to_owned(),
            "-q:v".to_owned(),
            "2".to_owned(),
            output.display().to_string(),
        ]
    }

    /// Mint a content-addressed asset id for an imported file.
    #[must_use]
    pub fn mint_asset_id() -> String {
        Uuid::new_v4().to_string()
    }

    /// Path of the ffmpeg binary (for capability probes).
    #[must_use]
    pub fn ffmpeg_path(&self) -> &Path {
        &self.ffmpeg
    }

    /// Run a lowered render graph (`argv[0]` = ffmpeg binary).
    pub fn run_render(&self, argv: &[String]) -> Result<(), MediaError> {
        self.run_ffmpeg("render", argv)
    }

    /// Run a render with live progress: inserts `-progress pipe:1`, parses
    /// `out_time_us` against `total_seconds`, and calls `on_progress` with
    /// 0–1 fractions. Checks `should_cancel` between lines and kills the
    /// child promptly (`Cancelled`, never a hung job).
    pub fn run_render_with_progress(
        &self,
        argv: &[String],
        total_seconds: f64,
        should_cancel: &std::sync::atomic::AtomicBool,
        on_progress: &dyn Fn(f64),
    ) -> Result<(), MediaError> {
        use std::io::BufRead;
        use std::sync::atomic::Ordering;
        let (binary, args) = argv
            .split_first()
            .ok_or_else(|| MediaError::ProcessFailed {
                op: "render",
                exit: "empty".to_owned(),
                stderr: String::new(),
            })?;
        let mut full_args = vec!["-progress".to_owned(), "pipe:1".to_owned()];
        full_args.extend(args.iter().cloned());
        let mut child = std::process::Command::new(binary)
            .args(&full_args)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| MediaError::BinaryUnavailable(e.to_string()))?;
        let mut tracker = ProgressTracker::new(total_seconds);
        if let Some(stdout) = child.stdout.take() {
            for line in std::io::BufReader::new(stdout).lines() {
                if should_cancel.load(Ordering::Relaxed) {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err(MediaError::Cancelled("render"));
                }
                let Ok(line) = line else { break };
                if let Some(fraction) = tracker.feed(&line) {
                    on_progress(fraction);
                }
            }
        }
        let output = child
            .wait_with_output()
            .map_err(|e| MediaError::BinaryUnavailable(e.to_string()))?;
        if output.status.success() {
            on_progress(1.0);
            Ok(())
        } else {
            Err(MediaError::ProcessFailed {
                op: "render",
                exit: output
                    .status
                    .code()
                    .map_or("signal".to_owned(), |c| c.to_string()),
                stderr: truncate(&String::from_utf8_lossy(&output.stderr)),
            })
        }
    }

    /// Run 16 kHz mono WAV extraction (Whisper input). Paths are explicit
    /// (project dir + cache); the caller owns their validity.
    pub fn extract_audio(&self, input: &Path, output: &Path) -> Result<(), MediaError> {
        self.run_ffmpeg("extractAudio", &self.extract_audio_command(input, output))
    }

    /// Run a command previously built by one of the `*_command` builders.
    /// `argv[0]` must be the ffmpeg binary; kept private so raw strings
    /// never leak out of this crate (AGENTS §11).
    fn run_ffmpeg(&self, op: &'static str, argv: &[String]) -> Result<(), MediaError> {
        let (binary, args) = argv
            .split_first()
            .ok_or_else(|| MediaError::ProcessFailed {
                op,
                exit: "empty".to_owned(),
                stderr: String::new(),
            })?;
        let output = Command::new(binary)
            .args(args)
            .output()
            .map_err(|e| MediaError::BinaryUnavailable(e.to_string()))?;
        if output.status.success() {
            Ok(())
        } else {
            Err(MediaError::ProcessFailed {
                op,
                exit: output
                    .status
                    .code()
                    .map_or("signal".to_owned(), |c| c.to_string()),
                stderr: truncate(&String::from_utf8_lossy(&output.stderr)),
            })
        }
    }
}

fn truncate(s: &str) -> String {
    const LIMIT: usize = 2000;
    if s.len() <= LIMIT {
        s.to_owned()
    } else {
        format!("{}…[truncated]", &s[..LIMIT])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROBE_JSON: &str = r#"{
        "streams": [
            {"index": 0, "codec_name": "h264", "codec_type": "video",
             "width": 1280, "height": 720},
            {"index": 1, "codec_name": "aac", "codec_type": "audio",
             "sample_rate": "48000", "channels": 2}
        ],
        "format": {"format_name": "mov,mp4,m4a,3gp,3g2,mj2",
                   "duration": "10.040000", "size": "1234567"}
    }"#;

    #[test]
    fn parses_ffprobe_json() {
        let info = parse_probe_output(PROBE_JSON).unwrap();
        assert_eq!(info.duration, Some(10.04));
        assert_eq!(info.size, Some(1_234_567));
        assert_eq!(info.streams.len(), 2);
        let video = info.video_stream().unwrap();
        assert_eq!(
            (video.codec_name.as_str(), video.width, video.height),
            ("h264", Some(1280), Some(720))
        );
        let audio = info.audio_stream().unwrap();
        assert_eq!((audio.sample_rate, audio.channels), (Some(48000), Some(2)));
    }

    #[test]
    fn rejects_malformed_probe_output() {
        assert!(matches!(
            parse_probe_output("{nope"),
            Err(MediaError::Parse(_))
        ));
    }

    #[test]
    fn rejects_unsafe_paths_before_spawning() {
        assert!(matches!(
            check_input_path(Path::new("/etc/passwd")),
            Err(MediaError::UnsafePath(_))
        ));
        assert!(matches!(
            check_input_path(Path::new("media/../../evil.mp4")),
            Err(MediaError::UnsafePath(_))
        ));
        assert!(check_input_path(Path::new("media/clip.mp4")).is_ok());
    }

    #[test]
    fn parses_progress_lines() {
        assert_eq!(
            parse_progress_line("out_time_us=5040000 speed=2.5x"),
            Some((5_040_000, "2.5x".to_owned()))
        );
        assert_eq!(parse_progress_line(""), None);
        assert_eq!(parse_progress_line("frame=42"), None);
    }

    #[test]
    fn silence_parser_pairs_authentic_output() {
        // Captured from fixtures/media/silence_5s.mp4 (ffmpeg 9.0.1).
        let stderr = "Input #0, mov,mp4,m4a,3gp,3g2,mj2\n\
            [Parsed_silencedetect_0 @ 0x7fe924716640] silence_start: 0\n\
            [Parsed_silencedetect_0 @ 0x7fe924716640] silence_end: 4.992 | silence_duration: 4.992\n";
        assert_eq!(
            parse_silence_output(stderr),
            vec![SilenceSpan {
                start: 0.0,
                end: 4.992
            }]
        );
        // Continuous tone: no spans (true negative).
        assert!(parse_silence_output("frame=  42 fps=30\n").is_empty());
        // Trailing unpaired start is dropped, not hallucinated.
        assert!(parse_silence_output("silence_start: 3.0\n").is_empty());
    }

    /// LIVE: real `silencedetect` on the silence fixture (expects ~5 s) and
    /// on the tone fixture (expects none). Ignored without fixtures.
    #[test]
    #[ignore]
    fn live_detect_silence_on_fixtures() {
        let engine = MediaEngine::system().expect("system ffmpeg/ffprobe");
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/media");
        if !dir.join("silence_5s.mp4").is_file() {
            return;
        }
        let staged = Path::new("silence_5s.mp4");
        std::fs::copy(dir.join("silence_5s.mp4"), staged).unwrap();
        let spans = engine.detect_silence(staged, -30.0, 0.5).unwrap();
        std::fs::remove_file(staged).ok();
        assert_eq!(spans.len(), 1);
        assert!((spans[0].end - spans[0].start - 5.0).abs() < 0.2);

        let staged = Path::new("talkinghead_10s.mp4");
        std::fs::copy(dir.join("talkinghead_10s.mp4"), staged).unwrap();
        let spans = engine.detect_silence(staged, -30.0, 0.5).unwrap();
        std::fs::remove_file(staged).ok();
        assert!(spans.is_empty());
    }

    #[test]
    fn tracker_reports_fractions_on_progress_markers() {
        let mut tracker = ProgressTracker::new(10.0);
        assert_eq!(tracker.feed("out_time_us=2500000"), None);
        assert_eq!(tracker.feed("speed=2.5x"), None);
        assert_eq!(tracker.feed("progress=continue"), Some(0.25));
        assert_eq!(tracker.feed("out_time_us=20000000"), None);
        assert_eq!(tracker.feed("progress=end"), Some(1.0));
        let mut empty = ProgressTracker::new(0.0);
        assert_eq!(empty.feed("progress=continue"), Some(1.0));
    }

    #[test]
    fn command_builders_emit_expected_filters() {
        let engine = MediaEngine {
            ffmpeg: PathBuf::from("ffmpeg"),
            ffprobe: PathBuf::from("ffprobe"),
        };
        let proxy = engine.proxy_command(Path::new("in.mp4"), Path::new("proxy.mp4"));
        assert!(proxy.contains(&"scale=-2:540".to_owned()));
        assert!(proxy.contains(&"+faststart".to_owned()));
        let audio = engine.extract_audio_command(Path::new("in.mp4"), Path::new("out.wav"));
        assert!(audio.contains(&"16000".to_owned()));
        assert!(audio.contains(&"pcm_s16le".to_owned()));
        let frame = engine.extract_frame_command(Path::new("in.mp4"), 5.0, Path::new("f.png"));
        assert!(frame.contains(&"5".to_owned()));
    }

    /// REAL integration test: probes a fixture generated by
    /// `scripts/make-fixtures.sh`. Ignored when fixtures are absent so CI
    /// without the fixture step stays green; run locally with fixtures built.
    #[test]
    #[ignore]
    fn probes_real_fixture_with_system_ffmpeg() {
        let fixture =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/media/talkinghead_10s.mp4");
        if !fixture.is_file() {
            return;
        }
        let engine = MediaEngine::system().expect("system ffmpeg/ffprobe");
        // check_input_path intentionally rejects absolute paths, so stage the
        // fixture next to the manifest (cargo test runs with CWD = crate root).
        let local = Path::new("talkinghead_10s.mp4");
        std::fs::copy(&fixture, local).unwrap();
        let info = engine.probe(local);
        std::fs::remove_file(local).ok();
        let info = info.unwrap();
        let duration = info.duration.unwrap();
        assert!((duration - 10.0).abs() < 0.15, "duration = {duration}");
        assert!(info.video_stream().is_some() && info.audio_stream().is_some());
    }
}
