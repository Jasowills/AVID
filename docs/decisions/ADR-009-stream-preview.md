# ADR-009 — Seekable preview via `stream://` Range protocol

- **Status:** accepted (autopilot pass 8)
- **Date:** 2026-09-20
- **Context:** Tauri's `asset://` has no HTTP Range support, so `<video>` seeking breaks on macOS/Linux webviews (research risk #1). Preview must seek proxy and original files without routing bytes through IPC.
- **Options:** (A) Custom `stream://` protocol with single-range 206 support. (B) Local HTTP server (`tauri-plugin-localhost`). (C) `convertFileSrc` + accept broken seeking (rejected — core UX).
- **Decision:** **(A).** `stream://project/<rel>` (open project dir) and `stream://cache/<rel>` (app cache). Pure core (`parse_range`, `serve_range` 200/206/416, `resolve_stream_url` with traversal + symlink containment, MIME map) fully unit-tested; Tauri handler is thin glue reading files fully into memory (preview targets are proxies — range-sliced IO is the documented follow-up).
- **Consequences:** Seek works on all OSes; full-file reads bound preview size in practice to proxies. Multi-range requests rejected (video elements never send them).
- **References:** `apps/desktop/src-tauri/src/stream.rs`; `docs/research/tauri.md` §5.
