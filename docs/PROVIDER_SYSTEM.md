# AVID Provider System (AGENTS §16–17, ADR-005, ADR-012)

Implementation: `crates/avid-ai` + `probe_provider` command + Settings UI.

## Layers

`Capability Router (planned full routing) → ProviderRegistry (register, local-first recommend) → Adapter (OpenAiCompatAdapter only) → Model`.

## Canonical shape

One OpenAI-compatible path serves Ollama (`/v1`), LM Studio, vLLM, generic compat, and OpenAI cloud. Temperature forced to 0 for structured calls. Capabilities probed (`probe_structured_output`), never assumed — failures report "responded but lacks structured output".

## Models (verified)

Local default `qwen2.5:7b` (live eval green: schema probe + edit plan with operations). Research recommends `qwen3:8b` / `llama3.1:8b` / `qwen3:4b` fallback (see `docs/research/local-ai.md`).

## Cloud posture

Optional Bearer key (blank = local; mock-tested absent for local). Keys in memory only, never logged, never persisted. `probe_provider` validates URL shape; reachability reported in-result. Per-operation consent UI is pending — tracked, not claimed.

## Capabilities (honest matrix)

Done: text, structured output. Partial: transcription (engine separate — whisper-rs). Missing: vision, audio understanding, image/video generation, embeddings, tool calling.
