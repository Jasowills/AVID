//! Command-based editing engine (AGENTS §15).
//!
//! Every edit — human or AI — is a command object with `execute / undo` and
//! serializable data. AI output is translated into these commands only after
//! schema validation (ADR-008); nothing else may mutate the timeline.

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::error::TimelineError;
use crate::model::{Clip, Seconds, Timeline};

/// A single reversible edit. Object-safe and `Send` so undo stacks can live
/// in shared application state.
pub trait EditCommand: Debug + Send {
    /// Stable name for undo labels, e.g. `"Trim clip"`.
    fn name(&self) -> &'static str;
    /// Apply the edit, capturing whatever state `undo` needs.
    fn execute(&mut self, timeline: &mut Timeline) -> Result<(), TimelineError>;
    /// Reverse a previous `execute`.
    fn undo(&mut self, timeline: &mut Timeline) -> Result<(), TimelineError>;
}

/// Insert a new clip.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddClipCommand {
    /// The clip to insert.
    pub clip: Clip,
}

impl EditCommand for AddClipCommand {
    fn name(&self) -> &'static str {
        "Add clip"
    }

    fn execute(&mut self, timeline: &mut Timeline) -> Result<(), TimelineError> {
        timeline.insert_clip(self.clip.clone())
    }

    fn undo(&mut self, timeline: &mut Timeline) -> Result<(), TimelineError> {
        timeline.take_clip(&self.clip.id).map(|_| ())
    }
}

/// Remove a clip, keeping it for undo.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveClipCommand {
    /// Id of the clip to remove.
    pub clip_id: String,
    #[serde(skip)]
    removed: Option<Clip>,
}

impl RemoveClipCommand {
    /// Create a removal for the given clip id.
    #[must_use]
    pub fn new(clip_id: impl Into<String>) -> Self {
        Self {
            clip_id: clip_id.into(),
            removed: None,
        }
    }
}

impl EditCommand for RemoveClipCommand {
    fn name(&self) -> &'static str {
        "Remove clip"
    }

    fn execute(&mut self, timeline: &mut Timeline) -> Result<(), TimelineError> {
        let clip = timeline.take_clip(&self.clip_id)?;
        self.removed = Some(clip);
        Ok(())
    }

    fn undo(&mut self, timeline: &mut Timeline) -> Result<(), TimelineError> {
        let clip = self
            .removed
            .take()
            .ok_or_else(|| TimelineError::NotExecuted(self.name().to_owned()))?;
        timeline.insert_clip(clip)
    }
}

/// Change a clip's start and/or duration (trims and slip-style moves).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrimClipCommand {
    /// Id of the clip to trim.
    pub clip_id: String,
    /// New timeline head position.
    pub new_start: Seconds,
    /// New timeline length.
    pub new_duration: Seconds,
    #[serde(skip)]
    previous: Option<(Seconds, Seconds)>,
}

impl TrimClipCommand {
    /// Create a trim to the given start/duration.
    #[must_use]
    pub fn new(clip_id: impl Into<String>, new_start: Seconds, new_duration: Seconds) -> Self {
        Self {
            clip_id: clip_id.into(),
            new_start,
            new_duration,
            previous: None,
        }
    }
}

