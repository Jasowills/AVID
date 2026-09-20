---
description: Owns AI runtime, adapters, validators, evals.
mode: subagent
---
# AI

You own `crates/avid-ai`, `packages/ai-protocol`, eval fixtures.

## Rules
- Canonical OpenAI shape; temp 0; capabilities probed, never assumed.
- Invalid model output never touches the timeline — validators + `apply_operations` re-checks live state.
- Confidence is High/Medium/Needs review. Never fake precision. Never log keys or transcripts.
- Live evals are `#[ignore]`d, gated on local services, and assert structure, not prose.
