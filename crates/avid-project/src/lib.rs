//! Versioned project manifest (ADR-003).
//!
//! The manifest (`project.json`) is the portable source of truth: id,
//! schema version, metadata, timeline, assets, transcripts, AI operations,
//! template references, render settings. Media is referenced by
//! content-addressed asset ids with project-relative paths — never absolute
//! paths, so projects move across macOS/Windows/Linux (AGENTS §113).
//!
//! Forward migrations only: unknown future versions open read-only with a
//! clear error, never silent breakage.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Current manifest schema version. Bump with a migration in `migrate()`.
pub const MANIFEST_VERSION: u32 = 1;

/// Project manifest failures.
#[derive(Debug, Error, PartialEq)]
pub enum ProjectError {
    /// JSON parsing or schema mismatch.
    #[error("project manifest is invalid: {0}")]
    InvalidManifest(String),
    /// Manifest version is newer than this build understands.
    #[error("project needs schema v{found} but this build supports v{supported}")]
    UnsupportedVersion {
        /// Version found in the file.
        found: u32,
        /// Version this build supports.
        supported: u32,
    },
    /// A media path escapes the project directory (`..` traversal).
    #[error("media path escapes the project directory: {0}")]
    PathTraversal(String),
}

impl ProjectError {
    /// Stable error code for UI mapping and logs.
    #[must_use]
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidManifest(_) => "AVID_PROJECT_001",
            Self::UnsupportedVersion { .. } => "AVID_PROJECT_002",
            Self::PathTraversal(_) => "AVID_PROJECT_003",
        }
    }
}

/// A media asset referenced by the project.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MediaAsset {
    /// Stable unique id (referenced by timeline clips).
    pub id: String,
    /// Original file name for display and relink matching.
    pub file_name: String,
    /// Path relative to the project directory. Never absolute, never `..`.
    pub relative_path: String,
    /// FFmpeg-probed duration in seconds, if known.
    pub duration: Option<f64>,
    /// Width × height for video, if known.
    pub dimensions: Option<(u32, u32)>,
    /// BLAKE-level identity for relink matching is a Phase 2 concern;
    /// the field is reserved here so the schema doesn't break later.
    pub hash: Option<String>,
}

/// Project metadata (user-visible).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectMeta {
    pub name: String,
    pub canvas: String,
    pub frame_rate: u32,
    pub resolution: String,
    pub template_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// The project manifest. Timeline payload is opaque JSON here — owned and
/// validated by `avid-timeline` (ADR-004) — so neither crate depends on the other.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectManifest {
    /// Stable unique id.
    pub id: String,
    /// Schema version of this file.
    pub version: u32,
    pub meta: ProjectMeta,
    /// Timeline document (avid-timeline JSON).
    pub timeline: serde_json::Value,
    /// Content-addressed media assets.
    pub assets: Vec<MediaAsset>,
    /// Transcripts by asset id (opaque JSON owned by `avid-ai`; schema v1
    /// files predate this field and load as empty via the default).
    #[serde(default)]
    pub transcripts: std::collections::HashMap<String, serde_json::Value>,
}

impl ProjectManifest {
    /// Parse and validate a manifest, migrating forward when possible.
    pub fn from_json(json: &str) -> Result<Self, ProjectError> {
        let manifest: Self =
            serde_json::from_str(json).map_err(|e| ProjectError::InvalidManifest(e.to_string()))?;
        if manifest.version > MANIFEST_VERSION {
            return Err(ProjectError::UnsupportedVersion {
                found: manifest.version,
                supported: MANIFEST_VERSION,
            });
        }
        let migrated = migrate(manifest);
        migrated.validate()?;
        Ok(migrated)
    }

    /// Serialize for `project.json`.
    pub fn to_json(&self) -> Result<String, ProjectError> {
        serde_json::to_string_pretty(self).map_err(|e| ProjectError::InvalidManifest(e.to_string()))
    }