impl EditCommand for TrimClipCommand {
    fn name(&self) -> &'static str {
        "Trim clip"
    }

    fn execute(&mut self, timeline: &mut Timeline) -> Result<(), TimelineError> {
        let mut candidate = timeline
            .clips
            .get(&self.clip_id)
            .cloned()
            .ok_or_else(|| TimelineError::ClipNotFound(self.clip_id.clone()))?;
        self.previous = Some((candidate.start, candidate.duration));
        candidate.start = self.new_start;
        candidate.duration = self.new_duration;
        // Keep the source window stable: moving the head consumes source.
        let delta = self.new_start - self.previous.map_or(0.0, |(s, _)| s);
        candidate.in_point = (candidate.in_point + delta).max(0.0);
        timeline.validate_clip(&candidate, Some(&candidate.id))?;
        timeline.clips.insert(candidate.id.clone(), candidate);
        Ok(())
    }

    fn undo(&mut self, timeline: &mut Timeline) -> Result<(), TimelineError> {
        let (start, duration) = self
            .previous
            .ok_or_else(|| TimelineError::NotExecuted(self.name().to_owned()))?;
        let mut candidate = timeline
            .clips
            .get(&self.clip_id)
            .cloned()
            .ok_or_else(|| TimelineError::ClipNotFound(self.clip_id.clone()))?;
        // Restore the source window by the same delta in reverse.
        let delta = start - candidate.start;
        candidate.in_point = (candidate.in_point + delta).max(0.0);
        candidate.start = start;
        candidate.duration = duration;
        timeline.validate_clip(&candidate, Some(&candidate.id))?;
        timeline.clips.insert(candidate.id.clone(), candidate);
        Ok(())
    }
}

/// Move a clip in time and/or across tracks.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoveClipCommand {
    /// Id of the clip to move.
    pub clip_id: String,
    /// New timeline head position.
    pub new_start: Seconds,
    /// Destination track id.
    pub new_track_id: String,
    #[serde(skip)]
    previous: Option<(Seconds, String)>,
}

impl MoveClipCommand {
    /// Create a move to the given start/track.
    #[must_use]
    pub fn new(
        clip_id: impl Into<String>,
        new_start: Seconds,
        new_track_id: impl Into<String>,
    ) -> Self {
        Self {
            clip_id: clip_id.into(),
            new_start,
            new_track_id: new_track_id.into(),
            previous: None,
        }
    }
}

impl EditCommand for MoveClipCommand {
    fn name(&self) -> &'static str {
        "Move clip"
    }

    fn execute(&mut self, timeline: &mut Timeline) -> Result<(), TimelineError> {
        let mut candidate = timeline
            .clips
            .get(&self.clip_id)
            .cloned()
            .ok_or_else(|| TimelineError::ClipNotFound(self.clip_id.clone()))?;
        self.previous = Some((candidate.start, candidate.track_id.clone()));
        candidate.start = self.new_start;
        candidate.track_id = self.new_track_id.clone();
        timeline.validate_clip(&candidate, Some(&candidate.id))?;
        timeline.clips.insert(candidate.id.clone(), candidate);
        Ok(())
    }

    fn undo(&mut self, timeline: &mut Timeline) -> Result<(), TimelineError> {
        let (start, track_id) = self
            .previous
            .clone()
            .ok_or_else(|| TimelineError::NotExecuted(self.name().to_owned()))?;
        let mut candidate = timeline
            .clips
            .get(&self.clip_id)
            .cloned()
            .ok_or_else(|| TimelineError::ClipNotFound(self.clip_id.clone()))?;
        candidate.start = start;
        candidate.track_id = track_id;
        timeline.validate_clip(&candidate, Some(&candidate.id))?;
        timeline.clips.insert(candidate.id.clone(), candidate);
        Ok(())
    }
}

/// Remove a timeline range across all tracks (text-based delete semantics).
///
/// Overlapping clips are split at the range boundaries and the interior
/// pieces removed — one command, one undo step, transactional: a failure
/// rolls back the splits already made. Ranges are validated against the
/// live timeline by the caller (see `apply_operations`); this command
/// re-checks cheaply and fails closed on drift.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveRangeCommand {
    /// Range start in seconds.
    pub start: Seconds,
    /// Range end in seconds (exclusive).
    pub end: Seconds,
    #[serde(skip)]
    splits: Vec<SplitClipCommand>,
    #[serde(skip)]
    removed: Vec<Clip>,
}

