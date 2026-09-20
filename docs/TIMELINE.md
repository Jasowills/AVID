# AVID Timeline (AGENTS §13–15)

Source of truth for timeline behavior. Implementation: `crates/avid-timeline`.

## Model (`model.rs`)

- `Track { id, kind: Video|Audio|Text|Graphics|Caption, index, name, locked, muted }`
- `Clip { id, source_media_id, track_id, start, duration, in_point, name }` — a non-destructive window into source media. No out-point/transform/opacity/effects **yet** (tracked gap vs §13 — add with render support, not before).
- `Timeline { tracks, clips: BTreeMap }`. Same-track overlap rejected (abutting legal, ε=1e-9). Locked tracks reject edits. `duration()` = max clip end.

## Commands (`commands.rs`)

`AddClip / RemoveClip / TrimClip (moves source window) / MoveClip / SplitClip (second half = {id}-b) / RemoveRange (split boundaries + remove interior, all tracks)`. Trait: `execute / undo`, `Debug + Send`, serde-serializable data. AI touches the timeline only through these (ADR-008).

## Undo (`undo.rs`)

`UndoStack`: single steps, `execute_group` (transactional — failed groups roll back), manual `begin/close_group`, redo cleared on new work. AI ops = one labeled group (`AI: <goal>`).

## Tests

22 tests: validation, overlap, trim source-window restore, split halves, range cuts incl. multi-clip, group atomicity, serialization round-trips.
