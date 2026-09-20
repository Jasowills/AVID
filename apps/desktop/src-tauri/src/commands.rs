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
        avid_media::MediaError::Cancelled(_) => "Cancelled.".to_owned(),
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
pub async fn transcribe_media(
    handle: AppHandle,
    state: State<'_, crate::AppState>,
    channel: tauri::ipc::Channel<crate::jobs::JobEvent>,
    asset_id: String,
    language: String,
) -> Result<avid_ai::Transcript, CommandError> {
    use crate::jobs::{JobEvent, JobKind, JobStatus};
    let engine = MediaEngine::system().map_err(CommandError::from)?;
    let cache = handle.path().app_cache_dir().map_err(|e| CommandError {
        code: "AVID_PROJECT_001".to_owned(),
        message: format!("AVID couldn't locate its cache folder: {e}"),
    })?;
    let model = model_path(&cache);
    let audio_cache = cache.join("avid-audio");
    // Snapshot under the lock; inference off-lock (indeterminate progress).
    let snapshot = state.with_session(|session| {
        let asset = session
            .manifest()
            .assets
            .iter()
            .find(|asset| asset.id == asset_id)
            .cloned()
            .ok_or_else(|| CommandError {
                code: "AVID_MEDIA_001".to_owned(),
                message: "Unknown media asset.".to_owned(),
            })?;
        Ok((session.dir().to_path_buf(), asset))
    })?;
    let jobs = state.jobs();
    let (job_id, _cancel) = jobs.start(JobKind::Transcribe, format!("Transcribe {asset_id}"));
    let event = |status: JobStatus| {
        let _ = channel.send(JobEvent {
            job_id: job_id.clone(),
            status,
            progress: None,
            message: None,
        });
    };
    event(JobStatus::Running);
    let transcript = tokio::task::spawn_blocking(move || {
        crate::session::Session::transcribe_snapshot(
            &engine,
            &model,
            &audio_cache,
            &snapshot.0,
            &snapshot.1,
            &language,
        )
    })
    .await
    .map_err(|e| CommandError {
        code: "AVID_TRANSCRIBE_003".to_owned(),
        message: format!("Transcription was interrupted: {e}"),
    })?;
    match transcript {
        Ok(transcript) => {
            state
                .with_session(|session| session.store_transcript(&asset_id, transcript.clone()))?;
            jobs.finish(&job_id, "Transcription complete.");
            event(JobStatus::Finished);
            Ok(transcript)
        }
        Err(error) => {
            jobs.fail(&job_id, error.message.clone());
            event(JobStatus::Failed);
            Err(error)
        }
    }
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
pub fn timeline_trim_clip(
    state: State<'_, crate::AppState>,
    clip_id: String,
    start: f64,
    duration: f64,
) -> Result<(), CommandError> {
    state.with_session(|session| session.trim_clip(&clip_id, start, duration))
}

#[tauri::command]
pub fn timeline_undo(state: State<'_, crate::AppState>) -> Result<String, CommandError> {
    state.with_session(|session| session.undo())
}

#[tauri::command]
pub fn timeline_redo(state: State<'_, crate::AppState>) -> Result<String, CommandError> {
    state.with_session(|session| session.redo())
}

/// Confidence in a proposal (honest buckets, never fake precision — §119).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Confidence {
    High,
    Medium,
}

/// One proposed cut for review. Never applied without explicit accept.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposedCut {
    pub start: f64,
    pub end: f64,
    pub reason: String,
    pub confidence: Confidence,
}

/// A rough-cut proposal: sorted cuts plus totals for the review UI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoughCutProposal {
    pub cuts: Vec<ProposedCut>,
    pub removable_seconds: f64,
    /// What was analyzed, e.g. `"silence+transcript"`.
    pub analyzed: String,
}

