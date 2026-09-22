# Diffusion UI port record (MPL-2.0 compliance)

> Source: `github.com/diffusionstudio/editor@57c3983` (MPL-2.0), cloned
> read-only to `/tmp` — never vendored into this repo. What moved across is
> **structure re-implemented in React** (new code, new files); no SolidJS,
> Kobalte, koota, reconciler, engine, dapi, or brand assets were copied.
> Every ported file carries the MPL header + this pointer. Legal row:
> `docs/LEGAL_AND_LICENSING.md` (Diffusion entry). Brand exclusion honored:
> AVID keeps its own logo, name, and monochrome identity.

## Source → target map

| Diffusion source | AVID target | Adaptation |
|---|---|---|
| `apps/web/src/pages/editor.tsx` (grid skeleton, resize handle, minimize, `uiVisible` fullscreen canvas, floating header) | `apps/desktop/src/pages/Editor.tsx` (rewritten) | React + zustand; grid rows/cols, divider strips, handle behavior, minimized ruler row, floating header kept 1:1 |
| `apps/web/src/context/layout.tsx` (uiVisible/timelineHeight/minimized, persisted) | `apps/desktop/src/stores/useLayoutStore.ts` | zustand + localStorage; same defaults (234px, min 120); max 560 added (preview usability) |
| `apps/web/src/components/timeline` (Layers column concept) | `apps/desktop/src/components/Layers.tsx` | Aligned to our lane metrics (28px ruler + 40px lanes); real lock toggles via the command engine |
| `apps/web/src/components/ui/command.tsx` (palette structure) | `apps/desktop/src/components/CommandPalette.tsx` | Hand-rolled (no cmdk dep at this volume); same overlay/panel/groups/hints/keyboard map; only real commands listed |
| `apps/web/src/components/ui/tooltip.tsx` | `packages/ui/src/Tooltip.tsx` | Hand-rolled CSS reveal (no Kobalte); always paired with aria-label |
| `apps/web/src/components/ui/kbd.tsx` | `packages/ui/src/Kbd.tsx` | Same chip |
| `apps/web/src/components/ui/context-menu.tsx` + `app-context-menu.tsx` | `packages/ui/src/ContextMenu.tsx` | Hand-rolled; no submenus; clamped to viewport; honest disabled items |
| `sidebar-left.tsx` (rail widths 264/340, project header) | `Editor.tsx` rail + `TopBar` | Rail widths adopted (AI tab 340, else 264); project header stays in TopBar (deviation D1) |

## Deviations (deliberate)
- **D1 — TopBar kept.** The reference has no top bar; AVID's spec requires undo/redo/save/AI/export/settings chrome. The grid sits below it.
- **D2 — No Soundboard cell.** The bottom-right grid cell holds timeline tools (zoom, minimize, hide-UI) instead; no audio-clip tool exists yet, and dead space is worse than honest tools.
- **D3 — Tokens mapped, not copied.** Their dark values (≈7% black, 3–6% white borders, blue primary) already match our monochrome family; porting hex values would fork the theme engine for zero gain.
- **D4 — Kobalte → hand-rolled.** Radix/Kobalte-equivalent deps rejected per dependency policy (bundle + surface); primitives re-implemented with the same structure and keyboard maps.
- **D5 — Minimized = restore strip.** Reference collapses to ruler height; we render a 28px restore strip (same height) rather than a bare ruler, so the control stays discoverable.

## Queued (not yet ported)
- `panel-section` / `item-row` / `tabs` chrome for Inspector/Jobs/Transcript
- `breadcrumbs`, `dialog` chrome (ExportDialog works; revisit its overlay)
- Media-library rows (`asset-item`, `folder-item`) for the Media panel refresh
- `menubar`, `dropdown-menu` if menus outgrow the context menu
