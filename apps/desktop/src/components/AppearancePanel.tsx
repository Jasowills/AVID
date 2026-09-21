import { useState } from "react";
import { applyThemePreset, loadThemeId, THEME_PRESETS } from "@avid/design-system";
import { Panel } from "@avid/ui";

/**
 * Appearance settings: theme preset picker. Switching repaints instantly
 * (CSS variables, no rebuild); the choice persists across launches.
 * Monochrome is the default identity — every other preset is an option.
 */
export function AppearancePanel() {
  const [active, setActive] = useState<string>(() => {
    if (typeof document !== "undefined") {
      return document.documentElement.dataset.theme ?? loadThemeId();
    }
    return loadThemeId();
  });

  const choose = (id: string): void => {
    const preset = applyThemePreset(id);
    setActive(preset.id);
  };

  return (
    <Panel title="Appearance">
      <div className="grid grid-cols-2 gap-2 sm:grid-cols-4" role="radiogroup" aria-label="Theme">
        {THEME_PRESETS.map((preset) => (
          <button
            key={preset.id}
            role="radio"
            aria-checked={active === preset.id}
            onClick={() => choose(preset.id)}
            title={`${preset.name}${preset.id === "monochrome" ? " (default)" : ""}`}
            className={`flex flex-col gap-1.5 rounded-avid-md border p-2 text-left transition-colors ${
              active === preset.id
                ? "border-avid-accent"
                : "border-avid-border hover:border-avid-border-strong"
            }`}
          >
            <span
              aria-hidden="true"
              className="flex h-8 overflow-hidden rounded-avid-sm"
              style={{ backgroundColor: preset.vars["--avid-base"] }}
            >
              <span className="flex-1" style={{ backgroundColor: preset.vars["--avid-panel"] }} />
              <span className="flex-1" style={{ backgroundColor: preset.vars["--avid-raised"] }} />
              <span className="flex-1" style={{ backgroundColor: preset.vars["--avid-action"] }} />
            </span>
            <span className="text-xs text-avid-primary">{preset.name}</span>
          </button>
        ))}
      </div>
      <p className="mt-3 text-xs text-avid-muted">
        Themes recolor chrome only — timeline content keeps its categorical colors so cuts stay
        readable in every theme.
      </p>
    </Panel>
  );
}
