//! Typed errors for the timeline engine.
//!
//! Every variant maps to a stable `AVID_TIMELINE_NNN` code so the UI can
//! render humane messages with expandable technical detail (AGENTS §51, §111).

use thiserror::Error;

/// Timeline engine failure modes.
#[derive(Debug, Error, PartialEq)]
pub enum TimelineError {
    /// A timestamp or duration was negative, NaN, infinite, or zero where positive is required.
    #[error("invalid time value: {0}")]
    InvalidTime(String),
    /// No clip with this id exists on the timeline.
    #[error("clip not found: {0}")]
    ClipNotFound(String),
    /// No track with this id exists on the timeline.
    #[error("track not found: {0}")]
    TrackNotFound(String),
    /// The target track is locked against edits.
    #[error("track is locked: {0}")]
    TrackLocked(String),
    /// A clip id is already in use.
    #[error("duplicate clip id: {0}")]
    DuplicateClipId(String),
    /// The edit would overlap another clip on the same track.
    #[error("clip would overlap another clip on track {track_id}: {clip_id}")]
    Overlap { track_id: String, clip_id: String },
    /// Nothing left to undo / redo.
    #[error("nothing to {0}")]
    HistoryEmpty(String),
    /// A command was asked to undo before it ever executed.
    #[error("command has not executed yet: {0}")]
    NotExecuted(String),
}

impl TimelineError {
    /// Stable error code for UI mapping and logs.
    #[must_use]
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidTime(_) => "AVID_TIMELINE_001",
            Self::ClipNotFound(_) => "AVID_TIMELINE_002",
            Self::TrackNotFound(_) => "AVID_TIMELINE_003",
            Self::TrackLocked(_) => "AVID_TIMELINE_004",
            Self::DuplicateClipId(_) => "AVID_TIMELINE_005",
            Self::Overlap { .. } => "AVID_TIMELINE_006",
            Self::HistoryEmpty(_) => "AVID_TIMELINE_007",
            Self::NotExecuted(_) => "AVID_TIMELINE_008",
        }
    }
}
