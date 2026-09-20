# ADR-011 — Rough-cut proposals + transactional apply

- **Status:** accepted (autopilot passes 5–6)
- **Date:** 2026-09-20
- **Context:** AI must propose edits the user reviews per-operation, never mutate silently — and a failed multi-op apply must not leave the timeline half-modified (AGENTS §54–55, §116).
- **Options:** (A) Detectors (silencedetect + filler heuristics) → proposal (sorted, deduped, honest High/Medium confidence) → review checkboxes → `apply_operations` re-validates vs live state → single grouped undo. (B) Direct detector-to-timeline application (rejected — no review, no trust). (C) Purist full-AI planning for rough cuts (rejected for MVP — deterministic detectors are testable and sufficient).
- **Decision:** **(A).** `RemoveRangeCommand` gives text-delete semantics as one undoable op. `apply_validated_ops` pre-validates each op (invalid → reported skip), executes the valid set via `execute_group` (atomic rollback), labels one undo step `AI: <goal>`. Visual ops validate for review but are honestly deferred (backend rejects unknown variants loudly rather than dropping them — the UI partitions them out with a Phase 8 note).
- **Consequences:** Proposal quality is bounded by detector quality (silence + fillers only; repetition/topic detection is future work). Pre-AI snapshots (AGENTS §117) follow as the next safety layer.
- **References:** `crates/avid-timeline/src/commands.rs` (`RemoveRangeCommand`); `commands.rs` (`propose_rough_cut`, `apply_operations`).
