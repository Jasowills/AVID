# AVID Project Format (AGENTS §12, ADR-003)

Implementation: `crates/avid-project` (+ `Session` in `apps/desktop/src-tauri/src/session.rs`).

## Layout

`project.json` + `media/` + `proxies/` + `exports/` (+ `audio-cache/` outside the project, in app cache).

## Manifest

`{ id, version (current: 1), meta { name, canvas, frameRate, resolution, templateId, createdAt, updatedAt }, timeline (avid-timeline JSON), assets[], transcripts {} }`.
`MediaAsset { id, file_name, relative_path (never absolute/`..`), duration, dimensions, hash (reserved), proxy_path }`.

## Rules

- Forward migrations only (`migrate()`); newer versions open read-only with `AVID_PROJECT_002`, never silent breakage.
- New optional fields use `#[serde(default)]` (precedent: `transcripts`, `proxy_path`) so old files always load.
- Paths validated (`AVID_PROJECT_003`); relink-by-hash is future work (§114).
- Session persists after every committed mutation (autosave); reopen rebuilds timeline + undo starts fresh (crash-recovery foundation).

## Tests

Round-trips, future-version rejection, traversal/absolute rejection (incl. Windows), shipped-example parses as manifest AND timeline.
