# MASTER-DESIGN Part 2 vs implementation — comparison

> Part 2 = UX, layout, interaction, states, micro-detail (§§1–111).
> Verified: part 2 §§86–111 are line-identical to part 1 §§86–111 (diffed, 8 closing lines differ), so only §§1–85 are compared here.
> Verdicts: ✅ aligned · 🟡 partial · ❌ missing · ⛔ violates.

## Hierarchy, structure, grid (§§1–8)

| Section | Verdict | Notes |
|---|---|---|
| 7-question hierarchy (§2) | 🟡 | Selection/change visible; "where am I" weak (no breadcrumbs); single-primary-action holds on most screens |
| Page structure/muscle memory (§3) | 🟡 | Home/Editor/Settings share chrome+header+workspace but no formal layout contract |
| Padding: workspace 16–24, dense 8–16, empty 32–64 (§4) | 🟡 | Roughly true; not audited per-screen |
| 4px scale + hierarchy (§§5–6) | 🟡 | Base scale exists; 20/40/48/64/80/96/120 steps + relationship mapping missing |
| Alignment (§7) | 🟡 | Unmeasured; no known egregious breaks |
| App grid NAV/WORKSPACE/INSPECTOR (§8) | 🟡 | Editor matches (sidebar/preview+timeline/inspector); no left NAV rail (tabs serve as nav); no bottom events region |

## Panels, navigation (§§9–13, §55–59)

| Section | Verdict | Notes |
|---|---|---|
| Resizable panels + persistence (§9, §55) | ❌ | Fixed grid; nothing resizes, nothing remembered. Research done: adopt `react-resizable-panels` (MIT, 0 deps) |
| Collapsible panels (§10) | ❌ | No collapse anywhere |
| 1px dividers with hover/active (§11) | ❌ | Borders exist but no divider component/states |
| Back buttons `← Label` (§12) | 🟡 | TopBar has "AVID" home link; no labeled-back pattern on child contexts |
| Breadcrumbs (§13) | ❌ | None |
| Deep links/routes/refresh (§58–59) | ✅ | HashRouter stable routes; refresh preserves route (not scroll — acceptable) |

## Headers, toolbars, states (§§14–24)

| Section | Verdict | Notes |
|---|---|---|
| Page headers 48–64px (§§14–15) | 🟡 | Home/Editor headers compact; no formal header component |
| Toolbar groups + full state matrix (§§16–17) | 🟡 | Dock toolbar groups Split/Remove \| Undo/Redo; pressed/loading states incomplete |
| Hover/active/focus/disabled/pressed (§§18–22) | 🟡 | Hover/focus/disabled exist; active-vs-hover distinction weak; pressed states missing |
| Loading preserves layout, specific copy (§23) | ⛔ | Buttons change width on `…` labels; copy is generic ("Transcribing…", "Rendering…") — spec demands objects ("Transcribing interview") |
| Skeletons (§24) | ❌ | None (correctly absent where structure unknown; needed for media grid + transcript list) |

## Empty, first-run, dialogs (§§25–31)

| Section | Verdict | Notes |
|---|---|---|
| Empty states answer why + what (§25) | ✅ | All guide with actions; no sparkle-emoji copy |
| First-run, no slideshow (§26) | 🟡 | Home welcomes but never offers theme/provider choice; provider hint exists — needs the chooser |
| Recent projects first (§27) | ✅ | Recents dominate Home; no dashboard metrics |
| Project creation fields (§28) | 🟡 | Name/canvas/fps/resolution/template-optional ✅; location/model-config missing |
| Confirmations only for destructive (§29) | ✅ | None exist; delete flows are undoable instead (correct) |
| Dialog anatomy + Esc/Enter (§30) | 🟡 | Export dialog has title/actions/Esc-via-overlay-click; Enter-primary + focus trap missing |
| Tooltips name+shortcut, debounced (§31) | ❌ | `title` attrs only; no tooltip component, no shortcut display, no timing control |

## Shortcuts, palette, notifications (§§32–36)

| Section | Verdict | Notes |
|---|---|---|
| Keyboard-first + customizable (§32) | 🟡 | S/Delete/Cmd-Z exist in dock; no central map, no settings surface, no customization |
| Palette UX: open/search/groups/Enter/Esc (§§33–34) | ❌ | No palette (research done: hand-rolled first) |
| Toasts: success/background only, bottom-right, dismissible, capped (§§35–36) | ❌ | No toast system at all (research done: hand-rolled token-native, not sonner — its palette fights our tokens) |

## Context, drag, selection, undo (§§37–42)

