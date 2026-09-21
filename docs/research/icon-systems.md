# Icon system research (MASTER-DESIGN §48)

Researched 2026-09-21. Primary sources: lucide.dev/license, github.com/lucide-icons/lucide (LICENSE), lucide README/packages.

## Options compared

### A. lucide-react (RECOMMENDED for the general set)
- ISC license (free commercial + personal, attribution via notice). 1600+ icons, consistent 24px grid + stroke width, `lucide-react` package, Figma plugin for design parity.
- Pros: one coherent geometric system (exactly §48); tiny per-icon cost when tree-shaken; no design maintenance.
- Cons: **no brand logos by policy** (legal + consistency stance) — provider marks (Ollama/OpenAI/…) must be custom subtle glyphs or text badges, which actually matches §72 ("one consistent badge, don't alter official marks" — text-first avoids the trademark problem entirely).
- Note: Feather-derived subset carries MIT (Cole Bemis) — same permissive class, record both notices.

### B. Hand-drawn custom set for everything
- Pros: maximum distinctiveness.
- Cons: weeks of design work; inconsistency risk; delays the §48 fix (text glyphs) by months. Rejected as the *general* set — but REQUIRED for the AVID logo mark (§7), which must be original.

### C. Emoji/text glyphs (status quo: `✕ ▦ ☰ ♪ →`)
- Rejected: violates §48 explicitly; inconsistent rendering across platforms; no stroke discipline.

## Decision
**A for UI icons** (`lucide-react`, tree-shaken imports only, 16–20px, `currentColor`), **B for the logo mark only** (original geometric symbol derived from edit-point/frames/direction per §7; must work at favicon size, monochrome, no gradients). Provider badges: single component, text-first (provider small, model primary), no third-party marks. LEGAL rows required before install.
