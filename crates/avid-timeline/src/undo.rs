//! Undo/redo stack with grouped (transactional) entries (AGENTS §115–116).
//!
//! AI operations execute as a group: one undo step reverts the whole
//! operation, never 37 individual ones. Groups are also the transactional
//! unit — a failed group rolls back the commands that already ran.

use crate::commands::EditCommand;
use crate::error::TimelineError;
use crate::model::Timeline;

/// A labeled set of commands that undo/redo as one step.
#[derive(Debug)]
pub struct CommandGroup {
    /// Human label, e.g. `"AI: Create Technical Explainer"`.
    pub label: String,
    commands: Vec<Box<dyn EditCommand>>,
}

impl CommandGroup {
    /// Execute every command in order; on failure, roll back the ones
    /// that already ran (in reverse) and return the original error.
    fn execute_all(&mut self, timeline: &mut Timeline) -> Result<(), TimelineError> {
        let mut failed: Option<(usize, TimelineError)> = None;
        for (index, command) in self.commands.iter_mut().enumerate() {
            if let Err(error) = command.execute(timeline) {
                failed = Some((index, error));
                break;
            }
        }
        if let Some((ran, error)) = failed {
            for done in self.commands[..ran].iter_mut().rev() {
                let _ = done.undo(timeline);
            }
            return Err(error);
        }
        Ok(())
    }

    fn undo_all(&mut self, timeline: &mut Timeline) -> Result<(), TimelineError> {
        for command in self.commands.iter_mut().rev() {
            command.undo(timeline)?;
        }
        Ok(())
    }
}

/// Undo/redo history over a timeline.
#[derive(Debug, Default)]
pub struct UndoStack {
    undo: Vec<CommandGroup>,
    redo: Vec<CommandGroup>,
    open_group: Option<CommandGroup>,
}

impl UndoStack {
    /// Run a single command as its own undo step, clearing the redo history.
    pub fn execute(
        &mut self,
        timeline: &mut Timeline,
        label: impl Into<String>,
        command: Box<dyn EditCommand>,
    ) -> Result<(), TimelineError> {
        self.execute_group(timeline, label, vec![command])
    }

    /// Run several commands as one undo step (transactional: all or nothing).
    pub fn execute_group(
        &mut self,
        timeline: &mut Timeline,
        label: impl Into<String>,
        commands: Vec<Box<dyn EditCommand>>,
    ) -> Result<(), TimelineError> {
        let mut group = CommandGroup {
            label: label.into(),
            commands,
        };
        group.execute_all(timeline)?;
        self.redo.clear();
        if let Some(open) = self.open_group.as_mut() {
            open.commands.push(Box::new(ExecutedGroup(group)));
        } else {
            self.undo.push(group);
        }
        Ok(())
    }

    /// Open a manual group; commands executed until [`UndoStack::close_group`]
    /// collapse into one undo step. Groups do not nest — opening twice is an error.
    pub fn begin_group(&mut self, label: impl Into<String>) -> Result<(), TimelineError> {
        if self.open_group.is_some() {
            return Err(TimelineError::InvalidTime(
                "a command group is already open".to_owned(),
            ));
        }
        self.open_group = Some(CommandGroup {
            label: label.into(),
            commands: Vec::new(),
        });
        Ok(())
    }

    /// Close the open group, pushing it as one undo step (even if empty).
    pub fn close_group(&mut self) -> Result<(), TimelineError> {
        let group = self
            .open_group
            .take()
            .ok_or_else(|| TimelineError::HistoryEmpty("close".to_owned()))?;
        self.redo.clear();
        self.undo.push(group);
        Ok(())
    }

    /// Undo the most recent step.
    pub fn undo(&mut self, timeline: &mut Timeline) -> Result<String, TimelineError> {
        if self.open_group.is_some() {
            return Err(TimelineError::InvalidTime(
                "cannot undo with an open group".to_owned(),
            ));
        }
        let mut group = self
            .undo
            .pop()
            .ok_or_else(|| TimelineError::HistoryEmpty("undo".to_owned()))?;
        group.undo_all(timeline)?;
        let label = group.label.clone();
        self.redo.push(group);
        Ok(label)
    }

    /// Redo the most recently undone step.
    pub fn redo(&mut self, timeline: &mut Timeline) -> Result<String, TimelineError> {
        if self.open_group.is_some() {
            return Err(TimelineError::InvalidTime(
                "cannot redo with an open group".to_owned(),
            ));
        }
        let mut group = self
            .redo
            .pop()
            .ok_or_else(|| TimelineError::HistoryEmpty("redo".to_owned()))?;
        group.execute_all(timeline)?;
        let label = group.label.clone();
        self.undo.push(group);
        Ok(label)
    }