| Section | Verdict | Notes |
|---|---|---|
| Context menus (§37) | ❌ | No right-click menus (clip: Split/Trim/Delete/Properties; media: Open/Reveal/Transcribe) |
| Drag feedback: ghost/insertion/highlight (§§38–39) | 🟡 | Clip drags move the original (no ghost), snapping has a dash indicator; no insertion line (N/A — move, not insert) |
| Selection obvious in monochrome (§40) | ✅ | Border + label; tested selectors |
| Multi-select + counts (§41) | ❌ | Single selection only |
| Undo labels, AI as one op (§42) | ✅ | `Undo AI: goal` pattern via grouped labels; Ctrl+Shift+Z redo |

## AI surfaces (§§43–48)

| Section | Verdict | Notes |
|---|---|---|
| Analyze→Plan→Propose→Review→Apply (§43) | ✅ | Real pipeline end to end |
| Review list + Apply all (§44) | ✅ | Checkboxes + counts + removable totals |
| Provenance: small icon/metadata (§45) | ❌ | AI-placed clips indistinguishable from manual ones |
| Model selector control + menu (§§46–47) | ❌ | Settings form only; no compact TopBar control, no grouped menu, no capability/cost display |
| Provider marks restrained (§48) | ✅ | Text-first by default (lucide has no brand marks — matches research) |

## Status, terminal, search (§§49–54)

| Section | Verdict | Notes |
|---|---|---|
| Agent status shape+text (§49) | 🟡 | Jobs show status text; needs shape+text pairs (`● Running`, `✓ Complete`) per spec |
| Agent activity detail (§50) | 🟡 | Label + progress real; elapsed/files-touched/model missing |
| Terminal/event states (§51) | ❌ | No event log surface (Jobs tab is the seed) |
| Search fast/keyboard/no-result (§54) | 🟡 | Media search real with no-result state; no keyboard nav, no loading state |
| File states (§52), git states (§53) | ❌ | No file browser / git surface in app (correctly out of scope for the editor; harness-owned) |

## Persistence, routes, landing detail (§§55–76)

| Section | Verdict | Notes |
|---|---|---|
| Nav persistence (§55) | ❌ | Nothing remembered (theme/density/panels/shortcuts) — localStorage only for provider URL + projects |
| Landing §§60–76 | ❌ | No website (gated behind design-system stability per part 1 plan) |
| Touch targets (§75) | 🟡 | Small buttons (zoom 11px, splitter-less); needs 24px+ hit areas with invisible padding |
| Breakpoints by content (§76) | 🟡 | One `md:` breakpoint; editor is fixed-grid desktop-first (acceptable per §54, documented) |
| Truncation + tooltip (§78) | 🟡 | `truncate` used; full-value tooltips missing |
| Monospace discipline (§79) | ✅ | Timecodes/paths/ids/timings mono; body text sans |
| Icon-button labeling (§80) | 🟡 | aria-labels present; tooltips + focus-visible need the tooltip component |
| Contextual actions (§81) | 🟡 | Actions live in owning panels; clip right-click + overflow menus missing |
| Progressive disclosure (§82) | 🟡 | Inspector is flat (trim/audio); Advanced groups pending engine support |
| Density modes (§83) | ❌ | Single density; needs Comfortable/Compact via var scale |
| Theme/accent transitions (§§84–85) | ❌ | Blocked on theme engine |

## Lifecycle states (§§86–99 tail; identical to part 1)

Covered by part 1 comparison except app-specific deltas:
- Autosave display (§90): ours says "Autosaved to project.json" — spec wants Saved/Saving…/Saved 4s ago. Needs timestamp state.
- Offline (§94): no `Ollama · Available` / `Claude · Offline` indicator anywhere. Needs model-status pill (online/offline events + probe).
- Model failure (§95): no retry/switch-to-local alternatives UI.
- Long tasks survive navigation (§96): jobs persist in registry ✅ but no global clickable activity indicator (TopBar pill shows count, not clickable).
- Cancel states (§97): cancel exists; "Cancelling…" intermediate missing.
- Version history (§98): snapshots exist, no browser UI.
- Destructive (§99): deletes are undoable (correct); no project-delete flow at all.
- Success (§100): reports exist; missing [Open file]-style next actions in some panels.

## QA/process (§§104–111 tail; identical to part 1)

- Interaction checklist (§105): panel resize/collapse, command palette, model selection, theme changes untestable — still red.
- Visual regression (§106): still no screenshot process.
- Product feel (§107): button-width jump on loading labels VIOLATES "should not shift" — fix with min-width.
- Screen checklist (§§108–109) + anti-slop (§110): adopted as the per-screen gate in the plan.
