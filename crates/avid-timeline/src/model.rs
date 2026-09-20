//! Timeline data model (AGENTS §13).
//!
//! Non-destructive by construction: clips reference source media by id and
//! describe `start / duration / in_point` windows into it. Originals are
//! never touched — every mutation below goes through a command (see
//! [`crate::commands`]).

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::error::TimelineError;

/// Seconds on the timeline. Always finite and non-negative where stored.
pub type Seconds = f64;

/// Track kinds supported by the model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrackKind {
    Video,
    Audio,
    Text,
    Graphics,
    Caption,
}

/// A timeline track.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Track {
    /// Stable unique id.
    pub id: String,
    pub kind: TrackKind,
    /// Visual stacking order (0 = bottom).
    pub index: u32,
    pub name: String,
    /// Locked tracks reject clip edits.
    pub locked: bool,
    /// Audio tracks only; video tracks ignore it.
    pub muted: bool,
}

/// A non-destructive clip: a window (`in_point`, `duration`) into source
/// media placed at `start` on a track.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Clip {
    /// Stable unique id.
    pub id: String,
    /// Source media asset id (never a filesystem path — see `avid-project`).
    pub source_media_id: String,
    /// Owning track id.
    pub track_id: String,
    /// Timeline position of the clip head, in seconds.
    pub start: Seconds,
    /// Timeline length, in seconds. Always positive.
    pub duration: Seconds,
    /// Offset into the source media, in seconds.
    pub in_point: Seconds,
    pub name: String,
}

impl Clip {
    /// Exclusive end position (`start + duration`).
    #[must_use]
    pub fn end(&self) -> Seconds {
        self.start + self.duration
    }
}

/// The editable timeline: tracks plus clips.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Timeline {
    pub tracks: Vec<Track>,
    /// Clip id → clip.
    pub clips: BTreeMap<String, Clip>,
}

/// Small epsilon for float edge comparisons (abutting clips are legal).
const EPSILON: f64 = 1e-9;

fn valid_time(value: Seconds, what: &str) -> Result<(), TimelineError> {
    if !value.is_finite() || value < 0.0 {
        return Err(TimelineError::InvalidTime(format!("{what} = {value}")));
    }
    Ok(())
}

fn valid_duration(value: Seconds) -> Result<(), TimelineError> {
    if !value.is_finite() || value <= 0.0 {
        return Err(TimelineError::InvalidTime(format!("duration = {value}")));
    }
    Ok(())
}

impl Timeline {
    /// Total duration: the maximum clip end, or 0 for an empty timeline.
    #[must_use]
    pub fn duration(&self) -> Seconds {
        self.clips.values().map(Clip::end).fold(0.0, f64::max)
    }

