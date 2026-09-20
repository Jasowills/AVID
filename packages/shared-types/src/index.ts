/**
 * @avid/shared-types — scaffold only (Phase 0).
 * Tauri IPC contracts live here from Phase 1. Rust structs in
 * `crates/avid-core` must mirror these types; drift is a CI failure.
 * No logic yet by design.
 */

export const SCAFFOLD = "@avid/shared-types scaffold" as const;

export type ScaffoldMarker = typeof SCAFFOLD;
