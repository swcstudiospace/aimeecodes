#!/usr/bin/env bash
# Runs each time a supporting tool attaches. No secrets. Do not run
# `aimee setup` (interactive; mutates ~/.zshrc).
set -euo pipefail

rustc_v="$(rustc --version 2>/dev/null || echo 'rustc unavailable')"
node_v="$(node --version 2>/dev/null || echo 'node unavailable')"
protoc_v="$(protoc --version 2>/dev/null || echo 'protoc unavailable')"

cat <<EOF
Aimee Codes devcontainer
  ${rustc_v}
  node ${node_v}
  ${protoc_v}

Verify (do not cargo build --release):
  cargo fmt
  cargo check -p aimee_main
  cargo clippy -p aimee_main --all-targets -- -D warnings
  cargo insta test --accept -p aimee_main

Provider credentials: \`aimee provider login\` (stored under ~/.aimee, not git).
House rules: AGENTS.md
EOF
