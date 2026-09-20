# ADR-012 — Cloud auth on the canonical adapter + explicit consent posture

- **Status:** accepted (autopilot pass 7)
- **Date:** 2026-09-20
- **Context:** BYOM requires cloud endpoints on the same code path as local ones, without ever leaking keys to local servers or uploading silently (AGENTS §18–19, §77).
- **Options:** (A) Optional Bearer on `OpenAiCompatAdapter` (blank = local, key never sent to keyless endpoints) + `probe_provider` + settings UI with Local/Cloud badge. (B) Separate cloud adapter type (rejected — duplicates the canonical path). (C) Key in URL query (rejected — leaks into logs).
- **Decision:** **(A).** `with_api_key` builder (blank → `None`); mock tests prove the header travels for cloud and is absent for local. `probe_provider` validates URL shape and reports reachability in-result. Settings persists endpoint/model only — keys stay in memory. Per-operation consent UI (`Allow Once / Always Allow`) is still pending and tracked.
- **Consequences:** OpenAI-compatible clouds work today untested-live (no keys in CI); local endpoints cannot receive credentials by construction.
- **References:** `crates/avid-ai/src/lib.rs`; `commands.rs` (`probe_provider`).
