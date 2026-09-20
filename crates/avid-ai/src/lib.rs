//! `avid-ai`: AI runtime — capability router → provider registry → adapters.
//!
//! Provider-specific logic lives ONLY in `adapters/` (AGENTS §16–§17).
//! Capabilities: text, structured-output, vision, audio, transcription,
//! image-gen, video-gen, embeddings, tool-calling. Status: scaffold (Phase 0).

#![forbid(unsafe_code)]

/// Scaffold marker. Removed when the provider registry lands (Phase 6).
pub fn scaffold_marker() -> &'static str {
    "avid-ai scaffold"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaffold_is_present() {
        assert_eq!(scaffold_marker(), "avid-ai scaffold");
    }
}
