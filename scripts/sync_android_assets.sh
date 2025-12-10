#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SRC="$ROOT_DIR/dist/index.html"
DEST="$ROOT_DIR/android-app/app/src/main/assets/index.html"

if [[ ! -f "$SRC" ]]; then
  echo "Build output not found at $SRC. Run npm run build first." >&2
  exit 1
fi

mkdir -p "$(dirname "$DEST")"
cp "$SRC" "$DEST"
echo "Synced $SRC -> $DEST"
