//! `avid-project`: versioned project format, migrations, snapshots.
//!
//! Owns `project.json` schema, relative-path media resolution, and
//! forward migrations. Never silently breaks old projects (AGENTS §12).
//! Status: scaffold only (Phase 0).

#![forbid(unsafe_code)]

/// Scaffold marker. Removed when the project manifest lands (Phase 1/3).
pub fn scaffold_marker() -> &'static str {
    "avid-project scaffold"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaffold_is_present() {
        assert_eq!(scaffold_marker(), "avid-project scaffold");
    }
}
