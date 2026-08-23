# CLI reference

Every command and top-level flag on the `aimee` binary, verified against `aimee --help`. Binary about line: "Aimee Codes — WEB3-native AI coding agent".

## Top-level flags

| Flag | Effect |
|---|---|
| `-p, --prompt <PROMPT>` | One-shot prompt, then exit. Piped input also works: `cat prompt.txt \| aimee` |
| `--conversation <FILE>` | Execute a conversation from a JSON file |
| `--conversation-id <ID>` (`--cid`) | Resume/continue an existing conversation instead of a new one |
| `-C, --directory <DIR>` | Change working directory before the session |
| `--sandbox <NAME>` | Run in an isolated **git worktree** (not a container) |
| `--pod <POD>` | Provision an isolated **container** workspace first (docker/ssh/cloud) |
| `--agent <ID>` (`--aid`) | Agent ID for this session |
| `-e, --event <JSON>` | Dispatch an event to the workflow |
| `--verbose` | Verbose logging |
| `-h, --help` / `-V, --version` | Help (summary with `-h`) / version |

## Commands

```text
aimee [OPTIONS] [COMMAND]
```

| Command | Purpose |
|---|---|
| `agent` | Manage agents |
| `zsh` | Generate shell extension scripts (`setup`, `doctor`, `format`, `keyboard`; `extension` is an alias) |
| `list` | List agents, models, providers, tools, commands, configs, MCP servers, conversations |
| `banner` | Banner + version info |
| `info` | Config, active model, environment status |
| `config` | Get/set/list configuration values |
| `conversation` (`session`) | Conversation lifecycle — see below |
| `commit` | AI-generated commit message; `--preview` to not commit |
| `mcp` | Import/list/show/remove/reload/login/logout MCP servers |
| `suggest` | Suggest shell commands from natural language |
| `provider` | `login` / `logout` / `list` (`--porcelain` for machines) |
| `cmd` | Run or list custom commands (`execute --cid …`) |
| `workspace` | Manage workspaces for semantic search |
| `data` | Process JSONL through LLMs with schema-constrained tools |
| `vscode` | `install-extension` — VS Code integration |
| `update` | Update aimee to latest version |
| `setup` | ZSH integration setup (alias of `zsh setup`) |
| `doctor` | Shell environment diagnostics (alias of `zsh doctor`) |
| `logs` | Stream log output (defaults to newest log file) |
| `pod` (`codespace`, `devpod`) | Container workspaces — see below |
| `select` | Interactive fuzzy item picker |

## conversation subcommands

```text
list · new · dump (JSON/HTML) · compact · retry · resume · show ·
info · stats · clone · delete · rename · pathway
```

## pod subcommands

Core lifecycle: `up` · `list` (--porcelain → JSON) · `stop` · `delete` · `ssh` (-L/-R forwarding) · `connect` (attach TUI via Anda bridge, no SSH) · `exec` · `status` · `logs` · `build`

Platform: `provider` · `ide` · `context` · `machine` · `pro` · `use` · `upgrade` · `ui` · `doctor` · plus external-command passthrough.

## Exit behavior

One-shot runs (`-p`, `--conversation`) execute and exit; interactive sessions persist automatically and can be resumed by ID. Errors are printed to stderr with non-zero exit codes.

## See also

* [Config reference](config.md)
* [Environment variables](env-vars.md)
* [Session management](../usage/sessions.md)

<!-- sources: crates/aimee_main/src/cli.rs, aimee --help output (verified against built binary) -->
