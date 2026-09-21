# Legal & licensing register (Phase 0.8 — living document)

> Rule: every dependency, codec, font, icon, template asset, model, and weight gets a row here BEFORE integration (AGENTS §89, §160). No dependency merges without a row. CI gates on this table from Phase 1.

| Dependency / Asset | Version pin | License | Usage | Redistribution | Commercial notes | Status |
|---|---|---|---|---|---|---|
| Tauri 2.x + plugins | `2.11.x` lockstep (pin in Phase 1) | MIT/Apache-2.0 | Bundled framework | OK, attribution | — | ✅ cleared |
| React / TypeScript / Vite / Zustand / Tailwind | Pin in Phase 1 | MIT | Bundled UI | OK | — | ✅ cleared |
| FFmpeg sidecar builds (LGPL, no `--enable-gpl/--enable-nonfree`) | Pin major (e.g. 7.x) in Phase 2 | LGPL-2.1+ | External process (`externalBin`) | OK unmodified: attribution + source offer + link ffmpeg.org/legal | `libx264/x265` are GPL — either exclude from distributed build (native/HW encoders) or ship GPL-compliant (source + notice + checklist). Never `--enable-nonfree` (FDK-AAC/OpenSSL unredistributable). H.264/AAC patents ≠ copyright license — rely on system/HW or documented sidecar, track exposure. | 🚧 strategy set, builds TBD |
| whisper.cpp 1.9.1 + ggml-tiny.en.bin (77 MB, verified) via `whisper-rs` 0.16 / `hound` | Lockfile pinned; model: huggingface.co/ggerganov/whisper.cpp `ggml-tiny.en.bin` | MIT (code + weights, permissive) | Bundled/in-process; model user-downloaded to app cache | OK; record model URL in code (`DEFAULT_SPEECH_MODEL_URL`) | — | ✅ integrated |
| `ureq` 3.x + `tokio` 1.x + `uuid` 1.x + `http` 1.x (Rust infra) | Lockfile pinned | MIT/Apache-2.0 | Bundled libs | OK | — | ✅ cleared |
| `lucide-react` (UI icons) | Pinned at install (0.5xx, ISC + MIT-Feather subset) | ISC (primary) + MIT (Feather-derived subset); free commercial/personal | Bundled, tree-shaken imports | No brand logos in set (matches provider-badge rule) — custom text badges for providers | ✅ cleared |
| Tauri plugins: opener + JS `@tauri-apps/api/cli/plugin-opener` 2.x | Lockfile pinned | MIT/Apache-2.0 | Bundled | OK, attribution | opener used only for reveal-in-folder | ✅ cleared |
| faster-whisper (optional sidecar) | Pin in Phase 5 if adopted | MIT (PyAV bundles FFmpeg libs — LGPL notice check) | Optional external process | OK with notice | CUDA DLL burden on Win | ⬜ optional |
| pyannote.audio (optional full diarization) | — | Models gated (HF token) | Optional sidecar, opt-in only | **Cannot bundle blindly** — disclose token + download | — | ⛔ not in MVP core |
| Ollama | Connect only | MIT | API-only (`localhost:11434`) | No redistribution (user-installed) | — | ✅ cleared |
| LM Studio | Connect only | Proprietary freeware | API-only (`localhost:1234/v1`) | No redistribution | Closed source — adapter only | ✅ cleared (adapter-only) |
| vLLM (reference) | — | Apache-2.0 | Reference/self-hosted | OK | GPU-first; not MVP desktop target | ✅ cleared |
| `qwen3:8b/4b` weights | Pin tag in Phase 6 | Apache-2.0 | Via Ollama (user-downloaded) | Redistribution OK with notice; AVID does not bundle | — | ✅ cleared |
| `llama3.1:8b` weights | Pin tag in Phase 6 | Llama Community License (NOT OSI) | Via Ollama (user-downloaded) | Redistributable with use/output restrictions — track | Review commercial terms before recommending | 🚧 track |
| OpenAI API (MVP cloud) | Adapter pin in Phase 6 | Commercial ToS | API-only, explicit consent | N/A (no redistribution) | Show per-call cost pre-flight; usage metering | 🚧 ToS review in Phase 6 |
| wavesurfer.js v7 (waveforms) | Pin in Phase 2/3 | BSD-3-Clause | Bundled | OK | — | ✅ cleared |
| Mermaid (AI diagram authoring → Scene Spec) | Pin + lazy-load in Phase 8 | MIT | Bundled (lazy) | OK | Bundle size — lazy-load | ✅ cleared |
| React Flow `@xyflow/react` (diagram editor) | Pin in Phase 8 | MIT | Bundled | OK | Moderate bundle, justified | ✅ cleared |
| tldraw | — | Custom SDK license (NOT OSI; watermark + paid removal) | — | **Do not embed in MVP** | Re-evaluate post-MVP as optional plugin with legal sign-off | ⛔ rejected MVP |
| Excalidraw | — | MIT | — | OK but wrong visual language | Rejected on product grounds | ⬜ not used |
| Remotion | — | Code MIT; bundled FFmpeg binary GPLv2+ | Selective preview-pipeline use only | GPL binary — do not bundle blindly | Evaluate if adopted | 🚧 evaluate if used |
| Fonts / icons / template assets | Each asset pinned | Per-asset (OFL/Apache/CC0 preferred) | Bundled | Verify per asset | No copyrighted logos fetched from web (AGENTS §71) | ⬜ per-asset rows in Phase 9 |
| Parakeet/NeMo weights | — | Often NC/research | — | Legal review required before any use | Not MVP | ⛔ not in MVP |

## Standing rules

1. Code license ≠ weights/patents. Track all three columns.
2. Codec rule: copyright compliance (LGPL/GPL) AND patent exposure (H.264/H.265/AAC) reviewed separately.
3. `tldraw`, bundled GPL FFmpeg, gated models, NC weights: blocked without explicit legal sign-off + ADR amendment.
4. Fonts/icons/templates: OFL/Apache-2.0/CC0 preferred; record every asset.
