---
description: Security review (filesystem, processes, credentials, uploads)
---
# /security-review

1. Grep for: shell execution, path joins with external input, logged secrets, network sends.
2. Verify: traversal guards, AI-output validation before state mutation, key handling, consent paths.
3. Verdict ranked by severity with exact lines.
