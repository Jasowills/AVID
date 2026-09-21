/**
 * @avid/design-system — tokens (Phase 1).
 *
 * SOURCE OF TRUTH for color, typography, spacing, radius, borders, shadows.
 * The Tailwind v4 `@theme` block in `apps/desktop/src/index.css` mirrors
 * these values for utility classes; this module serves JS-side consumers
 * (canvas timeline, charts — Phase 3+). Change a value here AND there.
 *
 * Identity: dark-first, calm, professional (AGENTS §60–61). No gradients,
 * no glow, no glassmorphism — a serious creative tool.
 */

export const colors = {
  /** App background / panel hierarchy. */
  bg: {
    base: "#0b0d10",
    panel: "#12151a",
    raised: "#181c22",
    overlay: "#1f242c",
  },
  /** Borders and dividers. */
  border: {
    subtle: "#23282f",
    default: "#2f3640",
    strong: "#455060",
  },
  /** Text. */
  text: {
    primary: "#e8eaed",
    secondary: "#a7b0bb",
    muted: "#6b7480",
  },
  /** Single accent — used sparingly (selection, primary actions, AI status). */
  accent: {
    DEFAULT: "#4f8cff",
    hover: "#6ba0ff",
    muted: "#274067",
  },
  /** Semantic. Never rely on color alone — pair with icon + label. */
  success: "#3fb96c",
  warning: "#d9a13b",
  danger: "#e05d5d",
  /** Timeline legibility. */
  timeline: {
    video: "#3a5f8a",
    audio: "#3f7d5d",
    text: "#7a6a3f",
    graphics: "#6a4f8a",
    caption: "#4f8a84",
    playhead: "#e8eaed",
  },
} as const;

export const spacing = {
  xs: "0.25rem",
  sm: "0.5rem",
  md: "1rem",
  lg: "1.5rem",
  xl: "2rem",
} as const;

export const radii = {
  sm: "0.25rem",
  md: "0.375rem",
  lg: "0.5rem",
  full: "9999px",
} as const;

export const typography = {
  fontFamily: {
    sans: 'Inter, ui-sans-serif, system-ui, -apple-system, "Segoe UI", sans-serif',
    mono: 'ui-monospace, "SF Mono", "Cascadia Code", Menlo, monospace',
  },
  fontSize: {
    xs: "0.75rem",
    sm: "0.875rem",
    md: "1rem",
    lg: "1.125rem",
    xl: "1.25rem",
  },
} as const;

export { DEFAULT_THEME_ID, THEME_PRESETS, applyThemePreset, loadThemeId } from "./themes";
export type { ThemePreset } from "./themes";
