# AVID — Strict Execution Plan

> This file is the contract. We follow it in order. We do not skip phases. Any deviation requires an ADR and an update to this file + `PROGRESS.md`.
> Source of truth: `AGENTS.MD` (§87 development process, §137–§139, §156–§157).

---

## 0. Operating rules (apply to every phase)

1. **Research → Plan → Design → Implement → Test → Review → Document → Verify → Next.** (§87, §138)
2. **Research-first.** ≥3 existing approaches per major subsystem, recorded in `docs/research/`. No blind reinvention. (§88, §130)
3. **Implementation order is fixed** (§156): repo → docs → Tauri shell → design system → Rust core → project model → media → playback → timeline → commands → undo/redo → persistence → rendering → export → transcription → AI providers → edit plans → captions → visuals → templates → examples → polish → perf → security → release.
4. **Definition of Done is mandatory** (§135): requirements, UX, arch review, impl, unit+integration (+E2E where applicable), error/loading/empty states, undo/redo, persistence, a11y, perf, security, docs, regression, manual verification.
5. **No fake completion** (§136). If tests didn't run / build failed / export doesn't work / AI is hard-coded / timeline isn't persistent — say so in `PROGRESS.md`.
6. **Small coherent commits** (§139): `feat(scope): …`, `fix(scope): …`, `test(scope): …`, `docs(scope): …`. One system per commit.
7. **Context preservation** (§137): at the start of each task read `AGENTS.MD`, `ROADMAP.md`, `PROGRESS.md`, relevant ADRs, tests, `git status`; determine current phase.
8. **Clean-code bar (collaborator-ready):** typed boundaries, no `any`/unwrap slop, no duplicated FFmpeg strings, no provider logic outside adapters, no god-components, every public API documented + tested.
9. **AI safety** (§76, §105–§106): AI emits validated commands only; never executes shell from model output; never writes outside project dirs; never silently uploads media.
10. **PROGRESS.md is updated in the same commit as the work.** A phase without a PROGRESS update didn't happen.

---

## Phase 0 — Research & Architecture [CURRENT]

**Goal:** de-risk everything before code. **No premature implementation** (§124).

| # | Deliverable | Done when |
|---|---|---|
| 0.1 | Repo scaffold (this commit) | Structure + README + PLAN + PROGRESS + verify script, pushed to `main` |
| 0.2 | Competitive/UX research | `docs/research/` covers editors (Resolve, Premiere, Final Cut, CapCut, Descript, Runway, VEED, Screen Studio, Riverside, OpusClip): likes/hates/time-sinks/differentiation |
| 0.3 | Technical research (≥3 options each) | Tauri arch, React/Tauri IPC, FFmpeg integration, Whisper integration, timeline libs, local AI APIs (Ollama/OpenAI-compat), licensing |
| 0.4 | Architecture proposal | `docs/ARCHITECTURE.md` + boundary diagram: frontend vs Rust vs MediaEngine vs AI runtime vs render graph |
| 0.5 | ADRs 001–008 | Tauri, Rust media core, project format, timeline model, AI provider abstraction, rendering arch, local-first AI, command-based editing |
| 0.6 | UX flows | Home, project creation, editor layout, timeline interactions, job center, error UX, AI review/diff |
| 0.7 | Dev environment + CI skeleton | Pinned Node/Rust/Tauri/FFmpeg, `verify-scaffold`, CI running lint+typecheck+unit (even if suites are empty) |
| 0.8 | Risk register + licensing | `docs/LEGAL_AND_LICENSING.md` + risks (codecs, fonts, models, platform gaps) |

**Gate to Phase 1:** 0.2–0.8 merged, ADRs approved, scaffold builds clean, `PROGRESS.md` reflects reality.

---

## Phase 1 — Desktop Shell

Tauri 2.x + React + TS + Tailwind + Zustand; design-system tokens; routing (home/new-project/editor/settings); project creation dialog (name, canvas, fps, resolution, template-optional); top bar (undo/redo/save/AI status/export). No media logic yet.

## Phase 2 — Media Foundation

