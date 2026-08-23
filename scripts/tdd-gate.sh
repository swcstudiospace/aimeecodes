#!/usr/bin/env bash
# TDD gate: tests must pass; optional 95% line coverage when llvm-cov is present.
set -euo pipefail
cd "$(dirname "$0")/.."
pkg="${1:-aimee_domain}"
CARGO_TERM_COLOR=never cargo test -p "$pkg" --offline --lib
if command -v cargo-llvm-cov >/dev/null 2>&1 || cargo llvm-cov --version >/dev/null 2>&1; then
  cargo llvm-cov -p "$pkg" --lib --fail-under-lines 95
else
  echo "cargo-llvm-cov not installed — tests passed; coverage gate skipped" >&2
fi
