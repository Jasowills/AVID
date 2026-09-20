# UX flows (Phase 0.6)

> Source: AGENTS §§35–58, §65, §145–147. Research basis: `docs/research/editors-comparison.md`.
> Design language: dark-first, calm, professional. No hype copy ("Remove pauses", not "AI Magic").

## 1. First run

`Welcome to AVID → "Where should your AI run?" [Local | Cloud | Both] → "Choose your experience" [Simple | Professional]`. Never block exploration on configuration. Local status visible on Home afterwards.

## 2. Home

Sections: Recent Projects · Create New · Templates (search + preview + "Use as starting point") · Examples · Getting Started · Local AI status (model, capabilities, missing-vision notice). Empty states always carry an action (`[Import Media] [Try an Example] [Explore Templates]`).

## 3. Project creation

Modal: name · canvas (16:9, 9:16, 1:1, 4:5, custom) · fps (24/25/30/50/60) · resolution (720p/1080p/4K) · optional template. `[Create Project]`.

## 4. Editor layout

```text
Top bar: project · undo/redo · save status · AI status · preview quality · Export · Settings
Left:  Media | Transcript | Scenes | AI | Templates | Assets | Audio | Captions
Center: Preview (quality 1/4 · 1/2 · Full · Auto)
Right: Inspector (selection context)
Bottom: Timeline
```

Left tabs are task panels, not chat windows. The AI panel is an **editor command center** ("What would you like to change?" + contextual suggestions: talking-head → "Remove filler words / Create a Short / Add captions"; technical → "Explain this visually / Create architecture diagram / Highlight the code"; podcast → "Find strongest moments / Create 3 Shorts").

## 5. Timeline interactions

Zoom, horizontal scroll, track height, snapping, split (S), trim, ripple delete, overwrite, insert, optional magnetic close-gaps, markers, single/multi-select. Standard shortcuts: Space play/pause, J/K/L, I/O in-out, S split, Delete, Cmd/Ctrl+Z / +Shift+Z, Cmd/Ctrl+S, E export (customizable later). Context rule: selection scopes AI ("Make this more dynamic" acts on the selected clip; "Simplify" on the selected visual; "60-second Short" on the selected range).

## 6. Transcript panel

Speaker · timestamp · text · search/highlight/select. Actions per selection: "Remove this sentence" (→ `remove_range`), "Create clip", "Add visual". Search "Kafka" jumps every occurrence.

## 7. AI rough-cut review (trust-critical)

Proposal card: `"AVID found 3m 42s of potentially removable material." [Review]`. Per-op rows: `REMOVE 00:12.4 → 00:16.8 · Reason: long pause · Confidence: High [Accept] [Reject] [Edit]`. Large ops show Before/After duration + counts (`14 cuts · 3 visuals · 2 caption changes`) with `[Review Changes] [Apply] [Cancel]`. Post-apply summary states what happened (`Removed 8 pauses · Added 3 diagrams · 12 caption segments`), never just "Done". Grouped undo always available. "Why?" answers per op ("removed because it repeated the previous sentence").

## 8. Jobs + errors

Global indicator (`Processing 3 tasks`) → Job Center: name, %, cancel/retry/error-inspect. Errors are humane: `"AVID couldn't create the proxy. Possible reason: FFmpeg could not decode this file. Try: Retry / Use original / Convert / View technical details"` (expandable). Never raw `subprocess exited 1`.

## 9. Provider config + cloud consent

Settings → AI Providers → Local (Ollama connected/models/refresh, custom endpoint base URL) / Cloud (OpenAI/Anthropic/Gemini keys, capabilities, `[Test]` → "Provider ready" or "lacks structured output"). Every AI op labels processing (`Local` vs `Cloud: Provider`). Cloud-required ops prompt `[Allow Once] [Always Allow] [Cancel]` with cost note where providers charge. "Prefer local even if slower" setting. Failure never auto-reroutes — "Claude failed. Try another configured provider? [Choose]".

## 10. Export

Dialog: preview · format (MP4/H.264/AAC) · resolution · fps · quality · estimated size/time · output path → `[Export]` → `[Open File] [Show in Folder] [Create Short]`. No auto-upload. Missing-media state: `"Media missing" [Locate] [Relink Folder]`.

## 11. Accessibility + i18n (built in from Phase 1)

Keyboard-only editing path, screen-reader labels, visible focus, high contrast, reduced motion, scalable UI; user strings through the i18n-ready path (no deep hard-coding).
