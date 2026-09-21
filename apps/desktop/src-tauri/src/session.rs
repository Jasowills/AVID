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
        self.sync_manifest()?;
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

    /// Sync the live timeline into the manifest without touching disk.
    fn sync_manifest(&mut self) -> Result<(), CommandError> {
        self.manifest.timeline =
            serde_json::to_value(&self.timeline).map_err(|e| CommandError {
                code: "AVID_PROJECT_001".to_owned(),
                message: format!("AVID couldn't serialize the timeline: {e}"),
            })?;
        Ok(())
    }

    /// Snapshot the current manifest JSON for recovery (AGENTS §117).
    /// Stored as `snapshots/<millis>-<label>.json`, pruned to the newest 10.
    /// Returns the snapshot file name (bare — safe to round-trip).
    pub fn snapshot(&mut self, label: &str) -> Result<String, CommandError> {
        self.sync_manifest()?;
        let safe_label: String = label
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '-' {
                    c
                } else {
                    '_'
                }
            })
            .take(40)
            .collect();
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let name = format!("{stamp}-{safe_label}.json");
        let dir = self.dir.join("snapshots");
        std::fs::create_dir_all(&dir).map_err(|e| CommandError {
            code: "AVID_PROJECT_001".to_owned(),
            message: format!("AVID couldn't prepare snapshots: {e}"),
        })?;
        std::fs::write(dir.join(&name), self.manifest.to_json()?).map_err(|e| CommandError {
            code: "AVID_PROJECT_001".to_owned(),
            message: format!("AVID couldn't write the snapshot: {e}"),
        })?;
        self.prune_snapshots()?;
        Ok(name)
    }

    /// Keep only the newest 10 snapshots (lexicographic = chronological here).
    fn prune_snapshots(&self) -> Result<(), CommandError> {
        let dir = self.dir.join("snapshots");
        let mut names: Vec<String> = std::fs::read_dir(&dir)
            .map_err(|e| CommandError {
                code: "AVID_PROJECT_001".to_owned(),
                message: format!("AVID couldn't list snapshots: {e}"),
            })?
            .flatten()
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .filter(|name| name.ends_with(".json"))
            .collect();
        names.sort();
        for stale in names.iter().take(names.len().saturating_sub(10)) {
            std::fs::remove_file(dir.join(stale)).ok();
        }
        Ok(())
    }

    /// Snapshot metadata for recovery UI (newest first).
    #[must_use]
    pub fn list_snapshots(&self) -> Vec<SnapshotInfo> {
        let mut entries: Vec<SnapshotInfo> = std::fs::read_dir(self.dir.join("snapshots"))
            .map(|read| {
                read.flatten()
                    .map(|entry| entry.file_name().to_string_lossy().into_owned())
                    .filter(|name| name.ends_with(".json"))
                    .map(|name| SnapshotInfo { name })
                    .collect()
            })
            .unwrap_or_default();
        entries.sort_by(|a, b| b.name.cmp(&a.name));
        entries
    }

    /// Restore a snapshot by bare file name (traversal rejected by construction:
    /// only names from `list_snapshots` are valid, and separators are refused).
    /// History restarts (documented: restore is itself the recovery point).
    pub fn restore_snapshot(&mut self, name: &str) -> Result<(), CommandError> {
        if name.is_empty()
            || name.len() > 100
            || name.contains('/')
            || name.contains('\\')
            || name.contains("..")
            || !name.ends_with(".json")
        {
            return Err(CommandError {
                code: "AVID_PROJECT_001".to_owned(),
                message: "That snapshot name isn't valid.".to_owned(),
            });
        }
        let json = std::fs::read_to_string(self.dir.join("snapshots").join(name)).map_err(|e| {
            CommandError {
                code: "AVID_PROJECT_001".to_owned(),
                message: format!("AVID couldn't read that snapshot: {e}"),
            }
        })?;
        let manifest = ProjectManifest::from_json(&json)?;
        let timeline: Timeline =
            serde_json::from_value(manifest.timeline.clone()).map_err(|e| CommandError {
                code: "AVID_PROJECT_001".to_owned(),
                message: format!("Snapshot timeline is corrupt: {e}"),
            })?;
        self.manifest = manifest;
        self.timeline = timeline;
        self.undo = UndoStack::default();
        self.persist()
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

    /// Execute several commands as one undo step (transactional: the group
    /// rolls back on failure) and persist. AI operations run here (ADR-008).
    pub fn mutate_group(
        &mut self,
        label: &str,
        commands: Vec<Box<dyn EditCommand>>,
    ) -> Result<(), CommandError> {
        self.undo
            .execute_group(&mut self.timeline, label, commands)
            .map_err(|e| CommandError {
                code: e.code().to_owned(),
                message: e.to_string(),
            })?;
        self.persist()
    }

    /// Ensure a caption track exists (additive setup step for caption ops).
    pub fn ensure_caption_track(&mut self) -> Result<(), CommandError> {
        if self
            .timeline
            .tracks
            .iter()
            .any(|track| track.id == "captions")
        {
            return Ok(());
        }
        let index = self.timeline.tracks.len() as u32;
        self.timeline.tracks.push(Track {
            id: "captions".to_owned(),
            kind: TrackKind::Caption,
            index,
            name: "Captions".to_owned(),
            locked: false,
            muted: false,
        });
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

    /// Trim a clip to a new start/duration (undoable, persisted).
    pub fn trim_clip(
        &mut self,
        clip_id: &str,
        start: f64,
        duration: f64,
    ) -> Result<(), CommandError> {
        self.mutate(
            "Trim clip",
            Box::new(avid_timeline::TrimClipCommand::new(
                clip_id, start, duration,
            )),
        )
    }

    /// Set a clip's gain/mute (undoable, persisted). Gain 0–4, 1 = unity.
    pub fn set_clip_audio(
        &mut self,
        clip_id: &str,
        volume: f32,
        muted: bool,
    ) -> Result<(), CommandError> {
        self.mutate(
            "Set clip audio",
            Box::new(avid_timeline::SetClipAudioCommand::new(
                clip_id, volume, muted,
            )),
        )
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
            proxy_path: None,
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
                    volume: 1.0,
                    muted: false,
                    name: file_name.to_owned(),
                };
                self.mutate("Import media", Box::new(AddClipCommand { clip }))?;
                return Ok(asset);
            }
        }
        self.persist()?;
        Ok(asset)
    }

    /// Generate a 540p editing proxy for a video asset and record it.
    /// Audio-only assets fail closed with guidance (nothing to preview).
    /// Always regenerates (deterministic output for identical input).
    pub fn generate_proxy(
        &mut self,
        engine: &MediaEngine,
        asset_id: &str,
    ) -> Result<MediaAsset, CommandError> {
        let asset = self
            .manifest
            .assets
            .iter()
            .find(|asset| asset.id == asset_id)
            .cloned()
            .ok_or_else(|| CommandError {
                code: "AVID_MEDIA_001".to_owned(),
                message: "Unknown media asset.".to_owned(),
            })?;
        let dir = self.dir.clone();
        Self::generate_proxy_file(engine, &dir, &asset)?;
        self.set_proxy_path(&asset.id, format!("proxies/{}.mp4", asset.id))
    }

    /// Render the proxy file without touching session state, so async
    /// commands can run it on a blocking thread. Returns the output path.
    pub fn generate_proxy_file(
        engine: &MediaEngine,
        dir: &Path,
        asset: &MediaAsset,
    ) -> Result<PathBuf, CommandError> {
        if asset.dimensions.is_none() {
            return Err(CommandError {
                code: "AVID_MEDIA_001".to_owned(),
                message: "Proxies are for video — audio files preview directly.".to_owned(),
            });
        }
        let proxies = dir.join("proxies");
        std::fs::create_dir_all(&proxies).map_err(|e| CommandError {
            code: "AVID_PROJECT_001".to_owned(),
            message: format!("AVID couldn't prepare the proxies folder: {e}"),
        })?;
        let output = proxies.join(format!("{}.mp4", asset.id));
        let source = dir.join(&asset.relative_path);
        engine
            .generate_proxy(&source, &output)
            .map_err(|error| command_error_from_media(&error))?;
        Ok(output)
    }

    /// Record a generated proxy path and persist.
    pub fn set_proxy_path(
        &mut self,
        asset_id: &str,
        relative_path: String,
    ) -> Result<MediaAsset, CommandError> {
        let index = self
            .manifest
            .assets
            .iter()
            .position(|asset| asset.id == asset_id)
            .ok_or_else(|| CommandError {
                code: "AVID_MEDIA_001".to_owned(),
                message: "Unknown media asset.".to_owned(),
            })?;
        self.manifest.assets[index].proxy_path = Some(relative_path);
        self.persist()?;
        Ok(self.manifest.assets[index].clone())
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
        let dir = self.dir.clone();
        let transcript =
            Self::transcribe_snapshot(engine, model_path, cache_dir, &dir, &asset, language)?;
        self.store_transcript(asset_id, transcript.clone())?;
        Ok(transcript)
    }

    /// Run extraction + inference without touching session state, so async
    /// commands can execute it on a blocking thread. Pure over a snapshot.
    pub fn transcribe_snapshot(
        engine: &MediaEngine,
        model_path: &Path,
        cache_dir: &Path,
        dir: &Path,
        asset: &MediaAsset,
        language: &str,
    ) -> Result<Transcript, CommandError> {
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
        let wav = cache_dir.join(format!("{}.wav", asset.id));
        let source = dir.join(&asset.relative_path);
        engine
            .extract_audio(&source, &wav)
            .map_err(|error| command_error_from_media(&error))?;
        avid_ai::transcribe_wav(model_path, &wav, language, TRANSCRIPT_PROVIDER).map_err(|e| {
            CommandError {
                code: e.code().to_owned(),
                message: e.to_string(),
            }
        })
    }

    /// Store a transcript on the manifest and persist.
    pub fn store_transcript(
        &mut self,
        asset_id: &str,
        transcript: Transcript,
    ) -> Result<(), CommandError> {
        self.manifest.transcripts.insert(
            asset_id.to_owned(),
            serde_json::to_value(&transcript).map_err(|e| CommandError {
                code: "AVID_PROJECT_001".to_owned(),
                message: format!("AVID couldn't store the transcript: {e}"),
            })?,
        );
        self.persist()
    }

    /// Validate a scene id (bare slug — ids travel through IPC and filenames).
    fn check_scene_id(id: &str) -> Result<(), CommandError> {
        if id.is_empty()
            || id.len() > 80
            || !id
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            return Err(CommandError {
                code: "AVID_PROJECT_001".to_owned(),
                message: "Scene ids use letters, numbers, - and _ (max 80).".to_owned(),
            });
        }
        Ok(())
    }

    /// Save a validated visual scene spec under an id (overwrites by design —
    /// scenes are versioned by the undoable timeline ops that place them).
    /// The spec must carry a positive `scene.duration` (placement length).
    pub fn save_visual_scene(
        &mut self,
        id: &str,
        spec: serde_json::Value,
    ) -> Result<(), CommandError> {
        Self::check_scene_id(id)?;
        let duration = spec
            .pointer("/scene/duration")
            .and_then(serde_json::Value::as_f64)
            .filter(|duration| duration.is_finite() && *duration > 0.0)
            .ok_or_else(|| CommandError {
                code: "AVID_PROJECT_001".to_owned(),
                message: "Scene needs scene.duration > 0.".to_owned(),
            })?;
        let _ = duration;
        self.manifest.visuals.insert(id.to_owned(), spec);
        self.persist()
    }

    /// Stored scenes as `(id, spec)` pairs, ordered by id.
    #[must_use]
    pub fn list_visual_scenes(&self) -> Vec<(String, serde_json::Value)> {
        let mut scenes: Vec<(String, serde_json::Value)> = self
            .manifest
            .visuals
            .iter()
            .map(|(id, spec)| (id.clone(), spec.clone()))
            .collect();
        scenes.sort_by(|a, b| a.0.cmp(&b.0));
        scenes
    }

    /// Place a stored scene on the graphics track at a start time.
    /// Creates the G1 track on first use (same additive precedent as captions).
    /// Duration comes from the scene spec; the clip is fully undoable.
    pub fn place_visual_on_timeline(
        &mut self,
        scene_id: &str,
        start: f64,
    ) -> Result<(), CommandError> {
        let spec = self
            .manifest
            .visuals
            .get(scene_id)
            .cloned()
            .ok_or_else(|| CommandError {
                code: "AVID_PROJECT_001".to_owned(),
                message: format!("Unknown visual scene: {scene_id}"),
            })?;
        let duration = spec
            .pointer("/scene/duration")
            .and_then(serde_json::Value::as_f64)
            .filter(|duration| duration.is_finite() && *duration > 0.0)
            .ok_or_else(|| CommandError {
                code: "AVID_PROJECT_001".to_owned(),
                message: "Scene needs scene.duration > 0.".to_owned(),
            })?;
        if !start.is_finite() || start < 0.0 {
            return Err(CommandError {
                code: "AVID_TIMELINE_001".to_owned(),
                message: "Placement start must be >= 0.".to_owned(),
            });
        }
        if !self
            .timeline
            .tracks
            .iter()
            .any(|track| track.id == "graphics")
        {
            let index = self.timeline.tracks.len() as u32;
            self.timeline.tracks.push(Track {
                id: "graphics".to_owned(),
                kind: TrackKind::Graphics,
                index,
                name: "Graphics".to_owned(),
                locked: false,
                muted: false,
            });
        }
        self.mutate(
            "Place visual",
            Box::new(AddClipCommand {
                clip: Clip {
                    id: format!("vis-{scene_id}"),
                    source_media_id: format!("visual:{scene_id}"),
                    track_id: "graphics".to_owned(),
                    start,
                    duration,
                    in_point: 0.0,
                    volume: 1.0,
                    muted: false,
                    name: format!("Visual: {scene_id}"),
                },
            }),
        )
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
        request: &ExportRequest,
        cancel: &std::sync::atomic::AtomicBool,
        on_progress: &dyn Fn(f64),
    ) -> Result<ExportResult, CommandError> {
        use avid_render::{
            argv_auto, graph_from_timeline, resolve_preset, validate_export_filename,
        };
        validate_export_filename(&request.filename)
            .map_err(|error| command_error_from_render(&error))?;
        let (width, fps) =
            resolve_preset(&request.preset_id, request.custom_width, request.custom_fps)
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
        let output = exports.join(&request.filename);
        let argv = argv_auto(engine.ffmpeg_path(), &output, &graph)
            .map_err(|error| command_error_from_render(&error))?;
        engine
            .run_render_with_progress(&argv, graph.total_duration(), cancel, on_progress)
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
            relative_path: format!("exports/{}", request.filename),
            absolute_path: output.display().to_string(),
            duration,
            width,
            fps,
            caption_path: write_caption_sidecar(&exports, &request.filename, timeline)?,
        })
    }

    /// Export the open timeline (locks briefly to snapshot, renders off-lock).
    pub fn export(
        &mut self,
        engine: &MediaEngine,
        request: &ExportRequest,
        cancel: &std::sync::atomic::AtomicBool,
        on_progress: &dyn Fn(f64),
    ) -> Result<ExportResult, CommandError> {
        Self::export_snapshot(
            engine,
            &self.dir.clone(),
            &self.timeline.clone(),
            &self.manifest.assets.clone(),
            request,
            cancel,
            on_progress,
        )
    }
}

