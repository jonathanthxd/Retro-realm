#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DEST="$ROOT/apps/retro-realm/assets"
mkdir -p "$DEST"
command -v ffmpeg >/dev/null 2>&1 || { echo "Se necesita ffmpeg en PATH" >&2; exit 1; }
for sound in menu bosque_nocturno bosque_oscuro jefe; do
  if [ ! -f "$ROOT/$sound.mp3" ]; then
    echo "No existe $ROOT/$sound.mp3" >&2
    exit 1
  fi
  ffmpeg -hide_banner -loglevel error -y -i "$ROOT/$sound.mp3" -vn \
    -c:a libvorbis -q:a 5 "$DEST/$sound.ogg"
  echo "Convertido $sound.ogg"
done
