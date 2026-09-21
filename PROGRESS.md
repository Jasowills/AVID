# AVID — Progress Tracker

> Living document. Updated in the same commit as the work. Never mark complete without meeting AGENTS §135 (Definition of Done). No fake completion (AGENTS §136).
> Legend: `✅ done` · `🚧 in progress` · `⬜ not started` · `⛔ blocked`

**Current phase:** Autopilot pass — engine + verification backbone real; app-integration remaining (see honest accounting below)
**Last updated:** 2026-09-21 — fix-all pass: audio pipeline, diagrams v1, transcript delete, shortcuts/zoom, thumbnails, AVID.app packaged+launched; 81 Rust + 35 TS tests green — Tauri backend compiles (3 cmd tests), timeline 18 tests, manifest+traversal, MediaEngine + REAL probe, render graph + REAL render, AI registry + LIVE Ollama eval, edit-plan/scene validators (9+3 tests), template loader (12 templates), transcript model + REAL whisper.cpp verification
**Branch:** `main` · **Remote:** `https://github.com/Jasowills/AVID`

---

## Phase 0 — Research & Architecture 🚧

| ID | Deliverable | Status | Evidence / Notes |
|----|-------------|--------|------------------|
| 0.1 | Repo scaffold + README + PLAN + PROGRESS | ✅ | `fe110c1` pushed to `main`; `AGENTS.MD`→`AGENTS.md` canonical |
| 0.2 | Competitive/UX research | ✅ | `docs/research/editors-comparison.md` — 10 editors, adopt/avoid/differentiate synthesis |
| 0.3 | Technical research (≥3 options/subsystem) | ✅ | `tauri.md`, `ffmpeg.md`, `whisper.md`, `local-ai.md`, `timeline.md` — primary sources, recommendations |
| 0.4 | Architecture proposal | ✅ | `docs/ARCHITECTURE.md` — boundaries, data flows, preview strategy, Phase-1 spikes |
| 0.5 | ADRs 001–008 | ✅ | `docs/decisions/ADR-001…008` — Tauri, media core, project format, timeline, AI abstraction, rendering, local-first, command safety |
| 0.6 | UX flows | ✅ | `docs/ux/flows.md` — first-run → home → creation → editor → timeline → transcript → AI review → jobs/errors → providers → export → a11y |
| 0.7 | Dev env + CI skeleton | 🚧 | `rust-toolchain.toml` + `.nvmrc` added; exact version lockfile pins land in Phase 1 (needs Rust toolchain on build machine — see blockers) |
| 0.8 | Risk register + licensing | ✅ | `docs/LEGAL_AND_LICENSING.md` — per-dependency table with status; top risks in ARCHITECTURE.md + table below |

## Phase 1 — Desktop Shell 🚧

