#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)

# Fail if LargeUtf8 appears outside goni-schema.
if command -v rg >/dev/null 2>&1; then
  if rg "LargeUtf8" "$ROOT_DIR/software/kernel" --glob "!goni-schema/**"; then
    echo "TXT lint failed: LargeUtf8 outside goni-schema" >&2
    exit 1
  fi
else
  if grep -R "LargeUtf8" "$ROOT_DIR/software/kernel" | grep -v "goni-schema/"; then
    echo "TXT lint failed: LargeUtf8 outside goni-schema" >&2
    exit 1
  fi
fi
