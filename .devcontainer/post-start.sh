#!/usr/bin/env bash
# Runs on every container start (create and resume). No secrets.
set -euo pipefail

bash .devcontainer/verify.sh
