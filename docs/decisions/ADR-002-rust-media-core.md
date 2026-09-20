# ADR-002 — Rust media core + FFmpeg sidecar strategy

- **Status:** accepted (Phase 0.5)
- **Date:** 2026-09-20
- **Context:** Need probe/proxy/thumbnail/waveform/extract/render behind one abstraction, without scattering FFmpeg strings or taking GPL copyleft accidentally. See `docs/research/ffmpeg.md`.
- **Options:** (A) Bundled sidecar ffmpeg+ffprobe, CLI-per-op via `tokio::process` — safest license, trivial CI, crash-isolated; weak for 60 fps scrub. (B) Linked libav* (`ffmpeg-next`/`rsmpeg`) — zero-copy preview control; build hell per OS, segfault = app crash, `ffmpeg-next` maintenance-only, GPL-infection risk. (C) Persistent streaming sidecar — middle ground, pipe complexity. (D) GStreamer — different model, plugin-license maze; rejected for MVP.
- **Decision:** **(A) for MVP** behind the `MediaEngine` trait in `avid-media` (the ONLY place raw FFmpeg strings may exist). Add (C) only if profiling demands. Defer (B) to post-MVP preview engine.
- **Consequences:** Stay LGPL (no `--enable-gpl/--enable-nonfree` builds; no `libx264` in distributed LGPL build or go GPL-compliant with checklist). ~25–80 MB per arch sidecar cost accepted. HW accel is export/proxy-only with silent software fallback; log encoder per job. Fixtures generated with ffmpeg itself, no binaries in git.
- **References:** `docs/research/ffmpeg.md`; ffmpeg.org/legal.html.
