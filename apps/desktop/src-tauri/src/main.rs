#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! AVID desktop backend (ADR-001): Tauri 2.x shell over the Rust core.
//!
//! State ownership: Rust holds project/media/job truth; the frontend holds
//! ephemeral UI in Zustand. Commands are the only mutation path; progress
//! streams over Channels; media bytes never cross IPC.

mod commands;

/// Shared application state (project root, job registry land here in Phase 2+).
#[derive(Debug, Default)]
pub struct AppState {
    /// Schema version handshake for the frontend.
    schema_version: u32,
}

impl AppState {
    fn new() -> Self {
        Self {
            schema_version: avid_project::MANIFEST_VERSION,
        }
    }

    /// Manifest schema this build reads/writes (exposed via `get_app_info`).
    #[must_use]
    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            commands::get_app_info,
            commands::create_project,
            commands::probe_media,
        ])
        .run(tauri::generate_context!())
        .expect("AVID backend failed to start");
}
