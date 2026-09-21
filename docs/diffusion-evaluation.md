# Diffusion Studio fork evaluation (HARNESS.MD reconnaissance)

> Status: **evaluation, not a decision**. Per HARNESS.MD §32: audit first, then stop and wait for approval before major architectural changes. No editor code was modified for this doc.
> Researched 2026-09-21 against `github.com/diffusionstudio/editor` (main) and `diffusion.studio`. Versions noted; re-verify before acting (fast-moving repo).

## 1. Verified upstream facts

| Fact | Evidence |
|---|---|
| Exists, active | `diffusionstudio/editor`, ~2.4k stars, 490 commits, created 2026-07-07, updated Sept 2026; org is YC F24 |
| License | **MPL-2.0** (NOT MIT/Apache). Brand assets in `apps/desktop/assets` explicitly excluded (all rights reserved) |
| Stack | TypeScript + **SolidJS** + **Electron 43** (+ Forge dmg/zip makers); Node 20+, npm workspaces |
| Document model | SolidJS/TSX modules as source ("edits become code"); canvas↔code two-way editing; koota ECS (`koota-solid`) |
| Media engine | `diffusionstudio/core`: browser compositing via **WebCodecs** + Canvas2D over Mediabunny; interactive + render modes |
| Agent surface | `dapi` tool catalog (zod schemas; app serves, CLI derives, renderer validates) + `agent-chat` host + `@modelcontextprotocol/sdk` in desktop + `skills/` repo for Claude Code/Codex/Cursor/Copilot/Gemini |
| CLI | `dapi media probe/grab/filmstrip/waveform/transcribe/listen` (probe ≈ ffprobe, transcribe timed word-level) |
| Desktop distribution | macOS arm64 `.dmg` (+ Homebrew tap); web app requires account sign-in |
| Comparables | `core` as a library carries a **"Made with Diffusion Studio" watermark unless a (one-time, perpetual) license key is purchased** |

## 2. Architecture map: theirs vs ours

| Layer | Diffusion Studio | AVID today |
|---|---|---|
| Language/runtime | TypeScript everywhere; browser WebCodecs | Rust core + TS frontend; FFmpeg sidecars |
| Desktop shell | Electron 43 + Forge | Tauri 2.x |
| UI framework | SolidJS | React 18 |
| Document model | TSX code-as-source, infinite canvas | Versioned JSON manifest (`project.json`), tracks/clips |
| Timeline | Canvas + reconciler packages | Custom Rust model + SVG canvas UI |
| Commands | dapi zod tool catalog (agent-facing) | Typed Rust commands, grouped undo, transactional apply |
| Transcription | dapi transcribe (word-level) | whisper.cpp in-process + JSON parser + phrase mapping |
| Render | WebCodecs/Canvas2D encode (watermarked w/o key) | FFmpeg DAG, verified, no watermark |
| AI abstraction | Agent skills + MCP + provider-via-agent | Capability registry + canonical OpenAI-shape adapters, BYOM |
| License | MPL-2.0 + excluded brand + paid watermark key | MIT throughout |
| Platforms | macOS arm64 download; browser w/ account | Tauri mac/win/linux (built `.app` verified) |

## 3. HARNESS.MD claim check

- Accurate: project exists/active; professional editor; agent integration real (dapi/MCP/skills); TSX compositions; transcript/captions/export; research→understand→plan discipline; no-fake-functionality rule (compatible with our §136).
- Inaccurate or risky:
  - "Preserve the editor" presumes stack compatibility. There is **zero overlap**: Electron≠Tauri, Solid≠React, WebCodecs≠FFmpeg, TSX-source≠JSON-manifest, MPL≠MIT. A fork keeps almost nothing we verified (81 Rust tests, render pipeline, whisper binding, session/autosave, Tauri backend).
  - "Generic Electron editor" dismissal (§3) cuts the other way: adopting means *becoming* an Electron app — larger bundles, Chromium-only, and their desktop is macOS-arm64-first (our matrix is wider).
  - Licensing understated: MPL-2.0 file copyleft (modified files stay MPL + source disclosure), brand assets excluded, `core` watermark/paid key. All manageable but each needs a LEGAL row and a decision — none is "just fork it".
  - Local-first (§17) vs their account-gated web app + cloud-adjacent agent flows: needs verification per feature, not assumed.
  - "Do NOT use the previous harness UI" (§21) would discard our working, tested editor UI for an unfamiliar codebase we haven't run.

## 4. Options

### A. Full fork (HARNESS.MD literal path)
Replace our tree with their repo as baseline; re-skin to AVID; rebuild AI Director against TSX/dapi; keep MPL compliance.
*Cost:* discards the entire verified engine; months of rework to re-reach current capability; team must learn SolidJS/ECS/WebCodecs; bundle/platform story narrows; YC-startup dependency risk (490 commits since July = fast-moving target).
*Gains:* professional canvas+timeline+reconciler day one; proven agent control surface (dapi/MCP/skills); community momentum.

### B. Continue current stack (recommended)
Keep Tauri+React+Rust+FFmpeg+MIT; execute the existing PLAN/PROGRESS (diagrams burn-in, drag/snap polish, template browser, E2E). Adopt *patterns* from Diffusion (dapi-style tool catalog discipline, skills packaging, agent-chat UX) without importing code.
*Cost:* we build canvas richness ourselves (already doing: playhead/drag/zoom/ruler landed).
*Gains:* everything verified stays; no license contamination; wider platform matrix; no watermark economics; BYOM preserved.

### C. Time-boxed spike, then decide (recommended next step)
Before either path: install their arm64 `.dmg`, run `dapi media probe/transcribe` against OUR fixtures, click their timeline, test agent edit end-to-end, read `packages/dapi` + one timeline package source. 1–2 days, produces a real inherit/extend/replace/build table. **This is what HARNESS.MD §32 actually orders.**

## 5. Decision required (maintainer gate — do not proceed without it)

1. Fork (A), continue (B), or spike-first (C — recommended)?
2. If fork: accept MPL-2.0 file copyleft + brand exclusion + watermark/key economics? (LEGAL sign-off)
3. If fork: Electron + SolidJS as the permanent stack (Tauri/Rust retired)?
4. Scope confirmation: desktop/harness only (landing site stays parked)?

## 6. No-regret moves either way (do regardless of the decision)

- Commit HARNESS.MD + MASTER-DESIGN.md into the repo record (done this pass).
- Keep ADRs current; any fork decision gets ADR-013 with the table above.
- dapi's tool-catalog discipline (one zod-validated definition served to app/CLI/renderer) is worth mirroring in our `ai-protocol` + commands regardless.
- The spike in (C) is cheap and de-risks both paths — highest value per hour on the table.
