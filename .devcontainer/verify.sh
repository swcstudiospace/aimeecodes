#!/usr/bin/env bash
# Image/Feature health check. Prints versions only — never env files or tokens.
# Default: tools baked into the image/Features.
# --full: also require cargo-nextest / cargo-insta / cargo-llvm-cov (post-create).
set -euo pipefail

full=0
if [[ "${1:-}" == "--full" ]]; then
  full=1
fi

need() {
  local bin="$1"
  if ! command -v "${bin}" >/dev/null 2>&1; then
    echo "missing required binary: ${bin}" >&2
    exit 1
  fi
}

need rustc
need cargo
need rustfmt
need clippy
need protoc
need node
need npm
need python3
need cmake
need nasm
need perl
need pkg-config
need git
need gh
need zsh
need sqlite3
need jq

rustc --version
cargo --version
protoc --version
node --version
python3 --version
git --version
gh --version | head -n 1

if ! rustc --version | grep -q '1\.97'; then
  echo "expected rustc 1.97 (rust-toolchain.toml pin)" >&2
  exit 1
fi

if ! node --version | grep -q '^v24'; then
  echo "expected node 24 (package.json @types/node)" >&2
  exit 1
fi

if ! protoc --version | grep -Eq '3\.28\.3|28\.3'; then
  echo "expected protoc 28.3 (Cross.toml pin)" >&2
  exit 1
fi

if [[ "${full}" -eq 1 ]]; then
  need cargo-nextest
  need cargo-insta
  need cargo-llvm-cov
  cargo nextest --version
  cargo insta --version
  cargo llvm-cov --version
fi

echo "devcontainer verify: ok"
