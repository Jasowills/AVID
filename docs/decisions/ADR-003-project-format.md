# ADR-003 — Versioned project format

- **Status:** accepted (Phase 0.5)
- **Date:** 2026-09-20
- **Context:** Projects must survive app updates, move across macOS/Windows/Linux, and recover from crashes. A project must never become unusable because the app DB changed (AGENTS §12).
- **Options:** (A) Versioned `project.json` manifest + relative media paths + migrations + snapshots (portable directory: media/proxies/thumbnails/audio/cache/assets/generated/exports). (B) SQLite-only project (fast queries, opaque, harder to hand-repair/version). (C) Opaque binary blob (rejected — untrappable).
- **Decision:** **(A)** owned by `avid-project`. Manifest holds id, version, metadata, timeline, tracks, clips, assets, compositions, captions, transcripts, AI ops, template refs, render settings. Forward migrations only; unknown future versions open read-only with a clear message, never silent breakage. Relative paths; missing media → Locate/Relink with hash/size/duration matching. Lightweight snapshot before every major AI op; autosave + crash recovery ("recovered from N minutes ago").
- **Consequences:** Slower than SQLite for huge projects — mitigate with sidecar indexes later, manifest stays source of truth. Every schema change ships a migration + test.
- **References:** AGENTS §12, §52–53, §113–114, §117.
