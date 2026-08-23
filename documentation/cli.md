# CLI reference

Complete command reference for `aimee`, grounded in `crates/aimee_main/src/cli.rs`. If a flag is not in that file, it does not exist.

About string: `Aimee Codes — WEB3-native AI coding agent` (`cli.rs:13-18`). Version is `CARGO_PKG_VERSION` (`0.1.0` from the workspace).

## Singular command names

Always use **singular** names for commands and subcommands (`cli.rs:1-2`):

```
aimee provider login     # yes
aimee providers login    # no
```

Several list verbs still accept a plural **alias** (`list agents`, `list models`). The canonical name is singular. The ZSH plugin depends on this shape (`cli.rs:4-6`).

## Top-level flags (`Cli`)

Defined on `Cli` (`cli.rs:19-77`). These apply to the process, not only to interactive mode.

| Flag | Field | What it does |
|---|---|---|
| `-p, --prompt <PROMPT>` | `prompt` | One-shot prompt; no TUI. `allow_hyphen_values`. Content can also be piped. |
| *(internal)* | `piped_input` | Populated when stdin is not a TTY and is non-empty. `#[arg(skip)]` — you cannot pass this flag. Skipped for `aimee select` (`main.rs:93-103`). |
| `--conversation <PATH>` | `conversation` | Execute a conversation from a **JSON file**. |
| `--conversation-id <ID>`, `--cid` | `conversation_id` | Resume / continue an existing conversation by ID. |
| `-C, --directory <DIR>` | `directory` | `chdir` (canonicalize) before start. |
| `--sandbox <NAME>` | `sandbox` | Isolated **git worktree** + branch named `NAME`. |
| `--pod <NAME>` | `pod` | Provision a **DevPod** workspace before the session. Not a worktree. |
| `--verbose` | `verbose` | Verbose logs. Default `false`. |
| `--agent <AGENT>`, `--aid` | `agent` | `AgentId` for this session. |
| `-e, --event <EVENT>` | `event` | Dispatch a workflow event as JSON. |
| `-h, --help` | *(clap)* | Help. |
| `-V, --version` | *(clap)* | Version. |

Interactive mode is `prompt.is_none() && piped_input.is_none() && subcommands.is_none()` (`cli.rs:80-86`). `--sandbox` and `--directory` can be combined: worktree first, then append the directory (`main.rs:111-115`).

There is **no** top-level `--model` or `--provider` flag on `Cli`. Session model/provider overrides from the ZSH plugin are passed as env (`AIMEE_SESSION__MODEL_ID`, `AIMEE_SESSION__PROVIDER_ID`) from `shell-plugin/lib/helpers.zsh:40-42`.

## `TopLevelCommand`

Enum at `cli.rs:89-171`.

| Command | Aliases | Group / args |
|---|---|---|
| `agent` | | `AgentCommandGroup` |
| `zsh` | `extension` | `ZshCommandGroup` |
| `list` | | `ListCommandGroup` |
| `banner` | | no args |
| `info` | | `--conversation-id` / `--cid`, `--porcelain` |
| `config` | | `ConfigCommandGroup` |
| `conversation` | `session` | `ConversationCommandGroup` |
| `commit` | | `CommitCommandGroup` |
| `mcp` | | `McpCommandGroup` |
| `suggest` | | `<prompt>` (`allow_hyphen_values`) |
| `provider` | | `ProviderCommandGroup` |
| `cmd` | `command`, `commands` | `CmdCommandGroup` |
| `workspace` | | `WorkspaceCommandGroup` |
| `data` | | `DataCommandGroup` |
| `vscode` | | `VscodeCommand` |
| `update` | | `UpdateArgs` |
| `setup` | | alias for `zsh setup` |
| `doctor` | | alias for `zsh doctor` |
| `logs` | | `LogsArgs` |
| `pod` | `codespace`, `devpod` | `PodCommandGroup` |
| `select` | | `SelectCommandGroup` |

Porcelain (`--porcelain`) is machine-readable output on the groups that declare it.

### `aimee agent`

`cli.rs:414-431`.

```bash
aimee agent list          # alias: ls
aimee agent list --porcelain
```

Only `List` exists today.

### `aimee zsh` (alias `extension`)

`cli.rs:612-641`.

```bash
aimee zsh plugin          # emit plugin script (eval'd by setup)
aimee zsh theme           # emit rprompt theme
aimee zsh doctor          # shell diagnostics
aimee zsh rprompt         # model + conversation stats for RPROMPT
aimee zsh setup           # write the managed .zshrc block
aimee zsh keyboard        # print ZLE shortcut sheet
aimee zsh format --buffer "<text>"   # wrap paths as @[…]
```

