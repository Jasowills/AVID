//! Transcript model + text-based edit mapping (Phase 5 data layer).
//!
//! Parsing targets whisper.cpp `--output-json` verbatim (verified by
//! `scripts/verify-transcription.sh` against `ggml-tiny.en.bin`):
//! ```json
//! {"transcription": [{"offsets": {"from": 60, "to": 430}, "text": " Kafka"}]}
//! ```
//! Offsets are milliseconds. The in-app whisper-rs binding (same engine,
//! same JSON) lands next; this module already owns the shapes, the mapping,
//! and the tests so nothing about the format is assumed twice.

use serde::{Deserialize, Serialize};

use crate::AiError;

/// A filler-word hit with its timeline range.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FillerHit {
    /// Surface text as spoken.
    pub word: String,
    /// Start in seconds.
    pub start: f64,
    /// End in seconds.
    pub end: f64,
}

/// A timed word (or word-segment with `-ml 1`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Word {
    /// Surface text, trimmed.
    pub word: String,
    /// Start in seconds.
    pub start: f64,
    /// End in seconds.
    pub end: f64,
    /// Token probability 0–1, when reported.
    pub confidence: Option<f32>,
}

/// A transcript segment (sentence-ish span with optional words).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Segment {
    pub start: f64,
    pub end: f64,
    pub text: String,
    /// Speaker label (`None` until diarization/manual assignment).
    pub speaker: Option<String>,
    #[serde(default)]
    pub words: Vec<Word>,
}

/// A full transcript: language + ordered segments.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transcript {
    /// BCP-47-ish code reported by the engine (e.g. `"en"`).
    pub language: String,
    /// Ordered, non-overlapping segments.
    pub segments: Vec<Segment>,
    /// Which engine produced this (`local-whispercpp`, …) — trust UX.
    pub provider: String,
}

impl Transcript {
    /// All words in order (flattened across segments).
    #[must_use]
    pub fn words(&self) -> Vec<&Word> {
        self.segments.iter().flat_map(|s| &s.words).collect()
    }

    /// Find filler words (`um`, `uh`, `like`, …) with their ranges.
    /// Heuristic by design (documented Medium confidence downstream):
    /// single tokens matched case-insensitively after stripping surrounding
    /// punctuation, plus the `you know` bigram.
    #[must_use]
    pub fn find_fillers(&self) -> Vec<FillerHit> {
        const SINGLES: &[&str] = &[
            "um",
            "uh",
            "erm",
            "ah",
            "eh",
            "like",
            "basically",
            "actually",
            "literally",
            "stuff",
        ];
        let words = self.words();
        let clean: Vec<String> = words
            .iter()
            .map(|word| {
                word.word
                    .trim_matches(|c: char| !c.is_alphanumeric())
                    .to_lowercase()
            })
            .collect();
        let mut hits = vec![];
        let mut index = 0;
        while index < clean.len() {
            if index + 1 < clean.len() && clean[index] == "you" && clean[index + 1] == "know" {
                hits.push(FillerHit {
                    word: "you know".to_owned(),
                    start: words[index].start,
                    end: words[index + 1].end,
                });
                index += 2;
                continue;
            }
            if SINGLES.contains(&clean[index].as_str()) {
                hits.push(FillerHit {
                    word: words[index].word.clone(),
                    start: words[index].start,
                    end: words[index].end,
                });
            }
            index += 1;
        }
        hits
    }

    /// Find timeline ranges covering a phrase (case-insensitive word run).
    /// Powers text-based delete: select "three partitions" → `remove_range`.
    /// Returns one `(start, end)` per match.
    #[must_use]
    pub fn ranges_for_phrase(&self, phrase: &str) -> Vec<(f64, f64)> {
        let needle: Vec<&str> = phrase.split_whitespace().collect();
        if needle.is_empty() {
            return vec![];
        }
        let words = self.words();
        let haystack: Vec<String> = words.iter().map(|w| w.word.to_lowercase()).collect();
        let mut ranges = vec![];
        for i in 0..=haystack.len().saturating_sub(needle.len()) {
            let matches = needle
                .iter()
                .enumerate()
                .all(|(j, word)| haystack[i + j] == word.to_lowercase());
            if matches {
                ranges.push((words[i].start, words[i + needle.len() - 1].end));
            }
        }
        ranges
    }
}

/// whisper.cpp `--output-json` envelope (relevant subset).
#[derive(Debug, Deserialize)]
struct WhisperJson {
    #[serde(default)]
    transcription: Vec<WhisperSegment>,
}

