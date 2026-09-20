# ADR-006 — Rendering architecture (deterministic render graph)

- **Status:** accepted (Phase 0.5)
- **Date:** 2026-09-20
- **Context:** Timeline must compile to a deterministic render representation so future engines can plug in without rewriting the editor (AGENTS §62–63). Preview must stay fast without full re-renders.
- **Options:** (A) Timeline compiler → render graph (`Source → Trim → Transform → Color → Overlay → Caption → Output`) → single FFmpeg DAG. (B) Direct per-clip FFmpeg calls from UI code (rejected — untestable, non-deterministic). (C) Remotion-as-renderer (rejected for core — heavy, GPL-adjacent FFmpeg binary; allowed selectively for graphics preview).
- **Decision:** **(A)** owned by `avid-render`. Preview uses proxies + cached frames + quality selector (¼/½/Full/Auto) + range-only invalidation. Export presets (YT 1080p/4K, Reel, TikTok, Short, Feed, custom) with estimates. HW accel export-only, software fallback mandatory. Render tests assert codec/duration/resolution/streams (never brittle bytes); frame/perceptual-hash spot checks where practical.
- **Consequences:** Deterministic, testable, engine-swappable. Upfront compiler work; pays off in export reliability and future engine options.
- **References:** AGENTS §62–65, §96; `docs/research/ffmpeg.md`.
