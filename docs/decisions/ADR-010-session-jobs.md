# ADR-010 — Backend session with autosave + job registry

- **Status:** accepted (autopilot passes 3–5)
- **Date:** 2026-09-20
- **Context:** Project truth must survive crashes, long operations must never freeze the UI, and every background task needs visible progress + cancel (AGENTS §49–52).
- **Options:** (A) Session object (manifest + live timeline + undo stack) with persist-on-mutation, plus a central job registry with Channel progress. (B) Frontend-owned truth with Rust as a thin FFI (rejected — splits authority, breaks crash recovery). (C) SQLite project store (rejected — opaque, harder to hand-repair; see ADR-003).
- **Decision:** **(A).** `Session::{mutate, mutate_group}` persist `project.json` after every committed change; reopening rebuilds state (recovery foundation). `JobRegistry` tracks start/progress/finish/fail/cancel; render streams real ffmpeg fractions, transcription registers indeterminate jobs; workers check cancel flags (ffmpeg child killed promptly). Commands snapshot under the lock and render off-lock.
- **Consequences:** At most the in-flight op is lost on crash. Long ops need Channel-capable commands; sync commands are reserved for sub-second work.
- **References:** `apps/desktop/src-tauri/src/{session,jobs}.rs`; AGENTS §52–53, §115–117.
