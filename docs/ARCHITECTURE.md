# AVID Architecture Proposal (Phase 0.4)

> Status: **approved for Phase 1**. Details per decision in `docs/decisions/ADR-001…008`.
> Research basis: `docs/research/{editors-comparison,tauri,ffmpeg,whisper,local-ai,timeline}.md`.

## System overview

```text
┌─ React UI (interaction only) ─────────────────────────┐
│ Home · Editor (media/preview/inspector) · Timeline     │
│ Transcript · AI panel · Job Center · Export · Settings │
│ Zustand: ephemeral UI (selection, viewport, toasts)    │
└─────────────────── typed IPC ─────────────────────────┘
   commands (only mutation path) · Channel<JobEvent> (progress) · events (fan-out)
   contracts: packages/shared-types (+ ai-protocol schemas) — drift = CI failure
┌─ Rust core ───────────────────────────────────────────┐
│ avid-core      kernel types, AvidError AVID_<D>_<NNN>  │  (no domain deps)
│ avid-project   versioned project.json + migrations    │
│ avid-timeline  model + commands + undo/redo           │
│ avid-media     MediaEngine — ONLY raw FFmpeg lives here│
│ avid-render    compiler → render graph → FFmpeg       │
│ avid-ai        capability router → registry → adapters│
│ avid-cli       post-engine CLI (stub until stable)     │
└───────────────────────────────────────────────────────┘
   sidecars (externalBin): ffmpeg/ffprobe (LGPL builds), whisper-rs in-process
   media bytes NEVER cross IPC — pass {mediaId, assetUrl}
```

## Key decisions (one line each; full rationale in ADRs)

1. **Tauri 2.11.x** pinned; OS webview + Rust backend; matrix CI; signed updater (ADR-001).
2. **Rust media core** behind `MediaEngine` trait; sidecar CLI-per-op for MVP; linked libav deferred (ADR-002).
3. **Versioned `project.json`** with relative media paths, migrations, snapshots; never silently break old projects (ADR-003).
4. **Custom timeline model + custom canvas UI**; commands with execute/undo/redo/serialize; grouped AI undo; wavesurfer peaks for waveforms only (ADR-004).
5. **Canonical OpenAI-shaped adapter** serving Ollama (default, `qwen3:8b`) + LM Studio + generic compat + OpenAI cloud; capability probing, never assumed (ADR-005).
6. **Deterministic render graph** compiled from timeline → single FFmpeg DAG; HW accel export-only with software fallback (ADR-006).
7. **Local-first AI**: whisper.cpp in-process default; cloud = explicit per-operation consent with `Local` / `Cloud: Provider` labels; no auto-failover rerouting (ADR-007).
8. **AI emits validated commands only** (EditPlan + SceneSpec schemas in `packages/ai-protocol`); transactional apply + pre-AI snapshot; reject hallucinated IDs/ranges (ADR-008).

## Data flows

- **Import → probe:** `import_media(path)` command → Rust validates path (allowlist, no traversal) → `MediaEngine.probe` (ffprobe JSON) → metadata in project → thumbnails/waveform/proxy as background jobs with Channel progress → Zustand mirrors `Job[]`.
- **Edit:** UI gesture → command object → `execute()` in Rust (or TS mirror for UI-only ops) → undo stack push → autosave debounce → preview invalidates affected range only.
- **AI assist:** transcript/media context → capability router picks model → structured output → `validateEditPlan()` → diff UI (accept/reject per op) → transactional `apply()` → grouped undo entry ("AI: Create Technical Explainer") → snapshot retained.
- **Visualize:** concept → Mermaid draft → Scene Spec (source of truth) → React Flow editor → SVG render → FFmpeg overlay at timeline range.
- **Export:** timeline → compiler → render graph → single FFmpeg DAG → presets → post-export actions. No auto-upload.

## Preview strategy (addresses Tauri risk #1)

`asset://` has no HTTP Range support → `<video>` seek breaks on macOS/Linux. Fix: custom `stream://` protocol with Range/206 + MIME/CORS (or `tauri-plugin-localhost`); proxy-first H.264/AAC; per-OS preview test matrix in CI. Images/thumbs via `convertFileSrc`; video/audio only via `stream://`.

## Security boundaries

Least-privilege capabilities: `fs` scoped to project dir + `$APPCACHE`; `shell` allowlist per sidecar + arg regex; secrets in Rust only (never frontend logs); no AI-generated shell ever; path normalize + traversal reject; per-operation cloud consent; `docs/LEGAL_AND_LICENSING.md` gate on every dependency.

## Open questions → Phase 1 spikes (timeboxed)

1. `stream://` Range protocol prototype on all 3 OSes (preview matrix).
2. FFmpeg sidecar signing/notarization sizes per arch.
3. whisper.cpp `base.en` CPU latency on 30-min audio (VAD chunking validation).
4. Ollama `qwen3:8b` structured-output reliability on edit-plan eval set (temp=0 + schema).

If any spike fails, file a proposed ADR amendment — do not silently compromise (PLAN.md).