/// Merge silence spans and filler hits into a sorted, deduplicated proposal.
/// Filler hits fully inside a silence span are dropped (one cut covers both).
/// Pure over explicit inputs — unit-tested without ffmpeg or models.
pub fn build_proposal(
    silences: &[avid_media::SilenceSpan],
    fillers: &[avid_ai::FillerHit],
    include_fillers: bool,
) -> RoughCutProposal {
    let mut cuts: Vec<ProposedCut> = silences
        .iter()
        .map(|span| ProposedCut {
            start: span.start,
            end: span.end,
            reason: "long pause".to_owned(),
            confidence: Confidence::High,
        })
        .collect();
    let mut analyzed = "silence".to_owned();
    if include_fillers {
        analyzed = "silence+transcript".to_owned();
        for hit in fillers {
            let inside_silence = silences
                .iter()
                .any(|span| span.start <= hit.start && hit.end <= span.end);
            if !inside_silence {
                cuts.push(ProposedCut {
                    start: hit.start,
                    end: hit.end,
                    reason: format!("filler word: {}", hit.word),
                    confidence: Confidence::Medium,
                });
            }
        }
    }
    cuts.sort_by(|a, b| {
        a.start
            .partial_cmp(&b.start)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let removable_seconds = cuts.iter().map(|cut| cut.end - cut.start).sum();
    RoughCutProposal {
        cuts,
        removable_seconds,
        analyzed,
    }
}

/// Propose a rough cut for an asset: silence detection plus optional filler
/// detection from a stored transcript. Async job (ffmpeg pass + inference
/// must never freeze the UI); returns a proposal for the review UI.
#[tauri::command]
pub async fn propose_rough_cut(
    state: State<'_, crate::AppState>,
    channel: tauri::ipc::Channel<crate::jobs::JobEvent>,
    asset_id: String,
    min_silence_seconds: f64,
    include_fillers: bool,
) -> Result<RoughCutProposal, CommandError> {
    use crate::jobs::{JobEvent, JobKind, JobStatus};
    if !(0.3..=10.0).contains(&min_silence_seconds) {
        return Err(CommandError {
            code: "AVID_MEDIA_003".to_owned(),
            message: "Minimum silence must be 0.3–10 seconds.".to_owned(),
        });
    }
    let snapshot = state.with_session(|session| {
        let asset = session
            .manifest()
            .assets
            .iter()
            .find(|asset| asset.id == asset_id)
            .cloned()
            .ok_or_else(|| CommandError {
                code: "AVID_MEDIA_001".to_owned(),
                message: "Unknown media asset.".to_owned(),
            })?;
        let transcript = if include_fillers {
            session.transcript_for(&asset_id)
        } else {
            None
        };
        Ok((session.dir().to_path_buf(), asset, transcript))
    })?;
    let engine = MediaEngine::system().map_err(CommandError::from)?;
    let jobs = state.jobs();
    let (job_id, _cancel) = jobs.start(JobKind::Transcribe, format!("Rough cut {asset_id}"));
    let event = |status: JobStatus| {
        let _ = channel.send(JobEvent {
            job_id: job_id.clone(),
            status,
            progress: None,
            message: None,
        });
    };
    event(JobStatus::Running);
    let proposal = tokio::task::spawn_blocking(move || {
        let path = snapshot.0.join(&snapshot.1.relative_path);
        let silences = engine.detect_silence_file(&path, -30.0, min_silence_seconds)?;
        let fillers = snapshot
            .2
            .as_ref()
            .map(|transcript| transcript.find_fillers())
            .unwrap_or_default();
        Ok::<_, CommandError>(build_proposal(
            &silences,
            &fillers,
            include_fillers && snapshot.2.is_some(),
        ))
    })
    .await
    .map_err(|e| CommandError {
        code: "AVID_MEDIA_003".to_owned(),
        message: format!("Rough-cut analysis was interrupted: {e}"),
    })?;
    match &proposal {
        Ok(proposal) => {
            jobs.finish(&job_id, format!("{} cuts proposed.", proposal.cuts.len()));
            event(JobStatus::Finished);
        }
        Err(error) => {
            jobs.fail(&job_id, error.message.clone());
            event(JobStatus::Failed);
        }
    }
    proposal
}

/// One frontend-validated edit operation (shape-checked by
/// `@avid/ai-protocol`; re-validated here against live timeline state).
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EditOperation {
    RemoveRange {
        start: f64,
        end: f64,
    },
    AddCaption {
        start: f64,
        end: f64,
        text: String,
    },
    SplitClip {
        #[serde(rename = "clipId")]
        clip_id: String,
        at: f64,
    },
}

/// Per-operation outcome for the diff UI.
#[derive(Debug, Clone, Serialize)]
pub struct OpResult {
    pub index: usize,
    pub applied: bool,
    pub message: String,
}

/// Grouped-apply report: one undo step, per-op honesty.
#[derive(Debug, Clone, Serialize)]
pub struct ApplyReport {
    pub label: String,
    pub results: Vec<OpResult>,
}

/// Apply accepted operations as ONE transactional undo step (ADR-008).
/// Each op is re-validated against the live timeline first; invalid ops are
/// reported (not applied), valid ones execute atomically — a mid-group
/// failure rolls everything back and surfaces as an error.
#[tauri::command]
pub fn apply_operations(
    state: State<'_, crate::AppState>,
    goal: String,
    operations: Vec<EditOperation>,
) -> Result<ApplyReport, CommandError> {
    state.with_session(|session| apply_validated_ops(session, &goal, operations))
}

/// Pure over an explicit session (unit-tested without Tauri state).
fn apply_validated_ops(
    session: &mut crate::session::Session,
    goal: &str,
    operations: Vec<EditOperation>,
) -> Result<ApplyReport, CommandError> {
    let label: String = format!("AI: {}", goal.trim().chars().take(60).collect::<String>());
    let duration = session.timeline().duration();
    let mut results = vec![];
    let mut commands: Vec<Box<dyn avid_timeline::EditCommand>> = vec![];
    let mut needs_captions = false;

    for (index, operation) in operations.into_iter().enumerate() {
        match validate_operation(session, &operation, duration) {
            Ok(Validated::Skip(message)) => {
                results.push(OpResult {
                    index,
                    applied: false,
                    message,
                });
            }
            Ok(Validated::Run(command)) => {
                if matches!(operation, EditOperation::AddCaption { .. }) {
                    needs_captions = true;
                }
                let message = describe_operation(&operation);
                commands.push(command);
                results.push(OpResult {
                    index,
                    applied: true,
                    message,
                });
            }
            Err(message) => {
                results.push(OpResult {
                    index,
                    applied: false,
                    message,
                });
            }
        }
    }
    if commands.is_empty() {
        return Ok(ApplyReport { label, results });
    }
    if needs_captions {
        session.ensure_caption_track()?;
    }
    session.mutate_group(&label, commands)?;
    Ok(ApplyReport { label, results })
}

/// Validation outcome: runnable command, honest skip, or rejection.
enum Validated {
    Run(Box<dyn avid_timeline::EditCommand>),
    Skip(String),
}

/// Check one op against live state without mutating.
fn validate_operation(
    session: &crate::session::Session,
    operation: &EditOperation,
    duration: f64,
) -> Result<Validated, String> {
    match operation {
        EditOperation::RemoveRange { start, end } => {
            let command = avid_timeline::RemoveRangeCommand::new(*start, *end)
                .map_err(|e| format!("Invalid range: {e}"))?;
            if *end > duration {
                return Err(format!(
                    "Range ends at {end}s but the timeline is {duration:.1}s."
                ));
            }
            let hits = session
                .timeline()
                .clips
                .values()
                .any(|clip| clip.start < *end && *start < clip.start + clip.duration);
            if !hits {
                return Ok(Validated::Skip(
                    "Range hits no clips — nothing to remove.".to_owned(),
                ));
            }
            Ok(Validated::Run(Box::new(command)))
        }
        EditOperation::AddCaption { start, end, text } => {
            let text = text.trim();
            if !start.is_finite() || !end.is_finite() || *start < 0.0 || *end <= *start {
                return Err("Caption needs 0 ≤ start < end.".to_owned());
            }
            if *end > duration {
                return Err(format!(
                    "Caption ends at {end}s but the timeline is {duration:.1}s."
                ));
            }
            if text.is_empty() {
                return Err("Caption text is empty.".to_owned());
            }
            if text.len() > 500 {
                return Err("Caption text exceeds 500 characters.".to_owned());
            }
            Ok(Validated::Run(Box::new(avid_timeline::AddClipCommand {
                clip: avid_timeline::Clip {
                    id: format!("cap-{}", uuid::Uuid::new_v4()),
                    source_media_id: "caption".to_owned(),
                    track_id: "captions".to_owned(),
                    start: *start,
                    duration: *end - *start,
                    in_point: 0.0,
                    name: text.chars().take(80).collect(),
                },
            })))
        }
        EditOperation::SplitClip { clip_id, at } => {
            let clip = session.timeline().clips.get(clip_id).ok_or_else(|| {
                format!("Unknown clip id: {clip_id} (timeline changed since review?).")
            })?;
            if !(clip.start < *at && *at < clip.start + clip.duration) {
                return Err(format!("Split at {at}s falls outside clip {clip_id}."));
            }
            Ok(Validated::Run(Box::new(
                avid_timeline::SplitClipCommand::new(clip_id, *at),
            )))
        }
    }
}

/// Human line for the diff UI and reports.
fn describe_operation(operation: &EditOperation) -> String {
    match operation {
        EditOperation::RemoveRange { start, end } => {
            format!("Remove {start:.1}s → {end:.1}s")
        }
        EditOperation::AddCaption { start, end, text } => {
            format!(
                "Caption {start:.1}s → {end:.1}s: {}",
                text.trim().chars().take(60).collect::<String>()
            )
        }
        EditOperation::SplitClip { clip_id, at } => {
            format!("Split {clip_id} at {at:.1}s")
        }
    }
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

/// Render the open timeline to `exports/` and verify the output.
/// Async + blocking thread: renders take seconds to minutes and must never
/// freeze the UI. Streams `JobEvent` progress over `channel` and honors
/// `cancel_job` (the worker kills ffmpeg promptly).
/// Request shape: [`crate::session::ExportRequest`] (mirrors `@avid/shared-types`).
#[tauri::command]
pub async fn render_export(
    state: State<'_, crate::AppState>,
    channel: tauri::ipc::Channel<crate::jobs::JobEvent>,
    request: crate::session::ExportRequest,
) -> Result<crate::session::ExportResult, CommandError> {
    use crate::jobs::{JobEvent, JobKind, JobStatus};
    // Snapshot under the lock; render off-lock.
    let snapshot = state.with_session(|session| {
        Ok((
            session.dir().to_path_buf(),
            session.timeline().clone(),
            session.manifest().assets.clone(),
        ))
    })?;
    let engine = MediaEngine::system().map_err(CommandError::from)?;
    let jobs = state.jobs();
    let (job_id, cancel) = jobs.start(JobKind::Render, format!("Export {}", request.filename));
    let outcome = tokio::task::spawn_blocking(move || {
        let event = |status: JobStatus, progress: Option<f32>| {
            let _ = channel.send(JobEvent {
                job_id: job_id.clone(),
                status,
                progress,
                message: None,
            });
        };
        event(JobStatus::Running, Some(0.0));
        let result = crate::session::Session::export_snapshot(
            &engine,
            &snapshot.0,
            &snapshot.1,
            &snapshot.2,
            &request,
            &cancel,
            &|fraction| {
                #[allow(clippy::cast_possible_truncation)]
                let progress = fraction.clamp(0.0, 1.0) as f32;
                event(JobStatus::Running, Some(progress));
                jobs.progress(&job_id, fraction);
            },
        );
        match &result {
            Ok(_) => {
                jobs.finish(&job_id, "Export complete.");
                event(JobStatus::Finished, Some(1.0));
            }
            Err(error) if error.code == "AVID_MEDIA_005" => {
                jobs.cancelled(&job_id);
                event(JobStatus::Cancelled, None);
            }
            Err(error) => {
                jobs.fail(&job_id, error.message.clone());
                event(JobStatus::Failed, None);
            }
        }
        result
    })
    .await
    .map_err(|e| CommandError {
        code: "AVID_RENDER_004".to_owned(),
        message: format!("Export was interrupted: {e}"),
    })?;
    outcome
}

/// List imported assets for the media library UI.
#[tauri::command]
pub fn list_assets(state: State<'_, crate::AppState>) -> Result<Vec<MediaAsset>, CommandError> {
    state.with_session(|session| Ok(session.manifest().assets.clone()))
}

/// Provider probe outcome for the Settings connection test (AGENTS §147).
/// Reachability problems are reported IN the result (a test reports, it
/// doesn't throw) — only malformed input is an error.
#[derive(Debug, Clone, Serialize)]
pub struct ProviderProbe {
    pub reachable: bool,
    pub structured_output: bool,
    pub message: String,
}

/// Test a provider endpoint: OpenAI-compatible base URL + model + optional
/// cloud key. Runs the same structured-output probe the app uses at setup.
#[tauri::command]
pub async fn probe_provider(
    base_url: String,
    model: String,
    api_key: Option<String>,
) -> Result<ProviderProbe, CommandError> {
    let base_url = base_url.trim().to_owned();
    if !(base_url.starts_with("http://") || base_url.starts_with("https://"))
        || base_url.contains(char::is_whitespace)
        || base_url.len() > 200
    {
        return Err(CommandError {
            code: "AVID_AI_001".to_owned(),
            message: "Enter an http(s) base URL, e.g. http://localhost:11434.".to_owned(),
        });
    }
    if model.trim().is_empty() || model.len() > 120 {
        return Err(CommandError {
            code: "AVID_AI_001".to_owned(),
            message: "Enter a model id, e.g. qwen2.5:7b.".to_owned(),
        });
    }
    tokio::task::spawn_blocking(move || {
        let mut adapter = avid_ai::OpenAiCompatAdapter::new(
            avid_ai::UreqTransport::new(),
            base_url,
            model,
        );
        if let Some(key) = api_key.filter(|key| !key.trim().is_empty()) {
            adapter = adapter.with_api_key(key);
        }
        match adapter.probe_structured_output() {
            Ok(true) => ProviderProbe {
                reachable: true,
                structured_output: true,
                message: "Provider ready.".to_owned(),
            },
            Ok(false) => ProviderProbe {
                reachable: true,
                structured_output: false,
                message: "Provider responded, but does not support structured output required for AI editing.".to_owned(),
            },
            Err(error) => ProviderProbe {
                reachable: false,
                structured_output: false,
                message: format!("Provider unreachable: {error}"),
            },
        }
    })
    .await
    .map_err(|e| CommandError {
        code: "AVID_AI_001".to_owned(),
        message: format!("Probe was interrupted: {e}"),
    })
}

/// List all known jobs for the job-center UI (newest last).
#[tauri::command]
pub fn list_jobs(state: State<'_, crate::AppState>) -> Vec<crate::jobs::JobRecord> {
    state.jobs().list()
}

/// Request cancellation of a running job. Returns false for unknown or
/// already-terminal jobs.
#[tauri::command]
pub fn cancel_job(state: State<'_, crate::AppState>, job_id: String) -> bool {
    state.jobs().request_cancel(&job_id)
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

    fn session_with_clips(dir: &std::path::Path) -> crate::session::Session {
        use avid_project::{ProjectMeta, MANIFEST_VERSION};
        let manifest = ProjectManifest {
            id: "apply-test".to_owned(),
            version: MANIFEST_VERSION,
            meta: ProjectMeta {
                name: "Apply".to_owned(),
                canvas: "16:9".to_owned(),
                frame_rate: 30,
                resolution: "1080p".to_owned(),
                template_id: None,
                created_at: "now".to_owned(),
                updated_at: "now".to_owned(),
            },
            timeline: serde_json::json!({"tracks": [], "clips": {}}),
            assets: vec![],
            transcripts: std::collections::HashMap::new(),
        };
        let mut session = crate::session::Session::create(dir.to_path_buf(), manifest).unwrap();
        session
            .add_clip(avid_timeline::Clip {
                id: "a".to_owned(),
                source_media_id: "m".to_owned(),
                track_id: "v1".to_owned(),
                start: 0.0,
                duration: 20.0,
                in_point: 0.0,
                name: "a".to_owned(),
            })
            .unwrap();
        session
    }

    #[test]
    fn apply_mixes_valid_and_rejected_ops_then_undoes_as_one() {
        let dir = std::env::temp_dir().join("avid-apply-test");
        std::fs::remove_dir_all(&dir).ok();
        let mut session = session_with_clips(&dir);
        let depth_before = session.undo_depth();
        let report = apply_validated_ops(
            &mut session,
            "trim pause",
            vec![
                EditOperation::RemoveRange {
                    start: 2.0,
                    end: 5.0,
                },
                EditOperation::AddCaption {
                    start: 6.0,
                    end: 8.0,
                    text: "Hello".to_owned(),
                },
                EditOperation::SplitClip {
                    clip_id: "ghost".to_owned(),
                    at: 1.0,
                },
                EditOperation::RemoveRange {
                    start: 50.0,
                    end: 60.0,
                },
            ],
        )
        .unwrap();
        assert!(report.label.starts_with("AI: "));
        assert_eq!(report.results.len(), 4);
        assert!(report.results[0].applied);
        assert!(report.results[1].applied);
        assert!(!report.results[2].applied);
        assert!(!report.results[3].applied);
        // Exactly one new undo step for the whole group.
        assert_eq!(session.undo_depth(), depth_before + 1);
        // Caption track was created; range was cut.
        assert!(session
            .timeline()
            .tracks
            .iter()
            .any(|track| track.id == "captions"));
        session.undo().unwrap();
        assert_eq!(session.timeline().clips.len(), 1);
        assert_eq!(session.timeline().clips["a"].duration, 20.0);
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn proposal_merges_silence_and_fillers() {
        use avid_ai::FillerHit;
        use avid_media::SilenceSpan;
        let silences = vec![
            SilenceSpan {
                start: 10.0,
                end: 14.0,
            },
            SilenceSpan {
                start: 2.0,
                end: 3.0,
            },
        ];
        let fillers = vec![
            FillerHit {
                word: "um".to_owned(),
                start: 11.0,
                end: 11.4,
            },
            FillerHit {
                word: "uh".to_owned(),
                start: 20.0,
                end: 20.5,
            },
        ];
        let proposal = build_proposal(&silences, &fillers, true);
        // Sorted; the in-silence "um" is covered by the pause cut.
        assert_eq!(proposal.cuts.len(), 3);
        assert_eq!((proposal.cuts[0].start, proposal.cuts[0].end), (2.0, 3.0));
        assert_eq!(proposal.cuts[1].reason, "long pause");
        assert_eq!(proposal.cuts[2].reason, "filler word: uh");
        assert!((proposal.removable_seconds - 5.5).abs() < 1e-9);
        assert_eq!(proposal.analyzed, "silence+transcript");

        let silence_only = build_proposal(&silences, &fillers, false);
        assert_eq!(silence_only.cuts.len(), 2);
        assert_eq!(silence_only.analyzed, "silence");
    }

    #[tokio::test]
    async fn probe_reports_unreachable_instead_of_throwing() {
        let probe = probe_provider("http://127.0.0.1:1".to_owned(), "m".to_owned(), None)
            .await
            .unwrap();
        assert!(!probe.reachable);
        assert!(!probe.structured_output);
    }

    #[tokio::test]
    async fn probe_rejects_non_http_urls() {
        assert!(probe_provider("ftp://x".to_owned(), "m".to_owned(), None)
            .await
            .is_err());
        assert!(
            probe_provider("http://x y".to_owned(), "m".to_owned(), None)
                .await
                .is_err()
        );
    }

    #[test]
    fn apply_with_nothing_valid_mutates_nothing() {
        let dir = std::env::temp_dir().join("avid-apply-empty");
        std::fs::remove_dir_all(&dir).ok();
        let mut session = session_with_clips(&dir);
        let depth_before = session.undo_depth();
        let report = apply_validated_ops(
            &mut session,
            "nope",
            vec![EditOperation::SplitClip {
                clip_id: "ghost".to_owned(),
                at: 1.0,
            }],
        )
        .unwrap();
        assert!(!report.results[0].applied);
        assert_eq!(session.undo_depth(), depth_before);
        std::fs::remove_dir_all(&dir).ok();
    }
}
