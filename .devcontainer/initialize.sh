#!/usr/bin/env bash
# Host-side initialization (runs wherever the source tree lives).
# Cloud hosts (Codespaces, Ona) already have an orchestrator; do not fail closed
# if docker is missing on a laptop that will use Codespaces instead.
set -euo pipefail

if command -v docker >/dev/null 2>&1; then
  docker version >/dev/null
elif command -v podman >/dev/null 2>&1; then
  podman version >/dev/null
else
  echo "initialize: docker/podman not on PATH (ok for cloud-hosted creates)" >&2
fi
