# Whisper / local transcription research (Phase 0.3)

> Researched 2026-09-20. Primary: `github.com/openai/whisper`, `github.com/ggml-org/whisper.cpp`, `github.com/SYSTRAN/faster-whisper`.
> Decision: local-first via whisper.cpp; see `docs/decisions/ADR-007-local-first-ai.md`.

## Options compared (all offline-capable)

| | `openai/whisper` (PyTorch ref, MIT) | `whisper.cpp` (ggml-org, MIT) | `faster-whisper` (SYSTRAN, MIT, CTranslate2) | `WhisperX` (+ distil-whisper, BSD-2) | Parakeet/NeMo (NVIDIA, mixed) |
|---|---|---|---|---|---|
| Quality | Baseline. tiny 39M → large 1550M; turbo 809M ~8× large. | Identical weights (± quant noise). q5_0/q8_0 safe; q4 degrades noisy audio. | Identical; int8 default (`large-v2` 13 min in 59 s/2.9 GB vs ref 2m23s/4.7 GB on RTX 3070 Ti). | Same ASR + distil-large-v3 ~6× faster (EN-only, slight accent loss). Best WER/speed for batch. | Parakeet-TDT 0.6B beats Whisper on clean EN, worse multilingual (Whisper's 99 langs win). |
| Word timestamps | Segment only (~1 s drift). | Experimental `-ml 1` per-word + `--print-colors` confidence. Usable, jittery. | `word_timestamps=True` (needs non-batched). | **Best:** wav2vec2 forced alignment <100 ms + VAD. Required for karaoke captions. | None natively; external aligner needed. |
| Confidence/lang/speakers | `language_prob`, no confidence, no diarization. | Token probs → confidence; lang detect; `tinydiarize` → `[SPEAKER_TURN]` only (no IDs). Silero-VAD built-in. | lang+prob, per-word prob, Silero-VAD. No diarization. | Word conf + lang + `pyannote.audio` speaker IDs (HF token, torch, ~2 GB). | Strong diarizer, heavy. |
| Size/RAM | ~1 GB tiny/base → 10 GB large VRAM. | tiny 75 MB/273 MB → large 2.9 GB/3.9 GB. ANE CoreML >3× encoder, Metal 10×RT (turbo 60 s→6 s on M2 Pro). | large int8 2.9 GB VRAM; CPU small int8 1.7 GB. CUDA12+cuBLAS+cuDNN9; CPU-only on Mac. | + align (~100 MB/lang) + diarizer (~1 GB). GPU-only practical. | 0.6–1.1B, CUDA-fast, weak CPU/Metal. |
| Rust bindability | Poor: Python + torch (~2 GB). Sidecar only. | **Best:** C API `whisper.h`, `whisper-rs`, single binary, iOS/Android/WASM/Pi, CUDA/Vulkan/OpenVINO. | Poor: Python lib; CTranslate2 C++ FFI or Python sidecar + DLL hell on Win. | Worst for desktop: Python + torch + HF token. Server-side only. | ONNX runtime custom; immature. |
| License | MIT (code+weights). | MIT. | MIT (PyAV bundles FFmpeg libs — LGPL notice check). | BSD-2 code; pyannote models gated — cannot bundle blindly. | Code Apache-2; weights often NC/research — legal review required. |

## Getting AVID's required fields (AGENTS §20)

- **Word timestamps (mandatory):** `whisper.cpp -ml 1` suffices for MVP transcript→timeline sync (sentence delete → `remove_range`). Upgrade to WhisperX alignment as optional power pass for broadcast-accurate captions.
- **Confidence:** token-`p` average → `High / Medium / Needs review` (never fake `97.234%`).
- **Language:** `info.language + probability` (all variants, 99 langs) → `project.json.transcript.language`.
- **Speakers:** MVP = `tinydiarize` turn markers + manual Speaker 1/2 relabel (zero deps, offline). Full diarization = `pyannote.audio` via optional sidecar, explicit opt-in (HF token — disclose).

## Recommendation (MVP + fallback chain)

**`whisper.cpp` via `whisper-rs` as in-app Rust background job.** Default `base.en` (142 MB, fast CPU) with auto-download `small` / `large-v3-turbo` (best Apple Silicon speed/accuracy). Flow: `extractAudio 16 kHz mono WAV (ffmpeg) → Silero-VAD split → whisper.cpp ggml → JSON {segments[], words[], lang, conf} → transcript panel`. Fully offline, MIT, Metal/CoreML/ANE on Apple Silicon, AVX/NEON/Vulkan/CUDA elsewhere, zero Python, crash-isolated, cancellable.

Fallbacks: (a) slow CPU / NVIDIA batch → optional `faster-whisper` Python sidecar (int8, `distil-large-v3` EN), same schema; (b) word accuracy below threshold → optional WhisperX align pass (GPU/Docker); (c) no local model / unsupported lang → explicit cloud STT opt-in dialog (`Cloud: Provider`, never silent). Persist `transcript.provider: local-whispercpp | local-faster | cloud` for trust UX.

Reject for MVP core: `openai-whisper` Python (slowest/heaviest), Parakeet (EN-only, license/Rust risk), bare pyannote (heavy, token-gated).
