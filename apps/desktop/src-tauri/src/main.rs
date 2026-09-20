#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! AVID desktop backend (ADR-001): Tauri 2.x shell over the Rust core.
//!
//! State ownership: Rust holds project/media/job truth in [`AppState`];
//! the frontend holds ephemeral UI in Zustand. Commands are the only
//! mutation path; progress streams over Channels; media bytes never cross IPC.

mod commands;
mod session;

/// Shared application state: schema handshake + the open project session.
/// Every timeline/media mutation persists `project.json` (autosave), so
/// reopening the project directory recovers all committed work.
#[derive(Debug, Default)]
pub struct AppState {
    /// Schema version handshake for the frontend.
    schema_version: u32,
    session: std::sync::Mutex<Option<session::Session>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            schema_version: avid_project::MANIFEST_VERSION,
            session: std::sync::Mutex::new(None),
        }
    }

    /// Manifest schema this build reads/writes (exposed via `get_app_info`).
    #[must_use]
    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// Open (or replace) the current session, e.g. after create/open.
    pub fn open(&self, session: session::Session) {
        if let Ok(mut guard) = self.session.lock() {
            *guard = Some(session);
        }
    }

    /// Run a closure against the open session, or report "open a project
    /// first" — the single funnel for every session-backed command.
    pub fn with_session<R>(
        &self,
        run: impl FnOnce(&mut session::Session) -> Result<R, commands::CommandError>,
    ) -> Result<R, commands::CommandError> {
        let mut guard = self.session.lock().map_err(|_| commands::CommandError {
            code: "AVID_PROJECT_001".to_owned(),
            message: "Project state is temporarily unavailable — retry.".to_owned(),
        })?;
        let Some(session) = guard.as_mut() else {
            return Err(commands::CommandError {
                code: "AVID_PROJECT_001".to_owned(),
                message: "Open a project first.".to_owned(),
            });
        };
        run(session)
    }
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            commands::get_app_info,
            commands::create_project,
            commands::open_project,
            commands::list_projects,
            commands::import_media,
            commands::transcribe_media,
            commands::probe_media,
            commands::timeline_get,
            commands::timeline_add_clip,
            commands::timeline_remove_clip,
            commands::timeline_split_clip,
            commands::timeline_undo,
            commands::timeline_redo,
        ])
        .run(tauri::generate_context!())
        .expect("AVID backend failed to start");
}
