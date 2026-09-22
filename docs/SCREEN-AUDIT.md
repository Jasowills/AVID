# Screen audit — layout & styling, every desktop screen

> Method: production `vite build` served via `vite preview`, screenshotted at
> 1440×900 with Playwright (HashRouter, so `#/` routes). Browser has no Tauri
> backend — backend-backed regions show their honest fallbacks, which are
> themselves audited. Shots: `/tmp/shot-{home,editor,new,settings,ai,jobs,export}.png`.
> Spec refs = MASTER-DESIGN Part 2 §numbers.

## Critical defects found (both fixed this pass)

### 1. Primary buttons invisible app-wide — FIXED
`Button variant="primary"` rendered as bare text on Home, New Project,
AI panel, Settings — no background, no hierarchy (§19 active/§21 disabled
states unreadable). Root cause: Tailwind v4 auto-detects content under the
Vite root (`apps/desktop`) only; `@avid/ui` source lives in
`packages/ui/src`, so ui-only utilities (`bg-avid-action`,
`text-avid-action-text`, `hover:bg-avid-action-hover`, EmptyState border
colors) were never generated. Fix: `@source "../../../packages/ui/src"`
in `apps/desktop/src/index.css`. Guard: `scripts/verify-web-build.sh`
fails the build if sentinel ui-only utilities are missing from dist CSS.

### 2. Propose row clipped in the 16rem sidebar — FIXED
`AiPanel` Min-silence + Fillers + Propose (min-w-32) in one flex row
overflowed the panel: "Fillers" label cut to "Fill", Propose button
fully off-screen and unreachable. Fix: stacked layout, Propose beside
the checkbox. Lesson: no fixed-min-width controls in the 16rem rail;
sidebar rows stack (§8 grid discipline).

## Screen verdicts (post-fix pixels)

### Home (`#/`)
- ✅ Header row (title + sub + primary action), card grid with truncation +
  `title` tooltips (§78), 2×2 status panels, tiny mono build stamp.
- ✅ Empty state guides with action (§25).
- 🟡 NavRail icon-only, no labels (§80 icon-button labeling — tooltips exist
  via title; acceptable interim).
- 🟡 Status panels describe missing features honestly, but Templates/Examples
  panels have no action affordance (dead-end cards; Phase 9 resolves).

### New project (`#/projects/new`)
- ✅ Centered card, canvas segmented control, selects, Cancel-quiet vs
  Create-primary hierarchy (post-fix).
- 🟡 No template-start path (Phase 9, tracked). No field validation yet
  (empty name → backend default; acceptable, honest).

### Editor (`#/editor/:id`)
- ✅ 40px TopBar: identity, project name, working Undo/Redo (disabled-dim
  when empty, §21), save status, AI pill, Export, Settings.
- ✅ gap-px dividers between regions (§11); tab strip with honest disabled
  coming-tabs + phase tooltips.
- ✅ Empty states guide in Inspector, Preview (browser), Timeline (browser),
  Media probe, Jobs (§25).
- 🟡 Timeline footer fixed h-56 — not resizable/collapsible (§§9–10 ❌).
- 🟡 Dock header controls are bare text (Split/Remove/Undo/Redo, zoom) —
  no toolbar grouping/icon leadership (§16 🟡).
- 🟡 AI pill shows "defaults", not the model (§46 ❌ — no selector control).
- 🟡 No breadcrumbs/back to Home in Editor (§§12–13 ❌; NavRail absent here).

### AI tab (post-fix, this pass)
- ✅ Analyzed checklist (Transcript/Timeline-audio/Visuals/Assets with
  honest not-covered), numbered `mm:ss.d` proposals, High/Medium confidence.
- ✅ Stacked propose row; Validate plan primary renders.

### Transcript / Visuals / Jobs tabs
- ✅ Honest loading/empty/error/cancel states; transcript seek + segment
  delete; Mermaid→SVG→place pipeline.
- 🟡 Visuals editor is form-fields, not canvas manipulation (Phase 8 depth).

### Settings (`#/settings`)
- ✅ Appearance grid (8 swatches render true), provider form, honesty copy,
  Test connection primary (post-fix).
- 🟡 No per-operation consent surface (§17 🟡); no density control (§83 ❌).

### ExportDialog
- ✅ Preset cards, estimate copy, honest browser fallback; success state
  offers Open/Show-in-folder.
- 🟡 No backdrop dim visible in shots — verify in Tauri (dialog spec §30).

### Toaster
- ✅ Token-native, Undo action on AI apply, auto-dismiss, no celebration
  (§100).

## Remaining layout/styling debt (ordered)
1. ~~Resizable + collapsible panels/timeline (§§9–10)~~ ✅ 2026-09-22 —
   Diffusion-grid port: resizable + minimizable timeline, persisted layout,
   hide-UI focus mode (`docs/DIFFUSION-PORT.md`). Collapsible side rails queued.
2. ~~Command palette (§§33–34)~~ ✅ 2026-09-22 — ⌘/Ctrl+K, real commands only.
3. ~~Styled tooltips (§31), context menus (§37)~~ ✅ 2026-09-22 — shared
   primitives, applied to timeline clips + tools cell (media rows queued).
4. Toolbar grouping in TimelineDock (§16) + model selector control (§46).
5. Breadcrumbs/back-nav (§§12–13), density modes (§83), light-mode
   deliberate pass (§86), offline/unavailable/retry states (§§94–95),
   version-history browser (§98), destructive confirms (§99).
