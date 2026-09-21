# Theme engine research (MASTER-DESIGN §4–5, §41)

Researched 2026-09-21. Primary sources: Tailwind CSS v4 docs (theme variables, dark-mode, upgrade guide), tailwindlabs discussion #15083, community v4-theming example (Next.js + shadcn/ui + next-themes).

## Options compared

### A. Semantic CSS vars on `:root` + `[data-theme]` overrides, bound via `@theme inline` (RECOMMENDED)
- How: `:root { --avid-bg: … }`, `[data-theme="graphite"] { --avid-bg: … }`, and Tailwind `@theme inline { --color-avid-bg: var(--avid-bg) }` so utilities (`bg-avid-bg`) follow the active theme at runtime.
- Sources: tailwindcss.com/docs/theme ("use those variables instead of the theme() function"), tailwindcss.com/docs/dark-mode (`@custom-variant` with `[data-theme=dark]` selectors), discussion #15083 (the `@theme inline` + `:root` pattern).
- Pros: runtime switching with zero rebuild; serializable themes = JSON of var values; per-theme overrides are one selector; system-theme support via `matchMedia` + attribute sync (documented pattern); `localStorage` persistence avoids FOUC.
- Cons: every themed utility must route through the vars (discipline); Tailwind can't statically see values (no `bg-[#...]` extraction issues if we never use raw hex — matches §4 "never raw colors").

### B. `dark:`-style variants per theme (`blue:`, `graphite:`…)
- Pros: pure Tailwind idiom.
- Cons: combinatorial explosion across 9+ presets; user-created themes impossible (build-time only). Rejected: violates §5 serializable/arbitrary themes.

### C. CSS-in-JS runtime theming (styled-components/emotion theme provider)
- Pros: ergonomic in React.
- Cons: new runtime + bundle cost; fights Tailwind v4 (two sources of truth); Tauri webview perf risk. Rejected.

## Decision
**A.** Theme = `{ id, name, base: monochrome|light, vars: Record<token, value> }` stored as JSON (`localStorage` interim, Rust store later), applied by setting `documentElement.dataset.theme` + injecting var overrides. JS consumers (SVG canvas) read the same resolved values via `getComputedStyle` or the TS token mirror. Presets ship as JSON: Monochrome (default), Graphite, Blue, Violet, Green, Red, Solar, High Contrast, System (follows OS). Accent-only customization guaranteed by construction (base layers stay monochrome).

## Contrast/accessibility note
Preset authoring must pass WCAG AA for text pairs; the theme editor shows live contrast ratios (blocks shipping an unreadable preset — §41 "remain readable regardless of accent").
