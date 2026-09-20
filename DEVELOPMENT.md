# AVID — Development Guide

> Scaffold phase. Toolchains get pinned in Phase 0.7. This file describes intent so collaborators don't guess.

## Toolchain (target, to be pinned in Phase 0)

| Layer | Choice |
|---|---|
| Desktop | Tauri 2.x |
| UI | React 18 + TypeScript 5 (strict) + Tailwind + Zustand |
| Rust | Stable via rustup; `rustfmt` + `clippy -D warnings` |
| Media | FFmpeg 6+ on PATH (abstracted behind `avid-media`) |
| Local AI | Ollama + OpenAI-compatible endpoints; local Whisper (Phase 5) |

## Commands (once Phase 1 wires them)

```bash
./scripts/verify-scaffold.sh   # no toolchain needed; checks structure + docs
npm install                    # Phase 1: install workspaces
npm run typecheck              # Phase 1: tsc --noEmit
npm run lint                   # Phase 1: eslint
npm test                       # Phase 1: vitest
cargo fmt --check              # Phase 1: rustfmt gate
cargo clippy -- -D warnings    # Phase 1: clippy gate
cargo test                     # Phase 1+: per-crate + workspace tests
npm run tauri:dev              # Phase 1: desktop shell
```

Until Phase 1 lands, only `verify-scaffold.sh` is expected to pass.

## Workspace layout

- `apps/desktop` — Tauri app (TS frontend + `src-tauri` Rust shell).
- `crates/*` — Cargo workspace members; `avid-core` has zero domain deps (others depend on it).
- `packages/*` — npm workspaces; `shared-types` mirrors Rust contracts; `design-system` is the only place for tokens.

## Troubleshooting

- `rustc/cargo not found` → install via `rustup` (Phase 0.7 will document the pinned version + Tauri system deps per OS). Scaffold intentionally does not require Rust.
- Case-sensitive filename: canonical spec file is `AGENTS.md`. On case-insensitive macOS checkouts `AGENTS.MD` and `AGENTS.md` are the same file — do not create both.
- Large media in git → don't. Use `fixtures/` manifests + tiny generated samples only.
