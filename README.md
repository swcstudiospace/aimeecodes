<p align="center">
  <img src="assets/brand/banner.jpg" alt="Aimee Codes — three chevrons as a flock over a void grid" width="100%">
</p>

<p align="center">
  <img src="assets/brand/icon.jpg" alt="Aimee Codes mark" width="96" height="96">
</p>

<h1 align="center"><code>aimee</code></h1>

<p align="center">
  <strong>Aimee Codes</strong> — a CLI agent flock for design engineers and backend engineers.<br>
  Sage researches. Muse plans. Aimee implements and verifies.<br>
  Vibrant ratatui TUI in the terminal. Installable PWA for browser and mobile.
</p>

<p align="center">
  <code>nix run github:swcstudiospace/omegaloops</code>
  &nbsp;·&nbsp;
  <code>aimee</code>
</p>

<p align="center">
  A <a href="https://www.swcstudio.space/">Spectrum Web Co LLC</a> product
  · brand <strong>@swcstudio</strong>
  · GitHub <a href="https://github.com/swcstudiospace">swcstudiospace</a>
</p>

<p align="center">
  <a href="https://github.com/swcstudiospace/omegaloops/actions"><img src="https://img.shields.io/github/actions/workflow/status/swcstudiospace/omegaloops/ci.yml?style=for-the-badge&label=CI" alt="CI"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-Apache%202.0-ff5a7a?style=for-the-badge" alt="Apache 2.0"></a>
  <a href="https://github.com/swcstudiospace/omegaloops"><img src="https://img.shields.io/badge/github-swcstudiospace%2Fomegaloops-181717?style=for-the-badge&logo=github" alt="GitHub"></a>
</p>

---

<p align="center">
  <a href="#quickstart">Quickstart</a> ·
  <a href="#the-flock">The flock</a> ·
  <a href="#three-modes">Three modes</a> ·
  <a href="#command-line">CLI</a> ·
  <a href="#configuration">Config</a> ·
  <a href="#repository-map">Map</a> ·
  <a href="#license">License</a>
</p>

---

## Quickstart

Rust 2024 Cargo workspace. The CLI binary is **`aimee`** (`crates/aimee_main`). Workspace version `0.1.0`. Pinned toolchain Rust `1.97` (MSRV `1.94`).

```bash
# Nix (Linux and macOS: x86_64 and aarch64)
nix run github:swcstudiospace/omegaloops

# From a local checkout
cargo install --path crates/aimee_main --locked --bin aimee

aimee provider login    # interactive provider credentials
aimee                   # start the TUI
aimee setup             # optional: install the ZSH `:` prefix plugin
```

