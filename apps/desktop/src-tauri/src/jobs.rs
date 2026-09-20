//! Background job registry + progress events (AGENTS §49–50).
//!
//! Long operations (render, transcribe) run off the UI thread and report
//! through Tauri [`Channel`](tauri::ipc::Channel)s. The registry is the
//! single source the job-center UI polls: every job has an id, a kind, a
//! status, a 0–1 progress, and a cancel flag the worker checks.

use std::collections::HashMap;
use std::sync::{atomic::AtomicBool, Arc, Mutex};

use serde::{Deserialize, Serialize};

/// What the job does (shown in the job center).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobKind {
    Render,
    Transcribe,
    Proxy,
}

/// Lifecycle state of a job.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    Running,
    Finished,
    Failed,
    Cancelled,
}

/// A job record for the job-center UI ( polled via `list_jobs` ).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobRecord {
    pub id: String,
    pub kind: JobKind,
    pub label: String,
    pub status: JobStatus,
    /// 0–1 progress (`None` = indeterminate, e.g. transcription).
    pub progress: Option<f32>,
    /// Human message: current stage or failure reason.
    pub message: Option<String>,
}

/// Progress event streamed over a Channel during the run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobEvent {
    pub job_id: String,
    pub status: JobStatus,
    pub progress: Option<f32>,
    pub message: Option<String>,
}

struct JobEntry {
    record: JobRecord,
    cancel: Arc<AtomicBool>,
}

impl std::fmt::Debug for JobEntry {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("JobEntry")
            .field("record", &self.record)
            .field(
                "cancel",
                &self.cancel.load(std::sync::atomic::Ordering::Relaxed),
            )
            .finish()
    }
}

#[derive(Debug, Default, Clone)]
pub struct JobRegistry {
    inner: Arc<Mutex<HashMap<String, JobEntry>>>,
}

impl JobRegistry {
    /// Register a running job; returns its id and cancel flag.
    pub fn start(&self, kind: JobKind, label: impl Into<String>) -> (String, Arc<AtomicBool>) {
        let id = format!("job-{}", uuid::Uuid::new_v4());
        let cancel = Arc::new(AtomicBool::new(false));
        if let Ok(mut guard) = self.inner.lock() {
            guard.insert(
                id.clone(),
                JobEntry {
                    record: JobRecord {
                        id: id.clone(),
                        kind,
                        label: label.into(),
                        status: JobStatus::Running,
                        progress: None,
                        message: None,
                    },
                    cancel: Arc::clone(&cancel),
                },
            );
        }
        (id, cancel)
    }

    fn update(&self, id: &str, run: impl FnOnce(&mut JobRecord)) {
        if let Ok(mut guard) = self.inner.lock() {
            if let Some(entry) = guard.get_mut(id) {
                run(&mut entry.record);
            }
        }
    }

    /// Record progress (0–1) for a running job.
    pub fn progress(&self, id: &str, fraction: f64) {
        #[allow(clippy::cast_possible_truncation)]
        let progress = fraction.clamp(0.0, 1.0) as f32;
        self.update(id, |record| {
            record.progress = Some(progress);
        });
    }

    /// Mark a job finished.
    pub fn finish(&self, id: &str, message: impl Into<String>) {
        let message = message.into();
        self.update(id, |record| {
            record.status = JobStatus::Finished;
            record.progress = Some(1.0);
            record.message = Some(message.clone());
        });
    }

    /// Mark a job failed with a humane reason.
    pub fn fail(&self, id: &str, message: impl Into<String>) {
        let message = message.into();
        self.update(id, |record| {
            record.status = JobStatus::Failed;
            record.message = Some(message.clone());
        });
    }

    /// Mark a job cancelled (after the worker acknowledged it).
    pub fn cancelled(&self, id: &str) {
        self.update(id, |record| {
            record.status = JobStatus::Cancelled;
            record.message = Some("Cancelled.".to_owned());
        });
    }

    /// Request cancellation; the worker checks the flag. Returns false for
    /// unknown or already-terminal jobs.
    pub fn request_cancel(&self, id: &str) -> bool {
        use std::sync::atomic::Ordering;
        if let Ok(guard) = self.inner.lock() {
            if let Some(entry) = guard.get(id) {
                if entry.record.status == JobStatus::Running {
                    entry.cancel.store(true, Ordering::Relaxed);
                    return true;
                }
            }
        }
        false
    }

    /// Snapshot for `list_jobs` (newest implicitly last; UI sorts).
    pub fn list(&self) -> Vec<JobRecord> {
        self.inner
            .lock()
            .map(|guard| guard.values().map(|entry| entry.record.clone()).collect())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifecycle_start_progress_finish_cancel() {
        let registry = JobRegistry::default();
        let (id, _cancel) = registry.start(JobKind::Render, "Export short.mp4");
        registry.progress(&id, 0.5);
        let listed = registry.list();
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].progress, Some(0.5));
        assert!(registry.request_cancel(&id));
        registry.cancelled(&id);
        assert_eq!(registry.list()[0].status, JobStatus::Cancelled);
        // Unknown ids fail closed.
        assert!(!registry.request_cancel("job-nope"));
        registry.finish(&id, "done");
        assert_eq!(registry.list()[0].status, JobStatus::Finished);
    }
}
