# Fixtures (deterministic test assets only)

- `fixtures/media/` — tiny generated samples (talking head, silence, multi-speaker, 4K, vertical, audio-only, image, corrupted, unusual codec). Generate, don't commit large binaries. Phase 2.
- `fixtures/projects/` — minimal versioned projects for migration tests. Phase 3.
- `fixtures/ai/prompts/` — eval pairs (`remove_pause`, `create_short`, `explain_kafka`, `add_captions`, `shorten_video`, `create_diagram`). Phase 7.

No copyrighted or large media in git. See `.gitignore`.
