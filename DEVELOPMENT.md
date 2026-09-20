# AVID — Development Guide

> Toolchain pins: `rust-toolchain.toml` (Rust 1.98.1 + rustfmt/clippy), `.nvmrc` (Node 20), `package.json` (`npm@11`, `engines: node >= 20`).
> Resolved Phase 1 versions (recorded 2026-09-20): React 18, react-router 7, Zustand 5, Vite 6.4.3, Vitest 3.2.7, Tailwind v4, TypeScript 5, Tauri 2.11.x (backend wiring pending).

## Commands

```bash
./scripts/verify-scaffold.sh   # no toolchain needed; checks structure + docs
npm install                    # install workspaces (root)
npm run dev                    # desktop shell (Vite, http://localhost:1420)
npm run typecheck              # tsc --noEmit on @avid/desktop
npm test                       # vitest run on @avid/desktop
npm run build                  # typecheck + vite production build
cargo fmt --check              # rustfmt gate
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace         # per-crate unit tests
npm run tauri:dev              # NOT WIRED — exits 1 until src-tauri backend exists
```

CI (`.github/workflows/ci.yml`) runs scaffold-integrity, node (typecheck/test/build), and rust (fmt/clippy/test) on every push/PR.

## Workspace layout

- `apps/desktop` — React frontend (Vite dev; Tauri webview target). HashRouter (file:// safe).
- `crates/*` — Cargo workspace members; `avid-core` has zero domain deps (others depend on it).
- `packages/*` — npm workspaces; `shared-types` mirrors Rust contracts; `design-system` is the only place for tokens; `ui` imports design-system only.

## Conventions that bite

- Tailwind v4 `@theme` in `apps/desktop/src/index.css` mirrors `packages/design-system` tokens — update both.
- Unwired controls must render **disabled with an honest `title`** naming the phase that wires them. Never a fake working button (AGENTS §136).
- Project persistence is localStorage until `avid-project` (ADR-003) lands — the `ProjectConfig` shape is already manifest-compatible.
- Case-sensitive filename: canonical spec file is `AGENTS.md`. On case-insensitive macOS checkouts don't create a second spelling.

## Troubleshooting

- `rustc/cargo not found` → `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal` then `rustup component add rustfmt clippy`. CI installs `rust-toolchain.toml` exactly.
- `esbuild` install-script warnings → `npm install-scripts approve esbuild` if `vite build` fails on esbuild binary.
- Large media in git → don't. Use `fixtures/` manifests + tiny generated samples only.
