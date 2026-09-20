//! `avid-timeline`: timeline model + command engine + undo/redo.
//!
//! The heart of AVID (AGENTS §13–§15). Non-destructive, command-based:
//! every edit is `execute / undo / redo / serialize`. AI acts only via
//! validated commands. Status: scaffold only (Phase 0).

#![forbid(unsafe_code)]

/// Scaffold marker. Removed when the timeline model lands (Phase 3).
pub fn scaffold_marker() -> &'static str {
    "avid-timeline scaffold"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaffold_is_present() {
        assert_eq!(scaffold_marker(), "avid-timeline scaffold");
    }
}
