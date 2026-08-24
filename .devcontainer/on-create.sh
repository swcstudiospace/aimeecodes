#!/usr/bin/env bash
# First container create, including Codespaces/DevPod prebuild.
# No user-scoped secrets. Fail closed.
set -euo pipefail

mkdir -p /commandhistory
if [[ -w /commandhistory ]]; then
  touch /commandhistory/.zsh_history || true
fi

git config --global --add safe.directory '*' || true

if [[ -f rust-toolchain.toml ]]; then
  rustup show
fi
rustup component add clippy rustfmt rust-src rust-analyzer