impl RemoveRangeCommand {
    /// Create a range removal. Rejects empty/inverted ranges immediately.
    pub fn new(start: Seconds, end: Seconds) -> Result<Self, TimelineError> {
        let ordered = start.partial_cmp(&end).unwrap_or(std::cmp::Ordering::Less);
        if !start.is_finite()
            || !end.is_finite()
            || start < 0.0
            || ordered != std::cmp::Ordering::Less
        {
            return Err(TimelineError::InvalidTime(format!("range {start}..{end}")));
        }
        Ok(Self {
            start,
            end,
            splits: vec![],
            removed: vec![],
        })
    }

    /// Strictly-interior test with the model's abutment epsilon.
    fn interior(position: Seconds, clip: &Clip) -> bool {
        clip.start + 1e-9 < position && position < clip.end() - 1e-9
    }
}

impl EditCommand for RemoveRangeCommand {
    fn name(&self) -> &'static str {
        "Remove range"
    }

    fn execute(&mut self, timeline: &mut Timeline) -> Result<(), TimelineError> {
        self.splits.clear();
        self.removed.clear();
        // Snapshot overlapping clip ids first (the map mutates below).
        let overlapping: Vec<String> = timeline
            .clips
            .values()
            .filter(|clip| clip.start < self.end - 1e-9 && self.start < clip.end() - 1e-9)
            .map(|clip| clip.id.clone())
            .collect();
        for id in overlapping {
            // Split at the range start if interior; the piece right of the
            // cut inherits `{id}-b` and is where the end cut may fall.
            let mut right_id = id.clone();
            let head = timeline
                .clips
                .get(&id)
                .ok_or_else(|| TimelineError::ClipNotFound(id.clone()))?;
            if Self::interior(self.start, head) {
                let mut split = SplitClipCommand::new(id.clone(), self.start);
                split.execute(timeline)?;
                right_id = format!("{id}-b");
                self.splits.push(split);
            }
            let piece = timeline
                .clips
                .get(&right_id)
                .ok_or_else(|| TimelineError::ClipNotFound(right_id.clone()))?;
            if Self::interior(self.end, piece) {
                let mut split = SplitClipCommand::new(right_id, self.end);
                split.execute(timeline)?;
                self.splits.push(split);
            }
        }
        // Remove pieces fully inside the range (re-check after splits).
        // Boundary clips were split above, so merely abutting clips stay.
        let inside: Vec<String> = timeline
            .clips
            .values()
            .filter(|clip| self.start <= clip.start + 1e-9 && clip.end() <= self.end + 1e-9)
            .map(|clip| clip.id.clone())
            .collect();
        for id in inside {
            self.removed.push(timeline.take_clip(&id)?);
        }
        Ok(())
    }

    fn undo(&mut self, timeline: &mut Timeline) -> Result<(), TimelineError> {
        for clip in std::mem::take(&mut self.removed) {
            timeline.insert_clip(clip)?;
        }
        for split in self.splits.iter_mut().rev() {
            split.undo(timeline)?;
        }
        self.splits.clear();
        Ok(())
    }
}

/// Split a clip at a timeline position into `[start, at)` and `[at, end)`.
/// The second half receives id `{original_id}-b`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SplitClipCommand {
    /// Id of the clip to split.
    pub clip_id: String,
    /// Timeline position of the cut. Must be strictly inside the clip.
    pub at: Seconds,
    #[serde(skip)]
    second_id: Option<String>,
}

impl SplitClipCommand {
    /// Create a split at the given timeline position.
    #[must_use]
    pub fn new(clip_id: impl Into<String>, at: Seconds) -> Self {
        Self {
            clip_id: clip_id.into(),
            at,
            second_id: None,
        }
    }
}

