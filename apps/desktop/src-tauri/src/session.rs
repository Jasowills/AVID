//! Open-project session: manifest + live timeline + undo stack (ADR-003).
//!
//! The session is the unit of autosave: every mutation executes through the
//! [`UndoStack`] and then persists `project.json`, so a crash loses at most
//! the in-flight operation. Reopening the project directory restores the
//! manifest and rebuilds the timeline from it (crash recovery foundation).

use std::path::{Path, PathBuf};

use avid_ai::Transcript;
use avid_media::{MediaEngine, MediaInfo};
use avid_project::{MediaAsset, ProjectManifest};
use avid_timeline::{
    AddClipCommand, Clip, EditCommand, RemoveClipCommand, SplitClipCommand, Timeline, Track,
    TrackKind, UndoStack,
};

use crate::commands::{command_error_from_media, command_error_from_render, CommandError};

/// Transcript storage key: asset id → transcript JSON (see manifest docs).
const TRANSCRIPT_PROVIDER: &str = "local-whispercpp";

/// An open project with live editing state.
#[derive(Debug)]
pub struct Session {
    dir: PathBuf,
    manifest: ProjectManifest,
    timeline: Timeline,
    undo: UndoStack,
}

impl Session {
    /// Project directory on disk.
    #[must_use]
    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// Current manifest (timeline synced in).
    #[must_use]
    pub fn manifest(&self) -> &ProjectManifest {
        &self.manifest
    }

    /// Live timeline (read-only view for IPC).
    #[must_use]
    pub fn timeline(&self) -> &Timeline {
        &self.timeline
    }

    /// Number of undoable steps (for UI state).
    #[must_use]
    pub fn undo_depth(&self) -> usize {
        self.undo.undo_depth()
    }

    /// Number of redoable steps (for UI state).
    #[must_use]
    pub fn redo_depth(&self) -> usize {
        self.undo.redo_depth()
    }

    /// Create a fresh session: default V1/A1 tracks, empty timeline, and an
    /// immediate persist so the directory is always a valid project.
    pub fn create(dir: PathBuf, manifest: ProjectManifest) -> Result<Self, CommandError> {
        let mut timeline = Timeline::default();
        timeline.tracks.push(Track {
            id: "v1".to_owned(),
            kind: TrackKind::Video,
            index: 0,
            name: "V1".to_owned(),
            locked: false,
            muted: false,
        });
        timeline.tracks.push(Track {
            id: "a1".to_owned(),
            kind: TrackKind::Audio,
            index: 0,
            name: "A1".to_owned(),
            locked: false,
            muted: false,
        });
        let mut session = Self {
            dir,
            manifest,
            timeline,
            undo: UndoStack::default(),
        };
        session.persist()?;
        Ok(session)
    }

    /// Load a session from a project directory (recovery path).
    pub fn load(dir: PathBuf) -> Result<Self, CommandError> {
        let json = std::fs::read_to_string(dir.join("project.json")).map_err(|e| CommandError {
            code: "AVID_PROJECT_001".to_owned(),
            message: format!("AVID couldn't open that project folder: {e}"),
        })?;
        let manifest = ProjectManifest::from_json(&json)?;
        let timeline: Timeline =
            serde_json::from_value(manifest.timeline.clone()).map_err(|e| CommandError {
                code: "AVID_PROJECT_001".to_owned(),
                message: format!("Project timeline is corrupt: {e}"),
            })?;
        Ok(Self {
            dir,
            manifest,
            timeline,
            undo: UndoStack::default(),
        })
    }

    /// Sync the timeline into the manifest and write `project.json`.
    pub fn persist(&mut self) -> Result<(), CommandError> {
        self.manifest.timeline =
            serde_json::to_value(&self.timeline).map_err(|e| CommandError {
                code: "AVID_PROJECT_001".to_owned(),
                message: format!("AVID couldn't serialize the timeline: {e}"),
            })?;
        std::fs::create_dir_all(&self.dir).map_err(|e| CommandError {
            code: "AVID_PROJECT_001".to_owned(),
            message: format!("AVID couldn't write the project folder: {e}"),
        })?;
        std::fs::write(self.dir.join("project.json"), self.manifest.to_json()?).map_err(|e| {
            CommandError {
                code: "AVID_PROJECT_001".to_owned(),
                message: format!("AVID couldn't write the project file: {e}"),
            }
        })?;
        Ok(())
    }