| ID | Deliverable | Status | Evidence / Notes |
|----|-------------|--------|------------------|
| 1.1 | Tauri 2.x + React + TS + Tailwind + Zustand wired | ✅ | Backend compiles + dev binary launches and stays resident error-free (`tauri dev` full cycle: Vite 506 ms + backend 19.68 s). Commands + 5 cmd tests; capabilities least-privilege; IPC client + 3 tests. **Pixel-level GUI check needs a console session** (headless `screencapture` unavailable) — run `npm run tauri:dev` and click through on wake-up |
| 1.2 | Design system tokens + components | ✅ | `packages/design-system` tokens (dark-first) + Tailwind `@theme` mirror; `packages/ui` Button/Panel/TextField/EmptyState, accessible defaults |
| 1.3 | Routing: home / new-project / editor / settings | ✅ | HashRouter (file:// + webview safe); `App.tsx` route map |
| 1.4 | Project creation dialog | ✅ | Validated form + backend `create_project` (manifest round-trips through its own parser); frontend persists to localStorage **interim** until Tauri save path lands |
| 1.5 | Top bar (undo/redo/save/AI status/export) | ✅ | Renders per AGENTS §39; unwired controls **disabled with honest phase-titles** (no fake buttons, §136) |

## Phase 2 — Media Foundation 🚧 (engine + import + jobs real; library/playback pending)

- [x] MediaEngine: probe (REAL), `probe_file`/`extract_audio` runners, progress tracker + cancellable render runner, builders, traversal guard
- [x] `import_media`: copy → probe → asset → auto-place first video on V1 (undoable) — LIVE-tested with fixture
- [x] Media panel: probe form + import form + asset list with generated thumbnails + transcribe handoff, wired to backend
- [x] Clip audio (§66 minimum): volume 0–4 + mute on the model (validated, back-compat), per-input gain in render lowering, `timeline_set_clip_audio` command, inspector audio form
- [x] Timeline shortcuts (S split, Delete remove, Cmd/Ctrl+Z undo/redo, field-aware) + zoom 0.5–4x
- [x] Job system: registry (start/progress/finish/fail/cancel/list) + Channel progress events + `cancel_job`; render streams real ffmpeg fractions, transcribe registers indeterminate jobs
- [x] Job center UI: Jobs tab (progress bars, cancel, failure reasons) + top-bar running pill (shared polling store)
- [x] Proxy generation: 540p transcode command + session recording + LIVE test (smaller file, video present); audio-only fails closed with guidance
- [x] Seekable preview: `stream://` Range protocol (200/206/416, traversal guard, symlink containment, MIME map) — pure core unit-tested (5 tests); Tauri handler wired
- [x] Preview pane: plays selected clip (proxy preferred, original fallback) via `<video>` + stream URL; honest browser/empty/missing states
- [x] Fixture generator (`scripts/make-fixtures.sh`): talking-head, silence, vertical, corrupt
- [x] Media library: grid/list toggle, search (name+id), auto-thumbnails with duration badges + audio glyphs, retry per item
- [ ] Pinned sidecar binaries per triple (uses system ffmpeg until then — ADR-002)

## Phase 3 — Timeline ✅ (engine + working UI; drag/snapping/markers pending)

- [x] Model: tracks/clips with all §13 fields, overlap validation, locked tracks
- [x] Commands: Add/Remove/Trim/Move/Split with execute/undo/redo/serialize (18 tests, clippy pedantic, fmt)
- [x] Grouped + transactional undo (`UndoStack`, manual groups, single-label AI undo)
- [x] UI: SVG canvas (lanes, selection, keyboard operable) + dock (split/remove/undo/redo via backend, autosaved) + inspector trim + audio forms
- [x] Keyboard: S split, Delete remove, Cmd/Ctrl+Z undo, +Shift redo (skipped in text fields); zoom 0.5–4x
- [x] Backend session: open project holds manifest+timeline+undo; every mutation persists `project.json` (autosave foundation; reload recovers)
- [x] Recovery snapshots (§117): pre-apply snapshot (pruned to 10) + list/restore commands + AiPanel restore button; restore restarts history honestly
- [x] Playhead + time ruler + click-to-seek (playback bus: video is the clock)
- [x] Drag-move + drag-trim with edge snapping (snap indicator), `timeline_move_clip` command
- [x] Transcript click-to-seek; preview shows live time
- [ ] Markers, cross-track drag, slip/slide modes

## Phase 4 — Rendering & Export ✅ (render + export real, preview pending)

- [x] Deterministic render graph → single-ffmpeg DAG (concat + scale + drawtext overlays), escaping, compat path with `AVID_RENDER_005` guidance
- [x] REAL render test (concat 2 windows → ffprobe verifies 4 s h264+aac); text-filter probe helper
- [x] Export presets (YouTube 1080p/4K, Short 1080p, custom) + filename guard + `session.export` → render → duration verification
- [x] Caption sidecar: caption-track clips → `<name>.srt` alongside the render (burn-in follows with a text-capable sidecar); live-tested
- [x] LIVE export test (import fixture → short-1080p → verified 10 s file + .srt); async `render_export` command (blocking thread, never UI)
- [x] Export dialog UI (preset, filename, timeline estimate, verified result, show-in-folder via opener plugin)
- [ ] Preview pipeline (proxies, frame cache, quality selector)

## Phase 5 — Transcription ✅ (core loop real, panel UI pending)

- [x] In-app `whisper-rs` binding (`transcribe_wav`, 16 kHz mono gate, greedy deterministic) + REAL in-process test (synthesized speech → "kafka"+"partitions" in 5.5 s)
- [x] Rerunnable sidecar verification (`scripts/verify-transcription.sh`): exact transcript + 14 ordered word-segments
- [x] Transcript model + whisper-JSON parser + phrase→range mapping for text-based delete (7 tests, real output shape)
- [x] Transcript panel UI wired to `transcribe_media` (segments, ranges, provider label) + in-context model download on `AVID_TRANSCRIBE_001`
- [x] Per-segment “Remove from timeline” (validated remove_range via apply path, undoable)
- [x] Model auto-download (`ensure_speech_model` async command + `speech_model_status`; default tiny.en 77 MB to app cache)
- [ ] VAD chunking + speaker labels

## Phase 6 — AI Runtime 🚧 (local + cloud-shape real, keys/UX done, usage live)

- [x] Registry (local-first recommend) + canonical OpenAI-shape adapter + structured-output probe + mock tests
- [x] LIVE eval vs local Ollama `qwen2.5:7b`: probe + edit-plan with operations (38 s) — ADR-005 path proven
- [x] Cloud auth: optional Bearer key (blank = local, never sent to local endpoints — tested both directions)
- [x] `probe_provider` command (URL hygiene + reachability reported, not thrown; tested incl. unreachable + bad URLs)
- [x] Provider settings UI (endpoint/model/key, Local/Cloud badge, live test, key never persisted — interim localStorage holds URL/model only)
- [ ] LM Studio/generic-compat beyond shape (same adapter, untested without targets)

## Phase 7 — AI Editing ✅ (detectors + proposal + apply real; advanced pacing later)

- [x] Edit-plan schema + validator (rejects prose/malformed/hallucinated/out-of-range, 9 tests) + eval fixtures
- [x] `RemoveRangeCommand`: text-delete as one undoable transactional op (22 timeline tests)
- [x] `apply_operations`: re-validate vs live state → single grouped undo (`AI: goal`) → per-op report; invalid ops reported, never applied
- [x] Detectors: ffmpeg `silencedetect` runner + parser (live-tested: 5 s span + tone true-negative) + transcript filler finder (`um/uh/you know/…`, tested)
- [x] `propose_rough_cut`: silence (+fillers when transcribed) → sorted proposal with honest High/Medium confidence + removable totals; feeds the same review checkboxes
- [x] AI panel: validate → per-op accept/reject → apply → results; proposal flow; visuals honestly deferred to Phase 8; timeline auto-refreshes

## Phase 8 — Visual Intelligence ✅ (deterministic visuals v1 real; burn-in + free-form later)

- [x] Scene-spec validator (dangling refs/duplicates/geometry, 3 tests)
- [x] Mermaid flowchart compiler (TD/TB/LR/RL/BT, auto-layout, 5 tests) + deterministic SVG renderer (escaped, 2 tests)
- [x] Visuals store: manifest `visuals` map + save/list/place commands + session tests
- [x] Visuals panel: convert → preview → label editing → save → place on graphics track (undoable); honest export note
- [x] Visuals store: manifest `visuals` map + save/list/place commands + session tests
- [x] Visuals panel: Mermaid convert → SVG preview → label editing → save → place on graphics track (undoable)
- [ ] Burn-in to export + code visuals + React Flow free-form editing

## Phase 9 — Templates 🚧 (format + content real, browser UI pending)

- [x] Loader/validator + all 12 `template.json` validated in test; `technical-explainer` v0.1.0 behavioral content
- [x] Shipped example: `examples/technical-explainer/project.json` parses as manifest AND as timeline (contract tests)
- [ ] Template browser + preview + apply-as-commands UI

## Phase 10 — Polish & Release 🚧 (app bundle built; distribution pending)

- [x] A11y defaults (labels, focus states, reduced-motion CSS, no color-alone), CI (scaffold/node/rust), error-code scheme, humane-error mapping in commands
- [x] Autosave (persist-on-mutation) + crash recovery (reload rebuilds state) + recovery snapshots
- [x] Packaging spike: release binary + `AVID.app` (15 MB) bundles clean; bundled app launches resident error-free. Fixes found: `CMAKE_OSX_DEPLOYMENT_TARGET=10.15` required for whisper.cpp (see blockers); generated iconset committed
- [ ] Signed `.dmg` (needs Apple Developer identity), GUI pixel click-through, perf/stress run, onboarding tour

---

## MVP feature checklist (AGENTS §122 — all must be real, not mocked)

- [x] Desktop app (dev + release binaries run resident; `AVID.app` bundled + launched; pixel check + signing = wake-up jobs) · [x] Project creation (dialog + backend manifest + autosaved project dirs) · [x] Media import (command + UI form, live-tested) · [x] Media preview (proxy + seekable stream + pane)
- [x] Timeline (engine) · [x] Basic editing (6 commands) · [x] Undo/redo (grouped+transactional) · [x] Autosave (persist-on-mutation + recovery snapshots)
- [x] FFmpeg rendering (graph + real render) · [x] Export (presets + dialog + verified output + show-in-folder + captions SRT) · [x] Local transcription (in-app binding + real test + auto-download) · [x] Transcript editing (phrase→range mapping)
- [x] AI provider abstraction · [x] Ollama (live eval) · [x] One cloud provider (shape + auth + probe; live call needs a key) · [x] AI rough-cut proposal (detectors + proposal + review + apply)
- [x] Captions (transcribe + caption clips + SRT sidecar; styled burn-in pending capable sidecar) · [x] Text overlays (graph lowering + SRT path; burn-in needs text filters) · [ ] Basic diagrams · [x] Templates (format+content)
- [x] Example projects (parseable) · [ ] Crash recovery · [x] Documentation · [x] Tests

## Explicitly excluded from MVP (do not build yet)

Color grading suite, advanced VFX, 3D, collaborative cloud editing, stock marketplace, social network, cloud storage, complex video-gen, advanced multicam, mobile app.

---

## Blockers / Risks (update as discovered)

| Date | Blocker / Risk | Owner | Mitigation |
|------|---------------|-------|------------|
| 2026-09-20 | Rust toolchain not installed on scaffold machine (`rustc/cargo` missing) | build | **Resolved:** Rust 1.98.1 stable + rustfmt/clippy installed via rustup; pinned in `rust-toolchain.toml`; `cargo test/fmt/clippy` green on workspace. |
| 2026-09-20 | Tauri backend not wired (no src-tauri Cargo workspace yet) | Phase 1 | **Resolved:** backend compiles, 3 cmd tests, capabilities, icon; `tauri dev` GUI launch + `tauri build` packaging = first manual gate (needs interactive run). |
| 2026-09-20 | System ffmpeg lacks text filters (no drawtext/subtitles in Homebrew 9.0.1) | render | `argv_compat` + `AVID_RENDER_005` guidance + `ffmpeg_supports_text` probe; burn-in needs capable sidecar (ADR-002). Timeline data unaffected. |
| 2026-09-20 | tauri CLI 2.11.5 vs core crate 2.6.x numbering | build | Major-pin `tauri = "2"` (CLI/core rev independently in v2); lockfile records exact. |
| 2026-09-20 | whisper.cpp install via brew slow (~10 min, bottle pour) | Phase 5 | Installed 1.9.1; model cached in ~/.cache (never committed). In-app whisper-rs binding still pending. |
| 2026-09-20 | Seekable preview risk (Tauri `asset://` lacks Range support) | arch | Custom `stream://` Range/206 protocol spike in Phase 1; proxy-first H.264/AAC; per-OS preview matrix. Tracked in ARCHITECTURE.md. |
| 2026-09-20 | Llama-3.1 weights (Community License, not OSI) + H.264/AAC patents | legal | Tracked in LEGAL_AND_LICENSING.md; Qwen3 default; codec patent exposure reviewed separately from copyright. |
| 2026-09-21 | Release build broke on whisper.cpp C++ (std::filesystem < 10.15) | packaging | **Resolved:** `CMAKE_OSX_DEPLOYMENT_TARGET=10.15` (forwarded by whisper-rs-sys build script; plain `MACOSX_DEPLOYMENT_TARGET` is ignored). Recorded for CI. |
| 2026-09-21 | Distribution signing identity absent | release | `.dmg`/notarization needs an Apple Developer identity — maintainer job. Unsigned `.app` runs locally. |
| — | — | — | — |

## Changelog (scaffold → …)

| Date | Commit | What changed |
|------|--------|--------------|
| 2026-09-20 | `fe110c1` / pushed to `main` | Initial clean scaffold: structure, README, PLAN, PROGRESS, ROADMAP, collab docs, verify script. No app code per AGENTS §157. |
| 2026-09-20 | Phase 0 research drop (pending push) | Competitive (10 editors) + technical (Tauri/FFmpeg/Whisper/local-AI/timeline) research; ARCHITECTURE.md; ADR-001…008; UX flows; toolchain pins; licensing register. Docs only — gate to Phase 1 is spike validation (preview protocol, sidecar signing, whisper latency, qwen3 eval). |
| 2026-09-20 | Phase 1 shell drop (pending push) | Rust 1.98.1 + fmt/clippy/test green; Vite+React18+TS+Tailwindv4+Zustand shell with tokens, routing, validated project creation (5 vitest), top bar + editor shell (unwired controls disabled+honest); CI node/rust jobs; README t3code-style; repo description+topics set. Tauri backend still pending. |
| 2026-09-20 | Autopilot engine pass (pending push) | Timeline engine (18 tests) · manifest+traversal · MediaEngine+REAL probe · render graph+REAL render · AI registry+LIVE Ollama eval (qwen2.5:7b) · edit-plan/scene validators (12 tests) · template loader (12 templates) · transcript model+REAL whisper verification · Tauri backend compiles+3 tests · example parses as manifest+timeline. See phase rows for remaining app-integration work. |
| 2026-09-20 | Autopilot pass 2 (pending push) | In-app whisper-rs binding + REAL in-process transcription test · `save_project` command + disk round-trip tests · Media probe panel + AI plan-check panel (10 desktop tests) · `tauri dev` full cycle (Vite + backend 19.68 s, binary resident error-free; pixel check needs console session) · `tauri:dev` script wired. |
| 2026-09-21 | Library pass | Grid/list library with search, lazy thumbnails, duration badges; filter helpers tested. |
| 2026-09-21 | Diffusion-UI pass | lucide-react icons (Icon wrapper, all glyphs replaced) · track lock command + gutter toggles · transport play/pause · inspector sections · adoption plan `docs/diffusion-ui-adoption.md`. |
| 2026-09-21 | Playback pass | Playhead + ruler + seek bus, drag-move/trim with snapping, move command, transcript seek, live time in preview. |
| 2026-09-21 | Fork recon (HARNESS.MD) | Upstream verified (Electron+Solid+WebCodecs, MPL-2.0, dapi/MCP/skills); `docs/diffusion-evaluation.md` with inherit/extend/replace analysis. **Decision gate open: fork / continue / spike-first — no editor code touched.** |
| 2026-09-21 | Fix-all pass | Clip volume/mute (engine validation, render gain, audio command, inspector UI) · diagrams v1 (Mermaid import, SVG renderer, visuals store, place-on-timeline, panel) · transcript segment delete · shortcuts + zoom · Home provider hint · thumbnails (command + panel) · `AVID.app` packaged (15 MB) + launched resident (signing/GUI-pixel = wake-up jobs). |