`avid-media` MediaEngine abstraction (`probe/transcode/proxy/thumbnail/waveform/extractAudio/extractFrame`); import; metadata; thumbnails; waveform; playback via proxies; media library grid/list; proxy offer flow; background jobs + job center; humane error UX (expandable technical details).

## Phase 3 — Timeline

`avid-timeline` model (tracks, clips with id/source/start/duration/in-out/track/transform/opacity/audio/effects/metadata); UI (zoom, scroll, snap, split/trim/move/delete, ripple, markers, multi-select, shortcuts per §45–46); **command engine** (`AddClip/RemoveClip/Trim/Move/Split/…` with execute/undo/redo/serialize); grouped undo; autosave + crash recovery hooks.

## Phase 4 — Rendering & Export

Timeline compiler → deterministic render graph → FFmpeg; preview (¼/½/full/auto, cached frames); export presets (YT 1080p/4K, Reel, TikTok, Short, Feed, custom); export dialog (estimates, output path); post-export actions. Render tests assert codec/duration/resolution/streams (no brittle byte compares).

## Phase 5 — Transcription

Local Whisper (+timestamps, word timings where available, speakers, confidence, language); transcript↔timeline mapping; transcript panel (search/highlight/select); text-based edit (delete sentence → remove media range).

## Phase 6 — AI Runtime

Capability router + provider registry + adapters (`ollama`, `openai-compatible`, one cloud provider, `custom`); model capability registry (text/vision/audio/structured/tool-calling/context/local-cloud/latency); provider config UI (no JSON for normal users) + connection test; explicit cloud-consent UI; failover asks user (never auto-reroutes).

## Phase 7 — AI Editing

Silence/filler/repetition detection → **proposed** rough cut (never auto-destroy); Edit Plan schema + validator; AI diff UI (accept/reject/edit per operation); transactional apply (rollback on partial failure); pre-AI snapshot; explainability ("why") + honest confidence (High/Medium/Needs review).

## Phase 8 — Visual Intelligence

Concept→visual-opportunity detection; Visual Scene Spec schema; deterministic diagram renderer (nodes/connections/animations); editable visuals (text/color/position/size/animation); code visuals (syntax highlight, line focus, diffs); Mermaid import where practical.

## Phase 9 — Templates

Data-driven format (`template.json + preview + assets/ + scenes/ + rules/`); browser + preview + apply-as-starting-point; 12 initial templates; template application as commands (undoable).

## Phase 10 — Polish & Release

A11y (keyboard-only editing, screen reader, focus, reduced motion, contrast), perf (large-project stress test), recovery/version history, error UX pass, onboarding + examples gallery, full docs, packaging (.dmg/.exe/.AppImage/.deb), release checklist (§140) on macOS/Windows/Linux where CI allows.

---

## Post-MVP (explicitly not now)

Smart reframing, advanced captions, Mermaid-full, image-gen (separate from deterministic), B-roll providers, multicam, audio enhancement, auto-Shorts, plugin system, MCP server, CLI, template marketplace. (§125)

## Explicitly excluded from MVP (§123)

Color-grading suite, advanced VFX, 3D, collaborative cloud editing, stock marketplace, social network, cloud storage, complex video-gen, advanced multicam, mobile app.

---

## Verification per phase

- `cargo test` (timeline math, trim/split, commands, undo/redo, serialization, migrations, render graph, path safety) + property tests where useful.
- `npm test` / `tsc` (state transitions, command parsing, schema validation, provider registry, template loading).
- E2E workflow (create → import → transcribe → cut → caption → visual → export → verify output).
- Offline test (editing works, cloud clearly unavailable). Network chaos (timeout/rate-limit/outage) recovers gracefully. Crash tests (renderer/FFmpeg/AI/app) recover via autosave.
- Large-project stress (hundreds of clips) measures load/interaction/memory/export/autosave.

---

## If stuck

Stop. Document the conflict before any destructive decision (§163). If architecture conflicts with a requirement, write it up in `docs/decisions/` as a proposed ADR amendment and update `PROGRESS.md` blockers — do not silently compromise data, privacy, or undo reliability.