    /// Execute one command as an undo step and persist.
    fn mutate(&mut self, label: &str, command: Box<dyn EditCommand>) -> Result<(), CommandError> {
        self.undo
            .execute(&mut self.timeline, label, command)
            .map_err(|e| CommandError {
                code: e.code().to_owned(),
                message: e.to_string(),
            })?;
        self.persist()
    }

    /// Add a clip (undoable, persisted).
    pub fn add_clip(&mut self, clip: Clip) -> Result<(), CommandError> {
        self.mutate("Add clip", Box::new(AddClipCommand { clip }))
    }

    /// Remove a clip (undoable, persisted).
    pub fn remove_clip(&mut self, clip_id: &str) -> Result<(), CommandError> {
        self.mutate("Remove clip", Box::new(RemoveClipCommand::new(clip_id)))
    }

    /// Split a clip at a timeline position (undoable, persisted).
    pub fn split_clip(&mut self, clip_id: &str, at: f64) -> Result<(), CommandError> {
        self.mutate("Split clip", Box::new(SplitClipCommand::new(clip_id, at)))
    }

    /// Undo the last step and persist.
    pub fn undo(&mut self) -> Result<String, CommandError> {
        let label = self
            .undo
            .undo(&mut self.timeline)
            .map_err(|e| CommandError {
                code: e.code().to_owned(),
                message: e.to_string(),
            })?;
        self.persist()?;
        Ok(label)
    }

    /// Redo the last undone step and persist.
    pub fn redo(&mut self) -> Result<String, CommandError> {
        let label = self
            .undo
            .redo(&mut self.timeline)
            .map_err(|e| CommandError {
                code: e.code().to_owned(),
                message: e.to_string(),
            })?;
        self.persist()?;
        Ok(label)
    }

    /// Import a user-chosen file: copy into `media/`, probe, register the
    /// asset, and — for the first video import on an empty V1 — place it as
    /// an undoable clip (IMPORT → EDIT in one gesture).
    pub fn import_file(
        &mut self,
        engine: &MediaEngine,
        source: &Path,
    ) -> Result<MediaAsset, CommandError> {
        let file_name =
            source
                .file_name()
                .and_then(|n| n.to_str())
                .ok_or_else(|| CommandError {
                    code: "AVID_MEDIA_001".to_owned(),
                    message: "That file name isn't usable — rename it and retry.".to_owned(),
                })?;
        if !source.is_file() {
            return Err(CommandError {
                code: "AVID_MEDIA_001".to_owned(),
                message: "AVID couldn't find that file.".to_owned(),
            });
        }
        let info = engine
            .probe_file(source)
            .map_err(|error| command_error_from_media(&error))?;
        let asset_id = MediaEngine::mint_asset_id();
        let dest_name = format!("{asset_id}_{file_name}");
        let media_dir = self.dir.join("media");
        std::fs::create_dir_all(&media_dir).map_err(|e| CommandError {
            code: "AVID_PROJECT_001".to_owned(),
            message: format!("AVID couldn't prepare the media folder: {e}"),
        })?;
        std::fs::copy(source, media_dir.join(&dest_name)).map_err(|e| CommandError {
            code: "AVID_PROJECT_001".to_owned(),
            message: format!("AVID couldn't copy the file in: {e}"),
        })?;
        let video = info.video_stream();
        let asset = MediaAsset {
            id: asset_id.clone(),
            file_name: file_name.to_owned(),
            relative_path: format!("media/{dest_name}"),
            duration: info.duration,
            dimensions: video.and_then(|v| v.width.zip(v.height)),
            hash: None,
        };
        self.manifest.assets.push(asset.clone());
        // First video import on empty V1 → place it (undoable, like any edit).
        let v1_empty = self.timeline.clips_on_track("v1").is_empty();
        if v1_empty {
            if let (Some(duration), true) = (asset.duration, video.is_some()) {
                let clip = Clip {
                    id: format!("clip-{asset_id}"),
                    source_media_id: asset_id.clone(),
                    track_id: "v1".to_owned(),
                    start: 0.0,
                    duration,
                    in_point: 0.0,
                    name: file_name.to_owned(),
                };
                self.mutate("Import media", Box::new(AddClipCommand { clip }))?;
                return Ok(asset);
            }
        }
        self.persist()?;
        Ok(asset)
    }

