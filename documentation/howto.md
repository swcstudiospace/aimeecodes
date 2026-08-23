# How to use

Typical ways humans drive Aimee Codes. Commands are real (`cli.rs`, the ZSH plugin, `sandbox.rs`, `pod.rs`). For the full flag list see [CLI reference](cli.md). For who does what see [The flock](flock.md).

## Surfaces

| Goal | Command |
|---|---|
| Interactive session | `aimee` |
| One prompt, then exit | `aimee -p "…"` or `echo "…" \| aimee` |
| Stay in zsh | `aimee setup`, then `: <prompt>` |
| Resume | `aimee conversation resume <id>` or `aimee --cid <id>` |
| Isolated git worktree | `aimee --sandbox <name>` |
| Isolated DevPod workspace | `aimee --pod <id>` or `aimee pod up …` |

Interactive mode is: no `-p`, no piped stdin, no subcommand (`crates/aimee_main/src/cli.rs:80-86`).

## Research → plan → implement

Use this when the change is large, multi-file, or the design is still open.

### 1. Research (Sage)

Sage is read-only. It maps architecture, traces flow, and cites `path:line`. It does not plan or edit (`crates/aimee_repo/src/agents/sage.md:24-25`).

```bash
aimee --agent sage
```

```zsh
:sage how does conversation resume work in aimee_main?
:ask  how does conversation resume work in aimee_main?   # zsh only
```

