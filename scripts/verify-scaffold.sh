#!/usr/bin/env bash
# AVID scaffold verifier — no toolchain required.
# Fails on any missing contract file/dir so scaffold regressions are loud.
set -euo pipefail
ROOT="$(cd "$(dirname "${0}")/.." && pwd)"
fail() { echo "SCAFFOLD FAIL: ${1}"; exit 1; }
ok() { echo "ok: ${1}"; }

required_files=(
  "AGENTS.md" "README.md" "PLAN.md" "PROGRESS.md" "ROADMAP.md"
  "CONTRIBUTING.md" "DEVELOPMENT.md" "LICENSE"
  "package.json" "Cargo.toml" ".gitignore" ".editorconfig"
  "apps/desktop/package.json" "apps/desktop/src-tauri/tauri.conf.json"
  "docs/ARCHITECTURE.md" "docs/LEGAL_AND_LICENSING.md"
)
required_dirs=(
  "apps/desktop/src" "apps/desktop/src-tauri"
  "crates/avid-core/src" "crates/avid-project/src" "crates/avid-timeline/src"
  "crates/avid-media/src" "crates/avid-render/src" "crates/avid-ai/src" "crates/avid-cli/src"
  "packages/ui/src" "packages/design-system/src" "packages/ai-protocol/src"
  "packages/templates/src" "packages/shared-types/src"
  "templates" "examples" "docs/architecture" "docs/research" "docs/decisions" "docs/ux" "docs/testing"
  "fixtures/media" "fixtures/projects" "fixtures/ai/prompts"
  "scripts" "tests" ".opencode/agents" ".opencode/commands" ".github/workflows"
)

for f in "${required_files[@]}"; do
  [[ -f "${ROOT}/${f}" ]] || fail "missing file ${f}"
  ok "${f}"
done
for d in "${required_dirs[@]}"; do
  [[ -d "${ROOT}/${d}" ]] || fail "missing dir ${d}"
  ok "${d}/"
done

# AGENTS.md canonical-name check (case-insensitive macOS safety)
[[ -f "${ROOT}/AGENTS.md" ]] || [[ -f "${ROOT}/AGENTS.MD" ]] || fail "AGENTS.md missing"

echo "SCAFFOLD OK — structure matches PLAN.md Phase 0.1"
