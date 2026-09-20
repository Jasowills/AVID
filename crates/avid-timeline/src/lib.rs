//! `avid-timeline`: timeline model + command engine + undo/redo.
//!
//! The heart of AVID (AGENTS §13–§15). Non-destructive and command-based:
//! every edit is an [`commands::EditCommand`] with execute/undo, grouped
//! into labeled [`undo::UndoStack`] steps. AI acts only via validated
//! commands (ADR-008) — nothing else may mutate the timeline.

#![forbid(unsafe_code)]

pub mod commands;
pub mod error;
pub mod model;
pub mod undo;

pub use commands::{
    AddClipCommand, EditCommand, MoveClipCommand, RemoveClipCommand, SplitClipCommand,
    TrimClipCommand,
};
pub use error::TimelineError;
pub use model::{Clip, Seconds, Timeline, Track, TrackKind};
pub use undo::UndoStack;