    /// Clips on one track, sorted by start time.
    #[must_use]
    pub fn clips_on_track(&self, track_id: &str) -> Vec<&Clip> {
        let mut clips: Vec<&Clip> = self
            .clips
            .values()
            .filter(|c| c.track_id == track_id)
            .collect();
        clips.sort_by(|a, b| {
            a.start
                .partial_cmp(&b.start)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        clips
    }

    fn require_unlocked_track(&self, track_id: &str) -> Result<(), TimelineError> {
        let track = self
            .tracks
            .iter()
            .find(|t| t.id == track_id)
            .ok_or_else(|| TimelineError::TrackNotFound(track_id.to_owned()))?;
        if track.locked {
            return Err(TimelineError::TrackLocked(track_id.to_owned()));
        }
        Ok(())
    }

    fn require_no_overlap(
        &self,
        clip: &Clip,
        ignore_id: Option<&str>,
    ) -> Result<(), TimelineError> {
        for other in self.clips_on_track(&clip.track_id) {
            if Some(other.id.as_str()) == ignore_id {
                continue;
            }
            let overlaps = clip.start < other.end() - EPSILON && other.start < clip.end() - EPSILON;
            if overlaps {
                return Err(TimelineError::Overlap {
                    track_id: clip.track_id.clone(),
                    clip_id: clip.id.clone(),
                });
            }
        }
        Ok(())
    }

    /// Validate a clip against track existence, lock state, time sanity,
    /// id uniqueness, and same-track overlap.
    pub fn validate_clip(&self, clip: &Clip, ignore_id: Option<&str>) -> Result<(), TimelineError> {
        self.require_unlocked_track(&clip.track_id)?;
        valid_time(clip.start, "start")?;
        valid_duration(clip.duration)?;
        valid_time(clip.in_point, "in_point")?;
        if ignore_id != Some(clip.id.as_str()) && self.clips.contains_key(&clip.id) {
            return Err(TimelineError::DuplicateClipId(clip.id.clone()));
        }
        self.require_no_overlap(clip, ignore_id)?;
        Ok(())
    }

    /// Insert a clip after validation. Prefer commands over direct use.
    pub fn insert_clip(&mut self, clip: Clip) -> Result<(), TimelineError> {
        self.validate_clip(&clip, None)?;
        self.clips.insert(clip.id.clone(), clip);
        Ok(())
    }

    /// Remove and return a clip. Prefer commands over direct use.
    pub fn take_clip(&mut self, id: &str) -> Result<Clip, TimelineError> {
        let clip = self
            .clips
            .get(id)
            .ok_or_else(|| TimelineError::ClipNotFound(id.to_owned()))?;
        self.require_unlocked_track(&clip.track_id)?;
        self.clips
            .remove(id)
            .ok_or_else(|| TimelineError::ClipNotFound(id.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn track(id: &str) -> Track {
        Track {
            id: id.to_owned(),
            kind: TrackKind::Video,
            index: 0,
            name: id.to_owned(),
            locked: false,
            muted: false,
        }
    }

    fn clip(id: &str, start: f64, duration: f64) -> Clip {
        Clip {
            id: id.to_owned(),
            source_media_id: "media-1".to_owned(),
            track_id: "v1".to_owned(),
            start,
            duration,
            in_point: 0.0,
            name: id.to_owned(),
        }
    }

    fn timeline() -> Timeline {
        let mut tl = Timeline::default();
        tl.tracks.push(track("v1"));
        tl
    }

    #[test]
    fn duration_is_max_clip_end() {
        let mut tl = timeline();
        assert_eq!(tl.duration(), 0.0);
        tl.insert_clip(clip("a", 0.0, 5.0)).unwrap();
        tl.insert_clip(clip("b", 10.0, 4.0)).unwrap();
        assert_eq!(tl.duration(), 14.0);
    }

    #[test]
    fn abutting_clips_are_legal_but_overlap_is_not() {
        let mut tl = timeline();
        tl.insert_clip(clip("a", 0.0, 5.0)).unwrap();
        tl.insert_clip(clip("b", 5.0, 5.0)).unwrap();
        let err = tl.insert_clip(clip("c", 4.999, 1.0)).unwrap_err();
        assert!(matches!(err, TimelineError::Overlap { .. }));
        assert_eq!(err.code(), "AVID_TIMELINE_006");
    }

    #[test]
    fn rejects_bad_times_and_duplicates() {
        let mut tl = timeline();
        assert!(matches!(
            tl.insert_clip(clip("a", -1.0, 5.0)).unwrap_err(),
            TimelineError::InvalidTime(_)
        ));
        assert!(matches!(
            tl.insert_clip(clip("b", 0.0, 0.0)).unwrap_err(),
            TimelineError::InvalidTime(_)
        ));
        tl.insert_clip(clip("c", 0.0, 2.0)).unwrap();
        assert!(matches!(
            tl.insert_clip(clip("c", 5.0, 2.0)).unwrap_err(),
            TimelineError::DuplicateClipId(_)
        ));
    }

    #[test]
    fn locked_tracks_reject_edits() {
        let mut tl = timeline();
        tl.tracks[0].locked = true;
        assert!(matches!(
            tl.insert_clip(clip("a", 0.0, 2.0)).unwrap_err(),
            TimelineError::TrackLocked(_)
        ));
    }

    #[test]
    fn serializes_and_round_trips() {
        let mut tl = timeline();
        tl.insert_clip(clip("a", 0.0, 5.0)).unwrap();
        let json = serde_json::to_string(&tl).unwrap();
        let back: Timeline = serde_json::from_str(&json).unwrap();
        assert_eq!(tl, back);
    }

    #[test]
    fn shipped_example_timeline_parses() {
        use std::path::Path;
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../examples/technical-explainer/project.json");
        if !path.is_file() {
            return;
        }
        let json = std::fs::read_to_string(path).unwrap();
        let manifest: serde_json::Value = serde_json::from_str(&json).unwrap();
        let timeline: Timeline = serde_json::from_value(manifest["timeline"].clone()).unwrap();
        assert_eq!(timeline.tracks.len(), 2);
        assert_eq!(timeline.duration(), 12.0);
    }
}
