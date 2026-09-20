import {
  CANVAS_ASPECTS,
  FRAME_RATES,
  PROJECT_SCHEMA_VERSION,
  RESOLUTIONS,
  type CanvasAspect,
  type NewProjectInput,
  type ProjectConfig,
} from "@avid/shared-types";

export interface ValidationError {
  field: "name" | "canvas" | "frameRate" | "resolution";
  message: string;
}

/** Pure validation for the New Project form — unit-tested, no React. */
export function validateNewProject(input: NewProjectInput): ValidationError[] {
  const errors: ValidationError[] = [];
  const name = input.name.trim();
  if (name.length === 0) {
    errors.push({ field: "name", message: "Give the project a name." });
  } else if (name.length > 80) {
    errors.push({ field: "name", message: "Keep the name under 80 characters." });
  }
  if (!CANVAS_ASPECTS.includes(input.canvas)) {
    errors.push({ field: "canvas", message: "Choose a canvas aspect ratio." });
  }
  if (!FRAME_RATES.includes(input.frameRate)) {
    errors.push({ field: "frameRate", message: "Choose a frame rate." });
  }
  if (!RESOLUTIONS.includes(input.resolution)) {
    errors.push({ field: "resolution", message: "Choose a resolution." });
  }
  return errors;
}

export interface CanvasPreset {
  canvas: CanvasAspect;
  label: string;
}

export const CANVAS_PRESETS: CanvasPreset[] = [
  { canvas: "16:9", label: "16:9 Widescreen" },
  { canvas: "9:16", label: "9:16 Vertical" },
  { canvas: "1:1", label: "1:1 Square" },
  { canvas: "4:5", label: "4:5 Portrait" },
  { canvas: "custom", label: "Custom" },
];

/** Build a ProjectConfig from validated input. ID + timestamps injected for testability. */
export function createProjectConfig(
  input: NewProjectInput,
  deps: { id: string; now: string },
): ProjectConfig {
  return {
    id: deps.id,
    version: PROJECT_SCHEMA_VERSION,
    name: input.name.trim(),
    canvas: input.canvas,
    frameRate: input.frameRate,
    resolution: input.resolution,
    templateId: input.templateId ?? null,
    createdAt: deps.now,
    updatedAt: deps.now,
  };
}
