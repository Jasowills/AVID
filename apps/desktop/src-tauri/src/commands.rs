//! Tauri command layer: the ONLY mutation path into Rust state (ADR-001).
//!
//! Thin wrappers over pure helpers — helpers hold the logic and the tests,
//! commands hold the `#[tauri::command]` attribute and nothing else.

use std::path::Path;

use avid_media::{MediaEngine, MediaInfo};
use avid_project::{ProjectManifest, ProjectMeta};
use serde::{Deserialize, Serialize};

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
        Self {
            code: error.code().to_owned(),
            message: soften(&error),
        }
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
    })
}

/// Probe a project-relative media file. Pure helper over [`MediaEngine`].
pub fn probe_media_file(
    engine: &MediaEngine,
    relative_path: &str,
) -> Result<MediaInfo, CommandError> {
    engine
        .probe(Path::new(relative_path))
        .map_err(CommandError::from)
}

/// Persist a validated manifest as `<directory>/project.json`.
/// The directory is user-chosen (dialog) so absolute paths are expected —
/// it must exist and be a directory. Pure helper; the command is a wrapper.
pub fn save_project_to_dir(
    manifest_json: &str,
    directory: &Path,
) -> Result<std::path::PathBuf, CommandError> {
    let manifest = ProjectManifest::from_json(manifest_json)?;
    if !directory.is_dir() {
        return Err(CommandError {
            code: "AVID_PROJECT_001".to_owned(),
            message: "Choose an existing folder to save the project in.".to_owned(),
        });
    }
    let path = directory.join("project.json");
    std::fs::write(&path, manifest.to_json()?).map_err(|e| CommandError {
        code: "AVID_PROJECT_001".to_owned(),
        message: format!("AVID couldn't write the project file: {e}"),
    })?;
    Ok(path)
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
pub fn create_project(input: NewProjectInput) -> Result<ProjectManifest, CommandError> {
    // Id + timestamp minted at the boundary (deterministic in tests via helper).
    let now = std::time::SystemTime::now();
    let now = format!("{now:?}");
    create_project_manifest(input, format!("proj-{}", MediaEngine::mint_asset_id()), now)
}

#[tauri::command]
pub fn probe_media(relative_path: String) -> Result<MediaInfo, CommandError> {
    // System binaries until the pinned sidecar ships (ADR-002).
    let engine = MediaEngine::system().map_err(CommandError::from)?;
    probe_media_file(&engine, &relative_path)
}

#[tauri::command]
pub fn save_project(manifest_json: String, directory: String) -> Result<String, CommandError> {
    save_project_to_dir(&manifest_json, Path::new(&directory))
        .map(|path| path.display().to_string())
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

    #[test]
    fn save_round_trips_through_disk() {
        let manifest =
            create_project_manifest(input("Disk"), "id-9".to_owned(), "now".to_owned()).unwrap();
        let json = manifest.to_json().unwrap();
        let dir = std::env::temp_dir().join("avid-save-test");
        std::fs::create_dir_all(&dir).unwrap();
        let path = save_project_to_dir(&json, &dir).unwrap();
        assert_eq!(path, dir.join("project.json"));
        let back = ProjectManifest::from_json(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(manifest, back);
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn save_rejects_bad_manifest_and_missing_dir() {
        let dir = std::env::temp_dir().join("avid-save-test-missing");
        std::fs::remove_dir_all(&dir).ok();
        let manifest =
            create_project_manifest(input("Disk"), "id-9".to_owned(), "now".to_owned()).unwrap();
        assert!(save_project_to_dir(&manifest.to_json().unwrap(), &dir).is_err());
        assert!(save_project_to_dir("{nope", &std::env::temp_dir()).is_err());
    }
}