/// Export request (mirrors `@avid/shared-types` `ExportRequest` for IPC).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportRequest {
    /// Preset id (`youtube-1080p`, …, `custom`).
    pub preset_id: String,
    /// Required when `preset_id` is `custom`.
    pub custom_width: Option<u32>,
    /// Required when `preset_id` is `custom`.
    pub custom_fps: Option<u32>,
    /// Bare `.mp4` file name.
    pub filename: String,
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
    /// Project-relative caption sidecar (`exports/<stem>.srt`), if any
    /// caption clips exist. Burn-in follows with a text-capable sidecar.
    pub caption_path: Option<String>,
}

/// Write the SRT sidecar for caption-track clips, if any.
/// Returns the project-relative path or `None` (no captions is normal).
fn write_caption_sidecar(
    exports: &Path,
    filename: &str,
    timeline: &avid_timeline::Timeline,
) -> Result<Option<String>, CommandError> {
    let cues = avid_render::caption_cues_from_timeline(timeline);
    if cues.is_empty() {
        return Ok(None);
    }
    let stem = filename.trim_end_matches(".mp4").trim_end_matches(".MP4");
    let relative_path = format!("exports/{stem}.srt");
    std::fs::write(
        exports.join(format!("{stem}.srt")),
        avid_render::captions_to_srt(&cues),
    )
    .map_err(|e| CommandError {
        code: "AVID_RENDER_004".to_owned(),
        message: format!("AVID couldn't write captions: {e}"),
    })?;
    Ok(Some(relative_path))
}

