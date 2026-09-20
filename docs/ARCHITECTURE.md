# AVID Architecture (scaffold stub → filled in Phase 0.4)

> Status: **stub**. Phase 0 replaces this with the full boundary proposal + diagram.
> Do not implement against this stub.

## Intended boundaries (from AGENTS.md)

```text
React UI (interaction, preview, panels)
  ↕ typed IPC (packages/shared-types)
Rust core
  ├── avid-core        kernel types / errors / events (no domain deps)
  ├── avid-project     versioned format + migrations + snapshots
  ├── avid-timeline    model + commands + undo/redo
  ├── avid-media       MediaEngine (ONLY raw FFmpeg lives here)
  ├── avid-render      compiler → render graph → FFmpeg
  ├── avid-ai          capability router → registry → adapters
  └── avid-cli         post-engine CLI
AI runtime: provider adapters behind capability interface (BYOM).
Render: deterministic graph; future engines plug behind it.
```

## Rules

- Frontend owns interaction; Rust owns native/perf work.
- AI emits validated commands only (edit-plan + scene-spec schemas in `packages/ai-protocol`).
- No provider SDK outside `avid-ai` adapters. No FFmpeg strings outside `avid-media`.

## Open questions (Phase 0 research must answer)

Tauri 2.x IPC patterns; FFmpeg integration (sidecar vs linked); Whisper runtime; timeline lib vs custom; Ollama/OpenAI-compat API surface; licensing (codecs/fonts/models).