    fn validate(&self) -> Result<(), ProjectError> {
        for asset in &self.assets {
            validate_relative_path(&asset.relative_path)?;
        }
        Ok(())
    }
}

/// Migrate older manifests forward. v1 is current — identity migration.
fn migrate(manifest: ProjectManifest) -> ProjectManifest {
    manifest
}

/// Reject absolute paths and `..` traversal (AGENTS §106).
/// Shared by the manifest validator and the backend's project-path resolver.
pub fn validate_relative_path(path: &str) -> Result<(), ProjectError> {
    if path.is_empty() {
        return Err(ProjectError::PathTraversal("(empty path)".to_owned()));
    }
    let normalized = path.replace('\\', "/");
    if normalized.starts_with('/') {
        return Err(ProjectError::PathTraversal(path.to_owned()));
    }
    // Windows absolute paths (`C:/…`) are absolute too.
    let mut chars = normalized.chars();
    if let (Some(drive), Some(colon)) = (chars.next(), chars.next()) {
        if drive.is_ascii_alphabetic() && colon == ':' {
            return Err(ProjectError::PathTraversal(path.to_owned()));
        }
    }
    for segment in normalized.split('/') {
        if segment == ".." {
            return Err(ProjectError::PathTraversal(path.to_owned()));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn manifest_json(version: u32, path: &str) -> String {
        json!({
            "id": "proj-1",
            "version": version,
            "meta": {
                "name": "Demo",
                "canvas": "16:9",
                "frame_rate": 30,
                "resolution": "1080p",
                "template_id": null,
                "created_at": "2026-09-20T00:00:00Z",
                "updated_at": "2026-09-20T00:00:00Z"
            },
            "timeline": {"tracks": [], "clips": {}},
            "assets": [
                {"id": "m1", "file_name": "a.mp4", "relative_path": path,
                 "duration": null, "dimensions": null, "hash": null}
            ]
        })
        .to_string()
    }

    #[test]
    fn round_trips_valid_manifest() {
        let manifest = ProjectManifest::from_json(&manifest_json(1, "media/a.mp4")).unwrap();
        let back = ProjectManifest::from_json(&manifest.to_json().unwrap()).unwrap();
        assert_eq!(manifest, back);
    }

    #[test]
    fn rejects_future_versions_without_breaking() {
        let err = ProjectManifest::from_json(&manifest_json(99, "media/a.mp4")).unwrap_err();
        assert!(matches!(
            err,
            ProjectError::UnsupportedVersion { found: 99, .. }
        ));
        assert_eq!(err.code(), "AVID_PROJECT_002");
    }

    #[test]
    fn rejects_traversal_and_absolute_paths() {
        for bad in [
            "../evil.mp4",
            "media/../../x.mp4",
            "/etc/passwd",
            "",
            "C:\\Windows\\x.mp4",
        ] {
            let err = ProjectManifest::from_json(&manifest_json(1, bad)).unwrap_err();
            assert!(matches!(err, ProjectError::PathTraversal(_)), "path: {bad}");
        }
        assert_eq!(
            ProjectError::PathTraversal("x".to_owned()).code(),
            "AVID_PROJECT_003"
        );
    }

    #[test]
    fn rejects_malformed_json() {
        let err = ProjectManifest::from_json("{not json").unwrap_err();
        assert!(matches!(err, ProjectError::InvalidManifest(_)));
    }

    #[test]
    fn shipped_example_parses() {
        use std::path::Path;
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../examples/technical-explainer/project.json");
        if !path.is_file() {
            return;
        }
        let json = std::fs::read_to_string(path).unwrap();
        let manifest = ProjectManifest::from_json(&json).unwrap();
        assert_eq!(manifest.id, "example-technical-explainer");
        assert_eq!(manifest.version, MANIFEST_VERSION);
        assert!(!manifest.assets.is_empty());
    }
}
