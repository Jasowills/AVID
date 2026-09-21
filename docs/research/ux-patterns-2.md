# Part-2 UX patterns research (MASTER-DESIGN Part 2 §§9–11, 35–36, 55, 83, 94)

Researched 2026-09-21. Primary sources: npmjs.com/package/sonner (MIT, 0 deps, ~166KB, React 18/19), github.com/bvaughn/react-resizable-panels (MIT, 0 deps, Panel/Group/Handle + collapsible + autoSaveId + imperative API), npmjs.com/package/react-resizable-panels (same).

## Toasts (§§35–36): sonner vs hand-rolled

- **sonner**: MIT, zero dependencies, huge adoption, promise API, rich colors, reduced-motion CSS built in. Costs: opinionated stylesheet (own gray palette, 8px radius) that fights a token-driven monochrome system; theming it means overriding its CSS surface-by-surface.
- **Hand-rolled Toaster** in `@avid/ui`: ~120 lines (store + bottom-right stack + auto-dismiss + action buttons + `role="status"`), token-native by construction, capped stack, never interrupts playback (no modal behavior).
- **Decision: hand-rolled.** Our needs (success confirmations with Undo/Open-file actions, capped, dismissible) are a subset; token discipline (§4) outweighs sonner's conveniences. Revisit only if toast requirements outgrow the component.

## Resizable + collapsible panels (§§9–11, §55): react-resizable-panels vs hand-rolled

- **react-resizable-panels** (bvaughn, MIT, 0 deps): `PanelGroup`/`Panel`/`PanelResizeHandle`, min/max/collapsible/collapsedSize, `autoSaveId` persistence to localStorage, imperative API (collapse/expand/setLayout), keyboard resizing, nested groups. Directly implements §9 (hover states, hit targets, min/max, remember dimensions, reset) + §10 (collapse with discoverability via `onCollapse` affordances we render).
- **Hand-rolled dividers**: full control of 1px/divider hover/active states, but reimplements constraint math, persistence, keyboard support, nesting.
- **Decision: adopt the library** (LEGAL row first). Style handles with our tokens (1px subtle, contrast bump on hover/active per §11). Persist under `avid.layout.*` keys; reset-to-default in Settings. Note: dist is ~540KB but tree-shaken usage is far smaller; measure at integration and record.

## Density modes (§83): Comfortable / Compact

- Precedent (VS Code, Linear, Gmail): a density class on the root scaling row heights, paddings, and font-size deltas — hierarchy untouched.
- **Decision:** `[data-density="compact"]` overrides a small set of vars (`--row-h`, `--panel-gap`, `--timeline-lane`). No component forks. Persisted with theme.

## Offline / model availability (§94): detection without a backend round-trip

- `navigator.onLine` + `online`/`offline` window events work inside the Tauri webview (no plugin needed) for the *network* half.
- Provider *availability* truth stays the existing `probe_provider` (localhost Ollama vs cloud). UI rule: `Ollama · Available` / `Claude · Offline` pill = network state AND last probe, never one alone.
- **Decision:** TopBar model-status pill combining both signals; cloud features render their unavailable state inline (never full-app breakage).

## Navigation/layout persistence (§55): localStorage now, Rust store later

- Keys: `avid.layout.*` (panel sizes/collapse, via lib autoSaveId), `avid.theme.*`, `avid.density`, `avid.shortcuts.*` (future), existing `avid.provider.v1` + `avid.projects.v1`.
- Migration rule: versioned keys (`.*.v1`), corrupt values fall back to defaults (established pattern in ProvidersPanel/Home).
