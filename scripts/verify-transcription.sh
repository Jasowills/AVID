#!/usr/bin/env bash
# End-to-end transcription verification (Phase 5 approach check).
# say (synthetic speech) → ffmpeg 16kHz mono → whisper.cpp tiny.en →
# assert keywords + sane timings + ordered word segments.
# Model lives in ~/.cache/avid/models (downloaded once, never committed).
set -euo pipefail
MODEL="${AVID_WHISPER_MODEL:-$HOME/.cache/avid/models/ggml-tiny.en.bin}"
WORK="${AVID_VERIFY_WORK:-/tmp/avid-verify}"
mkdir -p "${WORK}"

if [[ ! -f "${MODEL}" ]]; then
  echo "downloading ggml-tiny.en.bin (~75MB)…"
  curl -sL -o "${MODEL}" https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-tiny.en.bin
fi
command -v whisper-cli >/dev/null || { echo "need whisper-cli (brew install whisper-cpp)"; exit 1; }

SENTENCE="Kafka has three partitions, and each consumer group shares the work."
say -o "${WORK}/speech.aiff" "${SENTENCE}"
ffmpeg -y -v error -i "${WORK}/speech.aiff" -ac 1 -ar 16000 "${WORK}/speech.wav"

whisper-cli -m "${MODEL}" -f "${WORK}/speech.wav" \
  --output-json --output-file "${WORK}/out" 2>/dev/null >/dev/null
whisper-cli -m "${MODEL}" -f "${WORK}/speech.wav" -ml 1 \
  --output-json --output-file "${WORK}/words" 2>/dev/null >/dev/null

python3 - "${WORK}/out.json" "${WORK}/words.json" <<'EOF'
import json, sys
out = json.load(open(sys.argv[1]))["transcription"]
words = json.load(open(sys.argv[2]))["transcription"]
text = " ".join(s["text"] for s in out).lower()
for keyword in ["kafka", "three", "partitions", "consumer"]:
    assert keyword in text, f"missing keyword: {keyword} (got: {text!r})"
starts = [w["offsets"]["from"] for w in words]
assert starts == sorted(starts) and len(words) >= 10, f"word timings disordered: {starts}"
assert words[-1]["offsets"]["to"] > words[0]["offsets"]["from"], "empty span"
print(f"TRANSCRIPTION OK: {len(words)} word-segments, span 0..{words[-1]['offsets']['to']}ms")
print(f"TEXT: {text.strip()}")
EOF
