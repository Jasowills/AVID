//! `avid-media`: FFmpeg abstraction (MediaEngine).
//!
//! ONLY place where raw FFmpeg commands may exist (AGENTS §11):
//! `probe / transcode / proxy / thumbnail / waveform / extractAudio /
//! extractFrame / render / concatenate / composite / burnCaptions`.
//! Status: scaffold only (Phase 0).

#![forbid(unsafe_code)]

/// Scaffold marker. Removed when MediaEngine lands (Phase 2).
pub fn scaffold_marker() -> &'static str {
    "avid-media scaffold"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaffold_is_present() {
        assert_eq!(scaffold_marker(), "avid-media scaffold");
    }
}