/// Recovery snapshot metadata (returned to the UI).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SnapshotInfo {
    /// Bare file name (`<millis>-<label>.json`) — safe to round-trip.
    pub name: String,
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
            visuals: std::collections::HashMap::new(),
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
            volume: 1.0,
            muted: false,
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
    fn proxy_rejects_unknown_and_audio_only_assets() {
        let Ok(engine) = MediaEngine::system() else {
            return;
        };
        let dir = std::env::temp_dir().join("avid-proxy-invalid");
        std::fs::remove_dir_all(&dir).ok();
        let mut session = Session::create(dir.clone(), manifest("px")).unwrap();
        assert!(session.generate_proxy(&engine, "nope").is_err());
        // Audio-only asset (no dimensions) fails closed with guidance.
        session.manifest.assets.push(MediaAsset {
            id: "audio-1".to_owned(),
            file_name: "a.wav".to_owned(),
            relative_path: "media/a.wav".to_owned(),
            duration: Some(3.0),
            dimensions: None,
            hash: None,
            proxy_path: None,
        });
        let err = session.generate_proxy(&engine, "audio-1").unwrap_err();
        assert_eq!(err.code, "AVID_MEDIA_001");
        std::fs::remove_dir_all(&dir).ok();
    }

    /// LIVE: generate a proxy from the talking-head fixture and verify it
    /// is a smaller file with video dimensions. Ignored without fixtures.
    #[test]
    #[ignore]
    fn live_proxy_generation_produces_small_preview() {
        let engine = MediaEngine::system().expect("system ffmpeg/ffprobe");
        let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../fixtures/media/talkinghead_10s.mp4");
        assert!(fixture.is_file(), "missing fixture: {}", fixture.display());
        let dir = std::env::temp_dir().join("avid-proxy-live");
        std::fs::remove_dir_all(&dir).ok();
        let mut session = Session::create(dir.clone(), manifest("proxy-live")).unwrap();
        let asset = session.import_file(&engine, &fixture).unwrap();
        let proxied = session.generate_proxy(&engine, &asset.id).unwrap();
        assert_eq!(
            proxied.proxy_path,
            Some(format!("proxies/{}.mp4", asset.id))
        );
        let proxy_file = dir.join(proxied.proxy_path.unwrap());
        assert!(proxy_file.is_file());
        let info = engine.probe_file(&proxy_file).unwrap();
        let video = info.video_stream().expect("proxy has video");
        assert!(video.width.unwrap_or(9999) <= 960, "proxy must be small");
        assert!(
            std::fs::metadata(&proxy_file).unwrap().len()
                < std::fs::metadata(&fixture).unwrap().len()
        );
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn clip_audio_round_trips_and_persists() {
        let dir = std::env::temp_dir().join("avid-audio-test");
        std::fs::remove_dir_all(&dir).ok();
        let mut session = Session::create(dir.clone(), manifest("au")).unwrap();
        session.add_clip(clip("a", 0.0, 8.0)).unwrap();
        session.set_clip_audio("a", 0.25, true).unwrap();
        assert_eq!(
            (
                session.timeline().clips["a"].volume,
                session.timeline().clips["a"].muted
            ),
            (0.25, true)
        );
        session.undo().unwrap();
        assert_eq!(
            (
                session.timeline().clips["a"].volume,
                session.timeline().clips["a"].muted
            ),
            (1.0, false)
        );
        assert!(session.set_clip_audio("missing", 1.0, false).is_err());
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

    fn flag() -> std::sync::atomic::AtomicBool {
        std::sync::atomic::AtomicBool::new(false)
    }

    fn request(preset_id: &str, filename: &str) -> ExportRequest {
        ExportRequest {
            preset_id: preset_id.to_owned(),
            custom_width: None,
            custom_fps: None,
            filename: filename.to_owned(),
        }
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
            .export(
                &engine,
                &request("youtube-1080p", "out.mp4"),
                &flag(),
                &|_| {},
            )
            .unwrap_err();
        assert_eq!(err.code, "AVID_RENDER_001");
        // Bad preset / filename likewise never reach ffmpeg.
        let err = session
            .export(&engine, &request("nope", "out.mp4"), &flag(), &|_| {})
            .unwrap_err();
        assert_eq!(err.code, "AVID_RENDER_006");
        let err = session
            .export(
                &engine,
                &request("youtube-1080p", "../evil.mp4"),
                &flag(),
                &|_| {},
            )
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
        session.ensure_caption_track().unwrap();
        session
            .add_clip(avid_timeline::Clip {
                id: "cap-1".to_owned(),
                source_media_id: "caption".to_owned(),
                track_id: "captions".to_owned(),
                start: 1.0,
                duration: 2.0,
                in_point: 0.0,
                volume: 1.0,
                muted: false,
                name: "Hello captions".to_owned(),
            })
            .unwrap();
        let result = session
            .export(
                &engine,
                &request("short-1080p", "short.mp4"),
                &flag(),
                &|_| {},
            )
            .unwrap();
        assert_eq!(result.relative_path, "exports/short.mp4");
        assert_eq!((result.width, result.fps), (1080, 30));
        assert!(
            (result.duration - 10.0).abs() < 0.75,
            "duration = {}",
            result.duration
        );
        assert!(dir.join(&result.relative_path).is_file());
        // Caption sidecar written alongside the render.
        assert_eq!(result.caption_path, Some("exports/short.srt".to_owned()));
        let srt = std::fs::read_to_string(dir.join("exports/short.srt")).unwrap();
        assert!(srt.contains("Hello captions"), "{srt}");
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn snapshot_restore_recovers_and_restarts_history() {
        let dir = std::env::temp_dir().join("avid-snapshot-test");
        std::fs::remove_dir_all(&dir).ok();
        let mut session = Session::create(dir.clone(), manifest("snap")).unwrap();
        session.add_clip(clip("a", 0.0, 8.0)).unwrap();
        let name = session.snapshot("pre-apply").unwrap();
        assert!(name.ends_with(".json"));
        assert_eq!(session.list_snapshots().len(), 1);
        session.remove_clip("a").unwrap();
        assert!(session.timeline().clips.is_empty());
        session.restore_snapshot(&name).unwrap();
        assert!(session.timeline().clips.contains_key("a"));
        assert_eq!(session.undo_depth(), 0);
        // Traversal and missing names fail closed.
        assert!(session.restore_snapshot("../evil.json").is_err());
        assert!(session.restore_snapshot("nope.json").is_err());
        // Pruning keeps the newest 10.
        for _ in 0..12 {
            session.snapshot("x").unwrap();
        }
        assert_eq!(session.list_snapshots().len(), 10);
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn visual_scenes_save_list_place_and_undo() {
        let dir = std::env::temp_dir().join("avid-visual-test");
        std::fs::remove_dir_all(&dir).ok();
        let mut session = Session::create(dir.clone(), manifest("vis")).unwrap();
        let spec = serde_json::json!({
            "scene": {"width": 1920, "height": 1080, "duration": 8},
            "elements": [{"type": "node", "id": "a", "x": 0, "y": 0, "label": "A"}],
            "connections": []
        });
        session.save_visual_scene("kafka", spec.clone()).unwrap();
        assert!(session.save_visual_scene("../evil", spec).is_err());
        assert!(session.save_visual_scene("ok", serde_json::json!({"no": "scene"})).is_err());
        assert_eq!(session.list_visual_scenes().len(), 1);

        session.place_visual_on_timeline("kafka", 4.0).unwrap();
        let placed = &session.timeline().clips["vis-kafka"];
        assert_eq!((placed.start, placed.duration), (4.0, 8.0));
        assert_eq!(placed.track_id, "graphics");
        assert!(session.place_visual_on_timeline("ghost", 0.0).is_err());
        session.undo().unwrap();
        assert!(!session.timeline().clips.contains_key("vis-kafka"));
        // Scene survives undo (only the placement is reverted).
        assert_eq!(session.list_visual_scenes().len(), 1);
        std::fs::remove_dir_all(&dir).ok();
    }
}
