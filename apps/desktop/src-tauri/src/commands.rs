//! Tauri command layer: the ONLY mutation path into Rust state (ADR-001).
//!
//! Thin wrappers over pure helpers — helpers hold the logic and the tests,
//! commands hold the `#[tauri::command]` attribute and nothing else.

use std::collections::HashMap;
use std::path::Path;

use avid_media::{MediaEngine, MediaInfo};
use avid_project::{MediaAsset, ProjectManifest, ProjectMeta};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, State};

use crate::session::{projects_base, Session};

/// Structured command error: `{code, message}` for the UI's humane error
/// disclosure (AGENTS §51). Never leaks paths or internals beyond the message.
#[derive(Debug, Clone, Serialize)]
pub struct CommandError {
    /// e.g. `"AVID_PROJECT_002"`.
    pub code: String,
    pub message: String,
}

impl From<avid_project::ProjectError> for CommandError {
    fn from(error: avid_project::ProjectError) -> Self {
        Self {
            code: error.code().to_owned(),
            message: error.to_string(),
        }
    }
}

impl From<avid_media::MediaError> for CommandError {
    fn from(error: avid_media::MediaError) -> Self {
        command_error_from_media(&error)
    }
}

/// Map engine errors to user-actionable messages (single place, so the
/// session layer and commands never invent divergent copy).
#[must_use]
pub fn command_error_from_media(error: &avid_media::MediaError) -> CommandError {
    CommandError {
        code: error.code().to_owned(),
        message: soften(error),
    }
}

/// Map render errors to command errors, preserving stable codes.
#[must_use]
pub fn command_error_from_render(error: &avid_render::RenderError) -> CommandError {
    CommandError {
        code: error.code().to_owned(),
        message: error.to_string(),
    }
}

/// Translate engine errors into user-actionable messages. The technical
/// detail stays available via logs, not raw stderr dumps.
fn soften(error: &avid_media::MediaError) -> String {
    match error {
        avid_media::MediaError::UnsafePath(_) => {
            "That file is outside the project folder. Import it into the project first.".to_owned()
        }
        avid_media::MediaError::BinaryUnavailable(_) => {
            "AVID couldn't find FFmpeg. Install it and retry (see Development guide).".to_owned()
        }
        avid_media::MediaError::ProcessFailed { op, .. } => {
            format!("AVID couldn't {op} this file — it may use an unsupported codec. Try converting it.")
        }
        avid_media::MediaError::Parse(_) => {
            "AVID couldn't read this file's metadata. Try converting it.".to_owned()
        }
    }
}

/// App identity for the top bar and diagnostics.
#[derive(Debug, Clone, Serialize)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    /// Project manifest schema this build reads/writes.
    pub schema_version: u32,
}

/// Frontend's project-creation payload (mirrors `@avid/shared-types`).
#[derive(Debug, Clone, Deserialize)]
pub struct NewProjectInput {
    pub name: String,
    pub canvas: String,
    #[serde(rename = "frameRate")]
    pub frame_rate: u32,
    pub resolution: String,
    #[serde(rename = "templateId")]
    pub template_id: Option<String>,
}

const CANVASES: &[&str] = &["16:9", "9:16", "1:1", "4:5", "custom"];
const FRAME_RATES: &[u32] = &[24, 25, 30, 50, 60];
const RESOLUTIONS: &[&str] = &["720p", "1080p", "4K"];

/// Validate input and build the initial manifest (empty timeline).
/// Pure — unit-tested below; the command is a thin wrapper.
pub fn create_project_manifest(
    input: NewProjectInput,
    id: String,
    now: String,
) -> Result<ProjectManifest, CommandError> {
    let name = input.name.trim();
    if name.is_empty() || name.len() > 80 {
        return Err(CommandError {
            code: "AVID_PROJECT_001".to_owned(),
            message: "Give the project a name (1–80 characters).".to_owned(),
        });
    }
    if !CANVASES.contains(&input.canvas.as_str()) {
        return Err(CommandError {
            code: "AVID_PROJECT_001".to_owned(),
            message: "Choose a canvas aspect ratio.".to_owned(),
        });
    }
    if !FRAME_RATES.contains(&input.frame_rate) {
        return Err(CommandError {
            code: "AVID_PROJECT_001".to_owned(),
            message: "Choose a frame rate.".to_owned(),
        });
    }
    if !RESOLUTIONS.contains(&input.resolution.as_str()) {
        return Err(CommandError {
            code: "AVID_PROJECT_001".to_owned(),
            message: "Choose a resolution.".to_owned(),
        });
    }
    Ok(ProjectManifest {
        id,
        version: avid_project::MANIFEST_VERSION,
        meta: ProjectMeta {
            name: name.to_owned(),
            canvas: input.canvas,
            frame_rate: input.frame_rate,
            resolution: input.resolution,
            template_id: input.template_id,
            created_at: now.clone(),
            updated_at: now,
        },
        timeline: serde_json::json!({"tracks": [], "clips": {}}),
        assets: vec![],
        transcripts: HashMap::new(),
    })
}

