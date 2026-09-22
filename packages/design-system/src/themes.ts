/**
 * AVID theme presets (MASTER-DESIGN §5, §74–75).
 *
 * Monochrome is the default identity; every other preset — including Blue,
 * which preserves the pre-theme-engine values — is an option, never the brand.
 * A theme is a flat map of CSS variable values, therefore serializable,
 * exportable, and applicable without a rebuild.
 */

export interface ThemePreset {
  id: string;
  name: string;
  /** CSS variable name → value, e.g. { "--avid-base": "#0a0a0b" }. */
  vars: Record<string, string>;
}

const MONO_TEXT = {
  "--avid-primary": "#ececf1",
  "--avid-secondary": "#a2a2ad",
  "--avid-muted": "#696974",
};

export const THEME_PRESETS: ThemePreset[] = [
  {
    id: "monochrome",
    name: "Monochrome",
    vars: {
      "--avid-base": "#0a0a0b",
      "--avid-panel": "#101014",
      "--avid-raised": "#17171c",
      "--avid-overlay": "#202027",
      "--avid-border-subtle": "#1e1e25",
      "--avid-border": "#2a2a33",
      "--avid-border-strong": "#40404c",
      ...MONO_TEXT,
      "--avid-action": "#e8e8ef",
      "--avid-action-text": "#0a0a0b",
      "--avid-action-hover": "#ffffff",
      "--avid-accent": "#9aa0ae",
      "--avid-accent-hover": "#b6bcc8",
      "--avid-accent-muted": "#2c313a",
      "--avid-success": "#3fb96c",
      "--avid-warning": "#d9a13b",
      "--avid-danger": "#e05d5d",
    },
  },
  {
    id: "graphite",
    name: "Graphite",
    vars: {
      "--avid-base": "#0e0e12",
      "--avid-panel": "#14141a",
      "--avid-raised": "#1b1b23",
      "--avid-overlay": "#24242d",
      "--avid-border-subtle": "#23232c",
      "--avid-border": "#30303a",
      "--avid-border-strong": "#484856",
      "--avid-primary": "#f0f0f4",
      "--avid-secondary": "#ababb6",
      "--avid-muted": "#71717c",
      "--avid-action": "#ececf1",
      "--avid-action-text": "#0e0e12",
      "--avid-action-hover": "#ffffff",
      "--avid-accent": "#a7adc0",
      "--avid-accent-hover": "#c2c8d8",
      "--avid-accent-muted": "#31313d",
      "--avid-success": "#3fb96c",
      "--avid-warning": "#d9a13b",
      "--avid-danger": "#e05d5d",
    },
  },
  {
    id: "blue",
    name: "Blue",
    vars: {
      "--avid-base": "#0b0d10",
      "--avid-panel": "#12151a",
      "--avid-raised": "#181c22",
      "--avid-overlay": "#1f242c",
      "--avid-border-subtle": "#23282f",
      "--avid-border": "#2f3640",
      "--avid-border-strong": "#455060",
      "--avid-primary": "#e8eaed",
      "--avid-secondary": "#a7b0bb",
      "--avid-muted": "#6b7480",
      "--avid-action": "#4f8cff",
      "--avid-action-text": "#ffffff",
      "--avid-action-hover": "#6ba0ff",
      "--avid-accent": "#4f8cff",
      "--avid-accent-hover": "#6ba0ff",
      "--avid-accent-muted": "#274067",
      "--avid-success": "#3fb96c",
      "--avid-warning": "#d9a13b",
      "--avid-danger": "#e05d5d",
    },
  },
  {
    id: "violet",
    name: "Violet",
    vars: {
      "--avid-base": "#0b0a10",
      "--avid-panel": "#121218",
      "--avid-raised": "#181820",
      "--avid-overlay": "#20202a",
      "--avid-border-subtle": "#22222c",
      "--avid-border": "#2e2e3a",
      "--avid-border-strong": "#46465a",
      ...MONO_TEXT,
      "--avid-action": "#a78bfa",
      "--avid-action-text": "#0b0a10",
      "--avid-action-hover": "#c4b5fd",
      "--avid-accent": "#a78bfa",
      "--avid-accent-hover": "#c4b5fd",
      "--avid-accent-muted": "#2e2a4a",
      "--avid-success": "#3fb96c",
      "--avid-warning": "#d9a13b",
      "--avid-danger": "#e05d5d",
    },
  },
  {
    id: "green",
    name: "Green",
    vars: {
      "--avid-base": "#090c0a",
      "--avid-panel": "#0f1311",
      "--avid-raised": "#151b17",
      "--avid-overlay": "#1c231e",
      "--avid-border-subtle": "#1d2520",
      "--avid-border": "#2a352d",
      "--avid-border-strong": "#42503f",
      ...MONO_TEXT,
      "--avid-action": "#34d399",
      "--avid-action-text": "#090c0a",
      "--avid-action-hover": "#6ee7b7",
      "--avid-accent": "#34d399",
      "--avid-accent-hover": "#6ee7b7",
      "--avid-accent-muted": "#1d3a2f",
      "--avid-success": "#34d399",
      "--avid-warning": "#d9a13b",
      "--avid-danger": "#e05d5d",
    },
  },
  {
    id: "red",
    name: "Red",
    vars: {
      "--avid-base": "#0c0909",
      "--avid-panel": "#131010",
      "--avid-raised": "#1b1515",
      "--avid-overlay": "#241c1c",
      "--avid-border-subtle": "#251d1d",
      "--avid-border": "#352a2a",
      "--avid-border-strong": "#504040",
      ...MONO_TEXT,
      "--avid-action": "#f87171",
      "--avid-action-text": "#0c0909",
      "--avid-action-hover": "#fca5a5",
      "--avid-accent": "#f87171",
      "--avid-accent-hover": "#fca5a5",
      "--avid-accent-muted": "#402222",
      "--avid-success": "#3fb96c",
      "--avid-warning": "#d9a13b",
      "--avid-danger": "#f87171",
    },
  },
  {
    id: "solar",
    name: "Solar",
    vars: {
      "--avid-base": "#0d0b09",
      "--avid-panel": "#141109",
      "--avid-raised": "#1c1813",
      "--avid-overlay": "#24201a",
      "--avid-border-subtle": "#241f18",
      "--avid-border": "#332c21",
      "--avid-border-strong": "#4d4433",
      "--avid-primary": "#f0ece4",
      "--avid-secondary": "#aca396",
      "--avid-muted": "#6f675c",
      "--avid-action": "#f5b942",
      "--avid-action-text": "#0d0b09",
      "--avid-action-hover": "#ffd08a",
      "--avid-accent": "#f5b942",
      "--avid-accent-hover": "#ffd08a",
      "--avid-accent-muted": "#3a2e1a",
      "--avid-success": "#3fb96c",
      "--avid-warning": "#f5b942",
      "--avid-danger": "#e05d5d",
    },
  },
  {
    id: "contrast",
    name: "High Contrast",
    vars: {
      "--avid-base": "#000000",
      "--avid-panel": "#0a0a0a",
      "--avid-raised": "#141414",
      "--avid-overlay": "#1c1c1c",
      "--avid-border-subtle": "#3a3a3a",
      "--avid-border": "#5a5a5a",
      "--avid-border-strong": "#8a8a8a",
      "--avid-primary": "#ffffff",
      "--avid-secondary": "#d0d0d0",
      "--avid-muted": "#9a9a9a",
      "--avid-action": "#ffffff",
      "--avid-action-text": "#000000",
      "--avid-action-hover": "#ffffff",
      "--avid-accent": "#ffffff",
      "--avid-accent-hover": "#ffffff",
      "--avid-accent-muted": "#333333",
      "--avid-success": "#4ade80",
      "--avid-warning": "#fbbf24",
      "--avid-danger": "#ff6b6b",
    },
  },
  {
    id: "studio-dark",
    name: "Studio Dark",
    vars: {
      "--avid-base": "hsl(0, 0%, 7%)",
      "--avid-panel": "hsl(0, 0%, 7%)",
      "--avid-raised": "hsl(0, 0%, 9%)",
      "--avid-overlay": "hsl(0, 0%, 12%)",
      "--avid-border-subtle": "hsla(0, 0%, 100%, 0.03)",
      "--avid-border": "hsla(0, 0%, 100%, 0.06)",
      "--avid-border-strong": "hsla(0, 0%, 100%, 0.12)",
      "--avid-primary": "hsl(0, 0%, 95%)",
      "--avid-secondary": "hsla(0, 0%, 95%, 0.64)",
      "--avid-muted": "hsl(0, 0%, 45%)",
      "--avid-action": "hsl(207, 100%, 50%)",
      "--avid-action-text": "hsl(0, 0%, 100%)",
      "--avid-action-hover": "hsl(207, 98%, 40%)",
      "--avid-accent": "hsl(207, 100%, 65%)",
      "--avid-accent-hover": "hsl(207, 100%, 72%)",
      "--avid-accent-muted": "hsla(207, 100%, 50%, 0.16)",
      "--avid-success": "hsl(162, 66%, 45%)",
      "--avid-warning": "hsl(46, 90%, 62%)",
      "--avid-danger": "hsl(0, 79%, 60%)",
    },
  },
  {
    id: "studio-light",
    name: "Studio Light",
    vars: {
      "--avid-base": "hsl(0, 0%, 100%)",
      "--avid-panel": "hsl(0, 0%, 97%)",
      "--avid-raised": "hsl(0, 0%, 95%)",
      "--avid-overlay": "hsl(0, 0%, 100%)",
      "--avid-border-subtle": "hsl(0, 0%, 91%)",
      "--avid-border": "hsl(0, 0%, 88%)",
      "--avid-border-strong": "hsl(0, 0%, 78%)",
      "--avid-primary": "hsl(240, 5%, 9%)",
      "--avid-secondary": "hsl(0, 0%, 32%)",
      "--avid-muted": "hsl(0, 0%, 45%)",
      "--avid-action": "hsl(207, 100%, 45%)",
      "--avid-action-text": "hsl(0, 0%, 100%)",
      "--avid-action-hover": "hsl(207, 98%, 35%)",
      "--avid-accent": "hsl(207, 100%, 40%)",
      "--avid-accent-hover": "hsl(207, 100%, 32%)",
      "--avid-accent-muted": "hsla(207, 100%, 50%, 0.12)",
      "--avid-success": "hsl(162, 66%, 29%)",
      "--avid-warning": "hsl(46, 90%, 40%)",
      "--avid-danger": "hsl(0, 79%, 45%)",
    },
  },
];

export const DEFAULT_THEME_ID = "monochrome";

const STORAGE_KEY = "avid.theme.v1";

/** Apply a preset: set the theme attribute (CSS does the rest). */
export function applyThemePreset(id: string): ThemePreset {
  const preset = THEME_PRESETS.find((p) => p.id === id) ?? THEME_PRESETS[0];
  if (!preset) throw new Error("No theme presets defined.");
  document.documentElement.dataset.theme = preset.id;
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify({ preset: preset.id }));
  } catch {
    // Private mode etc. — theme simply doesn't persist.
  }
  return preset;
}

/** Read the persisted preset id, if any. */
export function loadThemeId(): string {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (raw) {
      const parsed = JSON.parse(raw) as { preset?: unknown };
      if (typeof parsed.preset === "string" && THEME_PRESETS.some((p) => p.id === parsed.preset)) {
        return parsed.preset;
      }
    }
  } catch {
    // Corrupt config falls back to default.
  }
  return DEFAULT_THEME_ID;
}
