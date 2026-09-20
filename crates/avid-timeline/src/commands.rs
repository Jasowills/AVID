//! Command-based editing engine (AGENTS §15).
//!
//! Every edit — human or AI — is a command object with `execute / undo` and
//! serializable data. AI output is translated into these commands only after
//! schema validation (ADR-008); nothing else may mutate the timeline.

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::error::TimelineError;
use crate::model::{Clip, Seconds, Timeline};

/// A single reversible edit.
pub trait EditCommand: Debug {
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
}