/// Probe a file already validated to live under the project directory.
/// Uses the explicit-path engine entry point: containment is established by
/// [`avid_project::validate_relative_path`] + joining under the session dir.
pub fn probe_media_file(
    engine: &MediaEngine,
    absolute_path: &str,
) -> Result<MediaInfo, CommandError> {
    engine
        .probe_file(Path::new(absolute_path))
        .map_err(CommandError::from)
}

#[tauri::command]
#[must_use = "commands must be registered in the invoke handler"]
pub fn get_app_info(state: tauri::State<'_, crate::AppState>) -> AppInfo {
    app_info(state.schema_version())
}

/// Pure helper behind `get_app_info` (unit-tested; the command adds State).
#[must_use]
pub fn app_info(schema_version: u32) -> AppInfo {
    AppInfo {
        name: "AVID".to_owned(),
        version: env!("CARGO_PKG_VERSION").to_owned(),
        schema_version,
    }
}

#[tauri::command]
pub fn create_project(
    handle: AppHandle,
    state: State<'_, crate::AppState>,
    input: NewProjectInput,
) -> Result<ProjectManifest, CommandError> {
    let now = format!("{:?}", std::time::SystemTime::now());
    let id = format!("proj-{}", MediaEngine::mint_asset_id());
    let manifest = create_project_manifest(input, id.clone(), now)?;
    let dir = projects_base_dir(&handle)?.join(&id);
    let session = Session::create(dir, manifest.clone())?;
    state.open(session);
    Ok(manifest)
}

/// Projects root under app data (`…/avid-projects`), created on demand.
fn projects_base_dir(handle: &AppHandle) -> Result<std::path::PathBuf, CommandError> {
    let base = projects_base(&handle.path().app_data_dir().map_err(|e| CommandError {
        code: "AVID_PROJECT_001".to_owned(),
        message: format!("AVID couldn't locate its data folder: {e}"),
    })?);
    std::fs::create_dir_all(&base).map_err(|e| CommandError {
        code: "AVID_PROJECT_001".to_owned(),
        message: format!("AVID couldn't prepare its data folder: {e}"),
    })?;
    Ok(base)
}

#[tauri::command]
pub fn open_project(
    handle: AppHandle,
    state: State<'_, crate::AppState>,
    id: String,
) -> Result<ProjectManifest, CommandError> {
    // Ids are minted as `proj-<uuid>`; reject traversal before joining paths.
    if id.contains('/') || id.contains('\\') || id.contains("..") {
        return Err(CommandError {
            code: "AVID_PROJECT_001".to_owned(),
            message: "That project id isn't valid.".to_owned(),
        });
    }
    let session = Session::load(projects_base_dir(&handle)?.join(&id))?;
    let manifest = session.manifest().clone();
    state.open(session);
    Ok(manifest)
}

#[tauri::command]
pub fn list_projects(handle: AppHandle) -> Result<Vec<ProjectManifest>, CommandError> {
    let base = projects_base_dir(&handle)?;
    let mut projects = vec![];
    let entries = std::fs::read_dir(&base).map_err(|e| CommandError {
        code: "AVID_PROJECT_001".to_owned(),
        message: format!("AVID couldn't list projects: {e}"),
    })?;
    for entry in entries.flatten() {
        let manifest_path = entry.path().join("project.json");
        if let Ok(json) = std::fs::read_to_string(&manifest_path) {
            if let Ok(manifest) = ProjectManifest::from_json(&json) {
                projects.push(manifest);
            }
        }
    }
    Ok(projects)
}

