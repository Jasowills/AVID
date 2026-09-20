# ADR-004 — Timeline model (custom, command-based)

- **Status:** accepted (Phase 0.5)
- **Date:** 2026-09-20
- **Context:** The timeline is the heart of AVID, not a UI component (AGENTS §13). Needs tracks/clips/markers, pro interactions, and AI-operability. See `docs/research/timeline.md` + `editors-comparison.md`.
- **Options:** (A) Custom Rust model + custom canvas UI + command engine. (B) Adopt an OSS web timeline library (none is a full NLE; DOM collapses past ~200 nodes). (C) Descript-style doc-only model (fast for podcasts, weak for multicam/pro work — rejected as sole model).
- **Decision:** **(A).** Every item: id/source/start/duration/in-out/track/transform/opacity/audio/effects/metadata. Interactions: zoom/scroll/snap, split/trim/move/delete, ripple, markers, multi-select, standard shortcuts (JKL/I/O/S). Every edit is a command (`AddClip/RemoveClip/Trim/Move/Split/AddText/AddGraphic/AddDiagram/…`) with execute/undo/redo/serialize. AI ops group into one undo entry. Reuse only: wavesurfer.js peaks for waveforms, native `<video>` for playback.
- **Consequences:** Upfront cost (hit-testing, drag, a11y roles) buys 60 fps on hundred-clip projects, deterministic math, and AI compatibility. Adopt FCP-style gap-closing + Resolve-style trim modes (research synthesis). Property tests for timeline math in Phase 3.
- **References:** `docs/research/timeline.md`, `docs/research/editors-comparison.md`; AGENTS §13–15, §45–46, §115.
