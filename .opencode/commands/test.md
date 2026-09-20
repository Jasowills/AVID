---
description: Run the full verification sweep
---
# /test

1. `./scripts/verify-scaffold.sh`
2. `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`
3. `npm run typecheck/test/build` for `@avid/desktop`, `@avid/ai-protocol`, `@avid/templates`
4. Live ignored tests where services exist (`-- --ignored` per crate, fixtures built).
5. Report green/red per suite with exact failures. Never claim green without output.
