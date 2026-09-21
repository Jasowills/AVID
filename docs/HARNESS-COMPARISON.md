# HARNESS.MD vs implementation — section audit

> HARNESS.MD §§1–32 against the repo. Verdicts: ✅ done · 🟡 partial · ❌ missing · N/A.
> Strategy question (fork vs continue) is decided in `docs/diffusion-evaluation.md` — this doc audits only
> what the spec demands of the *product*, which applies on either stack.

## Process & ground rules (§§1–2, §27–30)

| Section | Verdict | Notes |
|---|---|---|
| Audit-before-change (§1, §32) | ✅ | `docs/diffusion-evaluation.md` written; no editor code touched for it; decision gate open |
| Licensing honesty (§2) | ✅ | MIT tree clean; Diffusion MPL/brand/watermark constraints recorded, nothing copied |
| Dev process research→document (§27) | ✅ | Followed every pass (research docs → ADRs → plan → implement → test → verify → document) |
| No fake implementations (§28) | ✅ | Disabled-honest controls, verified-everything culture, fail-loud live tests |
| Testing incl. invalid AI output (§29) | ✅ | 82 Rust + 39 TS; malformed/hallucinated/out-of-range rejection tests on both validators |
| Docs list (§30) | 🟡 | Have: architecture, project-format, provider-system (≈model-providers), licensing, ROADMAP, AGENTS, timeline/rendering/testing/security guides. Missing: `ai-director.md`, `ai-protocol.md`, `visual-intelligence.md`, `explain-visualize.md`, `agent-integration.md`, `CHANGELOG.md` (PROGRESS changelog covers it — extract file) |

## Product philosophy (§§3–6)

| Section | Verdict | Notes |
|---|---|---|
| Not-a-generator, direction→editable-composition (§3) | ✅ | Architecture complies end to end |
| Preserve editor quality/interactions (§4) | N/A-as-written | Written for a fork; on our stack it reads as: don't regress the working editor — honored (no rewrites, additive changes) |
| Monochrome-first themeable identity (§5) | ✅ | Monochrome default + 8 presets + picker + persistence; logo mark geometric, no clichés |
| App structure: media → viewer → timeline + inspector/Director/transcript (§6) | ✅ | Editor matches this skeleton |

## AI Director (§§7–9, §20–21)

| Section | Verdict | Notes |
|---|---|---|
| Director as control surface, analyzed checklist, numbered proposals, Review/Apply (§7) | 🟡 | Review/apply pipeline real; **missing the Analyzed checklist** (Transcript/Timeline/Visual/Assets ✓✓) and timestamped proposal rows |
| Structured ops, 7 properties (§8) | 🟡 | Validated/undoable/redoable/serializable/reviewable ✅; **attributable (which model) ❌** |
| Command coverage (§9) | 🟡 | 7 commands real; missing AddText/Graphic/Diagram/Caption/Transform/Template/ReplaceAsset/Scene ops/Transition |
| Provenance: subtle, inspector/history (§20) | ❌ | No AI-origin markers anywhere |
| Coherent AI undo (§21) | ✅ | Single `AI: goal` undo steps |

## Intelligence (§§10–15)

| Section | Verdict | Notes |
|---|---|---|
| Footage intelligence pipeline (§10) | 🟡 | Transcription + silence + fillers + phrase search real; scene/speaker/object/semantic/"best take"/shorts missing |
| Transcript-first editing + sync (§11) | 🟡 | Segment delete real; paragraph-level, caption/speaker/scene sync missing |
| Script→video grammars (§12) | ❌ | Entirely absent |
| Asset intelligence + "Visual needed" (§13) | ❌ | Asset list only; no understanding, no explicit-missing states |
| Explain→Visualize auto-detect (§14) | ❌ | Manual Mermaid→scene→place real; **automatic concept detection missing** (the signature gap) |
| Scene spec editable + AI-modifiable (§15) | 🟡 | Editable scenes real; animation + AI scene-edit missing |

## Platform (§§16–19, §§24–26)

| Section | Verdict | Notes |
|---|---|---|
| Provider abstraction, per-job models (§16) | 🟡 | Registry + canonical adapters real; per-job routing UI missing |
| Local-first + privacy UI (§17) | 🟡 | Local default + badge; per-op consent dialog missing |
| Agents/MCP (§18) | ❌ | No MCP server, no tool catalog exposure |
| Composition model, one truth (§19) | ✅ | Manifest only; no competing model |
| Keyboard-first + palette (§24) | 🟡 | Dock shortcuts real; **no command palette**; no customization UI |
| Desktop mac/win/linux (§25) | 🟡 | mac `.app` built + launched; win/linux unbuilt |
| Portable versioned projects (§26) | ✅ | Migrations, traversal guards, snapshots, reload recovery |

## States & polish (§§22–23) + roadmap (§31)

| Section | Verdict | Notes |
|---|---|---|
| Performance: async, proxies, caches (§22) | 🟡 | Jobs/proxies/peaks-cache real; no virtualization, no measurements |
| Full state matrix, no fake progress (§23) | 🟡 | Loading/empty/error/success/disabled/cancel real; **offline + unavailable + retry + partial states missing** |
| Roadmap phases (§31) | ⚠️ | **Conflicts with PLAN.md** (their Phase 0 = Diffusion audit, 1 = fork builds…). Reconcile on the fork decision; until then PLAN.md governs |

## Top 5 deltas by value (build order proposed)

1. **Director Analyzed checklist + timestamped proposals** (small, high-trust)
2. **Command palette** (Cmd/Ctrl+K; research greenlit hand-rolled)
3. **Explain→Visualize auto-detect** (signature; LLM concept→scene-spec path)
4. **AI provenance markers + model attribution** (small, trust-critical)
5. **Offline/unavailable/retry states + version-history browser** (robustness)