On first run, Aimee walks you through provider login if no credentials are stored. Config lives in `~/.aimee` (see [Configuration](#configuration)). Existing `~/.omega` directories are still picked up until you migrate.

---

## The flock

Three built-in agents. One product loop.

| | Agent | Alias | Role | Writes? |
|---|---|---|---|---|
| **Sage** | `sage` | `:ask` | Research, architecture, reviews | No |
| **Muse** | `muse` | `:plan` | Plans as checkbox files in `plans/` | Plans only |
| **Aimee** | `aimee` | `:act` | Implement, verify, report evidence | Yes |

```zsh
: sage how does the caching layer work?
: muse design a deployment strategy
: aimee implement the plan in plans/2026-08-21-caching-v1.md
```

Custom agents live in `.aimee/agents/` (project) or `~/.aimee/agents/` (global). Built-in definitions are in `crates/aimee_repo/src/agents/`.

**Why this flock**

- **Terminal-native** — ratatui TUI, one-shot CLI, and a ZSH `:` prefix that never leaves your prompt.
- **Multi-provider** — 42 built-in providers including OpenAI, Anthropic, OpenRouter, GitHub Copilot, Vertex, Bedrock, xAI / SuperGrok (`xai_oauth`), Claude Code, Codex, Google AI Studio, Moonshot, and OpenAI-compatible endpoints. `aimee provider list` is the source of truth.
- **Project policy** — `AGENTS.md`, custom agents, skills, and commands travel with the repo.
- **Secure by design** — restricted mode, credentials in `.credentials.json` (not git), no secrets in logs or commits.
- **WEB3-aware** — Anda/KIP hash-chained session pathways, optional ICP durability, wallet-aware PWA shell (spend stays HITL).

---

## Three modes

### Interactive (TUI)

```bash
aimee                              # new interactive session
aimee conversation resume <id>     # resume a saved conversation
aimee --conversation-id <id>       # same: resume by ID
aimee --agent sage                 # start with a specific agent
aimee -C /path/to/project          # start in a specific directory
aimee --sandbox experiment-name    # isolated git worktree + branch
```

### One-shot CLI

```bash
aimee -p "Explain the purpose of src/main.rs"
echo "What does this do?" | aimee
aimee commit                       # AI commit message, then commit
aimee commit --preview             # print the message and exit
aimee suggest "find large log files"
```

### ZSH `:` prefix

Install once with `aimee setup`. Lines that start with `:` are routed to Aimee; everything else is a normal shell command.

```zsh
: refactor the auth module
:commit
:suggest "find large log files"
:conversation
```

<details>
<summary><strong>ZSH plugin reference</strong></summary>

A line starting with `:` is rewritten to an `aimee` invocation before the shell runs it.

```zsh
: <prompt>         # send a prompt to the active agent
:sage <prompt>     # research agent
:muse <prompt>     # planning agent
:aimee <prompt>    # implementer (alias: :act)
:agent <name>      # switch active agent (picker if omitted)
```

**Attach files:** type `@` then Tab to fuzzy-select. Paths are inserted as `@[filename]`.

```zsh
: review this code @[src/auth.rs] @[tests/auth_test.rs]
```

#### Conversations

```zsh
:new                      # fresh conversation (alias: :n)
:conversation             # interactive picker (alias: :c)
:conversation -           # toggle previous (like cd -)
:clone                    # branch a conversation
:rename <name>            # rename current (alias: :rn)
:retry                    # retry last prompt (alias: :r)
:copy                     # last assistant reply to clipboard
:dump                     # export JSON (alias: :d)
:compact                  # compact context
```

#### Git, shell, session

```zsh
:commit                   # AI message + commit
:suggest <description>    # natural language → command in the buffer
:edit                     # compose a multi-line prompt in $EDITOR

# Session-only (reset when the terminal closes)
:model <model-id>              # alias: :m
:reasoning-effort <lvl>        # alias: :re
:agent <id>                    # alias: :a

# Persistent (written to ~/.aimee/.aimee.toml)
:config-model                  # alias: :cm
:config-reload                 # alias: :cr
:info                          # session info (alias: :i)
:login                         # provider login
:supergrok                     # SuperGrok OAuth (`xai_oauth`)
:sync                          # index the cwd for semantic search
```

Indexing talks to the workspace server (`https://api.aimeecodes.dev/` by default). Override with `AIMEE_SERVICES_URL`.

Plugin source: `shell-plugin/`. File tagging (`@` + Tab) uses `aimee select file`. Keyboard shortcuts: `aimee zsh keyboard`. Shell diagnostics: `aimee doctor`.

</details>

---

## Command-line

| Option | Description |
| --- | --- |
| `-p, --prompt <PROMPT>` | One-shot prompt (no TUI) |
| `-e, --event <EVENT>` | Dispatch a workflow event as JSON |
| `--conversation <PATH>` | Execute a conversation from a JSON file |
| `--conversation-id <ID>` | Resume a conversation by ID (alias: `--cid`) |
| `--agent <AGENT>` | Agent ID for this session (alias: `--aid`) |
| `-C, --directory <DIR>` | `chdir` before start |
| `--sandbox <NAME>` | Isolated git worktree + branch |
| `--verbose` | Verbose logs |
| `-h, --help` | Help |
| `-V, --version` | Version (`0.1.0` from the workspace) |

<details>
<summary><strong>Subcommands</strong></summary>

```bash
aimee conversation list
aimee conversation resume <id>
aimee conversation pathway <id> list     # Anda hash-chained checkpoints

aimee commit
aimee commit --preview
aimee suggest "list files by size"

aimee provider login
aimee provider list
aimee list model
aimee list agent
aimee list skill
aimee list tool aimee

aimee config list
aimee config path
aimee config get model
aimee config set model <provider> <model>
aimee config migrate                     # ~/aimee, ~/.omega, or ~/omega → ~/.aimee

aimee workspace sync
aimee workspace query <text> -r "<use case>"

aimee mcp list
aimee mcp import '<json>'

aimee info
aimee doctor
aimee update
aimee setup
aimee banner
aimee select model
aimee agent list
```

</details>

---

## Configuration

### Provider credentials

```bash
aimee provider login
aimee provider logout
aimee provider list
```

Credentials are stored under the config base path as `.credentials.json`. Do not put API keys in git.

There are **42** built-in provider IDs (`ProviderId::built_in_providers()` in `crates/aimee_domain/src/provider.rs`). `aimee provider list` is the source of truth.

### `.aimee.toml`

Primary config file: **`~/.aimee/.aimee.toml`**.

Base-path resolution (`AIMEE_CONFIG` wins, then `OMEGA_CONFIG`):

1. `AIMEE_CONFIG` if set
2. `OMEGA_CONFIG` if set
3. Existing `~/aimee`, `~/.aimee`, `~/omega`, or `~/.omega`
4. Existing Forge-legacy `~/forge` or `~/.forge`
5. Otherwise `~/.aimee`

Defaults are embedded from `crates/aimee_config/.aimee.toml`. The JSON schema is `aimee.schema.json`.

```toml
# ~/.aimee/.aimee.toml  (illustrative — omit keys you do not need)
max_tool_failure_per_turn = 3
max_requests_per_turn = 100
restricted = false
tool_timeout_secs = 300
services_url = "https://api.aimeecodes.dev/"

[reasoning]
enabled = true
effort = "medium"
```

`AIMEE_`-prefixed variables map onto `.aimee.toml` (`AIMEE_` prefix, `__` nested separator). Legacy `OMEGA_` variables are still read.

| Variable | Role |
|---|---|
| `AIMEE_CONFIG` | Config base directory |
| `AIMEE_SERVICES_URL` | Workspace / indexing API (default `https://api.aimeecodes.dev/`) |
| `AIMEE_BIN` | Binary name used by the ZSH plugin (default `aimee`) |
| `AIMEE_LOG` | `tracing` filter (e.g. `aimee=info`) |
| `AIMEE_EDITOR` | Editor for `:edit` / `:config-edit` |

### Skills, agents, and project policy

**`AGENTS.md`** in the project root (or `~/.aimee/AGENTS.md`) is standing policy for every agent.

| Source | Path |
|---|---|
| Project skills | `.aimee/skills/<name>/SKILL.md` |
| Global skills | `<config-base>/skills/<name>/SKILL.md` |
| Project commands | `.aimee/commands/` |
| Project agents | `.aimee/agents/` |

Project-local `.mcp.json` takes precedence over `~/.aimee/.mcp.json`.

---

## WEB3 + PWA

- **CLI + TUI today.** `aimee` is the branded command. `:aimee` / `:muse` / `:sage` switch agents. Config is `~/.aimee`.
- **PWA.** `pwa/` is an installable app shell (theme `#ff5a7a`, service worker, agent chips). Drafts stay on-device until the agent API is wired.

  ```bash
  cd pwa && python3 -m http.server 4173
  ```

- **Anda / KIP.** Optional hash-chained conversation checkpoints (`aimee conversation pathway`). Enable `[anda]` in `.aimee.toml`. `aimee_anda_icp` is the ICP durability backend.
- **Wallet.** PWA wallet login sits beside provider auth. Payments and spend stay HITL.

---

## Repository map

Rust 2024 workspace (`crates/*`). TypeScript eval harness in `benchmarks/`.

| Path | Role |
|---|---|
| `AIMEE.md` | Full application discovery (this tree) |
| `documentation/` | GitBook documentation space (customer + technical docs, synced via `docs.yaml`) |
| `crates/aimee_domain` | Domain types, errors, tool catalog, policies |
| `crates/aimee_app` | Application orchestration, DTOs, tool registry |
| `crates/aimee_services` | Application services (generic over infra) |
| `crates/aimee_infra` | Infrastructure trait impls (fs, http, auth, mcp) |
| `crates/aimee_repo` | Persistence (Diesel, SQLite, proto, agent defs) |
| `crates/aimee_main` | CLI, TUI, zsh integration (`aimee` binary) |
| `crates/aimee_config` | `.aimee.toml` schema and IO |
| `crates/aimee_anda` / `aimee_anda_icp` | Eternal session pathways / ICP durability |
| `crates/aimee_ci` | GitHub workflow generation |
| `templates/` | Agent prompt templates |
| `shell-plugin/` | ZSH plugin |
| `pwa/` | Installable browser / mobile shell |
| `assets/brand/` | Wordmark, flock mark |
| `benchmarks/` | TypeScript eval harness (`tsx`, Node) |
| `plans/` | Muse plans — historical unless cited |

### Development

```bash
cargo fmt
cargo check -p aimee_main
cargo clippy -p aimee_main --all-targets -- -D warnings
cargo insta test --accept -p aimee_main
```

Nix: `nix run github:swcstudiospace/omegaloops` or `nix develop`. Dev Container (VS Code, Codespaces, DevPod / `aimee pod`): `.devcontainer/`. CI sets `RUSTFLAGS=-D warnings`. House rules for agents: `AGENTS.md`.

Do not commit secrets, `.env` files, or `target/`.

---

## License

Copyright 2026 Spectrum Web Co LLC (brand @swcstudio).

Aimee Codes is licensed under the [Apache License, Version 2.0](LICENSE) (`SPDX-License-Identifier: Apache-2.0`).

The copyright holder is **Spectrum Web Co LLC**. Trademarks including Aimee Codes, Omega Loops, Spectrum Web Co, and @swcstudio remain with the company (Apache-2.0 §6).

---

## Spectrum Web Co LLC

Aimee Codes is developed and maintained by **Spectrum Web Co LLC**. Formerly Omega Loops.

| | |
|---|---|
| Legal name | Spectrum Web Co LLC |
| Brand | [@swcstudio](https://github.com/swcstudio) |
| Product | [Aimee Codes](https://github.com/swcstudiospace/omegaloops) — CLI agent flock |
| Command | `aimee` |
| GitHub | [swcstudiospace/omegaloops](https://github.com/swcstudiospace/omegaloops) |
| Studio | [swcstudio.space](https://www.swcstudio.space/) |
| Contact | [ovesheng@spectrumweb.co](mailto:ovesheng@spectrumweb.co) |

```
    ___    ____ __  _______ ______
   /   |  /  _/  |/  / ____/ ____/
  / /| |  / // /|_/ / __/ / __/
 / ___ |_/ // /  / / /___/ /___
/_/  |_/___/_/  /_/_____/_____/
```
