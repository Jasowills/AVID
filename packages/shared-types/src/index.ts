/**
 * @avid/shared-types — Tauri IPC contracts (Phase 1).
 *
 * These types are the source of truth for the frontend↔Rust boundary.
 * The matching Rust structs in `crates/avid-core` / `avid-project` must
 * mirror them field-for-field; drift is a CI failure (from Phase 3).
 * Versioned: every breaking change bumps PROJECT_SCHEMA_VERSION and
 * ships a migration (ADR-003).
 */

/** Current project manifest schema version. */
export const PROJECT_SCHEMA_VERSION = 1 as const;

/** Canvas aspect ratios offered at project creation (AGENTS §37). */
export const CANVAS_ASPECTS = ["16:9", "9:16", "1:1", "4:5", "custom"] as const;
export type CanvasAspect = (typeof CANVAS_ASPECTS)[number];

/** Frame rates offered at project creation. */
export const FRAME_RATES = [24, 25, 30, 50, 60] as const;
export type FrameRate = (typeof FRAME_RATES)[number];

/** Resolutions offered at project creation. */
export const RESOLUTIONS = ["720p", "1080p", "4K"] as const;
export type Resolution = (typeof RESOLUTIONS)[number];

/** Minimal project record. Full manifest (timeline, assets, …) lands in Phase 3. */
export interface ProjectConfig {
  /** Stable unique id (uuid v4). */
  id: string;
  /** Manifest schema version. */
  version: typeof PROJECT_SCHEMA_VERSION;
  /** User-visible name. */
  name: string;
  canvas: CanvasAspect;
  frameRate: FrameRate;
  resolution: Resolution;
  /** Optional template this project started from. */
  templateId: string | null;
  /** ISO-8601 timestamps. */
  createdAt: string;
  updatedAt: string;
}

/** Input for project creation — validated by `validateNewProject` in the app. */
export interface NewProjectInput {
  name: string;
  canvas: CanvasAspect;
  frameRate: FrameRate;
  resolution: Resolution;
  templateId?: string | null;
}

/** Structured error returned over IPC (mirrors `AvidError`, AGENTS §111). */
export interface AvidError {
  /** e.g. "AVID_PROJECT_001". */
  code: string;
  message: string;
  /** Expanded technical detail for the error UX disclosure. */
  detail?: string;
}