`aimee setup` and `aimee doctor` are top-level aliases for `zsh setup` and `zsh doctor` (`cli.rs:155-160`).

### `aimee list`

`cli.rs:527-610`. Global `--porcelain`.

| Subcommand | Aliases | Flags |
|---|---|---|
| `agent` | `agents` | `--custom` |
| `provider` | `providers` | `--type` / `-t` (repeatable `ProviderType`) |
| `model` | `models` | |
| `command` | `commands` | `--custom` — **hidden** from clap help (`hide = true`) |
| `config` | `configs` | |
| `tool` | `tools` | `<agent>` required |
| `mcp` | `mcps` | |
| `conversation` | `session` | `--parent <ID>` |
| `cmd` | `cmds` | |
| `skill` | `skills` | `--custom` |
| `file` | `files` | hidden files included, `.gitignore` respected, dirs end with `/` |

```bash
aimee list agent
aimee list agent --custom
aimee list provider -t llm
aimee list model
aimee list tool aimee
aimee list skill
aimee list file
aimee list conversation --parent <id>
```

### `aimee banner`

`cli.rs:101-102`. Prints the ratatui splash + command sheet. No flags.

### `aimee info`

`cli.rs:104-113`.

```bash
aimee info
aimee info --cid <id>
aimee info --porcelain
```

### `aimee config`

`cli.rs:747-834`. Global `--porcelain`.

```bash
aimee config list
aimee config path
aimee config migrate
aimee config get model
aimee config get provider
aimee config get commit
aimee config get suggest
aimee config get reasoning-effort
aimee config set model <provider> <model>
aimee config set commit <provider> <model>
aimee config set suggest <provider> <model>
aimee config set reasoning-effort <effort>
```

`effort` is `none | minimal | low | medium | high | xhigh | max` (`crates/aimee_domain/src/agent.rs:77-91`). `migrate` moves `~/aimee`, `~/.omega`, or `~/omega` → `~/.aimee` (`cli.rs:772-773`).

There is no `aimee config edit` at the CLI layer. The TUI / ZSH `:config-edit` opens `$AIMEE_EDITOR` / `$EDITOR` / `nano` on `aimee config path`.

### `aimee conversation` (alias `session`)

`cli.rs:836-966`.

```bash
aimee conversation list [--porcelain]
aimee conversation new
aimee conversation dump <id> [--html]
aimee conversation compact <id>
aimee conversation retry <id>
aimee conversation resume <id>
aimee conversation show <id> [--md]
aimee conversation info <id>
aimee conversation stats <id> [--porcelain]
aimee conversation clone <id> [--porcelain]
aimee conversation delete <id>          # id is a raw String
aimee conversation rename <id> <name>
aimee conversation pathway <id> list [--porcelain]
aimee conversation pathway <id> show <seq>
aimee conversation pathway <id> rollback <seq>
```

Pathway rollback restores **chat context only**, not workspace files (`cli.rs:959-965`).

### `aimee commit`

`cli.rs:1006-1035`. Not a subcommand tree — flags sit on the group.

```bash
aimee commit
aimee commit --preview
aimee commit --max-diff 50000
aimee commit fix typo in readme
git diff | aimee commit --preview
```

| Flag | Default | Notes |
|---|---|---|
| `--preview` | off | Print the message; do not commit |
| `--max-diff <BYTES>` | `100000` | Minimum `5000` |
| `text…` | | Extra context words; no quotes required |
| `diff` | | `#[arg(skip)]` — filled from piped stdin |

### `aimee mcp`

`cli.rs:643-716`. Global `--porcelain`.

```bash
aimee mcp list
aimee mcp import '<json>' [-s local|user]
aimee mcp remove <name> [-s local|user]
aimee mcp show <name>
aimee mcp reload
aimee mcp login <name>
aimee mcp logout <name>          # or "all"
```

`Scope` default is `local` (`cli.rs:718-726`).

### `aimee suggest`

`cli.rs:128-133`.

```bash
aimee suggest "list files by size"
```

`<prompt>` is required and allows leading hyphens.

### `aimee provider`

`cli.rs:968-1004`. Global `--porcelain`.

```bash
aimee provider login                # interactive menu
aimee provider login <provider>
aimee provider logout
aimee provider logout <provider>
aimee provider list
aimee provider list -t llm
```

### `aimee cmd` (aliases `command`, `commands`)

`cli.rs:383-412`. Global `--cid`, `--porcelain`.

```bash
aimee cmd list
aimee cmd list --custom
aimee cmd execute <name> [args…]
```

### `aimee workspace`

`cli.rs:433-525`.

