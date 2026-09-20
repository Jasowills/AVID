# AVID Rendering (AGENTS §62–65)

Implementation: `crates/avid-render` + `MediaEngine` runners.

## Pipeline

`Timeline → graph_from_timeline (video tracks, start-ordered) → RenderGraph { segments, overlays, width, fps } → argv (single ffmpeg DAG: per-input -ss/-t, concat filter, optional drawtext chain, scale, yuv420p, libx264 + aac, faststart) → run → ffprobe verification (duration ±0.75 s)`.

## Rules

- Deterministic: same graph → same argv (tested).
- Drawtext overlay text is escaped (`\ ' : %`); user text never reaches a shell.
- `argv_compat` + `AVID_RENDER_005` for ffmpeg builds without text filters (e.g. Homebrew without libfreetype); `ffmpeg_supports_text` probes; `argv_auto` picks.
- Segments are expected to carry audio (camera recordings do); mixed timelines fail loudly at concat, not silently.
- Presets: YouTube 1080p/4K, Short 1080p, Custom (range-checked). Scale preserves aspect — no fake reframing.

## Tests

Unit (determinism, escaping, validation, presets, filenames, graph ordering) + LIVE render (concat 2 windows → verified 4 s h264+aac).