#[tauri::command]
pub fn import_media(
    state: State<'_, crate::AppState>,
    source_path: String,
) -> Result<MediaAsset, CommandError> {
    // System binaries until the pinned sidecar ships (ADR-002).
    let engine = MediaEngine::system().map_err(CommandError::from)?;
    state.with_session(|session| session.import_file(&engine, Path::new(&source_path)))
}

#[tauri::command]
pub fn transcribe_media(
    handle: AppHandle,
    state: State<'_, crate::AppState>,
    asset_id: String,
    language: String,
) -> Result<avid_ai::Transcript, CommandError> {
    let engine = MediaEngine::system().map_err(CommandError::from)?;
    let cache = handle.path().app_cache_dir().map_err(|e| CommandError {
        code: "AVID_PROJECT_001".to_owned(),
        message: format!("AVID couldn't locate its cache folder: {e}"),
    })?;
    let model = model_path(&cache);
    let audio_cache = cache.join("avid-audio");
    state.with_session(|session| {
        session.transcribe_asset(&engine, &model, &audio_cache, &asset_id, &language)
    })
}

/// Speech-model resolution: explicit env override (tests) else app cache.
/// A missing model is reported downstream with download guidance — never silent.
fn model_path(cache: &Path) -> std::path::PathBuf {
    if let Ok(path) = std::env::var("AVID_MODEL_PATH") {
        return std::path::PathBuf::from(path);
    }
    cache.join("avid/models/ggml-tiny.en.bin")
}

#[tauri::command]
pub fn timeline_get(
    state: State<'_, crate::AppState>,
) -> Result<avid_timeline::Timeline, CommandError> {
    state.with_session(|session| Ok(session.timeline().clone()))
}

#[tauri::command]
pub fn timeline_add_clip(
    state: State<'_, crate::AppState>,
    clip: avid_timeline::Clip,
) -> Result<(), CommandError> {
    state.with_session(|session| session.add_clip(clip))
}

#[tauri::command]
pub fn timeline_remove_clip(
    state: State<'_, crate::AppState>,
    clip_id: String,
) -> Result<(), CommandError> {
    state.with_session(|session| session.remove_clip(&clip_id))
}

#[tauri::command]
pub fn timeline_split_clip(
    state: State<'_, crate::AppState>,
    clip_id: String,
    at: f64,
) -> Result<(), CommandError> {
    state.with_session(|session| session.split_clip(&clip_id, at))
}

#[tauri::command]
pub fn timeline_undo(state: State<'_, crate::AppState>) -> Result<String, CommandError> {
    state.with_session(|session| session.undo())
}

#[tauri::command]
pub fn timeline_redo(state: State<'_, crate::AppState>) -> Result<String, CommandError> {
    state.with_session(|session| session.redo())
}

/// Speech-model status for Settings/AI transparency.
#[derive(Debug, Clone, Serialize)]
pub struct ModelStatus {
    pub downloaded: bool,
    pub path: String,
    pub bytes: Option<u64>,
}

fn speech_model_path(handle: &AppHandle) -> Result<std::path::PathBuf, CommandError> {
    if let Ok(path) = std::env::var("AVID_MODEL_PATH") {
        return Ok(std::path::PathBuf::from(path));
    }
    Ok(handle
        .path()
        .app_cache_dir()
        .map_err(|e| CommandError {
            code: "AVID_TRANSCRIBE_001".to_owned(),
            message: format!("AVID couldn't locate its cache folder: {e}"),
        })?
        .join("avid/models")
        .join(avid_ai::DEFAULT_SPEECH_MODEL_FILE))
}

#[tauri::command]
pub fn speech_model_status(handle: AppHandle) -> Result<ModelStatus, CommandError> {
    let path = speech_model_path(&handle)?;
    Ok(ModelStatus {
        downloaded: path.is_file(),
        path: path.display().to_string(),
        bytes: std::fs::metadata(&path).ok().map(|meta| meta.len()),
    })
}

