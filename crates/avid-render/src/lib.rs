//! `avid-render`: timeline compiler → render graph → FFmpeg (ADR-006).
//!
//! The timeline compiles to a deterministic [`RenderGraph`]; the graph
//! lowers to exactly one FFmpeg invocation. Future engines plug in behind
//! the graph — the editor never emits raw filter strings (those live only
//! in [`RenderGraph::argv`], next to `avid-media`'s builders).

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Render pipeline failures.
#[derive(Debug, Error)]
pub enum RenderError {
    /// No segments to render.
    #[error("render graph has no inputs")]
    Empty,
    /// A segment window is invalid (negative seek, non-positive duration).
    #[error("invalid segment window: {0}")]
    InvalidSegment(String),
    /// Overlay text or timing is invalid.
    #[error("invalid overlay: {0}")]
    InvalidOverlay(String),
    /// The ffmpeg subprocess failed.
    #[error("render failed (exit {exit}): {stderr}")]
    ProcessFailed {
        /// Exit code, if any.
        exit: String,
        /// Captured stderr (truncated).
        stderr: String,
    },
    /// Export request invalid (unknown preset, bad filename, …).
    #[error("invalid export request: {0}")]
    InvalidExport(String),
    /// The graph has burned-in text overlays but this ffmpeg build has no
    /// text filters (`drawtext`/`subtitles`). System builds (e.g. Homebrew
    /// without libfreetype/libass) often lack them — use a full build or the
    /// pinned AVID sidecar (ADR-002). The timeline data is unaffected.
    #[error("this ffmpeg cannot burn in text: {hint}")]
    TextOverlaysUnsupported {
        /// Actionable guidance for the user.
        hint: String,
    },
}

impl RenderError {
    /// Stable error code for UI mapping and logs.
    #[must_use]
    pub fn code(&self) -> &'static str {
        match self {
            Self::Empty => "AVID_RENDER_001",
            Self::InvalidSegment(_) => "AVID_RENDER_002",
            Self::InvalidOverlay(_) => "AVID_RENDER_003",
            Self::ProcessFailed { .. } => "AVID_RENDER_004",
            Self::TextOverlaysUnsupported { .. } => "AVID_RENDER_005",
            Self::InvalidExport(_) => "AVID_RENDER_006",
        }
    }
}

/// One timeline segment: a window into a source file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Segment {
    /// Source file (project-relative).
    pub path: PathBuf,
    /// Seek into the source, seconds.
    pub seek: f64,
    /// Length to take, seconds.
    pub duration: f64,
}

/// A burned-in text overlay bound to a timeline range.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextOverlay {
    /// Display text (escaped for `drawtext` at lowering time).
    pub text: String,
    /// Timeline start, seconds.
    pub start: f64,
    /// Timeline end, seconds.
    pub end: f64,
    /// Font size in pixels.
    pub fontsize: u32,
}

/// Deterministic render graph: ordered segments + overlays + output settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenderGraph {
    pub segments: Vec<Segment>,
    #[serde(default)]
    pub overlays: Vec<TextOverlay>,
    /// Output width; height follows the first input (`-2` preserves aspect).
    pub width: u32,
    /// Frames per second.
    pub fps: u32,
}

impl RenderGraph {
    /// Validate windows, ordering, and overlay ranges.
    pub fn validate(&self) -> Result<(), RenderError> {
        if self.segments.is_empty() {
            return Err(RenderError::Empty);
        }
        for segment in &self.segments {
            if !segment.seek.is_finite() || segment.seek < 0.0 {
                return Err(RenderError::InvalidSegment(format!(
                    "seek = {}",
                    segment.seek
                )));
            }
            if !segment.duration.is_finite() || segment.duration <= 0.0 {
                return Err(RenderError::InvalidSegment(format!(
                    "duration = {}",
                    segment.duration
                )));
            }
        }
        let total: f64 = self.segments.iter().map(|s| s.duration).sum();
        for overlay in &self.overlays {
            if overlay.text.trim().is_empty() {
                return Err(RenderError::InvalidOverlay("empty text".to_owned()));
            }
            if !(0.0 <= overlay.start && overlay.start < overlay.end && overlay.end <= total) {
                return Err(RenderError::InvalidOverlay(format!(
                    "range {}..{} outside 0..{total}",
                    overlay.start, overlay.end
                )));
            }
            if overlay.fontsize == 0 {
                return Err(RenderError::InvalidOverlay("fontsize = 0".to_owned()));
            }
        }
        Ok(())
    }

