# AVID AI Notes (AGENTS §22–25, §75–80, §97–99, §116–120)

## What the AI may do

Emit validated `EditPlan` / `VisualSceneSpec` JSON → `validateEditPlan` / `validateSceneSpec` (`packages/ai-protocol`, 12 tests) → per-op review → `apply_operations` (re-validated vs live state, one transactional undo group). Nothing else. No shell, no direct state mutation, no filesystem writes outside the project (ADR-008).

## Rough cut today (deterministic, no LLM needed)

`silencedetect` spans (High) + transcript fillers (Medium) → `propose_rough_cut` (sorted, deduped, removable totals) → same review checkboxes. Repetition/topic detection is future work.

## Live-verified

Ollama `qwen2.5:7b`: schema probe + edit plan with operations (ignored test, ~38 s). Whisper `tiny.en`: exact transcript + word timings (script + in-process test).

## Trust contract (§118–120)

Per-op accept/reject, Before/After via removable totals, grouped undo (`AI: <goal>`), reasons on every op ("Why?" = the reason string), confidence buckets (High/Medium/Needs review — never fake decimals). Pre-AI snapshots (§117) are next.

## Cost (§144)

Local is free and default. Cloud shows its endpoint before any call; per-call cost display lands with usage metering.