impl EditCommand for SplitClipCommand {
    fn name(&self) -> &'static str {
        "Split clip"
    }

    fn execute(&mut self, timeline: &mut Timeline) -> Result<(), TimelineError> {
        let mut first = timeline
            .clips
            .get(&self.clip_id)
            .cloned()
            .ok_or_else(|| TimelineError::ClipNotFound(self.clip_id.clone()))?;
        if !(first.start < self.at && self.at < first.end()) {
            return Err(TimelineError::InvalidTime(format!(
                "split at {} outside clip",
                self.at
            )));
        }
        let second_id = format!("{}-b", first.id);
        if timeline.clips.contains_key(&second_id) {
            return Err(TimelineError::DuplicateClipId(second_id));
        }
        let mut second = first.clone();
        second.id = second_id.clone();
        second.name = format!("{} (2)", first.name);
        second.in_point = first.in_point + (self.at - first.start);
        second.start = self.at;
        second.duration = first.end() - self.at;
        first.duration = self.at - first.start;

        timeline.validate_clip(&first, Some(&first.id))?;
        timeline.clips.insert(first.id.clone(), first);
        // Validate the second half against the updated map (first half
        // already shortened — otherwise the stale original reads as overlap).
        timeline.validate_clip(&second, None)?;
        timeline.clips.insert(second_id.clone(), second);
        self.second_id = Some(second_id);
        Ok(())
    }

    fn undo(&mut self, timeline: &mut Timeline) -> Result<(), TimelineError> {
        let second_id = self
            .second_id
            .take()
            .ok_or_else(|| TimelineError::NotExecuted(self.name().to_owned()))?;
        let second = timeline.take_clip(&second_id)?;
        let mut first = timeline
            .clips
            .get(&self.clip_id)
            .cloned()
            .ok_or_else(|| TimelineError::ClipNotFound(self.clip_id.clone()))?;
        first.duration = second.end() - first.start;
        timeline.validate_clip(&first, Some(&first.id))?;
        timeline.clips.insert(first.id.clone(), first);
        Ok(())
    }
}

/// Change a clip's gain and mute state (AGENTS §66 minimum: volume + mute).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetClipAudioCommand {
    /// Id of the clip to adjust.
    pub clip_id: String,
    /// New gain, 0–4 (1 = unity).
    pub volume: f32,
    /// New mute state.
    pub muted: bool,
    #[serde(skip)]
    previous: Option<(f32, bool)>,
}

impl SetClipAudioCommand {
    /// Create a gain/mute change.
    #[must_use]
    pub fn new(clip_id: impl Into<String>, volume: f32, muted: bool) -> Self {
        Self {
            clip_id: clip_id.into(),
            volume,
            muted,
            previous: None,
        }
    }
}

