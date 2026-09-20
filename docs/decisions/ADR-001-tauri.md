# ADR-001 — Tauri 2.x desktop shell

- **Status:** accepted (Phase 0.5)
- **Date:** 2026-09-20
- **Context:** Need a local-first, cross-platform desktop shell (macOS/Windows/Linux) for a media-heavy editor with a Rust core. See `docs/research/tauri.md`.
- **Options:** (A) Tauri 2.x — OS webview + Rust, ~3–15 MB, capabilities sandbox, signed updater. (B) Electron — bundled Chromium, identical rendering, ~150–300 MB, full Node (large attack surface). (C) Neutralinojs — thinnest (~2–5 MB) but too thin for job engine/undo/plugins/updater.
- **Decision:** **Tauri 2.11.x** pinned (`tauri 2.11.5`, `@tauri-apps/api 2.x`, plugins `2.x` lockstep, Rust ≥1.77.2 aim 1.85+, React 18 + TS 5 + Zustand 5). FFmpeg as versioned sidecars per triple, never system ffmpeg.
- **Consequences:** Small bundle + Rust media core + strong sandbox fit local-first posture. Accept per-OS webview QA burden (preview risk → custom `stream://` Range protocol, proxy-first, per-OS test matrix) and Rust hiring curve. CI: `tauri-action` matrix; `createUpdaterArtifacts` check.
- **References:** `docs/research/tauri.md`; v2.tauri.app docs.
