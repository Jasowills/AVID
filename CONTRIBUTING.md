# Contributing to AVID

> AVID is collaborator-ready by design. Read `AGENTS.MD`, `PLAN.md`, and `PROGRESS.md` before writing code.

## 1. Start here (mandatory)

1. Read `AGENTS.MD` (full spec, §§0–163).
2. Read `PLAN.md` (strict order + gates) and `PROGRESS.md` (current phase).
3. Check relevant ADRs in `docs/decisions/` and research in `docs/research/`.
4. Run `./scripts/verify-scaffold.sh` to confirm a clean tree.

## 2. Clean-code rules

- **Boundaries are law:** UI ↔ Rust via typed IPC (`packages/shared-types`). FFmpeg only inside `crates/avid-media`. Provider logic only inside `crates/avid-ai/**/adapters`. Template I/O only via `packages/templates` validator. No shortcuts.
- **No slop:** no `any` (TS) without a tracked TODO + issue; no `.unwrap()`/`.expect()` (Rust) outside tests; no god-components (>300 lines needs a split); no duplicated stringly-typed FFmpeg/provider logic; no dead code or commented-out blocks.
- **Naming:** files `kebab-case`, TS types `PascalCase`, functions `camelCase`, Rust modules `snake_case`. Commands end in `Command` (`TrimClipCommand`). Errors use `AVID_<DOMAIN>_<NNN>` codes.
- **Docs on public APIs:** every exported function/type/command gets a doc comment with purpose, args, errors, and an example where non-trivial.
- **Design system only:** no ad-hoc Tailwind values for colors/spacing/typography — use `packages/design-system` tokens. Dark-first, calm, professional (AGENTS §60–61).
- **Strings:** user-facing text goes through the i18n-ready path (no hard-coded copy deep in logic); product language is plain ("Remove pauses", not "AI Magic").

## 3. Branches, commits, PRs

- Branches: `feat/<scope>-<short>` / `fix/<scope>-<short>` / `docs/<…>` / `chore/<…>`.
- Commits (Conventional, one system per commit):
  - `feat(timeline): add split command`
  - `fix(render): preserve audio stream`
  - `test(timeline): add trim coverage`
  - `docs(plan): clarify phase gate`
- PRs: link the PLAN phase + PROGRESS row; include tests + docs; include screenshots/recordings for UI; confirm Definition of Done checklist (AGENTS §135) or state explicitly what is still missing. No mocked functionality merged as "done".
- Reviews: at least one reviewer; `reviewer`/`security`/`ux` agents for their domains. Address findings before merge.

## 4. Testing (part of implementation, not a final phase)

- Rust: `cargo test -p <crate>` — timeline math, trim/split, commands, undo/redo, serialization, migrations, render graph, path safety. Property tests where useful.
- TS: unit + component tests; schema validation tests for edit-plan and scene-spec (including malformed/hallucinated-input rejection).
- E2E: real workflow (create → import → transcribe → cut → caption → visual → export → verify).
- Fixtures only from `fixtures/` (deterministic). Never commit large binaries or copyrighted media.

## 5. Security & privacy

- Never execute AI-generated shell. Validate/normalize every path; reject `..` traversal; allowlist project dirs. Never log keys, tokens, or private transcripts. Cloud AI only with explicit per-operation consent. Report security issues privately (see `docs/SECURITY.md` when it lands in Phase 0).

## 6. Dependencies

Before adding one: check maintenance, license, platform support, bundle size, security, activity, alternatives — and record the decision (AGENTS §89 + `docs/LEGAL_AND_LICENSING.md`). Pin versions. Prefer std + small well-maintained crates/packages over frameworks.

## 7. What will get your PR sent back

Raw FFmpeg strings outside `avid-media`; provider SDKs imported in UI; unvalidated AI JSON touching state; missing undo/persistence; fake "done" without tests; giant multi-system commits; hard-coded copy that breaks i18n; telemetry without opt-in.
