#!/bin/sh
# Guard against Tailwind v4 content-detection drift: utilities used ONLY in
# packages/ui/src must exist in the built web CSS. Regression test for the
# invisible-primary-button incident (see docs/SCREEN-AUDIT.md defect 1 —
# @source line in apps/desktop/src/index.css must cover packages/ui/src).
set -eu
cd "$(dirname "$0")/.."
npx --workspace=@avid/desktop vite build >/dev/null 2>&1
CSS=$(ls apps/desktop/dist/assets/*.css | head -n 1)
missing=0
for token in bg-avid-action text-avid-action-text bg-avid-action-hover; do
  if ! grep -q "$token" "$CSS"; then
    echo "MISSING utility in web CSS: $token ($CSS)"
    missing=1
  fi
done
if [ "$missing" -ne 0 ]; then
  echo "FAIL: shared-component utilities missing — check @source coverage."
  exit 1
fi
echo "OK: shared-component utilities present in $CSS"
