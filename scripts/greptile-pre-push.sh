#!/usr/bin/env bash
# Greptile CLI review before git push. Fail closed if the CLI is installed;
# skip with a warning if it is not.
set -euo pipefail
if ! command -v greptile >/dev/null 2>&1; then
  echo "greptile CLI not on PATH — skipping pre-push review" >&2
  exit 0
fi
greptile review
