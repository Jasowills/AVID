# AVID — Roadmap

> Phases 0–10 = MVP. Post-MVP is explicitly out of scope until MVP meets the Definition of Done. Source: AGENTS §122–§125.

## MVP

- **Phase 0 — Research & Architecture** (current): competitive + technical research, architecture, ADR-001…008, UX flows, dev env, risks. No premature implementation.
- **Phase 1 — Desktop Shell:** Tauri + React + design system + routing + project creation.
- **Phase 2 — Media Foundation:** import, probe, thumbnails, waveform, playback, library, proxies, jobs.
- **Phase 3 — Timeline:** model, interactions, commands, undo/redo, autosave.
- **Phase 4 — Rendering & Export:** render graph, FFmpeg, preview, presets, export UX.
- **Phase 5 — Transcription:** local Whisper, timestamps, transcript↔timeline sync, text-based editing.
- **Phase 6 — AI Runtime:** provider abstraction, Ollama, one cloud provider, capability registry.
- **Phase 7 — AI Editing:** rough cut, silence/filler, edit plans, diff/review, transactional apply.
- **Phase 8 — Visual Intelligence:** scene spec, diagram renderer, editable diagrams.
- **Phase 9 — Templates:** system + browser + 12 initial templates.
- **Phase 10 — Polish:** a11y, perf, recovery, error UX, onboarding, docs, examples, packaging.

## Post-MVP (not now)

Smart reframing, advanced captions, full code visualization + Mermaid, image generation (kept separate from deterministic visuals), B-roll providers, multi-camera, audio enhancement, automatic Shorts, plugin system, MCP server, CLI, template marketplace, community templates.

## The north star (AGENTS §162)

Drop in a 30-min recording → editable 8-min technical explainer proposal → conversational refinement → every AI decision editable → media never leaves the machine unless the user chooses cloud → export. We don't move to advanced AI until import→export is reliable.
