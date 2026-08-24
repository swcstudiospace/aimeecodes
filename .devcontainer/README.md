# Aimee Codes Dev Container

Full Development Container Specification for this workspace. Supporting tools: VS Code Dev Containers, GitHub Codespaces, DevPod (`aimee pod`), JetBrains Gateway, and the [devcontainer CLI](https://github.com/devcontainers/cli).

This is the **developer** environment. It is also what `aimee pod up` / DevPod builds for an isolated workspace. It is not a production image and it is not a Kubernetes runtime.

## Layout

| Path | Spec role |
|---|---|
| `devcontainer.json` | Metadata: orchestrator, Features, lifecycle, users, ports, host requirements, editor customizations |
| `docker-compose.yml` | Compose orchestrator. Service `workspace`. Named cache volumes |
| `Dockerfile` | Native build deps (matches `flake.nix` + CI). No workspace COPY |
| `features/aimee-toolchain/` | Local Feature: pinned `protoc` 28.3, fzf, fd, ripgrep, jq |
| `initialize.sh` | Host-side (every create/start) |
| `on-create.sh` | First create / prebuild (no user secrets) |
| `update-content.sh` | `cargo fetch --locked` + `npm ci --ignore-scripts` (parallel) |
| `post-create.sh` | User-assigned create: `cargo install --locked cargo-nextest` (locked by `pod.rs`) |
| `post-start.sh` | Every start: git `safe.directory` + `verify.sh` |
| `post-attach.sh` | Attach banner (does not run `aimee setup`) |
| `verify.sh` | Fail-closed version checks. `--full` requires cargo tools |

## Pins (must match the tree)

| Kind | Value | Source |
|---|---|---|
| Rust pin | `1.97` | `rust-toolchain.toml` |
| MSRV | `1.94` | workspace `rust-version` |
| Edition | `2024` | `Cargo.toml` |
| `APP_VERSION` | `0.1.0-dev` | `flake.nix`, `aimee_main` `build.rs` fallback |
| `protoc` | `28.3` | `Cross.toml` |
| Node | `24` | root `package.json` `@types/node` |
| Base image | `mcr.microsoft.com/devcontainers/base:1-bookworm` (digest-pinned in the Dockerfile) | this Dockerfile |

## What is installed

**Dockerfile:** cmake, nasm, perl, pkg-config, libsqlite3-dev, clang/lldb, Linux X11/Wayland libs from `flake.nix`, python3 (docs `verify-docs.py`), sqlite3 CLI.

**Official Features:** common-utils (zsh), git, github-cli, node 24, rust 1.97 (`default` profile + rust-analyzer/rustfmt/clippy/rust-src), sshd (JetBrains / DevPod).

**Local Feature:** `protoc` 28.3 with SHA-256 verification, fzf, fd, ripgrep, jq.

**post-create (user layer):** `cargo-nextest --locked`, `cargo-insta`, `cargo-llvm-cov`.

**Not installed (on purpose):**

- Docker-in-Docker / docker-outside-of-docker — would expose the host engine to untrusted pod workspaces. Nested `aimee pod` is unsupported.
- Nix — `nix develop` is the host alternative (`documentation/ops/install.md`).
- `dfx`, gcloud, AWS CLI — not required to compile this tree.
- Privileged mode.

## Lifecycle

Order: `initializeCommand` (host) → image/compose build + Features → `onCreateCommand` → `updateContentCommand` → `postCreateCommand` → `postStartCommand` → `postAttachCommand`.

`waitFor` is `updateContentCommand` so the editor can attach while cargo-nextest installs in the background.

## Security

- Non-root `vscode`. `privileged: false`.
- `SYS_PTRACE` + `seccomp=unconfined` only for LLDB (Rust Feature default).
- No bind-mount of `~/.aimee`, `.env`, or Docker socket.
- `npm ci --ignore-scripts`. `protoc` zip is checksummed.
- Do not put API keys in this directory. Codespaces secrets (optional): `OPENROUTER_API_KEY` for evals — never echoed.

## How to open

```bash
# VS Code
# Command Palette → Dev Containers: Reopen in Container

# Dev Container CLI
npx @devcontainers/cli up --workspace-folder .

# DevPod / Aimee (headless)
aimee pod up . --id aimee-dev
aimee pod doctor
```

Minimum host: 8 CPUs, 16 GiB RAM, 64 GiB storage (`hostRequirements`). The workspace is large; Codespaces will pick a matching SKU.

## Verify

Inside the container:

```bash
bash .devcontainer/verify.sh --full
cargo check -p aimee_main
```

From the host (does not start Docker):

```bash
cargo test -p aimee_main --lib -- pod::tests
python3 documentation/scripts/verify-docs.py
```

Do not `cargo build --release` unless the task is a release binary.
