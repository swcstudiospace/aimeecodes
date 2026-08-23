# DevPod wrap (Aimee / Omega Loops)

Upstream clone: `/root/src/repos/devpod` (`loft-sh/devpod`). Runtime: `/usr/local/bin/devpod` (not compiled into Rust). CLI crate is `aimee_main`, binary `aimee` (repo dir still `omegaloops`).

## Commands

| User | DevPod |
|------|--------|
| `aimee pod up …` | `devpod up …` |
| `aimee pod list [--porcelain]` | `devpod list` (`--output json` if porcelain) |
| `aimee pod exec <id> <cmd>` | `devpod ssh <id> --command "<cmd>"` |
| `aimee pod ui` / `doctor` | Omega-native; do not spawn DevPod |
| `aimee --pod <id>` | `devpod up <cwd> --id <id> --open-ide=false` then TUI |
| `/goal <text>` | persist `pod_id` slug `omega-…` (max 40) |
| `/goal pod` | `provision_for_goal` |
| `/goal exec <cmd>` | `exec_in_workspace` |
| `/goal pr` | `gh pr create --fill` |

`--sandbox` remains git worktree. Docker is the default provider on this host once `devpod provider add docker` has run.

## Headless → Mac Mini

No DevPod Desktop on this Linux box. `aimee pod ui` prints SSH target (`user@host`). Mini: DevPod Desktop → Providers → SSH, or `ssh -N -L 8080:127.0.0.1:8080 user@host`.

## Not wired

Anda **dTEE** is not in this tree. Do not claim `/goal` auto-opens a PR or auto-`up`s a workspace; those are explicit `/goal pr` and `/goal pod`.