/// Download the speech model on a blocking thread (never the async executor).
/// Long first run (~77 MB); progress reporting lands with the job center.
#[tauri::command]
pub async fn ensure_speech_model(handle: AppHandle) -> Result<ModelStatus, CommandError> {
    let path = speech_model_path(&handle)?;
    if path.is_file() {
        return Ok(ModelStatus {
            downloaded: true,
            path: path.display().to_string(),
            bytes: std::fs::metadata(&path).ok().map(|meta| meta.len()),
        });
    }
    let url = avid_ai::DEFAULT_SPEECH_MODEL_URL.to_owned();
    let bytes = tokio::task::spawn_blocking(move || avid_ai::download_speech_model(&url, &path))
        .await
        .map_err(|e| CommandError {
            code: "AVID_TRANSCRIBE_001".to_owned(),
            message: format!("Model download was interrupted: {e}"),
        })?
        .map_err(|e| CommandError {
            code: e.code().to_owned(),
            message: e.to_string(),
        })?;
    Ok(ModelStatus {
        downloaded: true,
        path: speech_model_path(&handle)?.display().to_string(),
        bytes: Some(bytes),
    })
}

/// Export request from the dialog (mirrors `@avid/shared-types`).
#[derive(Debug, Clone, Deserialize)]
pub struct ExportRequest {
    pub preset_id: String,
    pub custom_width: Option<u32>,
    pub custom_fps: Option<u32>,
    pub filename: String,
}

/// Render the open timeline to `exports/` and verify the output.
/// Async + blocking thread: renders take seconds to minutes and must never
/// freeze the UI. Progress streaming lands with the job center.
#[tauri::command]
pub async fn render_export(
    state: State<'_, crate::AppState>,
    request: ExportRequest,
) -> Result<crate::session::ExportResult, CommandError> {
    // Snapshot under the lock; render off-lock.
    let snapshot = state.with_session(|session| {
        Ok((
            session.dir().to_path_buf(),
            session.timeline().clone(),
            session.manifest().assets.clone(),
        ))
    })?;
    let engine = MediaEngine::system().map_err(CommandError::from)?;
    tokio::task::spawn_blocking(move || {
        crate::session::Session::export_snapshot(
            &engine,
            &snapshot.0,
            &snapshot.1,
            &snapshot.2,
            &request.preset_id,
            request.custom_width,
            request.custom_fps,
            &request.filename,
        )
    })
    .await
    .map_err(|e| CommandError {
        code: "AVID_RENDER_004".to_owned(),
        message: format!("Export was interrupted: {e}"),
    })?
}

#[tauri::command]
pub fn probe_media(
    state: State<'_, crate::AppState>,
    relative_path: String,
) -> Result<MediaInfo, CommandError> {
    // System binaries until the pinned sidecar ships (ADR-002).
    let engine = MediaEngine::system().map_err(CommandError::from)?;
    state.with_session(|session| {
        avid_project::validate_relative_path(&relative_path).map_err(CommandError::from)?;
        probe_media_file(
            &engine,
            &session.dir().join(&relative_path).display().to_string(),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input(name: &str) -> NewProjectInput {
        NewProjectInput {
            name: name.to_owned(),
            canvas: "16:9".to_owned(),
            frame_rate: 30,
            resolution: "1080p".to_owned(),
            template_id: None,
        }
    }

    #[test]
    fn creates_valid_manifest() {
        let manifest =
            create_project_manifest(input("  Demo  "), "id-1".to_owned(), "now".to_owned())
                .unwrap();
        assert_eq!(manifest.meta.name, "Demo");
        assert_eq!(manifest.version, avid_project::MANIFEST_VERSION);
        assert!(manifest.assets.is_empty());
        // Manifest round-trips through its own parser.
        let back = ProjectManifest::from_json(&manifest.to_json().unwrap()).unwrap();
        assert_eq!(manifest, back);
    }

    #[test]
    fn rejects_bad_creation_input() {
        assert!(create_project_manifest(input(""), "id".to_owned(), "now".to_owned()).is_err());
        assert!(create_project_manifest(input("x"), "id".to_owned(), "now".to_owned()).is_ok());
        let mut bad_canvas = input("ok");
        bad_canvas.canvas = "21:9".to_owned();
        assert!(create_project_manifest(bad_canvas, "id".to_owned(), "now".to_owned()).is_err());
        let mut bad_fps = input("ok");
        bad_fps.frame_rate = 48;
        assert!(create_project_manifest(bad_fps, "id".to_owned(), "now".to_owned()).is_err());
    }

    #[test]
    fn app_info_reports_schema_version() {
        let info = app_info(avid_project::MANIFEST_VERSION);
        assert_eq!(info.name, "AVID");
        assert_eq!(info.schema_version, avid_project::MANIFEST_VERSION);
    }
}
