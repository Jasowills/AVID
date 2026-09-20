//! `avid-render`: timeline compiler → render graph → FFmpeg.
//!
//! Deterministic render representation (AGENTS §62–§63). Future engines
//! plug in behind the graph abstraction. Status: scaffold (Phase 0).

#![forbid(unsafe_code)]

/// Scaffold marker. Removed when the render graph lands (Phase 4).
pub fn scaffold_marker() -> &'static str {
    "avid-render scaffold"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaffold_is_present() {
        assert_eq!(scaffold_marker(), "avid-render scaffold");
    }
}
