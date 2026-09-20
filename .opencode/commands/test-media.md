---
description: Fixture-gated media verification (probe, render, transcription)
---
# /test-media

1. `./scripts/make-fixtures.sh`
2. `cargo test -p avid-media -- --ignored && cargo test -p avid-render -- --ignored`
3. `./scripts/verify-transcription.sh`
4. Confirm no fixture binaries leak into `git status` (gitignored by design).
