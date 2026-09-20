# AVID — Progress Tracker

> Living document. Updated in the same commit as the work. Never mark complete without meeting AGENTS §135 (Definition of Done). No fake completion (AGENTS §136).
> Legend: `✅ done` · `🚧 in progress` · `⬜ not started` · `⛔ blocked`

**Current phase:** Phase 1 — Desktop Shell (shell wired, Tauri backend pending)
**Last updated:** 2026-09-20 — Phase 1 frontend shell: Vite+React+TS+Tailwind+Zustand, tokens, routing, project creation, top bar, editor shell (typecheck/build/5 tests green; Rust toolchain installed, workspace green)
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
| 1.1 | Tauri 2.x + React + TS + Tailwind + Zustand wired | 🚧 | React 18 + TS 5 + Tailwind v4 + Zustand 5 + Vite 6.4.3 + router 7 wired; `dev`/`build`/`typecheck`/`test` green. **Tauri backend NOT wired** — `tauri:dev` exits 1 by design until src-tauri Cargo workspace + sidecars exist |
| 1.2 | Design system tokens + components | ✅ | `packages/design-system` tokens (dark-first) + Tailwind `@theme` mirror; `packages/ui` Button/Panel/TextField/EmptyState, accessible defaults |
| 1.3 | Routing: home / new-project / editor / settings | ✅ | HashRouter (file:// + webview safe); `App.tsx` route map |
| 1.4 | Project creation dialog | ✅ | Validated form (name/canvas/fps/resolution), pure validation fns under `vitest` (5 tests); persists to localStorage **interim** — `ProjectConfig` shape is manifest-compatible for the ADR-003 migration |
| 1.5 | Top bar (undo/redo/save/AI status/export) | ✅ | Renders per AGENTS §39; undo/redo/export/preview-quality render **disabled with honest phase-titles** (no fake buttons, §136) |

## Phase 2 — Media Foundation ⬜

- [ ] MediaEngine abstraction (probe/transcode/proxy/thumbnail/waveform/extractAudio/extractFrame)
- [ ] Import + metadata + library (grid/list, search)
- [ ] Playback via proxies + proxy offer flow
- [ ] Background jobs + job center (cancel/retry/errors)
- [ ] Humane error UX (no raw `subprocess exited 1`)

## Phase 3 — Timeline ⬜

- [ ] Model: tracks/clips/markers, all fields per AGENTS §13
- [ ] Interactions: zoom/scroll/snap/split/trim/move/delete/ripple/markers/multi-select/shortcuts
- [ ] Commands: Add/Remove/Trim/Move/Split/… with execute/undo/redo/serialize
- [ ] Grouped undo + autosave + crash recovery

## Phase 4 — Rendering & Export ⬜

- [ ] Timeline compiler → render graph → FFmpeg
- [ ] Preview quality selector + frame cache
- [ ] Export presets + dialog + post-export actions
- [ ] Render tests (codec/duration/resolution/streams)

## Phase 5 — Transcription ⬜

- [ ] Local Whisper + word timings/speakers/confidence
- [ ] Transcript↔timeline mapping + panel + text-based delete

## Phase 6 — AI Runtime ⬜

- [ ] Capability router + registry + adapters (ollama, openai-compat, 1 cloud, custom)
- [ ] Capability declarations + config UI + connection test + cloud consent

## Phase 7 — AI Editing ⬜

- [ ] Detectors (silence/filler/repetition) → proposed rough cut
- [ ] Edit-plan schema + validator + diff UI + transactional apply + snapshots

## Phase 8 — Visual Intelligence ⬜

- [ ] Scene spec + deterministic renderer + editable visuals + code visuals

## Phase 9 — Templates ⬜

- [ ] Format + browser + preview + apply + 12 initial templates

## Phase 10 — Polish & Release ⬜

- [ ] A11y, perf/stress, recovery/history, error pass, onboarding, examples, docs, packaging

---

## MVP feature checklist (AGENTS §122 — all must be real, not mocked)

- [ ] Desktop app · [ ] Project creation · [ ] Media import · [ ] Media preview
- [ ] Timeline · [ ] Basic editing · [ ] Undo/redo · [ ] Autosave
- [ ] FFmpeg rendering · [ ] Export · [ ] Local transcription · [ ] Transcript editing
- [ ] AI provider abstraction · [ ] Ollama · [ ] One cloud provider · [ ] AI rough-cut proposal
- [ ] Captions · [ ] Text overlays · [ ] Basic diagrams · [ ] Templates
- [ ] Example projects · [ ] Crash recovery · [ ] Documentation · [ ] Tests

## Explicitly excluded from MVP (do not build yet)

Color grading suite, advanced VFX, 3D, collaborative cloud editing, stock marketplace, social network, cloud storage, complex video-gen, advanced multicam, mobile app.

---

## Blockers / Risks (update as discovered)

| Date | Blocker / Risk | Owner | Mitigation |
|------|---------------|-------|------------|
| 2026-09-20 | Rust toolchain not installed on scaffold machine (`rustc/cargo` missing) | build | **Resolved:** Rust 1.98.1 stable + rustfmt/clippy installed via rustup; pinned in `rust-toolchain.toml`; `cargo test/fmt/clippy` green on workspace. |
| 2026-09-20 | Tauri backend not wired (no src-tauri Cargo workspace yet) | Phase 1 | Frontend shell runs on Vite; `tauri:dev` exits 1 honestly. Next: `cargo add tauri`, capabilities, sidecar config, `stream://` preview spike (ARCHITECTURE.md). |
| 2026-09-20 | Seekable preview risk (Tauri `asset://` lacks Range support) | arch | Custom `stream://` Range/206 protocol spike in Phase 1; proxy-first H.264/AAC; per-OS preview matrix. Tracked in ARCHITECTURE.md. |
| 2026-09-20 | Llama-3.1 weights (Community License, not OSI) + H.264/AAC patents | legal | Tracked in LEGAL_AND_LICENSING.md; Qwen3 default; codec patent exposure reviewed separately from copyright. |
| — | — | — | — |

## Changelog (scaffold → …)

| Date | Commit | What changed |
|------|--------|--------------|
| 2026-09-20 | `fe110c1` / pushed to `main` | Initial clean scaffold: structure, README, PLAN, PROGRESS, ROADMAP, collab docs, verify script. No app code per AGENTS §157. |
| 2026-09-20 | Phase 0 research drop (pending push) | Competitive (10 editors) + technical (Tauri/FFmpeg/Whisper/local-AI/timeline) research; ARCHITECTURE.md; ADR-001…008; UX flows; toolchain pins; licensing register. Docs only — gate to Phase 1 is spike validation (preview protocol, sidecar signing, whisper latency, qwen3 eval). |
| 2026-09-20 | Phase 1 shell drop (pending push) | Rust 1.98.1 + fmt/clippy/test green; Vite+React18+TS+Tailwindv4+Zustand shell with tokens, routing, validated project creation (5 vitest), top bar + editor shell (unwired controls disabled+honest); CI node/rust jobs; README t3code-style; repo description+topics set. Tauri backend still pending. |
