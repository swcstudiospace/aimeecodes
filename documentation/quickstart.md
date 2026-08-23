# Quickstart

Get `aimee` on the path, log a provider in, and start the TUI. Commands on this page exist in the product tree (`aimeecodes/README.md:53-63`, `AIMEE.md:418-431`, `crates/aimee_main/src/cli.rs`).

## What you need

- Linux or macOS, `x86_64` or `aarch64` (Nix flake)
- Or a local checkout with the pinned Rust toolchain (`1.97`; MSRV `1.94`)
- A provider account. There are 42 built-in provider IDs; `aimee provider list` is the source of truth (`AIMEE.md:268`)

## Install

### Nix (no checkout)

```bash
nix run github:swcstudiospace/omegaloops
```

That is the live flake homepage (`AIMEE.md:38-39`). `nix develop` is the local-dev shell (`aimeecodes/README.md:364`).

### From a local checkout

```bash
cargo install --path crates/aimee_main --locked --bin aimee
```

The binary is **`aimee`** from `crates/aimee_main` (`AIMEE.md:20`). Workspace version is `0.1.0`.

Do not run `cargo build --release` unless the task is a release binary (`AIMEE.md:399`).

## Provider login

```bash
aimee provider login
```

No provider name opens an interactive menu (`crates/aimee_main/src/cli.rs:981-987`). Pass a provider ID to skip the picker:

```bash
aimee provider login anthropic
aimee provider list
aimee provider logout
```

Credentials are stored under the config base path as `.credentials.json`. Do not put API keys in git (`aimeecodes/README.md:262`). SuperGrok / SuperGrok Heavy is `xai_oauth` (no API key):

```bash
aimee provider login xai_oauth
```

The product README says that on first run Aimee walks you through provider login if no credentials are stored (`aimeecodes/README.md:65`). Prefer running `aimee provider login` yourself so the first TUI session is not blocked on auth.

Existing `~/.omega` directories are still picked up until you migrate:

```bash
aimee config migrate
```

That moves `~/aimee`, `~/.omega`, or `~/omega` → `~/.aimee` (`crates/aimee_main/src/cli.rs:772-773`).

## First TUI session

```bash
aimee
```

Interactive mode starts when there is no `-p`, no piped stdin, and no subcommand (`crates/aimee_main/src/cli.rs:80-86`). The TUI:

1. Prints the ratatui splash (`crates/aimee_main/src/ui.rs:364-365`)
2. Initializes state, hydrates model/tool/agent caches, and opens a conversation (`crates/aimee_main/src/ui.rs:366-370`)
3. Prompts on a rustyline line. Type a message, or `/` / `:` then Tab for the command palette (`crates/aimee_main/src/editor.rs:76-85`)

Useful first-session flags:

```bash
aimee --agent sage                 # start as Sage (alias: --aid)
aimee -C /path/to/project          # chdir before start
aimee --conversation-id <id>       # resume by ID (alias: --cid)
aimee conversation resume <id>     # same, as a subcommand
```

One-shot instead of the TUI:

```bash
aimee -p "Explain the purpose of src/main.rs"
echo "What does this do?" | aimee
```

Piped stdin is treated as a prompt unless the subcommand is `aimee select` (`crates/aimee_main/src/main.rs:93-103`).

See [TUI](surfaces/tui.md) for keys, theme, and slash commands.

## Optional: ZSH `:` prefix

```bash
aimee setup
```

`aimee setup` is an alias for `aimee zsh setup` (`crates/aimee_main/src/cli.rs:155-157`). It writes a managed block into `.zshrc` that loads the plugin and theme (`shell-plugin/aimee.setup.zsh:1-20`):

```zsh
eval "$(aimee zsh plugin)"
eval "$(aimee zsh theme)"
```

Open a new zsh, then:

```zsh
: sage how does the caching layer work?
:muse design a deployment strategy
:aimee implement the plan in plans/2026-08-21-caching-v1.md
```

Diagnostics: `aimee doctor` (alias for `aimee zsh doctor`). Keyboard sheet: `aimee zsh keyboard`. Full command list: [ZSH plugin](zsh.md).

## Isolated work

```bash
# Git worktree + branch next to the repo (not a container)
aimee --sandbox experiment-name

# DevPod workspace (docker / ssh / cloud) before the session
aimee --pod experiment-name
```

`--sandbox` creates `../<name>` as a git worktree and optional branch (`crates/aimee_main/src/sandbox.rs:19-117`). `--pod` provisions a DevPod workspace (`crates/aimee_main/src/cli.rs:57-60`, `crates/aimee_main/src/main.rs:125-129`). They are not interchangeable. See [How to use](howto.md#sandbox-vs-pod).

## Config you will touch

| What | Where |
|---|---|
| Config file | `~/.aimee/.aimee.toml` |
| Credentials | `<config-base>/.credentials.json` |
| Config env | `AIMEE_CONFIG` (wins over `OMEGA_CONFIG`) |
| Project policy | `AGENTS.md` (or `~/.aimee/AGENTS.md`) |
| Project agents | `.aimee/agents/` |

Base-path resolution (`AIMEE.md:208-216`): `AIMEE_CONFIG` → `OMEGA_CONFIG` → existing `~/aimee`, `~/.aimee`, `~/omega`, `~/.omega` → Forge-legacy `~/forge` / `~/.forge` → otherwise `~/.aimee`.

```bash
aimee config path
aimee config list
aimee info
```

## File interactions

| Path | Role |
|---|---|
| `flake.nix` | Nix package / `nix run` |
| `crates/aimee_main` | Binary you just installed |
| `crates/aimee_config/.aimee.toml` | Embedded defaults |
| `shell-plugin/aimee.setup.zsh` | Block `aimee setup` writes |
| `~/.aimee/.credentials.json` | Provider secrets — not git |

## Best practices

- Log a provider in **before** the first long TUI session.
- Stay in a git repo if you plan to use `--sandbox`.
- After `aimee setup`, open a new shell so `.zshrc` is sourced.
- Run `aimee doctor` if Tab / Option / Alt bindings look wrong.

## Anti-patterns

- Putting API keys in `.aimee.toml` or in the repo. Credentials belong in `.credentials.json`.
- Inventing an install URL other than `nix run github:swcstudiospace/omegaloops`.
- Using `--sandbox` when you need a container. That flag is a worktree.
- Expecting `:act` to work in zsh. The plugin remaps `:ask` → `sage` and `:plan` → `muse` only (`shell-plugin/lib/dispatcher.zsh:125-132`). Use `:aimee` in the shell. `:act` is a TUI command (`crates/aimee_main/src/model.rs:637`).

## Related

- [Overview](README.md)
- [The flock](flock.md)
- [CLI reference](cli.md)
- [ZSH plugin](zsh.md)
- [TUI](surfaces/tui.md)
- [Install and Nix](ops/install.md)
