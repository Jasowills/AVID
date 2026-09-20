#!/usr/bin/env bash
# Generate deterministic media fixtures with ffmpeg itself — no binaries in git.
# Usage: ./scripts/make-fixtures.sh [out-dir]   (default: fixtures/media)
set -euo pipefail
OUT="${1:-fixtures/media}"
mkdir -p "${OUT}"

# 10s 720p testsrc + 440Hz tone — the talking-head stand-in (known duration/codec).
ffmpeg -y -v error \
  -f lavfi -i "testsrc2=size=1280x720:rate=30:duration=10" \
  -f lavfi -i "sine=frequency=440:sample_rate=48000:duration=10" \
  -c:v libx264 -pix_fmt yuv420p -c:a aac -shortest "${OUT}/talkinghead_10s.mp4"

# 5s black + digital silence — VAD / silence-detect negative control.
ffmpeg -y -v error \
  -f lavfi -i "color=size=640x360:rate=30:duration=5:color=black" \
  -f lavfi -i "anullsrc=sample_rate=48000:cl=stereo" \
  -c:v libx264 -pix_fmt yuv420p -c:a aac -shortest "${OUT}/silence_5s.mp4"

# 3s vertical bars — 9:16 reframe fixture.
ffmpeg -y -v error \
  -f lavfi -i "smptebars=size=720x1280:rate=30:duration=3" \
  -c:v libx264 -pix_fmt yuv420p "${OUT}/vertical_3s.mp4"

# Corrupt sample — error-path fixture (first KiB only, undecodable).
head -c 1024 "${OUT}/talkinghead_10s.mp4" > "${OUT}/corrupt.mp4"

echo "fixtures written to ${OUT}:"
ls -la "${OUT}"