    /// Number of undoable steps (for UI state).
    #[must_use]
    pub fn undo_depth(&self) -> usize {
        self.undo.len()
    }

    /// Number of redoable steps (for UI state).
    #[must_use]
    pub fn redo_depth(&self) -> usize {
        self.redo.len()
    }
}

/// Wrapper that nests an already-executed group inside a manual group.
///
/// The inner commands ran once via `execute_group`; this wrapper replays
/// them on redo and reverts them on undo so the outer group stays a single
/// flattened, repeatable step.
#[derive(Debug)]
struct ExecutedGroup(CommandGroup);

impl EditCommand for ExecutedGroup {
    fn name(&self) -> &'static str {
        "Grouped commands"
    }

    fn execute(&mut self, timeline: &mut Timeline) -> Result<(), TimelineError> {
        self.0.execute_all(timeline)
    }

    fn undo(&mut self, timeline: &mut Timeline) -> Result<(), TimelineError> {
        self.0.undo_all(timeline)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::{AddClipCommand, RemoveClipCommand};
    use crate::model::{Clip, Track, TrackKind};

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
        tl
    }

    fn clip(id: &str, start: f64) -> Clip {
        Clip {
            id: id.to_owned(),
            source_media_id: "media-1".to_owned(),
            track_id: "v1".to_owned(),
            start,
            duration: 4.0,
            in_point: 0.0,
            name: id.to_owned(),
        }
    }

    #[test]
    fn undo_redo_single_command() {
        let mut tl = timeline();
        let mut stack = UndoStack::default();
        stack
            .execute(
                &mut tl,
                "Add a",
                Box::new(AddClipCommand {
                    clip: clip("a", 0.0),
                }),
            )
            .unwrap();
        assert_eq!(tl.clips.len(), 1);
        assert_eq!(stack.undo(&mut tl).unwrap(), "Add a");
        assert!(tl.clips.is_empty());
        assert_eq!(stack.redo(&mut tl).unwrap(), "Add a");
        assert_eq!(tl.clips.len(), 1);
    }

    #[test]
    fn group_undoes_as_one_labeled_step() {
        let mut tl = timeline();
        let mut stack = UndoStack::default();
        stack
            .execute_group(
                &mut tl,
                "AI: Create Technical Explainer",
                vec![
                    Box::new(AddClipCommand {
                        clip: clip("a", 0.0),
                    }),
                    Box::new(AddClipCommand {
                        clip: clip("b", 5.0),
                    }),
                ],
            )
            .unwrap();
        assert_eq!(stack.undo_depth(), 1);
        assert_eq!(
            stack.undo(&mut tl).unwrap(),
            "AI: Create Technical Explainer"
        );
        assert!(tl.clips.is_empty());
    }

    #[test]
    fn failed_group_rolls_back_and_pushes_nothing() {
        let mut tl = timeline();
        let mut stack = UndoStack::default();
        let result = stack.execute_group(
            &mut tl,
            "bad group",
            vec![
                Box::new(AddClipCommand {
                    clip: clip("a", 0.0),
                }),
                Box::new(RemoveClipCommand::new("missing")),
            ],
        );
        assert!(result.is_err());
        assert!(tl.clips.is_empty());
        assert_eq!(stack.undo_depth(), 0);
    }

    #[test]
    fn new_execute_clears_redo() {
        let mut tl = timeline();
        let mut stack = UndoStack::default();
        stack
            .execute(
                &mut tl,
                "Add a",
                Box::new(AddClipCommand {
                    clip: clip("a", 0.0),
                }),
            )
            .unwrap();
        stack.undo(&mut tl).unwrap();
        assert_eq!(stack.redo_depth(), 1);
        stack
            .execute(
                &mut tl,
                "Add b",
                Box::new(AddClipCommand {
                    clip: clip("b", 5.0),
                }),
            )
            .unwrap();
        assert_eq!(stack.redo_depth(), 0);
    }

    #[test]
    fn manual_group_collapses_steps() {
        let mut tl = timeline();
        let mut stack = UndoStack::default();
        stack.begin_group("manual").unwrap();
        stack
            .execute(
                &mut tl,
                "Add a",
                Box::new(AddClipCommand {
                    clip: clip("a", 0.0),
                }),
            )
            .unwrap();
        stack
            .execute(
                &mut tl,
                "Add b",
                Box::new(AddClipCommand {
                    clip: clip("b", 5.0),
                }),
            )
            .unwrap();
        stack.close_group().unwrap();
        assert_eq!(stack.undo_depth(), 1);
        stack.undo(&mut tl).unwrap();
        assert!(tl.clips.is_empty());
        assert_eq!(stack.redo(&mut tl).unwrap(), "manual");
        assert_eq!(tl.clips.len(), 2);
    }
}