In the TUI, switch with `:sage` (not `:ask` — see [The flock](flock.md#aliases-ask-plan-act)).

Sage hands off: Muse if design is open, Aimee if the change is already obvious (`sage.md:39`).

### 2. Plan (Muse)

Muse writes one checkbox plan under `plans/` via the `plan` tool. Filename `{YYYY-MM-DD}-{plan_name}-{version}.md`. It never overwrites; it bumps `version` (`crates/aimee_repo/src/agents/muse.md:34-37`).

```bash
aimee --agent muse
```

```zsh
:muse design a deployment strategy for the workspace indexer
:plan design a deployment strategy for the workspace indexer   # zsh only
```

If you ask Muse to implement, it refuses and hands off to Aimee (`muse.md:37`).

### 3. Implement and verify (Aimee)

Give Aimee the plan path, the verify command, and the boundaries. Aimee applies the plan or dispatches specialists, then verifies on the tree (`crates/aimee_repo/src/agents/aimee.md:35-39`).

```bash
aimee --agent aimee
```

```zsh
:aimee implement the plan in plans/2026-08-21-caching-v1.md
```

In the TUI, `:act` (aliases `:aimee`, `:omega`) switches to Aimee (`crates/aimee_main/src/model.rs:637`). In zsh, use `:aimee` — `:act` is not remapped.

Aimee does not rewrite the plan unless it is wrong or you ask.

## One-shot

No TUI. One prompt, then exit (`crates/aimee_main/src/cli.rs:20-26`, `crates/aimee_main/src/ui.rs:377-392`). Ctrl+C cancels the in-flight request.

```bash
aimee -p "Explain the purpose of src/main.rs"
aimee --agent sage -p "Trace conversation resume"
echo "What does this do?" | aimee
aimee -C /path/to/project -p "List the public API of aimee_domain"
```

Other one-shot verbs:

```bash
aimee commit                       # AI message, then commit
aimee commit --preview             # print the message and exit
aimee suggest "find large log files"
```

`aimee commit` accepts extra words as context (`aimee commit fix typo in readme`) and `--max-diff` (default `100000`, minimum `5000`) (`crates/aimee_main/src/cli.rs:1008-1034`). You can pipe a diff: `git diff | aimee commit --preview`.

## Resume a conversation

Conversations are first-class. `session` is an alias for `conversation` (`crates/aimee_main/src/cli.rs:119`).

```bash
aimee conversation list
aimee conversation resume <id>
aimee --conversation-id <id>       # alias: --cid
aimee conversation show <id>
aimee conversation show <id> --md  # raw markdown
aimee conversation info <id>
aimee conversation stats <id>
aimee conversation compact <id>
aimee conversation retry <id>
aimee conversation clone <id>
aimee conversation rename <id> "new name"
aimee conversation dump <id>       # JSON
aimee conversation dump <id> --html
aimee conversation delete <id>
```

`--conversation <PATH>` executes a conversation from a JSON file (`crates/aimee_main/src/cli.rs:36-38`). That is not the same as `--conversation-id`.

Anda hash-chained checkpoints (chat-only rollback — workspace files are not reverted):

```bash
aimee conversation pathway <id> list
aimee conversation pathway <id> show <seq>
aimee conversation pathway <id> rollback <seq>
```

Enable `[anda]` in `.aimee.toml` first (`AIMEE.md:304-310`).

### Resume from zsh

The plugin keeps `_AIMEE_CONVERSATION_ID` for the life of the shell (`shell-plugin/lib/config.zsh:13`).

```zsh
:conversation             # picker (alias :c)
:conversation <id>
:conversation -           # toggle previous, like cd -
:new                      # clear current (alias :n); :c - still returns
:clone                    # branch a conversation
:rename <name>
:retry
:dump                     # JSON; :dump html for HTML
:compact
:copy                     # last assistant reply to clipboard
```

`:new` with no text only prints the banner. `:new <prompt>` creates an ID and sends the prompt (`shell-plugin/lib/actions/core.zsh:6-31`).

## Sandbox vs pod

They are different isolation mechanisms. Do not mix the names.

### `--sandbox` — git worktree

```bash
aimee --sandbox experiment-name
```

`Sandbox::create` (`crates/aimee_main/src/sandbox.rs:19-117`):

1. Requires a git working tree
2. Resolves the repo root
3. Creates `../<name>` as a worktree
4. Creates branch `<name>` from HEAD if that branch does not exist
5. Reuses the worktree if it already is one

Combined with `-C`: the worktree is created first, then `<dir>` is appended (`crates/aimee_main/src/main.rs:111-115`). This is **not** a container.

### `--pod` / `aimee pod` — DevPod workspace

```bash
aimee --pod experiment-name          # provision, then start the session
aimee pod up --id my-ws .
aimee pod list --porcelain
aimee pod exec my-ws cargo test
aimee pod ssh my-ws
aimee pod stop my-ws
aimee pod delete my-ws
aimee pod ui                         # Aimee-native; does not spawn DevPod
aimee pod doctor
```

`aimee pod` is a rebranded DevPod wrapper (`crates/aimee_main/src/cli.rs:165-176`, `crates/aimee_main/src/pod.rs:1-5`). Binary is `$AIMEE_POD_BIN` or `devpod` on `PATH` (`pod.rs:16-27`). Aliases: `codespace`, `devpod`. Trailing args after each verb are forwarded.

`--pod` on the top-level CLI calls `provision_for_goal` before `UI::init` (`crates/aimee_main/src/main.rs:125-129`). Use a pod for untrusted agent PRs. Use a sandbox for a cheap local branch.

## Custom commands, skills, policy

Project files travel with the repo (`AIMEE.md:231`):

| Kind | Path |
|---|---|
| Policy | `AGENTS.md` (or `~/.aimee/AGENTS.md`); `SOUL.md` if present |
| Skills | `.aimee/skills/<name>/SKILL.md` |
| Commands | `.aimee/commands/` |
| Agents | `.aimee/agents/` |
| MCP | `.mcp.json` (wins over `~/.aimee/.mcp.json`) |

```bash
aimee list skill
aimee cmd list
aimee cmd execute check
aimee mcp list
```

In the TUI: `:skill`, `:help`. In zsh: `:skill`, `:help` (`:help` runs `aimee list command` — `shell-plugin/lib/actions/core.zsh:65-68`).

## File interactions

| Human action | Touches |
|---|---|
| TUI / one-shot | `crates/aimee_main/src/ui.rs` `run_inner` |
| Resume by ID | conversation store in `aimee_repo` |
| `--sandbox` | `git worktree add` via `sandbox.rs` |
| `--pod` / `aimee pod` | `devpod` via `pod.rs` |
| Muse plan | `plans/*.md` via the `plan` tool |
| ZSH session | `_AIMEE_CONVERSATION_ID` in the plugin |

## Best practices

- Research before planning; plan before a large implementation.
- Pass the plan path to Aimee. Do not paste the whole plan unless you must.
- Resume with `--cid` / `:conversation` instead of re-explaining context.
- Prefer `--sandbox` for local experiments; `--pod` for untrusted or containerized work.
- Keep `AGENTS.md` in the project so every agent sees the same policy.

## Anti-patterns

- Asking Sage to implement or Muse to edit product code.
- Nesting Sage / Muse / Aimee inside each other.
- Using `--sandbox` and expecting Docker isolation.
- Using `--conversation <file>` when you meant `--conversation-id`.
- Putting secrets in prompts, logs, or `plans/`.
- Treating `plans/` as standing policy unless you cited that file.

## Related

- [The flock](flock.md)
- [CLI reference](cli.md)
- [ZSH plugin](zsh.md)
- [TUI](surfaces/tui.md)
- [Pods and sandboxes](ops/pod.md)
- [Configuration](configuration.md)
