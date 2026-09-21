# Command palette research (MASTER-DESIGN §47)

Researched 2026-09-21. Primary sources: github.com/pacocoursey/cmdk + dip/cmdk (MIT), npm cmdk/react-cmdk pages.

## Options compared

### A. Hand-rolled palette on our primitives (RECOMMENDED first)
- Our command set is small and known: navigation (go Media/Transcript/AI/Jobs/Visuals/Settings), timeline ops (split/remove/undo/redo/trim), projects (open/create), media (import/probe), export, jobs (cancel), settings sections, models (selector follows in D-phase).
- Pros: zero new deps; exact §47 behavior (Cmd/Ctrl+K, searches commands/projects/files/media/scenes/models/settings/timeline ops); styling = our tokens automatically; testable pure filter fn.
- Cons: rebuilding fuzzy-match + a11y (aria-activedescendant) that libraries give for free — bounded cost (~200 lines).

### B. `cmdk` (MIT, unstyled primitives, auto filter/sort, a11y built in)
- Pros: best-in-class accessibility + filtering; composable; used by Linear/Vercel-class products.
- Cons: pulls Radix Dialog/popover tree (bundle + second design language to tame); styling via `data-[cmdk-*]` attributes parallel to our tokens; overkill for <60 items.
- Verdict: fallback if hand-rolled filtering proves weak. Not first.

### C. `react-cmdk` (MIT, last publish ~3 years ago)
- Rejected: stale, heavier API, brings its own CSS.

## Decision
**A**, with a `filterCommands(query)` pure function (unit-tested ranking) and a `CommandPalette` component in `@avid/ui` (overlay + input + grouped list + keyboard nav + `Cmd/Ctrl+K` global listener that yields inside text fields, same guard as timeline shortcuts). Graduate to **B** only on measured filtering/a11y gaps.
