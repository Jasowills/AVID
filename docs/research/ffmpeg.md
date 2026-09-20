# FFmpeg integration research (Phase 0.3)

> Researched 2026-09-20. Primary: `ffmpeg.org` docs/legal, `crates.io/crates/ffmpeg-next`, `docs.rs/ffmpeg-next`.
> Decision recorded in `docs/decisions/ADR-002-rust-media-core.md`.

## Three approaches compared

### (1) Bundled sidecar ffmpeg+ffprobe, one CLI per operation — RECOMMENDED for MVP

Ship version-pinned static builds (e.g. ffmpeg 7.1/8.x LGPL builds) and spawn via `tokio::process` from `avid-media`. Parse `ffprobe -print_format json`, `ffmpeg -progress pipe:1`. Helpers: `ffmpeg-sidecar` crate or `tauri-plugin-ffmpeg` progress-callback pattern.

### (2) Linked libav* in-process (`ffmpeg-next` 9.0.0 / `rsmpeg` / `ez-ffmpeg`)

Direct `libavformat/avcodec/avfilter/avutil` bindings. Full control: custom IO, packet surgery, zero-copy frame push.

### (3) Persistent streaming sidecar

Single long-lived ffmpeg piping raw frames over stdout, or local HTTP job server. Middle ground.

| Dimension | (1) CLI sidecar | (2) Linked libav | (3) Streaming sidecar |
|---|---|---|---|
| Licensing | Safest. Unmodified LGPL binary = attribution + source offer. Stays LGPL if builds avoid `--enable-gpl/--enable-nonfree`. `libx264/x265` GPL encoders infect → use `libopenh264` / native HW or go GPL-compliant. | Highest risk. Static link pulls LGPL into Rust binary; GPL filters/encoders force whole app GPL (dynamic link + source hosting + EULA carve-out). `ffmpeg-next` is maintenance-only; tracks FFmpeg 3.4–8.0. | Same as (1). |
| Bundling (Tauri) | ~25–80 MB per arch. `externalBin` + shell allowlist. No build-time clang/pkg-config. CI trivial. Signing/notarization + size cost. | ~5–15 MB linked, but build hell: clang/pkg-config/libav-dev per OS, version must match system FFmpeg major. Breaks clean `cargo build`. | Same as (1), one binary reused. |
| Performance | Spawn ~10–50 ms; fine for batch jobs. Bad for 60 fps scrub (re-spawn per frame). | Best for preview: decode → texture, no copy, custom seek index. | Good: one decode → thumbs+waveform+proxy in a single pass. |
| Error handling | Excellent isolation: crash ≠ app crash. Parse stderr + exit code → `AVID_MEDIA_xxx`. Pin version (string parsing is version-fragile). Sanitize paths, timeout, kill on cancel. | Excellent granularity, terrible robustness: segfault in unsafe bindings = app crash. `send_packet/receive_frame` ~40 lines before first frame. | Good isolation; handle broken-pipe/backpressure. |
| Alternative | `gstreamer-rs`: real option (in-process, good HW, appsink/appsrc) but different pipeline model, plugin-licensing maze (`ugly/bad`), larger bundle, smaller hiring pool. Reject for MVP. | | |

## Operation → CLI mapping (MVP; raw strings live ONLY in `avid-media`)

| AVID op | CLI |
|---|---|
| `probe` | `ffprobe -v quiet -print_format json -show_format -show_streams -show_chapters file` |
| `proxy` | `ffmpeg -hwaccel auto -i in -vf scale=-2:540 -c:v … -c:a aac -movflags +faststart proxy.mp4` |
| `thumbnails` | `ffmpeg -ss 5 -i in -frames:v 1 -vf "thumbnail,scale=320:-1" thumb.jpg`; sprite via `fps=1/5,scale=160:-1,tile=4x4` |
| `waveform` | `ffmpeg -i in -map 0:a -ac 1 -ar 8000 -f f32le -` → downsample to ~200 peaks/s JSON; or `showwavespic` PNG |
| `extractAudio` | `ffmpeg -i in -vn -ac 1 -ar 16000 -c:a pcm_s16le out.wav` (Whisper input) |
| `extractFrame` | `ffmpeg -ss <t> -i in -frames:v 1 -q:v 2 frame.png` (fast-seek before `-i`, accurate after) |
| `concat` | Same-codec: `concat demuxer -f concat -safe 0 -c copy`; else `concat`/`xfade` filters |
| `composite/overlay` | `-filter_complex "[0:v][1:v]overlay=W-w-20:H-h-20:format=auto,scale,format=yuv420p"` |
| `burnCaptions` | `subtitles=file.srt:force_style=…` (needs `libass` build) else `drawtext=…` (escape `'`, `:`, `\`; never pass raw AI text) |
| `final render` | Render graph → single `ffmpeg -filter_complex` DAG + `-c:v libx264/hw -pix_fmt yuv420p -c:a aac -movflags +faststart -shortest out.mp4` |

## Hardware acceleration (export/proxy only — never required for correctness)

| OS | Path | Breaks |
|---|---|---|
| macOS (VideoToolbox) | `h264_videotoolbox`, `-hwaccel videotoolbox` | ~10–20% worse size/quality vs libx264; overlay/subtitles force HW→CPU download; some static builds lack videotoolbox — verify `ffmpeg -encoders \| grep videotoolbox`. Always fallback libx264. |
| Windows (NVENC > QSV > AMF) | Auto-probe in order | Wrong-GPU picks, CUDA DLL hell if bundling CUDA FFmpeg, AMF variance, QSV needs Media SDK. `try hw → log → fallback`. |
| Linux (VAAPI/NVENC/v4l2m2m) | `-hwaccel vaapi -hwaccel_device /dev/dri/renderD128` | Most fragile: missing `/dev/dri`, Mesa vs proprietary, Wayland sandbox denies device. Always offer software path; CI with no-GPU must prove fallback. |

Log the encoder used in every job receipt for bug reports.

## Deterministic fixtures (generate with ffmpeg itself — no binaries in git)

`scripts/make-fixtures.sh` (Phase 2): `testsrc2`+`sine` talking-head, `color`+`anullsrc` silence, `smptebars` vertical, 4K proxy trigger, `head -c` corrupt sample, `ffprobe` JSON golden. Verify: exists, duration ±0.1 s, streams, thumbnail non-black (histogram), waveform ~0 for silence. `drawtext` burn-in timecode for frame-accuracy asserts.

## Recommendation

MVP on **(1) version-pinned sidecar + `MediaEngine` abstraction**. Add (3) streaming pipe only if profiling demands for preview/waveform. Defer linked `ffmpeg-next` until post-MVP preview engine. Licensing: stay LGPL (no `--enable-gpl`, no `libx264` in distributed LGPL build or go GPL-compliant; never `--enable-nonfree`).
