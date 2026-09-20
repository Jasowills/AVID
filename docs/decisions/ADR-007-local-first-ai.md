# ADR-007 — Local-first AI

- **Status:** accepted (Phase 0.5)
- **Date:** 2026-09-20
- **Context:** AVID must work without cloud AI and never secretly send media to providers (AGENTS §2, §18–19). Transcription and edit-planning must run offline. See `docs/research/whisper.md` + `local-ai.md`.
- **Options:** Transcription — (A) whisper.cpp via whisper-rs in-process (offline, MIT, Metal/ANE, best Rust story). (B) faster-whisper Python sidecar (fast on CUDA, DLL hell on Win, CPU-only Mac). (C) Cloud STT (rejected as default — privacy). Planning — (A) Ollama local default. (B) Cloud-first (rejected).
- **Decision:** **Transcription: (A)** — `base.en` default (142 MB), auto-download `small`/`large-v3-turbo`; pipeline `extractAudio 16 kHz mono → Silero-VAD → whisper.cpp → JSON {segments, words, lang, conf}`; `tinydiarize` turn markers + manual relabel for MVP speakers. Fallbacks: (B) optional faster-whisper sidecar → WhisperX align pass → explicit cloud STT opt-in. **Planning: Ollama default** (`qwen3:8b`). Cloud (OpenAI MVP) only via per-operation consent dialog (`[Allow Once] [Always Allow] [Cancel]`), labeled `Local` vs `Cloud: Provider` on every op. Provider failure never auto-reroutes — user chooses.
- **Consequences:** Offline editing/transcription/planning; larger download (models + FFmpeg sidecars); explicit consent UX cost accepted. Persist `transcript.provider` + per-op processing labels for trust.
- **References:** `docs/research/whisper.md`, `docs/research/local-ai.md`; AGENTS §2, §18–19, §77, §100.