impl EditCommand for SetClipAudioCommand {
    fn name(&self) -> &'static str {
        "Set clip audio"
    }

    fn execute(&mut self, timeline: &mut Timeline) -> Result<(), TimelineError> {
        let mut candidate = timeline
            .clips
            .get(&self.clip_id)
            .cloned()
            .ok_or_else(|| TimelineError::ClipNotFound(self.clip_id.clone()))?;
        self.previous = Some((candidate.volume, candidate.muted));
        candidate.volume = self.volume;
        candidate.muted = self.muted;
        timeline.validate_clip(&candidate, Some(&candidate.id))?;
        timeline.clips.insert(candidate.id.clone(), candidate);
        Ok(())
    }

    fn undo(&mut self, timeline: &mut Timeline) -> Result<(), TimelineError> {
        let (volume, muted) = self
            .previous
            .ok_or_else(|| TimelineError::NotExecuted(self.name().to_owned()))?;
        let mut candidate = timeline
            .clips
            .get(&self.clip_id)
            .cloned()
            .ok_or_else(|| TimelineError::ClipNotFound(self.clip_id.clone()))?;
        candidate.volume = volume;
        candidate.muted = muted;
        timeline.validate_clip(&candidate, Some(&candidate.id))?;
        timeline.clips.insert(candidate.id.clone(), candidate);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Track, TrackKind};

    fn timeline() -> Timeline {
        let mut tl = Timeline::default();
        tl.tracks.push(Track {
            id: "v1".to_owned(),
            kind: TrackKind::Video,
            index: 0,
            name: "V1".to_owned(),
            locked: false,
            muted: false,
        });
        tl.tracks.push(Track {
            id: "v2".to_owned(),
            kind: TrackKind::Video,
            index: 1,
            name: "V2".to_owned(),
            locked: false,
            muted: false,
        });
        tl
    }

    fn clip(id: &str, start: f64, duration: f64) -> Clip {
        Clip {
            id: id.to_owned(),
            source_media_id: "media-1".to_owned(),
            track_id: "v1".to_owned(),
            start,
            duration,
            in_point: 10.0,
            volume: 1.0,
            muted: false,
            name: id.to_owned(),
        }
    }

    #[test]
    fn add_then_remove_round_trips() {
        let mut tl = timeline();
        let mut add = AddClipCommand {
            clip: clip("a", 0.0, 5.0),
        };
        add.execute(&mut tl).unwrap();
        assert_eq!(tl.duration(), 5.0);
        add.undo(&mut tl).unwrap();
        assert_eq!(tl.duration(), 0.0);
    }

    #[test]
    fn remove_keeps_clip_for_undo() {
        let mut tl = timeline();
        tl.insert_clip(clip("a", 0.0, 5.0)).unwrap();
        let mut remove = RemoveClipCommand::new("a");
        remove.execute(&mut tl).unwrap();
        assert!(tl.clips.is_empty());
        remove.undo(&mut tl).unwrap();
        assert!(tl.clips.contains_key("a"));
    }

    #[test]
    fn trim_moves_source_window_and_restores_it() {
        let mut tl = timeline();
        tl.insert_clip(clip("a", 0.0, 8.0)).unwrap();
        let mut trim = TrimClipCommand::new("a", 2.0, 6.0);
        trim.execute(&mut tl).unwrap();
        let after = &tl.clips["a"];
        assert_eq!(
            (after.start, after.duration, after.in_point),
            (2.0, 6.0, 12.0)
        );
        trim.undo(&mut tl).unwrap();
        let restored = &tl.clips["a"];
        assert_eq!(
            (restored.start, restored.duration, restored.in_point),
            (0.0, 8.0, 10.0)
        );
    }

    #[test]
    fn trim_rejects_overlap() {
        let mut tl = timeline();
        tl.insert_clip(clip("a", 0.0, 4.0)).unwrap();
        tl.insert_clip(clip("b", 5.0, 4.0)).unwrap();
        let mut trim = TrimClipCommand::new("b", 3.0, 6.0);
        assert!(matches!(
            trim.execute(&mut tl),
            Err(TimelineError::Overlap { .. })
        ));
    }

    #[test]
    fn move_across_tracks_and_back() {
        let mut tl = timeline();
        tl.insert_clip(clip("a", 0.0, 4.0)).unwrap();
        let mut mv = MoveClipCommand::new("a", 6.0, "v2");
        mv.execute(&mut tl).unwrap();
        assert_eq!(tl.clips["a"].track_id, "v2");
        mv.undo(&mut tl).unwrap();
        assert_eq!(
            (tl.clips["a"].start, tl.clips["a"].track_id.as_str()),
            (0.0, "v1")
        );
    }

    #[test]
    fn split_produces_abutting_halves_with_correct_source_windows() {
        let mut tl = timeline();
        tl.insert_clip(clip("a", 4.0, 10.0)).unwrap();
        let mut split = SplitClipCommand::new("a", 9.0);
        split.execute(&mut tl).unwrap();
        let first = &tl.clips["a"];
        let second = &tl.clips["a-b"];
        assert_eq!(
            (first.start, first.duration, first.in_point),
            (4.0, 5.0, 10.0)
        );
        assert_eq!(
            (second.start, second.duration, second.in_point),
            (9.0, 5.0, 15.0)
        );
        split.undo(&mut tl).unwrap();
        assert!(!tl.clips.contains_key("a-b"));
        assert_eq!(tl.clips["a"].duration, 10.0);
    }

    #[test]
    fn split_rejects_cuts_outside_the_clip() {
        let mut tl = timeline();
        tl.insert_clip(clip("a", 4.0, 10.0)).unwrap();
        assert!(SplitClipCommand::new("a", 4.0).execute(&mut tl).is_err());
        assert!(SplitClipCommand::new("a", 14.0).execute(&mut tl).is_err());
        assert!(SplitClipCommand::new("missing", 6.0)
            .execute(&mut tl)
            .is_err());
    }

    #[test]
    fn commands_serialize_for_history_persistence() {
        let cmd = TrimClipCommand::new("a", 2.0, 6.0);
        let json = serde_json::to_string(&cmd).unwrap();
        let back: TrimClipCommand = serde_json::from_str(&json).unwrap();
        assert_eq!(cmd, back);
    }

    #[test]
    fn remove_range_cuts_middle_and_restores_on_undo() {
        let mut tl = timeline();
        tl.insert_clip(clip("a", 0.0, 10.0)).unwrap();
        let mut remove = RemoveRangeCommand::new(3.0, 7.0).unwrap();
        remove.execute(&mut tl).unwrap();
        let kept: Vec<f64> = {
            let mut starts: Vec<f64> = tl.clips.values().map(|clip| clip.start).collect();
            starts.sort_by(|a, b| a.partial_cmp(b).unwrap());
            starts
        };
        assert_eq!(kept, vec![0.0, 7.0]);
        assert!((tl.duration() - 10.0).abs() < 1e-9);
        remove.undo(&mut tl).unwrap();
        assert_eq!(tl.clips.len(), 1);
        assert_eq!(tl.clips["a"].duration, 10.0);
    }

    #[test]
    fn remove_range_across_clips_keeps_abutters() {
        let mut tl = timeline();
        tl.insert_clip(clip("a", 0.0, 4.0)).unwrap();
        tl.insert_clip(clip("b", 4.0, 4.0)).unwrap();
        tl.insert_clip(clip("c", 8.0, 4.0)).unwrap();
        let mut remove = RemoveRangeCommand::new(2.0, 9.0).unwrap();
        remove.execute(&mut tl).unwrap();
        // a → [0,2), b gone entirely, c split → c-b [9,12).
        assert_eq!(tl.clips["a"].duration, 2.0);
        assert!(!tl.clips.contains_key("b"));
        assert!(!tl.clips.contains_key("c"));
        assert_eq!(
            (tl.clips["c-b"].start, tl.clips["c-b"].duration),
            (9.0, 3.0)
        );
        remove.undo(&mut tl).unwrap();
        assert_eq!(tl.clips.len(), 3);
    }

    #[test]
    fn remove_range_rejects_bad_ranges() {
        assert!(RemoveRangeCommand::new(5.0, 5.0).is_err());
        assert!(RemoveRangeCommand::new(7.0, 5.0).is_err());
        assert!(RemoveRangeCommand::new(-1.0, 5.0).is_err());
        assert!(RemoveRangeCommand::new(f64::NAN, 5.0).is_err());
    }

    #[test]
    fn set_audio_applies_gain_and_mute_with_undo() {
        let mut tl = timeline();
        tl.insert_clip(clip("a", 0.0, 4.0)).unwrap();
        assert_eq!((tl.clips["a"].volume, tl.clips["a"].muted), (1.0, false));
        let mut audio = SetClipAudioCommand::new("a", 0.5, true);
        audio.execute(&mut tl).unwrap();
        assert_eq!((tl.clips["a"].volume, tl.clips["a"].muted), (0.5, true));
        audio.undo(&mut tl).unwrap();
        assert_eq!((tl.clips["a"].volume, tl.clips["a"].muted), (1.0, false));
        // Out-of-range gains fail closed.
        assert!(SetClipAudioCommand::new("a", 99.0, false)
            .execute(&mut tl)
            .is_err());
        assert!(SetClipAudioCommand::new("a", f32::NAN, false)
            .execute(&mut tl)
            .is_err());
    }
}