    /// Transcribe an imported asset: extract 16 kHz mono audio, run the
    /// in-process Whisper engine, store the transcript on the manifest.
    /// `model_path` is explicit (cache dir in-app, env override in tests).
    pub fn transcribe_asset(
        &mut self,
        engine: &MediaEngine,
        model_path: &Path,
        cache_dir: &Path,
        asset_id: &str,
        language: &str,
    ) -> Result<Transcript, CommandError> {
        let asset = self
            .manifest
            .assets
            .iter()
            .find(|a| a.id == asset_id)
            .cloned()
            .ok_or_else(|| CommandError {
                code: "AVID_MEDIA_001".to_owned(),
                message: "Unknown media asset.".to_owned(),
            })?;
        if !model_path.is_file() {
            return Err(CommandError {
                code: "AVID_TRANSCRIBE_001".to_owned(),
                message:
                    "Speech model isn't downloaded yet — fetch it in Settings → AI, then retry."
                        .to_owned(),
            });
        }
        std::fs::create_dir_all(cache_dir).map_err(|e| CommandError {
            code: "AVID_PROJECT_001".to_owned(),
            message: format!("AVID couldn't prepare the audio cache: {e}"),
        })?;
        let wav = cache_dir.join(format!("{asset_id}.wav"));
        let source = self.dir.join(&asset.relative_path);
        engine
            .extract_audio(&source, &wav)
            .map_err(|error| command_error_from_media(&error))?;
        let transcript = avid_ai::transcribe_wav(model_path, &wav, language, TRANSCRIPT_PROVIDER)
            .map_err(|e| CommandError {
            code: e.code().to_owned(),
            message: e.to_string(),
        })?;
        self.manifest.transcripts.insert(
            asset_id.to_owned(),
            serde_json::to_value(&transcript).map_err(|e| CommandError {
                code: "AVID_PROJECT_001".to_owned(),
                message: format!("AVID couldn't store the transcript: {e}"),
            })?,
        );
        self.persist()?;
        Ok(transcript)
    }

    /// Stored transcript for an asset, if transcribed.
    #[must_use]
    pub fn transcript_for(&self, asset_id: &str) -> Option<Transcript> {
        self.manifest
            .transcripts
            .get(asset_id)
            .and_then(|value| serde_json::from_value(value.clone()).ok())
    }

    /// Media metadata passthrough for command handlers.
    pub fn probe_any(engine: &MediaEngine, path: &Path) -> Result<MediaInfo, CommandError> {
        engine
            .probe_file(path)
            .map_err(|error| command_error_from_media(&error))
    }

    /// Export the timeline: compile video-track clips → render → verify.
    /// Pure over an explicit snapshot (dir + timeline + assets) so the async
    /// command can run it on a blocking thread without holding the lock.
    /// Returns the project-relative output path and verified duration.
    pub fn export_snapshot(
        engine: &MediaEngine,
        dir: &Path,
        timeline: &avid_timeline::Timeline,
        assets: &[MediaAsset],
        preset_id: &str,
        custom_width: Option<u32>,
        custom_fps: Option<u32>,
        filename: &str,
    ) -> Result<ExportResult, CommandError> {
        use avid_render::{
            argv_auto, graph_from_timeline, resolve_preset, validate_export_filename,
        };
        validate_export_filename(filename).map_err(|error| command_error_from_render(&error))?;
        let (width, fps) = resolve_preset(preset_id, custom_width, custom_fps)
            .map_err(|error| command_error_from_render(&error))?;
        let graph = graph_from_timeline(
            timeline,
            &|media_id| {
                assets
                    .iter()
                    .find(|asset| asset.id == media_id)
                    .map(|asset| dir.join(&asset.relative_path))
            },
            width,
            fps,
        )
        .map_err(|error| command_error_from_render(&error))?;
        let exports = dir.join("exports");
        std::fs::create_dir_all(&exports).map_err(|e| CommandError {
            code: "AVID_RENDER_004".to_owned(),
            message: format!("AVID couldn't prepare the exports folder: {e}"),
        })?;
        let output = exports.join(filename);
        let argv = argv_auto(engine.ffmpeg_path(), &output, &graph)
            .map_err(|error| command_error_from_render(&error))?;
        engine
            .run_render(&argv)
            .map_err(|error| command_error_from_media(&error))?;
        let info = engine
            .probe_file(&output)
            .map_err(|error| command_error_from_media(&error))?;
        let duration = info.duration.ok_or_else(|| CommandError {
            code: "AVID_RENDER_004".to_owned(),
            message: "AVID couldn't verify the export — the output has no duration.".to_owned(),
        })?;
        if (duration - graph.total_duration()).abs() > 0.75 {
            return Err(CommandError {
                code: "AVID_RENDER_004".to_owned(),
                message: format!(
                    "Export verification failed: expected {:.1}s, got {:.1}s.",
                    graph.total_duration(),
                    duration
                ),
            });
        }
        Ok(ExportResult {
            relative_path: format!("exports/{filename}"),
            absolute_path: output.display().to_string(),
            duration,
            width,
            fps,
        })
    }

