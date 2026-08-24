#!/usr/bin/env bash
# Refresh cached/prebuilt content when the source tree changes.
# Invoked in parallel as `cargo` and `npm` (see devcontainer.json).
# No user-scoped secrets. Fail closed.
set -euo pipefail

target="${1:-all}"

fetch_cargo() {
  if [[ -f Cargo.lock ]]; then
    cargo fetch --locked
  else
    echo "update-content: Cargo.lock missing" >&2
    exit 1
  fi
}

fetch_npm() {
  if [[ -f package-lock.json ]]; then
    npm ci --ignore-scripts
  else
    echo "update-content: package-lock.json missing" >&2
    exit 1
  fi
}

case "${target}" in
  cargo) fetch_cargo ;;
  npm) fetch_npm ;;
  all)
    fetch_cargo
    fetch_npm
    ;;
  *)
    echo "usage: update-content.sh [cargo|npm|all]" >&2
    exit 1
    ;;
esac