    /// Total output duration (sum of segments).
    #[must_use]
    pub fn total_duration(&self) -> f64 {
        self.segments.iter().map(|s| s.duration).sum()
    }

    /// Lower the graph to one FFmpeg invocation (deterministic: same graph,
    /// same argv). `argv[0]` is the ffmpeg binary itself, so the result can
    /// be spawned directly: `Command::new(&argv[0]).args(&argv[1..])`.
    ///
    /// Requires a drawtext-capable ffmpeg when `overlays` is non-empty; use
    /// [`RenderGraph::argv_compat`] for builds without text filters.
    pub fn argv(&self, ffmpeg: &Path, output: &Path) -> Result<Vec<String>, RenderError> {
        self.lower(ffmpeg, output, true)
    }

    /// Lower for ffmpeg builds without text filters (`drawtext`/`subtitles`).
    /// Succeeds only when `overlays` is empty; otherwise returns
    /// [`RenderError::TextOverlaysUnsupported`] with user guidance instead of
    /// a cryptic `Filter not found` failure.
    pub fn argv_compat(&self, ffmpeg: &Path, output: &Path) -> Result<Vec<String>, RenderError> {
        if !self.overlays.is_empty() {
            return Err(RenderError::TextOverlaysUnsupported {
                hint: "burned-in captions need an ffmpeg with drawtext/subtitles (libfreetype/libass); \
                       use the AVID sidecar build or export without overlays"
                    .to_owned(),
            });
        }
        self.lower(ffmpeg, output, false)
    }

    fn lower(
        &self,
        ffmpeg: &Path,
        output: &Path,
        _with_text: bool,
    ) -> Result<Vec<String>, RenderError> {
        self.validate()?;
        let mut args: Vec<String> = vec![ffmpeg.display().to_string(), "-y".to_owned()];
        for segment in &self.segments {
            args.push("-ss".to_owned());
            args.push(segment.seek.to_string());
            args.push("-t".to_owned());
            args.push(segment.duration.to_string());
            args.push("-i".to_owned());
            args.push(segment.path.display().to_string());
        }
        let n = self.segments.len();
        // [0:v][0:a][1:v][1:a]…concat → optional drawtext chain → scale/format.
        let mut filter = String::new();
        for i in 0..n {
            filter.push_str(&format!("[{i}:v][{i}:a]"));
        }
        filter.push_str(&format!("concat=n={n}:v=1:a=1[vcat][aout];"));
        let mut current = "[vcat]".to_owned();
        for (i, overlay) in self.overlays.iter().enumerate() {
            let next = format!("[vov{i}]");
            filter.push_str(&format!(
                "{current}drawtext=text='{text}':fontsize={fs}:x=(w-text_w)/2:y=h-120:enable='between(t,{s},{e})'{next};",
                current = current,
                text = escape_drawtext(&overlay.text),
                fs = overlay.fontsize,
                s = overlay.start,
                e = overlay.end,
                next = next,
            ));
            current = next;
        }
        filter.push_str(&format!(
            "{current}scale={w}:-2,format=yuv420p[vout]",
            w = self.width
        ));
        args.push("-filter_complex".to_owned());
        args.push(filter);
        args.extend([
            "-map".to_owned(),
            "[vout]".to_owned(),
            "-map".to_owned(),
            "[aout]".to_owned(),
            "-r".to_owned(),
            self.fps.to_string(),
            "-c:v".to_owned(),
            "libx264".to_owned(),
            "-preset".to_owned(),
            "veryfast".to_owned(),
            "-c:a".to_owned(),
            "aac".to_owned(),
            "-movflags".to_owned(),
            "+faststart".to_owned(),
            "-shortest".to_owned(),
            output.display().to_string(),
        ]);
        Ok(args)
    }
}