    /// Export the open timeline (locks briefly to snapshot, renders off-lock).
    pub fn export(
        &mut self,
        engine: &MediaEngine,
        preset_id: &str,
        custom_width: Option<u32>,
        custom_fps: Option<u32>,
        filename: &str,
    ) -> Result<ExportResult, CommandError> {
        Self::export_snapshot(
            engine,
            &self.dir.clone(),
            &self.timeline.clone(),
            &self.manifest.assets.clone(),
            preset_id,
            custom_width,
            custom_fps,
            filename,
        )
    }
}

/// Verified export result (returned to the UI + tests).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExportResult {
    /// Project-relative output path (`exports/final.mp4`).
    pub relative_path: String,
    /// Absolute output path (for "show in folder" actions).
    pub absolute_path: String,
    /// Probed output duration in seconds.
    pub duration: f64,
    pub width: u32,
    pub fps: u32,
}

/// Base directory for projects under app data.
#[must_use]
pub fn projects_base(app_data: &Path) -> PathBuf {
    app_data.join("avid-projects")
}

#[cfg(test)]
mod tests {
    use super::*;
    use avid_project::{ProjectMeta, MANIFEST_VERSION};
    use std::collections::HashMap;

    fn manifest(id: &str) -> ProjectManifest {
        ProjectManifest {
            id: id.to_owned(),
            version: MANIFEST_VERSION,
            meta: ProjectMeta {
                name: "Session test".to_owned(),
                canvas: "16:9".to_owned(),
                frame_rate: 30,
                resolution: "1080p".to_owned(),
                template_id: None,
                created_at: "now".to_owned(),
                updated_at: "now".to_owned(),
            },
            timeline: serde_json::json!({"tracks": [], "clips": {}}),
            assets: vec![],
            transcripts: HashMap::new(),
        }
    }

    fn clip(id: &str, start: f64, duration: f64) -> Clip {
        Clip {
            id: id.to_owned(),
            source_media_id: "media-1".to_owned(),
            track_id: "v1".to_owned(),
            start,
            duration,
            in_point: 0.0,
            name: id.to_owned(),
        }
    }

