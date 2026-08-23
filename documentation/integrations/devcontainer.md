# Dev Container

A batteries-included development container for working on Aimee Codes itself (or inside it), defined under `.devcontainer/` per the open devcontainer spec.

## What's in the box

| File | Role |
|---|---|
| `devcontainer.json` | Compose-based config, ports, lifecycle hooks |
| `docker-compose.yml` | The `workspace` service |
| `Dockerfile` | Base image with toolchain |
| `features/` | Devcontainer features |
| `initialize.sh`, `on-create.sh`, `post-create.sh`, `post-attach.sh`, `post-start.sh`, `update-content.sh` | Lifecycle hooks, in order |
| `verify.sh` | Environment verification |

## Configuration highlights

* **Name:** "Aimee Codes"; workspace folder `/workspaces/aimeecodes`; runs as user `vscode`.
* **Resources:** 8 CPUs / 16 GB RAM / 64 GB storage recommended (`hostRequirements`).
* **Shell:** `/bin/zsh` — so the ZSH plugin workflow works inside the container too.
* **Forwarded ports:** `4173` (PWA static shell via `python3 -m http.server`) and `8091` (Anda KIP nexus, local default). Auto-forward is silent for everything.
* **Debugging support:** `SYS_PTRACE` capability and `seccomp=unconfined` are enabled — debuggers (lldb) work against Rust binaries in the container.

## Using it

Open the repository in a devcontainer-compatible editor (VS Code / Codex / any spec-compliant client) and reopen in container. Lifecycle scripts run automatically: initialize → create → content update → start → attach.

Verify the environment when in doubt:

```bash
.devcontainer/verify.sh
```

## When to prefer pods

The devcontainer is for developing *with* Aimee in a prepared environment. For running agents against untrusted code or isolated PR work, `aimee pod` provisions purpose-built container workspaces instead — see [Pods and sandboxes](../surfaces/pods.md).

## See also

* [Nix and reproducible installs](nix.md)
* [Installing](../getting-started/install.md)
* [Testing and evals](../operations/testing-evals.md)

<!-- sources: .devcontainer/devcontainer.json, Dockerfile, README.md, flake.nix -->