#[derive(Debug, Deserialize)]
struct WhisperSegment {
    #[serde(default)]
    offsets: WhisperOffsets,
    #[serde(default)]
    text: String,
}

#[derive(Debug, Default, Deserialize)]
struct WhisperOffsets {
    #[serde(default)]
    from: u64,
    #[serde(default)]
    to: u64,
}

/// Parse whisper.cpp `--output-json` into a [`Transcript`].
/// `language`/`provider` come from the pipeline (engine reports language
/// separately); words carry segment-level spans unless transcribed `-ml 1`.
pub fn parse_whisper_json(
    json: &str,
    language: &str,
    provider: &str,
) -> Result<Transcript, AiError> {
    let envelope: WhisperJson =
        serde_json::from_str(json).map_err(|e| AiError::InvalidJson(e.to_string()))?;
    let segments = envelope
        .transcription
        .into_iter()
        .map(|s| {
            let text = s.text.trim().to_owned();
            let start = f64::from(u32::try_from(s.offsets.from).unwrap_or(u32::MAX)) / 1000.0;
            let end = f64::from(u32::try_from(s.offsets.to).unwrap_or(u32::MAX)) / 1000.0;
            let words = text
                .split_whitespace()
                .map(|word| Word {
                    word: word.to_owned(),
                    start,
                    end,
                    confidence: None,
                })
                .collect();
            Segment {
                start,
                end,
                text,
                speaker: None,
                words,
            }
        })
        .collect();
    Ok(Transcript {
        language: language.to_owned(),
        segments,
        provider: provider.to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Shape captured from the real verification run
    /// (`scripts/verify-transcription.sh`, tiny.en, `-ml 1`).
    const REAL_JSON: &str = r#"{"transcription": [
        {"offsets": {"from": 60, "to": 430}, "text": " Kafka"},
        {"offsets": {"from": 430, "to": 720}, "text": " has"},
        {"offsets": {"from": 720, "to": 1500}, "text": " three partitions,"}
    ]}"#;

    #[test]
    fn parses_real_whisper_output_shape() {
        let transcript = parse_whisper_json(REAL_JSON, "en", "local-whispercpp").unwrap();
        assert_eq!(transcript.language, "en");
        assert_eq!(transcript.segments.len(), 3);
        assert!((transcript.segments[1].start - 0.43).abs() < 1e-9);
        assert_eq!(transcript.segments[2].text, "three partitions,");
    }

    #[test]
    fn phrase_mapping_finds_timeline_ranges() {
        let transcript = parse_whisper_json(REAL_JSON, "en", "local-whispercpp").unwrap();
        let ranges = transcript.ranges_for_phrase("three partitions,");
        assert_eq!(ranges.len(), 1);
        assert!((ranges[0].0 - 0.72).abs() < 1e-9);
        assert!((ranges[0].1 - 1.5).abs() < 1e-9);
        // Case-insensitive; no match returns empty (never panics).
        assert_eq!(transcript.ranges_for_phrase("THREE PARTITIONS,").len(), 1);
        assert!(transcript.ranges_for_phrase("not in transcript").is_empty());
        assert!(transcript.ranges_for_phrase("  ").is_empty());
    }

    #[test]
    fn rejects_malformed_engine_output() {
        assert!(parse_whisper_json("{nope", "en", "x").is_err());
    }

    #[test]
    fn filler_finder_hits_singles_and_you_know() {
        let transcript = Transcript {
            language: "en".to_owned(),
            provider: "test".to_owned(),
            segments: vec![Segment {
                start: 0.0,
                end: 6.0,
                text: "Well, um, you know, Kafka is, like, great.".to_owned(),
                speaker: None,
                words: [
                    "Well,", "um,", "you", "know,", "Kafka", "is,", "like,", "great.",
                ]
                .into_iter()
                .enumerate()
                .map(|(i, word)| Word {
                    word: word.to_owned(),
                    start: i as f64,
                    end: i as f64 + 0.8,
                    confidence: None,
                })
                .collect(),
            }],
        };
        let hits = transcript.find_fillers();
        assert_eq!(hits.len(), 3);
        assert_eq!(hits[0].word, "um,");
        assert_eq!((hits[1].start, hits[1].end), (2.0, 3.8));
        assert_eq!(hits[1].word, "you know");
        assert_eq!(hits[2].word, "like,");
    }
}