```bash
aimee workspace sync [path=.] [--init]
aimee workspace list [--porcelain]
aimee workspace query <query> -r "<use case>" [path=.] [-l 10] [--top-k N] [--starts-with P] [--ends-with S]
aimee workspace info [path=.]
aimee workspace delete <id…>
aimee workspace status [path=.] [--porcelain]
aimee workspace init [path=.] [-y]
```

`query` **requires** `-r` / `--use-case`. Indexing talks to `services_url` (default `https://api.aimeecodes.dev/`). Override with `AIMEE_SERVICES_URL`.

### `aimee data`

`cli.rs:1037-1059`. JSONL through an LLM with a schema-constrained tool.

```bash
aimee data --input in.jsonl --schema schema.json \
  [--system-prompt sys.hbs] [--user-prompt user.hbs] [--concurrency 10]
```

### `aimee vscode`

`cli.rs:1073-1078`.

```bash
aimee vscode install-extension
```

Only `InstallExtension` exists.

### `aimee update`

`cli.rs:1080-1086`.

```bash
aimee update
aimee update --no-confirm
```

The ZSH plugin calls `aimee update --no-confirm` in the background after prompts (`shell-plugin/lib/helpers.zsh:233-241`).

### `aimee logs`

`cli.rs:1088-1107`. Defaults to the most recent log file.

```bash
aimee logs
aimee logs -n 50
aimee logs --no-follow
aimee logs -l                 # list log files
aimee logs -f /path/to.log    # specific file
```

| Flag | Default |
|---|---|
| `-n, --lines` | `20` |
| `--no-follow` | follow on |
| `-l, --list` | off |
| `-f, --file` | most recent |

### `aimee pod` (aliases `codespace`, `devpod`)

`cli.rs:173-282`. Most verbs forward trailing args to `devpod`. `ui` and `doctor` (plus `External`) are special.

```bash
aimee pod up [devpod-args…]
aimee pod list [--porcelain] [devpod-args…]
aimee pod stop [devpod-args…]
aimee pod delete [devpod-args…]
aimee pod ssh [devpod-args…]          # forwards -L / -R
aimee pod exec <workspace> <cmd…>     # required command
aimee pod status [devpod-args…]
aimee pod logs [devpod-args…]
aimee pod build [devpod-args…]
aimee pod provider [devpod-args…]
aimee pod ide [devpod-args…]
aimee pod context [devpod-args…]
aimee pod machine [devpod-args…]
aimee pod pro [devpod-args…]
aimee pod use [devpod-args…]
aimee pod upgrade [devpod-args…]
aimee pod version [devpod-args…]
aimee pod ui                          # Aimee-native; no DevPod
aimee pod doctor                      # DevPod, Docker, gh, SSH
aimee pod <other-verb> …              # external_subcommand
```

Binary: `$AIMEE_POD_BIN` or `devpod` (`crates/aimee_main/src/pod.rs:16-27`).

### `aimee select`

`cli.rs:285-381`. Interactive nucleo pickers. Print the selection to stdout; print nothing on cancel. Used by the ZSH plugin.

```bash
aimee select model [-q query]                 # line 1: model_id; line 2: provider_id
aimee select agent [-q query]
aimee select provider [-q query] [--configured]
aimee select reasoning-effort [-q query]      # none…max
aimee select command [-q query]
aimee select conversation [-q query] [--parent <id>]
aimee select file [-q query]
```

Stdin is **not** stolen for piped prompts when the subcommand is `select` (`main.rs:93-95`).

## File interactions

| Layer | Path |
|---|---|
| Parse | `crates/aimee_main/src/cli.rs` |
| Dispatch | `crates/aimee_main/src/ui.rs` `handle_subcommands` (`:467`) |
| Process entry | `crates/aimee_main/src/main.rs` |
| Sandbox | `crates/aimee_main/src/sandbox.rs` |
| Pod | `crates/aimee_main/src/pod.rs` |
| ZSH emit | `crates/aimee_main/src/zsh/` |

## Best practices

- Use singular verbs. Reach for `--porcelain` in scripts.
- Pass `--cid` when a one-shot should continue a conversation.
- Prefer `aimee select …` over home-grown fuzzy finders — the plugin already does.
- Treat `--conversation <file>` and `--conversation-id` as different tools.

## Anti-patterns

- Inventing `--model` / `--provider` CLI flags. They are not on `Cli`.
- Documenting `aimee providers` or `aimee sessions` as canonical names.
- Assuming `aimee list command` is hidden from the binary — it is only hidden from clap help (`cli.rs:562`).
- Forwarding secrets on `aimee mcp import` in shell history. Prefer a file you do not commit, then a local scope.

## Related

- [Quickstart](quickstart.md)
- [How to use](howto.md)
- [ZSH plugin](zsh.md)
- [Configuration](configuration.md)
- [Providers](providers.md)
- [Pods and sandboxes](ops/pod.md)
