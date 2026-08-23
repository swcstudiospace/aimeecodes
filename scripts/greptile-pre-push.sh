#!/usr/bin/env bash
# Greptile CLI review before git push. Fail closed if the CLI is installed;
# skip with a warning if it is not.
set -euo pipefail

if [[ -f "${HOME}/.config/aimee/secrets.env" ]]; then
  set -a
  # shellcheck disable=SC1091
  source "${HOME}/.config/aimee/secrets.env"
  set +a
fi

if ! command -v greptile >/dev/null 2>&1; then
  echo "greptile CLI not on PATH — skipping pre-push review" >&2
  exit 0
fi

if [[ -z "${GREPTILE_API_KEY:-}" ]]; then
  echo "GREPTILE_API_KEY unset — skipping pre-push review" >&2
  exit 0
fi

greptile review
