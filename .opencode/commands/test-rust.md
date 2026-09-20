---
description: fmt, clippy, and tests for the Rust workspace
---
# /test-rust — $ARGUMENTS

1. `cargo fmt --all -- --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace` (add `-- --ignored` + fixtures for live paths: $ARGUMENTS)
