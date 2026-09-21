# AVID Design Implementation Plan (MASTER-DESIGN §79 order)

> AGENTS.md stays the engineering source of truth; this plan governs the visual layer only.
> Rule echoed from §77–78: inspect repo → reuse tokens/components/routes → plan → then code. No duplicate components.
> Each phase ends with: exploration checkpoint (§76) + visual QA (§80) + anti-regression sweep (§81) + PROGRESS update. TS-compile ≠ done.

## D1 — Design tokens (retune, not rebuild)
- Collapse radii to 4/6/8 (+none), formalize the 4px spacing scale (4…96), add missing tokens: `--surface-hover/active/selected`, `--text-disabled`, `--selection`, `--focus`, `--ai-generated/suggested`, `--info`.
- Keep TS mirror in sync (single edit point each side; CI check optional later).
- Deliverable: tokens match §§4/9/10 exactly.

## D2 — Typography
- Interface font → platform default stack (keep Inter as fallback, not forced); expose Interface Font + Monospace Font settings (stored, applied via CSS vars).
- Audit numeral/punctuation rendering at 11–12px; lock type scale (no giant display text in-app).

## D3 — Theme engine (the core build)
- Architecture per `docs/research/theme-engines.md`: JSON themes → CSS vars on `:root` + `[data-theme]` overrides, `@theme inline` bindings, `dataset.theme` switching, localStorage now / Rust store later.
- Presets: Monochrome (**default**), Graphite, Blue, Violet, Green, Red, Solar, High Contrast, System. Blue = today's values preserved as one preset.
- Migrate every hard-coded hex (incl. `accent-[#4f8cff]` instances, SVG canvas fills, timeline colors) to semantic vars. Monochrome must be fully usable with accent removed.
- Contrast gate: theme editor shows live AA ratios; unreadable presets can't save.

## D4 — Icon system
- Install `lucide-react` (ISC/MIT row in LEGAL first); replace ALL text glyphs (`✕ ▦ ☰ ♪ →` etc.) with 16–20px `currentColor` icons.
- Design the original AVID mark (§7: edit-point/frames/direction, favicon-capable, monochrome) + regenerate app icons from it.
- Provider badges: single text-first component (provider small, model primary, local/cloud indicator).

## D5 — Buttons / inputs / controls
- Systematize heights (28/32/36–40), hover/active/focus/loading/success states per §44, no bounce/scale.
- Add missing primitives to `@avid/ui`: IconButton, Select, Tooltip, Slider, Toggle, Progress (job bars converge here), Toast.
- Loading copy audit: specific verbs with objects ("Transcribing interview.mp4").

## D6 — Panels / navigation
- Collapsible sidebars, resizable timeline dock, inspector overlay mode, timeline-expand (§54). Density setting.
- Settings categorized navigation (General/Appearance/Themes/Typography/Models/Providers/Keyboard/…) — providers UI moves under it.

## D7 — Command palette
- Per `docs/research/command-palettes.md`: hand-rolled `filterCommands` + `CommandPalette` in `@avid/ui`, Cmd/Ctrl+K, searches commands/projects/media/scenes/models/settings/timeline ops.

## D8 — Model selector + AI states
- Compact model control (provider mark small, model primary, context secondary) + selector dialog (provider/model/local-cloud/context/capabilities/cost).
- Canonical AI states (IDLE/ANALYZING/PLANNING/PROPOSING/RUNNING/WAITING/COMPLETE/FAILED/CANCELLED) across Jobs/AI panel/TopBar; suggested-vs-committed provenance tint on AI-placed timeline clips.

## D9 — Editor shell refresh
- Apply D1–D8 to Home/Editor/Settings/Export; remove remaining card-look panels (alignment + separators per §10); verify monochrome + blue preset coherence.

## D10 — Timeline visuals
- In-timeline waveforms (peaks from MediaEngine), clip thumbnails, markers (+ add/jump), captions lane display, AI-provenance tint, transitions affordance (UI first, engine render later, honestly labeled).

## D11 — AI Director surface
- Restructure AI panel toward §24: UNDERSTANDING checklist → PROPOSED EDIT numbered list → Review Changes → timeline responds. Reuse existing validator/apply pipeline; this is presentation, not new AI.

## D12 — Landing system (gated: starts only after D1–D9 stable)
- Separate surface (`apps/landing` or `website/`): narrative sequence per §15, hero = real assembling editor (§19), script→timeline + explain→visualize sections driven by OUR renderer/validator packages (dogfooding, never fake UI), provider toolchain strip, "Open AVID / Start editing" CTAs (§62). No testimonials/pricing/FAQ cards (§14). Recordly film (§20/83) after harness flows freeze.

## D13 — Motion, responsive, a11y, perf, QA (continuous, hardened at the end)
- Motion tokens (120–400ms app scale) + reduced-motion enforcement audit.
- Panel behavior per §54; landing responsive per §55 (focused compositions, not shrunk editor).
- Screen-reader pass, preset-contrast validation, perf measurements (transform/opacity discipline), visual-QA checklist per screen.

## Explicitly out of scope
- Harness screens (§27–38): the harness is this dev environment. Revisit only if a second desktop surface is scoped.
- Mobile editor app, collaborative editing, stock marketplace (AGENTS §123 stands).

## Definition of done (per visual task)
Tokens/components/routes reused · monochrome-usable · blue-coherent · focus/hover/loading/error/empty states · reduced-motion correct · no banned copy · no new hard-coded hex · PROGRESS + verify-script updated.
