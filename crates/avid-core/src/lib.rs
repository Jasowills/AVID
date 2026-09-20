//! `avid-core`: shared kernel types for AVID.
//!
//! Zero domain dependencies — every other crate may depend on this one,
//! but this crate depends on nothing AVID-specific.
//! Phase 3+ will add: `ProjectId`, `ClipId`, `Timestamp`, `AvidError` (`AVID_<DOMAIN>_<NNN>`), events.
//!
//! Status: scaffold only (Phase 0). No logic yet by design.

#![forbid(unsafe_code)]

/// Scaffold marker. Removed when real kernel types land.
pub fn scaffold_marker() -> &'static str {
    "avid-core scaffold"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaffold_is_present() {
        assert_eq!(scaffold_marker(), "avid-core scaffold");
    }
}
