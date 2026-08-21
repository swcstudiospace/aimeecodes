#!/usr/bin/env bash
# Incremental debug install + non-interactive CLI smoke tests.
#
# Usage:
#   scripts/dev-aimee.sh              # build + install + smoke
#   scripts/dev-aimee.sh build        # cargo build -p aimee_main
#   scripts/dev-aimee.sh install      # build + symlink ~/.local/bin/aimee
#   scripts/dev-aimee.sh smoke        # smoke-test AIMEE_BIN or the debug binary
#
# Never does a release build. Override the binary with AIMEE_BIN=...
# Isolated smoke config lives under a temp AIMEE_CONFIG dir.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN_NAME="aimee"
DEBUG_BIN="${ROOT}/target/debug/${BIN_NAME}"
INSTALL_DIR="${AIMEE_INSTALL_DIR:-${HOME}/.local/bin}"
INSTALLED_BIN="${INSTALL_DIR}/${BIN_NAME}"
CMD="${1:-all}"

BOLD='\033[1m'
GREEN='\033[32m'
RED='\033[31m'
YELLOW='\033[33m'
DIM='\033[2m'
RESET='\033[0m'

PASS=0
FAIL=0

log() { printf '%b\n' "$*"; }
die() { log "${RED}$*${RESET}"; exit 1; }

build() {
  log "${BOLD}Building debug ${BIN_NAME}${RESET}"
  (cd "${ROOT}" && cargo build -p aimee_main)
  [[ -x "${DEBUG_BIN}" ]] || die "debug binary missing: ${DEBUG_BIN}"
  log "${GREEN}ok${RESET} ${DEBUG_BIN} ($(du -h "${DEBUG_BIN}" | awk '{print $1}'))"
}

install_link() {
  [[ -x "${DEBUG_BIN}" ]] || build
  mkdir -p "${INSTALL_DIR}"
  ln -sfn "${DEBUG_BIN}" "${INSTALLED_BIN}"
  log "${GREEN}ok${RESET} ${INSTALLED_BIN} -> ${DEBUG_BIN}"
  if ! command -v "${BIN_NAME}" >/dev/null 2>&1; then
    log "${YELLOW}warn${RESET} ${BIN_NAME} is not on PATH; add ${INSTALL_DIR}"
  fi
}

aimee_bin() {
  if [[ -n "${AIMEE_BIN:-}" ]]; then
    printf '%s\n' "${AIMEE_BIN}"
    return
  fi
  if [[ -x "${DEBUG_BIN}" ]]; then
    printf '%s\n' "${DEBUG_BIN}"
    return
  fi
  if [[ -x "${INSTALLED_BIN}" ]]; then
    printf '%s\n' "${INSTALLED_BIN}"
    return
  fi
  die "no aimee binary. Run: $0 install"
}

run_cmd() {
  local bin="$1"
  shift
  timeout --preserve-status 30s "${bin}" "$@"
}

check() {
  local name="$1"
  shift
  local expect="${EXPECT:-}"
  local out rc
  set +e
  out="$(run_cmd "${AIMEE}" "$@" 2>&1)"
  rc=$?
  set -e
  if [[ "${rc}" -ne 0 ]]; then
    FAIL=$((FAIL + 1))
    log "${RED}FAIL${RESET} ${name} (exit ${rc})"
    printf '%s\n' "${out}" | sed 's/^/    /' | tail -20
    return
  fi
  if [[ -n "${expect}" ]] && ! grep -Fqi -- "${expect}" <<<"${out}"; then
    FAIL=$((FAIL + 1))
    log "${RED}FAIL${RESET} ${name} (missing '${expect}')"
    printf '%s\n' "${out}" | sed 's/^/    /' | tail -20
    return
  fi
  PASS=$((PASS + 1))
  log "${GREEN}PASS${RESET} ${name}"
}

smoke() {
  AIMEE="$(aimee_bin)"
  log "${BOLD}Smoke${RESET} ${DIM}${AIMEE}${RESET}"

  local tmp
  tmp="$(mktemp -d "${TMPDIR:-/tmp}/aimee-smoke.XXXXXX")"
  trap 'rm -rf "${tmp}"' RETURN
  export AIMEE_CONFIG="${tmp}"

  EXPECT="0.1.0" check "version" --version
  EXPECT="Usage: aimee" check "help" --help
  EXPECT="provider" check "help provider" provider --help
  EXPECT="list" check "help list" list --help
  check "banner" banner
  check "info porcelain" info --porcelain
  EXPECT="xai" check "list provider" list provider --porcelain
  check "list agent" list agent --porcelain
  check "list config" list config --porcelain
  check "list mcp" list mcp --porcelain
  check "list conversation" list conversation --porcelain
  check "list cmd" list cmd --porcelain
  check "list skill" list skill --porcelain
  check "workspace list" workspace list --porcelain
  check "conversation list" conversation list --porcelain
  EXPECT="Usage: aimee pod" check "help pod" pod --help
  EXPECT="doctor" check "pod doctor help" pod --help
  check "pod doctor" pod doctor

  log ""
  if [[ "${FAIL}" -gt 0 ]]; then
    die "${FAIL} failed, ${PASS} passed"
  fi
  log "${GREEN}${PASS} passed${RESET}"
}

case "${CMD}" in
  build) build ;;
  install) install_link ;;
  smoke) smoke ;;
  all)
    build
    install_link
    smoke
    ;;
  -h|--help|help)
    sed -n '2,14p' "$0"
    ;;
  *)
    die "unknown command: ${CMD} (build|install|smoke|all)"
    ;;
esac