/// Escape user text for `drawtext=text='…'`: backslash, quote, colon, percent.
fn escape_drawtext(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('\'', "\\'")
        .replace(':', "\\:")
        .replace('%', "\\%")
}

/// Probe whether an ffmpeg binary offers text filters (`drawtext` or
/// `subtitles`). Returns `false` for missing/unrunnable binaries — never panics.
///
/// System builds vary (Homebrew without libfreetype/libass has neither);
/// the pinned AVID sidecar guarantees them (ADR-002).
#[must_use]
pub fn ffmpeg_supports_text(ffmpeg: &Path) -> bool {
    let output = std::process::Command::new(ffmpeg)
        .arg("-hide_banner")
        .arg("-filters")
        .output();
    let Ok(output) = output else { return false };
    if !output.status.success() {
        return false;
    }
    let list = String::from_utf8_lossy(&output.stdout);
    list.lines().any(|line| {
        let name = line.split_whitespace().nth(1).unwrap_or("");
        name == "drawtext" || name == "subtitles"
    })
}

/// Lower with automatic text-filter fallback: full [`RenderGraph::argv`]
/// when the ffmpeg offers text filters, [`RenderGraph::argv_compat`]
/// otherwise (empty overlays lower identically either way; present overlays
/// on incapable builds fail loudly via `AVID_RENDER_005`).
pub fn argv_auto(
    ffmpeg: &Path,
    output: &Path,
    graph: &RenderGraph,
) -> Result<Vec<String>, RenderError> {
    if ffmpeg_supports_text(ffmpeg) {
        graph.argv(ffmpeg, output)
    } else {
        graph.argv_compat(ffmpeg, output)
    }
}

// ---------------------------------------------------------------------------
// Caption sidecars (SRT export — Phase 7 captions data path)
// ---------------------------------------------------------------------------

/// A caption cue extracted from a caption-track clip.
#[derive(Debug, Clone, PartialEq)]
pub struct CaptionCue {
    pub start: f64,
    pub end: f64,
    pub text: String,
}

