---
description: Boundary audit against ARCHITECTURE.md + ADRs
---
# /check-architecture

1. Grep for violations: raw ffmpeg outside `avid-media`, provider SDKs outside `avid-ai` adapters, AI state mutation outside commands, untyped IPC.
2. Report file + line per violation with the ADR it breaks.
