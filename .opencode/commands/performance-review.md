---
description: Performance review (main thread, bundle, render costs)
---
# /performance-review

1. Identify main-thread blocking (sync commands doing IO/inference), unmeasured heavy deps, missing progress.
2. Measure where possible; otherwise mark as need-measurement, never as fine.
3. Report with costs (CPU/RAM/disk/bundle/startup) per finding.
