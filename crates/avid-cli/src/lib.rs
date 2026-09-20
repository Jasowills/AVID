//! `avid-cli`: future command-line interface.
//!
//! Explicitly post-core-engine (AGENTS §112): `avid open/render/transcribe/export`.
//! Status: scaffold only — do not build until the engine is stable.

#![forbid(unsafe_code)]

/// Scaffold marker.
pub fn scaffold_marker() -> &'static str {
    "avid-cli scaffold"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaffold_is_present() {
        assert_eq!(scaffold_marker(), "avid-cli scaffold");
    }
}
