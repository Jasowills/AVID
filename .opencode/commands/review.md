---
description: Implementation quality review of the working tree
---
# /review

1. `git status`, `git diff` — review every hunk.
2. Check: boundaries, naming, dead code, error paths, test coverage (incl. rejections), docs updated, PROGRESS.md updated.
3. Verdict: must-fix vs nice-to-have, file + line each. No approval with failing suites.
