# Local AI APIs research (Phase 0.3)

> Researched 2026-09-20. Primary: `docs.ollama.com`, `lmstudio.ai/docs`, `docs.vllm.ai`, `platform.openai.com`, `docs.anthropic.com`, `ai.google.dev`.
> Decision recorded in `docs/decisions/ADR-005-ai-provider-abstraction.md` + `ADR-007-local-first-ai.md`.

## API surface + capability detection

**Ollama** (`http://localhost:11434`, MIT): native `/api/chat` + `/api/embed` + OpenAI-compat `/v1/chat/completions`, `/v1/completions`, `/v1/embeddings`, `/v1/models`, partial `/v1/responses`. `format: "json"` or JSON-schema, `response_format` via compat layer, tool-calling via `tools`, embeddings (`embeddinggemma`, `mxbai-embed-large`, `qwen3-embedding`), vision via `images: [base64]` (`qwen3-vl`, `llama3.2-vision`). Detection: `GET /api/tags` + `POST /api/show` (family, parameter_size, capabilities). `/v1/models` alone is insufficient (IDs only).

**LM Studio** (`http://localhost:1234/v1`, proprietary freeware — connect only, never bundle): OpenAI-shaped `/v1/chat/completions`, `/v1/embeddings`, `/v1/models`, Anthropic-compat `/v1/messages`. Structured output via `response_format: {type:"json_schema"}` enforced by llama.cpp grammar (GGUF) or Outlines (MLX). Tool-use + MCP-local, vision via `image_url`. Detection: `GET /v1/models` returns IDs only. Docs warn: "Not all models capable of structured output, particularly <7B."

**vLLM** (`vllm serve`, Apache-2.0, GPU-first Linux/CUDA; macOS CPU not a target): broadest OpenAI coverage (`chat/completions`, `completions`, `embeddings`, `audio/transcriptions`, `tokenize`, `responses`, `score`). Strongest structured output (`response_format` + `extra_body.structured_outputs`, xgrammar/guidance, on by default). Named-function tool-calling with guided decoding. Irrelevant for MVP desktop, ideal self-hosted-team reference for the generic adapter.

**Generic OpenAI-compatible endpoint** (llama.cpp server, llamafile, text-generation-webui, vLLM, Ollama): assume least-common-denominator `POST /v1/chat/completions {model, messages}` + `GET /v1/models`. Everything else (`response_format`, `tools`, `embeddings`, `image_url`) must be **probed, never assumed**. AVID implements `probeCapabilities(baseURL)`: (a) tiny `response_format: json_schema` edit-plan test, (b) dummy `tools: [{get_project}]` test, (c) `GET /v1/models`. Cache in the Model Capability Registry. This is the BYOM contract.

## Structured JSON on CPU — recommended models

Grammar-constrained sampling (Ollama `format`, llama.cpp grammar) makes 8B reliable at temp=0 + schema validation + retry. Below ~7B reliability collapses.

| Model | Size/ctx | License | Role |
|---|---|---|---|
| `qwen3:8b` Q4_K_M (~5.2 GB, 40K ctx) | Apache-2.0 weights | Default edit-plan generation. Best JSON/tool follower in class, multilingual. |
| `llama3.1:8b` (128K ctx) | Llama Community License (track — use/output restrictions, not OSI) | Long-transcript summarization + tool-use; weaker strict-JSON → always schema-constrained + validator. |
| `qwen3:4b` fallback (~2.5 GB, 256K ctx) | Apache-2.0 weights | 8 GB-RAM machines; simple ops only (`remove_range`, `add_caption`); route complex scene specs to 8B/cloud. |

Pattern: `temperature: 0` + schema-in-prompt + `format=schema` + strict parse + retry. Never accept raw prose as an edit.

## One MVP cloud option: OpenAI vs Anthropic vs Gemini

| | OpenAI | Anthropic | Gemini |
|---|---|---|---|
| Structured output | `response_format: {type:"json_schema", strict:true}` — FSM-constrained, model literally cannot emit invalid token. Mature since Aug 2024. | Single-tool-as-schema + native `output_config.format: json_schema` on Claude 4.5+. Excellent reasoning; non-OpenAI Messages API shape. | `generationConfig: {responseMimeType:"application/json", responseSchema}`. Cheapest Flash, large ctx; strict-guarantee historically weakest. |
| Cost transparency | Per-1M pricing + per-response `usage{prompt,completion}`; Batch/Flex halve; cached-input −10%. Most actionable. | Transparent ($1/$5 Haiku 4.5 → $5/$25 Opus 5), prompt-caching −90%, Batch −50%. | Cheap Flash tier. |

**Recommendation: OpenAI for MVP.** (1) Only mature strict guarantee — critical because invalid edit-plans must never touch the timeline. (2) Request shape is isomorphic to local Ollama/LM Studio/vLLM adapters — one canonical `Capability Router → OpenAI-shape` path serves local + cloud, halving adapter work; Anthropic/Gemini need bespoke translators. (3) Most actionable cost signal for "show cost before cloud call." Anthropic second post-MVP for complex reasoning.

## Insulating against API drift (adapter pattern)

Providers drift constantly (vLLM `guided_json` → `structured_outputs`; Ollama docs moves; OpenAI Responses API alongside Chat Completions). Never call providers from editor code:

```text
AI Runtime → Capability Router (text/structured/vision/transcribe/embed/tools)
  → Provider Registry → Provider Adapter (ollama | openai-compat | openai | …) → Model
```

Rules: canonical internal types only (EditPlan, VisualSceneSpec, schemas in `packages/ai-protocol`); each adapter translates wire formats; central `validateEditPlan()` rejects hallucinated IDs, overlapping ranges, out-of-bounds timestamps before command construction; AI emits Commands, never mutates state or shell; pin adapter per provider API version, probe at startup, fail closed ("Provider responded but lacks structured output").
