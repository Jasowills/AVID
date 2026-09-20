# AVID — Progress Tracker

> Living document. Updated in the same commit as the work. Never mark complete without meeting AGENTS §135 (Definition of Done). No fake completion (AGENTS §136).
> Legend: `✅ done` · `🚧 in progress` · `⬜ not started` · `⛔ blocked`

**Current phase:** Phase 0 — Research & Architecture (scaffold only)
**Last updated:** 2026-09-20 — scaffold commit (pre-implementation)
**Branch:** `main` · **Remote:** `https://github.com/Jasowills/AVID`

---

## Phase 0 — Research & Architecture 🚧

| ID | Deliverable | Status | Evidence / Notes |
|----|-------------|--------|------------------|
| 0.1 | Repo scaffold + README + PLAN + PROGRESS | 🚧 | This commit: structure, docs, verify script; then push to `main` |
| 0.2 | Competitive/UX research | ⬜ | Target: `docs/research/editors-*.md` + UX pain synthesis |
| 0.3 | Technical research (≥3 options/subsystem) | ⬜ | Tauri, IPC, FFmpeg, Whisper, timeline libs, Ollama/compat APIs |
| 0.4 | Architecture proposal | ⬜ | `docs/ARCHITECTURE.md` + boundaries |
| 0.5 | ADRs 001–008 | ⬜ | `docs/decisions/ADR-00*.md` |
| 0.6 | UX flows | ⬜ | `docs/ux/` — home, creation, editor, timeline, jobs, AI review |
| 0.7 | Dev env + CI skeleton | ⬜ | Pinned toolchains, CI green on empty suites |
| 0.8 | Risk register + licensing | ⬜ | `docs/LEGAL_AND_LICENSING.md` |

## Phase 1 — Desktop Shell ⬜

| ID | Deliverable | Status | Notes |
|----|-------------|--------|-------|
| 1.1 | Tauri 2.x + React + TS + Tailwind + Zustand wired | ⬜ | Pinned versions, `tauri dev` runs |
| 1.2 | Design system tokens + components | ⬜ | Colors/type/spacing/radius/buttons/inputs/panels/dialogs/toasts |
| 1.3 | Routing: home / new-project / editor / settings | ⬜ | — |
| 1.4 | Project creation dialog | ⬜ | Name, canvas, fps, resolution, template-optional |
| 1.5 | Top bar (undo/redo/save/AI status/export) | ⬜ | Non-functional export stub must be labeled as such |

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
| 2026-09-20 | Rust toolchain not installed on scaffold machine (`rustc/cargo` missing) | build | Phase 1 must pin + document toolchain; CI must install it; do not assume local Rust |
| — | — | — | — |

## Changelog (scaffold → …)

| Date | Commit | What changed |
|------|--------|--------------|
| 2026-09-20 | `fe110c1` / pushed to `main` | Initial clean scaffold: structure, README, PLAN, PROGRESS, ROADMAP, collab docs, verify script. No app code per AGENTS §157. |
