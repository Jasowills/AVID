# MASTER-DESIGN vs implementation — section-by-section comparison

> Scope: MASTER-DESIGN.md §§1–85 against the repo at fix-all pass.
> Verdicts: ✅ aligned · 🟡 partial (works, gap documented) · ❌ missing · ⛔ violates (must change).
> AGENTS.md remains the engineering source of truth; MASTER-DESIGN governs the visual layer. Where they overlap (no hype copy, honest states, dark-first), they agree.

## Identity & philosophy (§0–7)

| Section | Verdict | Notes |
|---|---|---|
| Purpose/workstation, not SaaS/chatbot (§0) | ✅ | App is a real editor; no marketing cards in-app |
| Work-is-interface (§1) | 🟡 | Editor centers preview+timeline; but panels are fixed-ratio, no collapse/resize (§54) |
| Anti-slop (§2) | ✅ | No gradients/orbs/robots/glass; copy audit clean (banned words appear only inside AGENTS.md's own ban list) |
| Monochrome identity (§3) | ⛔ | Electric blue `#4f8cff` is hard-coded as THE accent. Spec: monochrome default, blue one optional preset |
| Color architecture (§4) | 🟡 | Semantic tokens exist (`packages/design-system` + Tailwind `@theme` mirror) but are build-time constants, not a theme system |
| Theme customization (§5) | ❌ | No presets, no user themes, no save/export/import, no serialization |
| Color usage (§6) | ✅ | Accent used for selection/focus/primary; no rainbow |
| Logo (§7) | ❌ | Text "AVID" in TopBar; app icon is a generated placeholder, not a designed geometric mark |
| Typography (§8) | 🟡 | Inter hard-coded (spec: platform default stack + customizable interface/mono fonts); mono usage correct (timecodes, metadata, shortcuts) |
| Spacing 4px base (§9) | 🟡 | rem scale maps to 4/8/16/24/32 but 20/40/48/64/80/96 steps missing; some ad-hoc values |
| Radius 4/6/8 max (§10) | ⛔ | Tokens are 4/6/8/12px (`lg: 0.5rem`, `full`); spec caps structural surfaces at 8px |
| Depth, no glow (§11) | ✅ | Borders/contrast only |
| Motion intent + timing (§12–13) | ❌ | No motion tokens; transitions ad-hoc; reduced-motion CSS guard exists (good foundation) |

## Landing website (§14–26, 55, 59–62, 69–70, 83)

| Section | Verdict | Notes |
|---|---|---|
| All landing sections | ❌ | No website exists at all. Per §79 order this comes AFTER the design system stabilizes — plan it, don't build yet |

## Harness (§27–38)

N/A as specified: the "harness" (agent control room) is this development environment, not the video editor. No action; revisit only if a second desktop surface is scoped.

## Settings (§39–42)

| Section | Verdict | Notes |
|---|---|---|
| Categorized settings | ❌ | Single scrolling page; needs nav (General/Appearance/Themes/Typography/Models/Providers/Keyboard/…) |
| Appearance + theme editor | ❌ | Nothing exists (see theme system) |
| Brand vs theme (§42, §73) | 🟡 | Principle understood; unenforceable without the theme engine |

## Components (§43–48, §71–72)

| Section | Verdict | Notes |
|---|---|---|
| Buttons 28/32/36–40px, restrained variants | 🟡 | Variants exist (primary/secondary/ghost/danger) but heights unsystematized (~36px+ everywhere); motion states minimal |
| Inputs/menus | 🟡 | Native selects + TextField are honest; no compact menu/dropdown primitives |
| Command palette (§47) | ❌ | Missing entirely (Cmd/Ctrl+K). Research done: hand-rolled first (cmdk pulls Radix tree) |
| Iconography (§48) | ⛔ | Text glyphs as icons (`✕ ▦ ☰ ♪ →`). Spec bans emoji-as-icons and demands one coherent geometric set |
| AI states (§49) | 🟡 | Text statuses exist; missing canonical IDLE/ANALYZING/… set + suggested-vs-committed provenance in timeline |
| Primitives inventory (§71) | 🟡 | Have: Button/Panel/TextField/EmptyState/Dialog-ish/Tabs-ish/Timeline/Transcript/AIPlan/AIStatus/Toast-ish/Progress. Missing: IconButton/Select/Tooltip/Slider/Toggle/ModelSelector/ProviderBadge/Terminal/EventLog/ThemeEditor/Waveform/CommandPalette |

## Editor surfaces (§50–54)

| Section | Verdict | Notes |
|---|---|---|
| Timeline (§50) | 🟡 | Tracks/clips/playhead/ruler/selection/zoom/drag/trim/split/snap/shortcuts real. Missing: waveforms, in-timeline thumbnails, markers, transitions, captions display, keyframes, multi-select |
| Viewer (§51) | 🟡 | Native `<video controls>` is honest but not the spec (play/pause/step/timecode/volume/fullscreen/zoom/fit, minimal overlay) |
| Media browser (§52) | 🟡 | Grid/list, thumbs, duration, search real. Missing: resolution/FPS/audio-presence columns, filter, folders |
| Inspector (§53) | 🟡 | Trim + audio real; Transform/Crop/Speed/Color/Captions/Effects/AI-metadata groups need engine support first |
| Responsive panels (§54) | ❌ | Fixed grid; no collapse/resize/timeline-expand |

## Systems (§56–68)

| Section | Verdict | Notes |
|---|---|---|
| A11y/reduced motion (§56–57) | 🟡 | Labels, focus, roles, no-color-alone, reduced-motion guard. Missing: screen-reader pass, preset-contrast validation |
| Performance (§58) | 🟡 | Off-thread discipline good; no measurements, no lazy-motion policy |
| Copy systems (§63–65) | ✅ | Short/specific product verbs; empty states guide with actions |
| Loading/error/success (§66–68) | 🟡 | Errors explain + guide (humane mapping); loading copy needs asset names ("Transcribing interview", not "Transcribing…"); success has undo in AI reports, needs [Open file] parity everywhere |

## Governance (§73–84)

| Section | Verdict | Notes |
|---|---|---|
| Default monochrome theme (§74) + accent presets (§75) | ❌ | The core build item |
| Exploration rule (§76) | 🟡 | Research-first culture exists; add explicit exploration checkpoints to DESIGN-PLAN |
| No random AI UI (§77–78) | ✅ | Spec-driven; repo inspection before UI code is the norm |
| Implementation order (§79) | — | Adopted as DESIGN-PLAN's spine (see that doc) |
| Visual QA (§80) + anti-regression (§81) | ❌ | No screenshot/visual QA process; TS-compile ≠ done is stated but unenforced |
| No fake functionality (§82) | ✅ | Strongest alignment: disabled-honest controls, verified-everything culture |
| Final test (§84) | 🟡 | 5-second editor recognition: yes. Landing questions: N/A (no site). Monochrome self-sufficiency: untested (blocked on theme engine) |

## North star (§85)

"Real video editor that happens to have an AI Director" — architecture says yes; the visual layer needs the theme engine + icon system + command palette before the *feel* matches the claim.
