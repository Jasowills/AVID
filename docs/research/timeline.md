# Timeline / editor libraries research (Phase 0.3)

> Researched 2026-09-20. Decision: custom timeline model (Rust) + custom canvas UI; reuse utilities only.
> Recorded in `docs/decisions/ADR-004-timeline-model.md`.

## Verdict up front

No OSS web library is a full NLE timeline. **Build custom** (Rust model + React canvas renderer); reuse only waveform/playback helpers.

## Timeline core: custom canvas

- **Custom canvas (recommended for tracks/clips):** hundreds of clips, zoom/scroll/snapping at 60 fps; DOM collapses beyond ~200 nodes. Cost: implement hit-testing, drag, a11y roles yourself.
- **DOM-based timelines / OSS NLE UIs:** none mature enough for Resolve/Premiere-grade magnetic editing. Skip.
- **wavesurfer.js v7** (BSD-3-Clause, ~838k weekly DL): reuse for **waveform display only**. Canvas + WebAudio + plugins (Regions, Timeline, Minimap, Hover, Envelope). Do NOT reuse its playback as timeline clock; it decodes full audio in-browser — fails on large files. AVID pattern: Rust/FFmpeg pre-compute peaks → `peaks` option or custom canvas. Migrate hot path to custom canvas if scroll jank appears in the large-project test.
- **video.js / shaka-player** (Apache-2.0): built for HLS/DASH streaming, DRM, ABR — wrong problem for local-file Tauri editor. Use native `<video>` + `<canvas>` preview. Skip both (bundle saving).
- **Remotion** (core MIT, bundled FFmpeg binary GPLv2+, Chromium BSD): React-to-video programmatic renderer. Excellent for deterministic graphics/caption preview and template scenes, but heavy and not a timeline editor. Reuse selectively for visual-scene preview/export pipeline, not interaction.

## Waveforms + captions

Waveforms: wavesurfer.js v7 with server-side peaks → custom canvas if needed. Captions: WebVTT/SRT are the open standards (native `<track>`, FFmpeg `subtitles` burn-in). Only a small SRT/VTT parser + style model needed. No proprietary caption components.

## Deterministic diagram rendering (Explain → Visualize)

Needs editable nodes (move DB right = mutate spec, not regenerate PNG).

| Option | License | Verdict |
|---|---|---|
| Custom SVG + AVID Scene Spec | — (ours) | **Core.** Fully editable, zero license risk, smallest bundle, 1:1 with render graph. Cost: build layout + themes. The signature path. |
| Mermaid (text → SVG: flowcharts, sequences, Gantt) | MIT | **AI authoring language.** LLMs emit text reliably; diffable. Compile to Scene Spec, not final renderer. Lazy-load (bundle is large). |
| React Flow / @xyflow/react (interactive node editor) | MIT | **Diagram editor canvas** wrapping Scene Spec. Moderate bundle, justified. |
| tldraw (infinite canvas, `@tldraw/mermaid`) | Custom SDK license — NOT OSI; free with watermark, paid removal | **Reject for MVP core** (license/watermark + churn). Re-evaluate post-MVP as optional plugin. |
| Excalidraw embed | MIT | Wrong visual language (hand-drawn) for technical architecture; JSON diffs unwieldy. Reject. |

Pipeline: **Mermaid (AI authoring) → Scene Spec (source of truth) → React Flow (editing) → SVG/FFmpeg overlay (render).**
