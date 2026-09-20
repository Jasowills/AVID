# AVID

AVID (AI Video Intelligence & Direction) is a local-first, cross-platform AI video editor. It combines a real non-linear timeline with an AI director that understands your footage and operates an editable project — plus a deterministic visual engine that turns explanations into editable diagrams, code visuals, and callouts.

Desktop apps for macOS, Windows, and Linux (Tauri 2.x). The frontend owns interaction; Rust owns the media core.

Works with the models you already have: Ollama, LM Studio-compatible and other OpenAI-compatible local endpoints, and (optionally) OpenAI, Anthropic, or Gemini. If it's running on your machine, AVID can use it — cloud is always explicit and opt-in.

## "Wait, what are you selling me?"

Nothing. AVID is MIT-licensed and local-first. We built it because we wanted a video editor that understands technical content: you explain something, it creates the diagram — editable, at the right point in the timeline.

We were inspired by existing solutions like Premiere's text-based editing, Descript's doc-as-timeline, Resolve's editing speed, and CapCut's one-click captions — but none of them combine a real NLE with local-first AI and editable AI-generated visuals.

We want something fast, offline-capable, private by default, and truly open. If we ever go the wrong direction, we want you to have everything you need to fork and build the editor that you want.

AI proposes. AVID structures. You control. The renderer executes.

## Installation

> [!WARNING]
> AVID is very, very early. There are no releases yet and no runnable editor — Phase 0 (research + architecture) just completed. To run anything today you build from source, and what you get is the desktop shell (Phase 1, in progress).
> You need Node.js 20+, a Rust stable toolchain, FFmpeg 6+ on PATH, and (optionally) Ollama for local AI.

### Build from source

```bash
git clone https://github.com/Jasowills/AVID.git
cd AVID

# No toolchain required: verifies repo structure + docs contract
./scripts/verify-scaffold.sh

# Frontend (desktop shell UI, runs in the browser until Tauri is wired)
npm install
npm run dev --workspace=@avid/desktop
```

Rust workspace commands (require the toolchain from `rust-toolchain.toml`):

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

Read [DEVELOPMENT.md](./DEVELOPMENT.md) for the full toolchain setup and troubleshooting.

### Desktop app

No packaged builds yet. Planned artifacts once Phase 1–10 land:

| OS | Packages |
|---|---|
| macOS | `.dmg` |
| Windows | `.exe` / installer |
| Linux | `.AppImage`, `.deb` |

### Local AI (optional, for Phase 6+)

```bash
# Default local provider — any recent Ollama build
ollama pull qwen3:8b
```

Cloud providers stay disabled until you add a key in Settings → AI Providers, per operation, with the processing label shown (`Local` vs `Cloud: Provider`).

## Some notes

We are very very early in this project. Expect missing features, not just bugs — see [PROGRESS.md](./PROGRESS.md) for what's actually done (nothing is marked complete without tests, docs, undo, and error handling).

We welcome collaborators: read [PLAN.md](./PLAN.md) for the strict phase order, then pick up the current phase in [PROGRESS.md](./PROGRESS.md). Small fixes and docs improvements are always fair game; big features follow the plan so the foundation stays solid.

## Documentation

Full docs live in [docs/](./docs). There's no docs site yet. Start here:

- [PLAN.md](./PLAN.md) — the strict execution plan we follow in order
- [PROGRESS.md](./PROGRESS.md) — living tracker of what's actually done
- [ROADMAP.md](./ROADMAP.md) — MVP phases 0–10 + post-MVP
- [Architecture proposal](./docs/ARCHITECTURE.md)
- Research: [editors](./docs/research/editors-comparison.md) · [Tauri + IPC](./docs/research/tauri.md) · [FFmpeg](./docs/research/ffmpeg.md) · [Whisper](./docs/research/whisper.md) · [local AI](./docs/research/local-ai.md) · [timeline](./docs/research/timeline.md)
- Decisions: [ADR-001 Tauri](./docs/decisions/ADR-001-tauri.md) · [ADR-002 media core](./docs/decisions/ADR-002-rust-media-core.md) · [ADR-003 project format](./docs/decisions/ADR-003-project-format.md) · [ADR-004 timeline](./docs/decisions/ADR-004-timeline-model.md) · [ADR-005 AI abstraction](./docs/decisions/ADR-005-ai-provider-abstraction.md) · [ADR-006 rendering](./docs/decisions/ADR-006-rendering-architecture.md) · [ADR-007 local-first AI](./docs/decisions/ADR-007-local-first-ai.md) · [ADR-008 command safety](./docs/decisions/ADR-008-command-based-editing.md)
- [UX flows](./docs/ux/flows.md)
- [Legal & licensing register](./docs/LEGAL_AND_LICENSING.md)

The full product spec is [AGENTS.md](./AGENTS.md) (163 sections — the source of truth).

## If you want to contribute, read this first

### Install the toolchains

#### macOS

```bash
# Node 20+ (check .nvmrc)
nvm install && nvm use
# Rust stable (see rust-toolchain.toml)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# FFmpeg 6+
brew install ffmpeg
# Optional: local AI
brew install ollama
```

#### Linux

```bash
# Node 20+, Rust (rustup), FFmpeg 6+ via your package manager,
# plus Tauri system deps (webkit2gtk etc. — see DEVELOPMENT.md)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Windows

```powershell
# Node 20+ (nvm-windows), Rust via rustup-init.exe,
# MSVC C++ build tools + WebView2 — see DEVELOPMENT.md
```

### Install dependencies

```bash
npm install
```

Verify the tree with `./scripts/verify-scaffold.sh`, then read [CONTRIBUTING.md](./CONTRIBUTING.md) before opening a PR — clean-code boundaries, small coherent commits, and the Definition of Done apply to everything.

Have a feature request? Open an [issue](https://github.com/Jasowills/AVID/issues) with the pain point and the workflow it would unblock.
