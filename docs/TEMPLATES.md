# AVID Templates (AGENTS §31–34)

Implementation: `packages/templates` (loader/validator) + `templates/`.

## Format

`template.json { name, version (semver), description, category, aspectRatios, supportedFeatures?, requiredAssets?, rules? }` with `rules { captions, visuals { preferDiagramsFor, maxPerMinute }, pacing }` encoding editing BEHAVIOR, not just looks.

## Status

- All 12 `template.json` validate in-test. `technical-explainer` v0.1.0 has full behavioral content; the other 11 are honest scaffolds (status: Phase 9 content pass).
- Missing: template browser/preview/apply-as-commands UI, template authoring (§127).

## Rules

Applying a template must produce undoable commands ("starting point", §34) — never a destructive rewrite. That UI is pending; the format is frozen enough to build against.
