# Diffusion UI adoption plan (desktop/harness focus)

> Source: 5 reference screenshots of Diffusion Studio + `diffusionstudio/editor` repo structure (SolidJS/Electron, inspected remotely — no code copied; MPL-2.0 + brand exclusion respected).
> Rule: adopt layout language, styling patterns, and component *shapes*. Rebuild everything on our stack (React/Tauri/Rust). No copied assets, icons, or brand marks. collaborated with MASTER-DESIGN (monochrome chrome, colored content) and AGENTS (no fake controls).

## Observed language (screenshots)

- Near-black monochrome chrome (`~#0a0a0a–#121212`), subtle 1px borders, 11–12px UI text, mono numerals, dense panels.
- Layout: left assets browser (search + grid + counts) | center viewer + floating transport + timeline bottom | right inspector sections | mixer docked right of timeline.
- Inspector: `Section [+]` headers; rows = label-left/control-right; compact numeric fields with unit prefixes (X/Y/W/H); stepper −/+; diamond keyframe marks; dropdown selects; nudge pad; eye toggles.
- Assets: 2-col cards, duration badge top-left, filename below; audio = dark-green card + bright-green waveform; folders with counts.
- Timeline: labeled track headers with lock icons; clips colored by kind (captions magenta w/ text, adjustment purple, graphics teal/gray, groups orange, audio blue/green w/ waveforms); thin blue playhead; time ruler; right-side mixer with faders + dB + M.
- Transport: floating pill under viewer, blue play button + tool icons.
- Scene cards: thumbnail/title/duration right/Active badge.
- AI composer (genai): rounded card, attachment thumbs, @mention pills, model/settings row, blue send.
- Viewer: scene breadcrumb + Active badge, timecode top-right, blue selection border w/ handles, dimension badge.

## Adoption map

| Pattern | Us | Decision |
|---|---|---|
| Monochrome chrome + colored content | Our blue-accent chrome | **Adopt**: retune toward near-black chrome; keep ONE accent (theme engine will make it a preset) |
| Inspector sections/rows/fields/steppers | Trim+audio forms | **Adopt**: restyle to section/row pattern; diamonds shown disabled with "keyframes land with engine support" honesty |
| Asset cards + badges + audio waveforms | List/grid + thumbs | **Adopt**: card restyle, count headers, folder grouping later (needs manifest folders — later) |
| Track headers + lock | Plain labels | **Adopt**: icons + working lock toggle (engine supports locks; add command) |
| Transport pill + timecode | Native controls only | **Adopt**: custom play/pause + time display wired to video element; fullscreen native |
| Playhead handle + ruler | Line only | **Adopt**: handle triangle + time tooltip |
| Clip colors by kind | Muted fills | **Adopt**: saturated kind colors on mono chrome (matches MASTER-DESIGN color-belongs-to-content) |
| Waveforms in timeline | None | **Later**: needs peaks pipeline (MediaEngine waveform → JSON → canvas); recorded, not faked |
| Mixer faders | None | **Later**: needs track-level gain (model has clip gain only); honest defer |
| Scene cards/composer/genai | None | **Later**: scenes model doesn't exist; composer arrives with Director surface |
| Toolbar icon rows (align/distribute) | None | **Skip**: no transform engine to drive them; revisit with transforms |
| Nudge pad, rotate/flip, blending, radius, stroke, shadow | None | **Skip**: each needs engine fields + render support; tracked per-field, never stubbed |

## Phases (this pass: U1–U3)

- **U1 — Icons + chrome**: lucide-react install; `Icon` wrapper (16px, currentColor); replace ALL text glyphs; TopBar/Editor chrome alignment; LEGAL row (done).
- **U2 — Inspector + assets**: section/row restyle; lock toggle command; asset cards with badges; search input styling.
- **U3 — Transport + timeline chrome**: play/pause/time in PreviewPane; playhead handle; kind colors; ruler ticks styling.
- **Later**: waveforms, mixer, scenes, composer, folders, toolbar rows, transform fields — each gated on engine support.

## Anti-copy rules (binding)

- No Diffusion code, icons, logos, brand assets, or text copied into the repo — patterns only.
- No control that implies engine support we lack (diamonds/steppers for keyframes stay disabled with honest titles until the engine lands).
- Every adopted pattern keeps our a11y bar (labels, focus, keyboard, reduced motion).