/// Extract caption cues from caption-kind tracks, ordered by start.
/// Empty-text clips are skipped (they carry no readable content).
pub fn caption_cues_from_timeline(timeline: &avid_timeline::Timeline) -> Vec<CaptionCue> {
    let caption_tracks: Vec<&str> = timeline
        .tracks
        .iter()
        .filter(|track| track.kind == avid_timeline::TrackKind::Caption)
        .map(|track| track.id.as_str())
        .collect();
    let mut cues: Vec<CaptionCue> = timeline
        .clips
        .values()
        .filter(|clip| caption_tracks.contains(&clip.track_id.as_str()))
        .map(|clip| CaptionCue {
            start: clip.start,
            end: clip.start + clip.duration,
            text: clip.name.trim().to_owned(),
        })
        .filter(|cue| !cue.text.is_empty() && cue.end > cue.start)
        .collect();
    cues.sort_by(|a, b| {
        a.start
            .partial_cmp(&b.start)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    cues
}

/// Format cues as SubRip (`.srt`): numbered `HH:MM:SS,mmm --> ...` blocks.
/// Written alongside the render (`<name>.srt`) for players and for later
/// burn-in once a text-capable sidecar is pinned.
pub fn captions_to_srt(cues: &[CaptionCue]) -> String {
    let mut output = String::new();
    for (index, cue) in cues.iter().enumerate() {
        output.push_str(&format!(
            "{}\n{} --> {}\n{}\n\n",
            index + 1,
            srt_timestamp(cue.start),
            srt_timestamp(cue.end),
            cue.text
        ));
    }
    output
}

/// Format seconds as `HH:MM:SS,mmm` (clamped at zero, millis rounded).
fn srt_timestamp(seconds: f64) -> String {
    let total_ms = (seconds.max(0.0) * 1000.0).round() as u64;
    let (hours, remainder) = (total_ms / 3_600_000, total_ms % 3_600_000);
    let (minutes, remainder) = (remainder / 60_000, remainder % 60_000);
    format!(
        "{hours:02}:{minutes:02}:{:02},{:03}",
        remainder / 1000,
        remainder % 1000
    )
}

// ---------------------------------------------------------------------------
// Export presets + timeline compilation (Phase 4 export path)
// ---------------------------------------------------------------------------

/// A named export target (resolution + frame rate; codec is H.264/AAC).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExportPreset {
    /// Stable id for IPC (`"youtube-1080p"`, `"custom"`).
    pub id: &'static str,
    /// UI label.
    pub label: &'static str,
    /// Output width; height follows the source aspect (`scale=W:-2`).
    /// Smart reframing (crop/recompose) is post-MVP — presets never claim it.
    pub width: u32,
    /// Output frame rate.
    pub fps: u32,
}

/// Shipped presets. `custom` is resolved from explicit width/fps.
pub const EXPORT_PRESETS: &[ExportPreset] = &[
    ExportPreset {
        id: "youtube-1080p",
        label: "YouTube 1080p",
        width: 1920,
        fps: 30,
    },
    ExportPreset {
        id: "youtube-4k",
        label: "YouTube 4K",
        width: 3840,
        fps: 30,
    },
    ExportPreset {
        id: "short-1080p",
        label: "Short / Reel 1080p",
        width: 1080,
        fps: 30,
    },
    ExportPreset {
        id: "custom",
        label: "Custom",
        width: 1920,
        fps: 30,
    },
];

const ALLOWED_FPS: &[u32] = &[24, 25, 30, 50, 60];

/// Resolve a preset id (+ optional custom width/fps) to `(width, fps)`.
pub fn resolve_preset(
    preset_id: &str,
    custom_width: Option<u32>,
    custom_fps: Option<u32>,
) -> Result<(u32, u32), RenderError> {
    if preset_id == "custom" {
        let (Some(width), Some(fps)) = (custom_width, custom_fps) else {
            return Err(RenderError::InvalidExport(
                "custom preset needs an explicit width and frame rate".to_owned(),
            ));
        };
        check_dimensions(width, fps)?;
        return Ok((width, fps));
    }
    EXPORT_PRESETS
        .iter()
        .find(|preset| preset.id == preset_id)
        .map(|preset| (preset.width, preset.fps))
        .ok_or_else(|| RenderError::InvalidExport(format!("unknown preset: {preset_id}")))
}

fn check_dimensions(width: u32, fps: u32) -> Result<(), RenderError> {
    if !(240..=7680).contains(&width) {
        return Err(RenderError::InvalidExport(format!(
            "width out of range: {width}"
        )));
    }
    if !ALLOWED_FPS.contains(&fps) {
        return Err(RenderError::InvalidExport(format!(
            "unsupported frame rate: {fps}"
        )));
    }
    Ok(())
}

/// Validate an export filename: bare name, `.mp4` suffix, no traversal.
/// The file always lands in the project's `exports/` directory.
pub fn validate_export_filename(name: &str) -> Result<(), RenderError> {
    if name.is_empty() || name.len() > 120 {
        return Err(RenderError::InvalidExport(
            "export name must be 1–120 characters".to_owned(),
        ));
    }
    if !name.to_lowercase().ends_with(".mp4") {
        return Err(RenderError::InvalidExport(
            "export name must end in .mp4".to_owned(),
        ));
    }
    if name.contains('/') || name.contains('\\') || name.contains("..") || name.starts_with('.') {
        return Err(RenderError::InvalidExport(
            "export name must be a bare file name".to_owned(),
        ));
    }
    Ok(())
}

/// Compile video-track clips (ordered by start) into a [`RenderGraph`].
/// `asset_path` resolves a clip's `source_media_id` to a project file;
/// `None` means the timeline references unknown media and fails loudly.
///
/// Limitation (documented, next): every segment is expected to carry audio
/// (camera/screen recordings do). Mixed audio-less timelines fail at the
/// concat filter with `AVID_RENDER_004`, not silently.
pub fn graph_from_timeline(
    timeline: &avid_timeline::Timeline,
    asset_path: &dyn Fn(&str) -> Option<PathBuf>,
    width: u32,
    fps: u32,
) -> Result<RenderGraph, RenderError> {
    check_dimensions(width, fps)?;
    let video_tracks: Vec<&str> = timeline
        .tracks
        .iter()
        .filter(|track| track.kind == avid_timeline::TrackKind::Video)
        .map(|track| track.id.as_str())
        .collect();
    let mut clips: Vec<&avid_timeline::Clip> = timeline
        .clips
        .values()
        .filter(|clip| video_tracks.contains(&clip.track_id.as_str()))
        .collect();
    clips.sort_by(|a, b| {
        a.start
            .partial_cmp(&b.start)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let mut segments = vec![];
    for clip in clips {
        let Some(path) = asset_path(&clip.source_media_id) else {
            return Err(RenderError::InvalidSegment(format!(
                "unknown media for clip {}",
                clip.id
            )));
        };
        segments.push(Segment {
            path,
            seek: clip.in_point,
            duration: clip.duration,
        });
    }
    let graph = RenderGraph {
        segments,
        overlays: vec![],
        width,
        fps,
    };
    graph.validate()?;
    Ok(graph)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn graph() -> RenderGraph {
        RenderGraph {
            segments: vec![
                Segment {
                    path: PathBuf::from("a.mp4"),
                    seek: 1.0,
                    duration: 3.0,
                },
                Segment {
                    path: PathBuf::from("b.mp4"),
                    seek: 0.0,
                    duration: 2.0,
                },
            ],
            overlays: vec![TextOverlay {
                text: "Hello: it's 100%".to_owned(),
                start: 0.5,
                end: 4.0,
                fontsize: 48,
            }],
            width: 1280,
            fps: 30,
        }
    }

    #[test]
    fn argv_is_deterministic_and_structured() {
        let first = graph()
            .argv(Path::new("ffmpeg"), Path::new("out.mp4"))
            .unwrap();
        let second = graph()
            .argv(Path::new("ffmpeg"), Path::new("out.mp4"))
            .unwrap();
        assert_eq!(first, second);
        let joined = first.join(" ");
        assert!(first[0].ends_with("ffmpeg"));
        assert!(joined.contains("concat=n=2:v=1:a=1"));
        assert!(joined.contains("scale=1280:-2"));
        assert!(joined.contains("between(t,0.5,4)"));
        // Escaped overlay text.
        assert!(joined.contains(r"Hello\: it\'s 100\%"));
        assert!(joined.ends_with("out.mp4"));
    }

    #[test]
    fn rejects_empty_graph_and_bad_windows() {
        let mut empty = graph();
        empty.segments.clear();
        assert!(matches!(
            empty.argv(Path::new("ffmpeg"), Path::new("o.mp4")),
            Err(RenderError::Empty)
        ));

        let mut bad = graph();
        bad.segments[0].duration = 0.0;
        assert!(matches!(
            bad.argv(Path::new("ffmpeg"), Path::new("o.mp4")),
            Err(RenderError::InvalidSegment(_))
        ));

        let mut bad_overlay = graph();
        bad_overlay.overlays[0].end = 99.0;
        assert!(matches!(
            bad_overlay.argv(Path::new("ffmpeg"), Path::new("o.mp4")),
            Err(RenderError::InvalidOverlay(_))
        ));
    }

    #[test]
    fn total_duration_sums_segments() {
        assert_eq!(graph().total_duration(), 5.0);
    }

    #[test]
    fn srt_sidecar_formats_cues_with_millis() {
        let cues = vec![
            CaptionCue {
                start: 1.0,
                end: 3.256,
                text: "Hello".to_owned(),
            },
            CaptionCue {
                start: 65.0,
                end: 67.5,
                text: "Line one\nLine two".to_owned(),
            },
        ];
        assert_eq!(
            captions_to_srt(&cues),
            "1\n00:00:01,000 --> 00:00:03,256\nHello\n\n2\n00:01:05,000 --> 00:01:07,500\nLine one\nLine two\n\n"
        );
        assert_eq!(captions_to_srt(&[]), "");
    }

    #[test]
    fn cue_extraction_reads_caption_tracks_in_order() {
        use avid_timeline::{Clip, Timeline, Track, TrackKind};
        let mut timeline = Timeline::default();
        for (id, kind) in [("v1", TrackKind::Video), ("cc", TrackKind::Caption)] {
            timeline.tracks.push(Track {
                id: id.to_owned(),
                kind,
                index: 0,
                name: id.to_owned(),
                locked: false,
                muted: false,
            });
        }
        let mk = |id: &str, track: &str, start: f64, name: &str| Clip {
            id: id.to_owned(),
            source_media_id: "x".to_owned(),
            track_id: track.to_owned(),
            start,
            duration: 2.0,
            in_point: 0.0,
            name: name.to_owned(),
        };
        timeline
            .insert_clip(mk("late", "cc", 10.0, "Second"))
            .unwrap();
        timeline
            .insert_clip(mk("early", "cc", 1.0, "First"))
            .unwrap();
        timeline
            .insert_clip(mk("vid", "v1", 0.0, "Not a caption"))
            .unwrap();
        timeline.insert_clip(mk("blank", "cc", 5.0, "   ")).unwrap();
        let cues = caption_cues_from_timeline(&timeline);
        assert_eq!(cues.len(), 2);
        assert_eq!(
            (cues[0].text.as_str(), cues[1].text.as_str()),
            ("First", "Second")
        );
    }

    #[test]
    fn presets_resolve_including_custom() {
        assert_eq!(
            resolve_preset("youtube-1080p", None, None).unwrap(),
            (1920, 30)
        );
        assert_eq!(
            resolve_preset("short-1080p", None, None).unwrap(),
            (1080, 30)
        );
        assert_eq!(
            resolve_preset("custom", Some(1280), Some(25)).unwrap(),
            (1280, 25)
        );
        assert!(resolve_preset("nope", None, None).is_err());
        assert!(resolve_preset("custom", None, Some(30)).is_err());
        assert!(resolve_preset("custom", Some(100), Some(30)).is_err());
        assert!(resolve_preset("custom", Some(1280), Some(48)).is_err());
    }

    #[test]
    fn export_filenames_are_bare_mp4_names() {
        assert!(validate_export_filename("final.mp4").is_ok());
        for bad in [
            "",
            "x".repeat(121).as_str(),
            "final.mov",
            "a/b.mp4",
            "..\\x.mp4",
            "../x.mp4",
            ".hidden.mp4",
        ] {
            assert!(validate_export_filename(bad).is_err(), "name: {bad}");
        }
    }

    #[test]
    fn graph_from_timeline_orders_video_clips_and_resolves_media() {
        use avid_timeline::{Clip, Timeline, Track, TrackKind};
        let mut timeline = Timeline::default();
        timeline.tracks.push(Track {
            id: "v1".to_owned(),
            kind: TrackKind::Video,
            index: 0,
            name: "V1".to_owned(),
            locked: false,
            muted: false,
        });
        let mk = |id: &str, start: f64| Clip {
            id: id.to_owned(),
            source_media_id: "m1".to_owned(),
            track_id: "v1".to_owned(),
            start,
            duration: 2.0,
            in_point: start,
            name: id.to_owned(),
        };
        // Insert out of order; graph must sort by start.
        timeline.insert_clip(mk("b", 4.0)).unwrap();
        timeline.insert_clip(mk("a", 0.0)).unwrap();
        let graph =
            graph_from_timeline(&timeline, &|_| Some(PathBuf::from("m.mp4")), 1280, 30).unwrap();
        assert_eq!(graph.segments.len(), 2);
        assert_eq!((graph.segments[0].seek, graph.segments[1].seek), (0.0, 4.0));
        assert_eq!(graph.total_duration(), 4.0);

        let missing = graph_from_timeline(&timeline, &|_| None, 1280, 30).unwrap_err();
        assert!(matches!(missing, RenderError::InvalidSegment(_)));

        let empty = Timeline {
            tracks: timeline.tracks.clone(),
            ..Timeline::default()
        };
        assert!(matches!(
            graph_from_timeline(&empty, &|_| Some(PathBuf::from("m.mp4")), 1280, 30),
            Err(RenderError::Empty)
        ));
    }

    #[test]
    fn compat_path_rejects_overlays_with_guidance() {
        let err = graph()
            .argv_compat(Path::new("ffmpeg"), Path::new("o.mp4"))
            .unwrap_err();
        assert!(matches!(err, RenderError::TextOverlaysUnsupported { .. }));
        assert_eq!(err.code(), "AVID_RENDER_005");

        let mut plain = graph();
        plain.overlays.clear();
        let argv = plain
            .argv_compat(Path::new("ffmpeg"), Path::new("o.mp4"))
            .unwrap();
        assert!(!argv.join(" ").contains("drawtext"));
    }

    #[test]
    fn text_probe_is_safe_on_missing_binaries() {
        assert!(!ffmpeg_supports_text(Path::new(
            "definitely-not-ffmpeg-xyz"
        )));
    }

    /// REAL render test: concatenates two windows of the talking-head fixture
    /// (compat path — text burn-in needs a drawtext-capable sidecar build;
    /// overlay lowering is covered by unit tests). Probed output must be
    /// ~4 s of h264 + aac. Ignored without fixtures.
    #[test]
    #[ignore]
    fn renders_real_fixture_with_system_ffmpeg() {
        let fixture =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/media/talkinghead_10s.mp4");
        if !fixture.is_file() {
            return;
        }
        // Stage beside the manifest (relative-path discipline, like the app).
        for name in ["seg_a.mp4", "seg_b.mp4"] {
            std::fs::copy(&fixture, Path::new(name)).unwrap();
        }
        let graph = RenderGraph {
            segments: vec![
                Segment {
                    path: PathBuf::from("seg_a.mp4"),
                    seek: 1.0,
                    duration: 2.0,
                },
                Segment {
                    path: PathBuf::from("seg_b.mp4"),
                    seek: 4.0,
                    duration: 2.0,
                },
            ],
            overlays: vec![],
            width: 640,
            fps: 30,
        };
        let argv = graph
            .argv_compat(Path::new("ffmpeg"), Path::new("render_test_out.mp4"))
            .unwrap();
        let status = std::process::Command::new(&argv[0])
            .args(&argv[1..])
            .output()
            .expect("spawn ffmpeg");
        for name in ["seg_a.mp4", "seg_b.mp4"] {
            std::fs::remove_file(name).ok();
        }
        assert!(
            status.status.success(),
            "stderr = {}",
            String::from_utf8_lossy(&status.stderr)
        );
        // Verify with ffprobe: ~4s, h264 video + aac audio present.
        let probe = std::process::Command::new("ffprobe")
            .args([
                "-v",
                "quiet",
                "-print_format",
                "json",
                "-show_format",
                "-show_streams",
                "render_test_out.mp4",
            ])
            .output()
            .unwrap();
        std::fs::remove_file("render_test_out.mp4").ok();
        assert!(probe.status.success());
        let json = String::from_utf8_lossy(&probe.stdout);
        assert!(json.contains("h264") && json.contains("aac"), "{json}");
        assert!(json.contains("\"duration\": \"4.0"), "{json}");
    }
}
