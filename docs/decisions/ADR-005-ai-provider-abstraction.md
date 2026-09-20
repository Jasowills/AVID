# ADR-005 — AI provider abstraction (canonical OpenAI shape)

- **Status:** accepted (Phase 0.5)
- **Date:** 2026-09-20
- **Context:** No provider-specific logic scattered through the app; user owns the intelligence layer (BYOM). Local models vary wildly in capability; APIs drift. See `docs/research/local-ai.md`.
- **Options:** (A) Canonical OpenAI-shaped internal path: `AI Runtime → Capability Router → Provider Registry → Adapter (ollama | openai-compat | openai | …)`. (B) Per-provider call sites (rejected — drift + lock-in). (C) Anthropic-Messages-canonical (rejected — local servers speak OpenAI shape, not Messages).
- **Decision:** **(A).** One canonical path serves Ollama (default, `qwen3:8b` + `llama3.1:8b` long-ctx + `qwen3:4b` low-RAM fallback) + LM Studio + generic compat + **OpenAI as the one MVP cloud option** (only mature FSM strict JSON guarantee; shape isomorphic to local adapters; most actionable cost signal). Anthropic second post-MVP. `probeCapabilities(baseURL)` at startup (schema test + tool test + model list); cache in the Model Capability Registry; fail closed with "responded but lacks structured output". Canonical types only (EditPlan, SceneSpec); adapters translate wire formats; temp=0 + schema + strict parse + retry.
- **Consequences:** Half the adapter work; strict-schema reliability for edit-plans; explicit capability UX ("use 8B for planning, another model for vision"). Must maintain adapter pins against provider drift.
- **References:** `docs/research/local-ai.md`; AGENTS §16–17, §79, §147.
