# AVID Testing (AGENTS §91–104)

Testing is part of implementation — every feature ships with its tests (counts: 74 Rust + 25 TS at last audit).

## Layers

- **Rust unit** (`cargo test --workspace`): timeline math/commands/undo, manifest validation + migrations, probe parsing, graph lowering, registry/adapters (mock transport), validators.
- **Live ignored** (`-- --ignored`, need local toolchain): real ffprobe, real render + verify, real whisper (sidecar + in-process), live Ollama eval, silence detection, session import/transcribe/export slices. CI-safe (skip without fixtures) and fail-LOUD on missing prerequisites (a silent early-return once hid a wrong path — policy since).
- **TS** (`vitest run` per workspace): validation schemas + adversarial cases, template dir validation, stores, pure UI helpers.
- **Fixtures** (`scripts/make-fixtures.sh`): talking-head, silence, vertical, corrupt — generated, never committed. AI eval pairs in `fixtures/ai/prompts/`.

## Gates

`cargo fmt --check`, `clippy -D warnings` (incl. pedantic), `tsc --noEmit`, `vite build`, `verify-scaffold.sh`, `verify-transcription.sh`. CI runs scaffold + node + rust jobs.

## Gaps (tracked, not hidden)

No browser E2E (Playwright), no network-chaos suite, no large-project stress run, no screen-reader pass. Render tests assert streams/duration — never brittle bytes.
