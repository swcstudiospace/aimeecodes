#!/usr/bin/env bash
# First-boot hook for `aimee pod up`. Keep this fail-closed and cheap:
# optional cargo crates must use --locked when the crate requires it
# (cargo-nextest refuses unlocked source installs).
set -euo pipefail

if [[ -f rust-toolchain.toml ]]; then
  rustup show
fi
rustup component add clippy rustfmt

# Required for the repo's nextest/insta workflow. --locked is mandatory for nextest.
cargo install --locked cargo-nextest
cargo install cargo-insta
cargo install cargo-llvm-cov
