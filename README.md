# AVID — AI Video Intelligence & Direction

> A real, local-first, cross-platform AI video editor. Not a video generator. Not a chatbot. A genuine editor with an AI director, a deterministic visual engine, and a model-agnostic AI runtime.

**IMPORT → UNDERSTAND → EDIT → VISUALIZE → REVIEW → MODIFY → EXPORT**

AI proposes. AVID structures. The user controls. The renderer executes.

---

## What is AVID?

AVID is a local-first desktop video editor (Tauri + React + Rust) that combines traditional non-linear editing with AI-assisted editing:

- **Real timeline** — video/audio/text/graphics/caption tracks, non-destructive, command-based with undo/redo.
- **Local-first AI** — Ollama / OpenAI-compatible local endpoints / Whisper by default. Cloud providers (OpenAI, Anthropic, Gemini) are optional and explicit.
- **Explain → Visualize** — AVID detects concepts in your transcript and creates *editable* diagrams, code visuals, callouts, and charts — not flattened AI images.
- **AI-native editable timeline** — AI outputs clips, cuts, graphics, captions, and transitions. Everything stays editable.
- **Bring Your Own Model** — provider adapters behind a capability interface. No provider-specific logic scattered through the app.

Target users: technical creators, YouTubers, educators, podcasters, and general creators. Initial wedge: **technical explainers** (diagrams, code highlights, architecture visuals).

Full product/UX/architecture spec: [`AGENTS.MD`](./AGENTS.MD) (163 sections — the source of truth).
Execution plan we strictly follow: [`PLAN.md`](./PLAN.md).
Live build tracker: [`PROGRESS.md`](./PROGRESS.md).
Phase roadmap: [`ROADMAP.md`](./ROADMAP.md).

---

## Status

> **Phase 0 — Scaffold.** Repository structure, docs, and plan are in place. No application code yet, by design (see `PLAN.md` / AGENTS §157). Do not expect a runnable editor from this commit.

See `PROGRESS.md` for the phase checklist. Nothing is marked complete until it meets the Definition of Done (AGENTS §135).

---

## Repository structure

```text
apps/desktop/          Tauri shell + React frontend (UI owns interaction)
crates/
  avid-core/           Shared kernel types, errors, IDs, events
  avid-project/        Versioned project format, migrations, snapshots
  avid-timeline/       Timeline model, commands, undo/redo
  avid-media/          FFmpeg abstraction: probe, proxy, thumb, waveform
  avid-render/         Timeline compiler → render graph → FFmpeg
  avid-ai/             AI runtime, capability router, provider registry
  avid-cli/            Future CLI (after core engine is stable)
packages/
  shared-types/        TS↔Rust shared contracts (source of truth for IPC)
  ai-protocol/         Edit-plan + visual-scene JSON schemas + validators
  ui/                  Shared React components (imports design-system only)
  design-system/       Tokens, colors, typography, spacing (single source)
  templates/           Template loader/validator (TS side)
templates/             Data-driven editing templates (12 initial, see below)
examples/              Shipped example projects (teach the product)
docs/                  Architecture, research, ADRs, UX, testing, legal
fixtures/              Deterministic test media/projects/AI eval prompts
.opencode/agents/      Researcher, architect, frontend, rust, media, ai, …
.opencode/commands/    /research, /plan, /test, /review, /security-review, …
scripts/               Dev/verify/release scripts (no app logic here)
tests/                 Cross-crate / E2E workflow tests
```

Clean-code rule: **frontend owns interaction, Rust owns native performance work.** No raw FFmpeg strings outside `avid-media`. No provider-specific logic outside `avid-ai` adapters. No AI mutation of state except via validated commands.

---

## Quickstart (contributors)

Prerequisites (Phase 0 — not all wired yet):

- Node 20+, npm 10+
- Rust stable (rustup), Tauri 2.x system deps
- FFmpeg 6+ on PATH (scaffold only; engine lands in Phase 2/4)
- Ollama (optional, for local AI phases)

```bash
# 1. Clone
git clone https://github.com/Jasowills/AVID.git
cd AVID

# 2. Read the contract first (mandatory for collaborators)
cat AGENTS.MD        # full spec
cat PLAN.md          # strict execution plan
cat PROGRESS.md      # what is actually done
cat CONTRIBUTING.md  # clean-code + commit rules

# 3. Verify scaffold integrity (no toolchain required)
./scripts/verify-scaffold.sh

# 4. Per-phase dev commands live in DEVELOPMENT.md
cat DEVELOPMENT.md
```

> Rust/Node workspaces exist as manifests only in this scaffold commit. `npm install`, `cargo build`, and `tauri dev` will be wired in Phase 1 with pinned versions and CI.

---

## Principles (non-negotiable)

1. **Local-first.** Works offline for basic editing. Cloud AI is opt-in, per-operation, with visible processing labels (`Local` vs `Cloud: Provider`).
2. **Non-destructive.** Original media is never modified. Edits are commands against sources.
3. **Command-based editing.** Every edit (human or AI) is a serializable command with `execute / undo / redo`. AI output is validated against schemas before it touches the timeline.
4. **Deterministic visuals first.** Diagrams/code/charts are structured, editable scene specs — not generated images.
5. **User control.** Large AI changes require Preview → Review → Apply/Cancel, per-operation accept/reject, and grouped undo.
6. **No fake completion.** A feature is done only when it meets AGENTS §135 (tests, errors, empty/loading states, undo, persistence, a11y, perf, docs, recovery, manual verification).
7. **Privacy.** Never secretly send media to cloud. Never log keys, tokens, or private transcripts. No invasive telemetry by default.

---

## Templates (planned initial set)

`technical-explainer`, `youtube-talking-head`, `podcast`, `podcast-short`, `educational-lesson`, `product-demo`, `documentary`, `social-short`, `code-tutorial`, `architecture-breakdown`, `product-launch`, `minimal-caption-video`.

Templates encode *editing behavior*, not just visuals. Format: `template.json + preview + assets/ + scenes/ + rules/`.

---

## Documentation map

| Doc | Purpose |
|---|---|
| `AGENTS.MD` | Master spec (§0–§163), source of truth |
| `PLAN.md` | Strict phased plan + gates (we do not skip phases) |
| `PROGRESS.md` | Living tracker — phase status, no lost features |
| `ROADMAP.md` | MVP phases 0–10 + post-MVP |
| `CONTRIBUTING.md` | Clean-code, branching, commits, PRs |
| `DEVELOPMENT.md` | Toolchain, commands, troubleshooting |
| `docs/ARCHITECTURE.md` | System boundaries (scaffold stub → Phase 0 fills) |
| `docs/decisions/` | ADRs (ADR-001…ADR-008 planned) |
| `docs/LEGAL_AND_LICENSING.md` | Dependency/license tracking |
| `docs/research/` | Competitive + technical research (3 options per subsystem) |

---

## Security

Threat model: malicious media, malicious project files, malicious AI output, third-party assets, credential leakage. Rules: never execute AI-generated shell, validate/normalize all paths (reject traversal), sandbox FFmpeg where practical, never expose secrets to frontend logs. See AGENTS §105–§107 and `docs/SECURITY.md` (Phase 0).

---

## License

MIT — see [`LICENSE`](./LICENSE). Codec/font/model/template licensing is tracked separately in `docs/LEGAL_AND_LICENSING.md` before any integration.

---

## Contributing

We welcome collaborators. Start with `CONTRIBUTING.md`, then `PLAN.md` + `PROGRESS.md` to find the current phase. Small coherent commits only (`feat(timeline): …`). No large unrelated commits. No mocked functionality marked as done.
