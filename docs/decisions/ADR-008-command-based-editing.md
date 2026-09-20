# ADR-008 — Command-based editing + AI safety contract

- **Status:** accepted (Phase 0.5)
- **Date:** 2026-09-20
- **Context:** If AI can mutate state directly, there is no undo, no review, no trust — and a hallucinated clip ID can destroy a project. AI must operate the editor like a careful human: through commands (AGENTS §15, §76, §105–106, §115–116).
- **Options:** (A) AI emits validated EditPlan/SceneSpec → commands → transactional apply. (B) AI patches state directly (rejected). (C) AI generates shell/FFmpeg strings (rejected — never execute AI-generated shell).
- **Decision:** **(A).** Schemas live in `packages/ai-protocol` (EditPlan §25, SceneSpec §29). `validateEditPlan()` rejects malformed JSON, missing fields, hallucinated IDs, invalid/overlapping timestamps, nonexistent media, unsupported capabilities — before any `execute()`. Apply is transactional (op 8/12 fails → rollback all). Pre-AI snapshot + grouped undo ("AI: …"). Diff UI with per-op Accept/Reject/Edit; Before/After duration + change counts; honest confidence (High/Medium/Needs review); "Why?" explanations. Tools (`get_project`, `search_transcript`, `create_edit_plan`, …) are permissioned. Filesystem: normalize + traversal-reject + project-dir allowlist; secrets never touch frontend logs.
- **Consequences:** Some valid-looking AI output gets rejected — correct tradeoff. Eval dataset (`fixtures/ai/prompts/`) runs per provider. AI failure tests are mandatory (§98).
- **References:** AGENTS §15, §25, §29, §54–55, §75–76, §97–99, §105–107, §115–120.