    #[test]
    fn mutations_persist_and_reload() {
        let dir = std::env::temp_dir().join("avid-session-test");
        std::fs::remove_dir_all(&dir).ok();
        let mut session = Session::create(dir.clone(), manifest("s1")).unwrap();
        // Default V1/A1 tracks exist on fresh sessions.
        assert_eq!(session.timeline().tracks.len(), 2);
        session.add_clip(clip("a", 0.0, 8.0)).unwrap();
        session.split_clip("a", 3.0).unwrap();
        assert_eq!(session.undo_depth(), 2);
        session.undo().unwrap();
        assert!(!session.timeline().clips.contains_key("a-b"));
        session.redo().unwrap();
        assert!(session.timeline().clips.contains_key("a-b"));
        // Reload from disk: committed work survives (crash-recovery basis).
        let loaded = Session::load(dir.clone()).unwrap();
        assert_eq!(loaded.timeline(), session.timeline());
        assert_eq!(loaded.manifest().id, "s1");
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn import_rejects_missing_files() {
        let Ok(engine) = MediaEngine::system() else {
            return;
        };
        let dir = std::env::temp_dir().join("avid-session-missing");
        std::fs::remove_dir_all(&dir).ok();
        let mut session = Session::create(dir.clone(), manifest("s2")).unwrap();
        let err = session
            .import_file(&engine, Path::new("/definitely/not/here.mp4"))
            .unwrap_err();
        assert_eq!(err.code, "AVID_MEDIA_001");
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn export_rejects_bad_requests_without_rendering() {
        let Ok(engine) = MediaEngine::system() else {
            return;
        };
        let dir = std::env::temp_dir().join("avid-export-invalid");
        std::fs::remove_dir_all(&dir).ok();
        let mut session = Session::create(dir.clone(), manifest("s3")).unwrap();
        // Empty timeline — validation fires before any ffmpeg spawn.
        let err = session
            .export(&engine, "youtube-1080p", None, None, "out.mp4")
            .unwrap_err();
        assert_eq!(err.code, "AVID_RENDER_001");
        // Bad preset / filename likewise never reach ffmpeg.
        let err = session
            .export(&engine, "nope", None, None, "out.mp4")
            .unwrap_err();
        assert_eq!(err.code, "AVID_RENDER_006");
        let err = session
            .export(&engine, "youtube-1080p", None, None, "../evil.mp4")
            .unwrap_err();
        assert_eq!(err.code, "AVID_RENDER_006");
        std::fs::remove_dir_all(&dir).ok();
    }

    /// LIVE vertical slice: import a real fixture (auto-places the V1 clip),
    /// synthesize speech, import + transcribe it, reload from disk.
    /// Ignored by default (needs ffmpeg, `say`, cached model); run explicitly.
    #[test]
    #[ignore]
    fn live_import_transcribe_and_recover() {
        let engine = MediaEngine::system().expect("system ffmpeg/ffprobe");
        let model = std::env::var("AVID_TEST_MODEL").map_or_else(
            |_| {
                let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_owned());
                Path::new(&home).join(".cache/avid/models/ggml-tiny.en.bin")
            },
            PathBuf::from,
        );
        assert!(model.is_file(), "missing speech model: {}", model.display());
        let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../fixtures/media/talkinghead_10s.mp4");
        assert!(fixture.is_file(), "missing fixture: {}", fixture.display());
        let dir = std::env::temp_dir().join("avid-session-live");
        std::fs::remove_dir_all(&dir).ok();
        let mut session = Session::create(dir.clone(), manifest("live")).unwrap();

        // Video import → probed asset + auto-placed V1 clip.
        let asset = session.import_file(&engine, &fixture).unwrap();
        assert_eq!(asset.dimensions, Some((1280, 720)));
        let placed: Vec<_> = session.timeline().clips_on_track("v1");
        assert_eq!(placed.len(), 1);
        assert!((placed[0].duration - 10.0).abs() < 0.2);

        // Speech fixture → audio import (no auto-clip: audio-only) → transcribe.
        let speech_src = dir.join("speech-src.wav");
        let say = std::process::Command::new("say")
            .arg("-o")
            .arg(dir.join("speech.aiff"))
            .arg("Kafka has three partitions.")
            .status();
        if say.map(|s| !s.success()).unwrap_or(true) {
            std::fs::remove_dir_all(&dir).ok();
            return;
        }
        assert!(std::process::Command::new("ffmpeg")
            .args(["-y", "-v", "error", "-i"])
            .arg(dir.join("speech.aiff"))
            .args(["-ac", "1", "-ar", "16000"])
            .arg(&speech_src)
            .status()
            .map(|s| s.success())
            .unwrap_or(false));
        let speech = session.import_file(&engine, &speech_src).unwrap();
        assert!(
            session.timeline().clips_on_track("v1").len() == 1,
            "audio must not auto-place video"
        );
        let audio_cache = dir.join("audio-cache");
        let transcript = session
            .transcribe_asset(&engine, &model, &audio_cache, &speech.id, "en")
            .unwrap();
        let text = transcript
            .segments
            .iter()
            .map(|s| s.text.as_str())
            .collect::<Vec<_>>()
            .join(" ");
        assert!(text.to_lowercase().contains("kafka"), "got: {text}");

        // Reload: assets, clips, and transcript all survive.
        let loaded = Session::load(dir.clone()).unwrap();
        assert_eq!(loaded.manifest().assets.len(), 2);
        assert!(loaded.transcript_for(&speech.id).is_some());
        std::fs::remove_dir_all(&dir).ok();
    }

    /// LIVE export: import the fixture, export a preset, verify the file.
    /// Ignored by default (needs ffmpeg + fixture); run explicitly.
    #[test]
    #[ignore]
    fn live_export_preset_produces_verified_file() {
        let engine = MediaEngine::system().expect("system ffmpeg/ffprobe");
        let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../fixtures/media/talkinghead_10s.mp4");
        assert!(fixture.is_file(), "missing fixture: {}", fixture.display());
        let dir = std::env::temp_dir().join("avid-export-live");
        std::fs::remove_dir_all(&dir).ok();
        let mut session = Session::create(dir.clone(), manifest("export-live")).unwrap();
        session.import_file(&engine, &fixture).unwrap();
        let result = session
            .export(&engine, "short-1080p", None, None, "short.mp4")
            .unwrap();
        assert_eq!(result.relative_path, "exports/short.mp4");
        assert_eq!((result.width, result.fps), (1080, 30));
        assert!(
            (result.duration - 10.0).abs() < 0.75,
            "duration = {}",
            result.duration
        );
        assert!(dir.join(&result.relative_path).is_file());
        std::fs::remove_dir_all(&dir).ok();
    }
}
