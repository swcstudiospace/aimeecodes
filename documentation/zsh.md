# ZSH plugin

The ZSH plugin rewrites lines that start with `:` into `aimee` invocations and never leaves your prompt. Source of truth is `shell-plugin/`, not the product README. Where they disagree, this page follows the plugin.

Install once:

```bash
aimee setup
```

That is an alias for `aimee zsh setup` (`crates/aimee_main/src/cli.rs:155-157`). It writes a managed block into `.zshrc` (`shell-plugin/aimee.setup.zsh:1-20`):

```zsh
# !! Contents within this block are managed by 'aimee zsh setup' !!
plugins+=(zsh-autosuggestions)          # if missing
plugins+=(zsh-syntax-highlighting)      # if missing
eval "$(aimee zsh plugin)"              # unless $_AIMEE_PLUGIN_LOADED
eval "$(aimee zsh theme)"               # unless $_AIMEE_THEME_LOADED
```

Do not edit that block by hand — `aimee setup` overwrites it.

Entry point: `shell-plugin/aimee.plugin.zsh` sources config, highlight, helpers, context, completion, every `lib/actions/*.zsh`, the dispatcher, then bindings.

## How `:` rewrite works

`aimee-accept-line` (`shell-plugin/lib/dispatcher.zsh:90-281`) binds Enter (`^M`) and `^J` (`lib/bindings.zsh:42-43`).

| Buffer | Result |
|---|---|
| `:foo` / `:foo bar baz` | `user_action=foo`, optional `input_text` (`dispatcher.zsh:99-107`) |
| `: something` | default action, `input_text=something` (`:108-111`) |
| anything else | normal `zle accept-line` (`:113-115`) |

The original line is pushed to zsh history **before** transformation (`:119`).

### Alias remap (plugin only)

```zsh
# dispatcher.zsh:125-132
:ask  → sage
:plan → muse
```

`:act` is **not** remapped. In zsh, implement with `:aimee` or `:agent aimee`. The product README lists `:act` as a zsh alias (`aimeecodes/README.md:140`); the plugin does not implement that. `:act` exists in the TUI (`crates/aimee_main/src/model.rs:637`).

Unknown actions go to `_aimee_action_default` (`dispatcher.zsh:10-88`, `dispatcher.zsh:268-270`). The name must appear in `aimee list commands --porcelain` (`lib/helpers.zsh:7-11`) or the plugin prints `Command '<name>' not found` (`dispatcher.zsh:21-24`):

1. If the type is **custom** → `aimee cmd execute --cid …` (`dispatcher.zsh:31-46`)
2. If there is no text and the type is **agent** → set `_AIMEE_ACTIVE_AGENT` (`dispatcher.zsh:51-64`)
3. If there is text → set `_AIMEE_ACTIVE_AGENT` to that name when one was given, then `aimee --agent <active> -p "<text>" --cid <id>` (`dispatcher.zsh:67-82`, `lib/helpers.zsh:52-77`)
4. Otherwise → `Command '<name>' not found` (`dispatcher.zsh:54-57`)

Conversation IDs are created with `aimee conversation new` when missing (`lib/actions/core.zsh:18`, `dispatcher.zsh:68-71`).

## `@` file tagging

Type `@` then Tab. The completion widget (`lib/completion.zsh:5-24`) strips `@`, runs `aimee select file`, and inserts `@[path]`.

```zsh
: review this code @[src/auth.rs] @[tests/auth_test.rs]
```

Paste of paths onto a `:` line is wrapped the same way via `aimee zsh format --buffer` (`lib/bindings.zsh:14-27`). Non-`:` lines are left alone so `vim /some/path` is not mangled.

Highlight (`lib/highlight.zsh:8-14`):

- `@[…]` → cyan bold
- `:<command>` → yellow bold
- rest of a `:` line → white

The plugin README says tagged files are **green** bold (`shell-plugin/README.md:204`). The highlighter is **cyan** bold. Prefer the plugin source.

Tab on `:partial` opens `aimee select command` (`lib/completion.zsh:27-43`).

## Commands the plugin actually implements

Dispatch table: `shell-plugin/lib/dispatcher.zsh:147-270`. Every row below is a `case` arm or the default agent/custom path.

### Conversations

| Command | Aliases | Handler | What it runs |
|---|---|---|---|
| `:new [prompt]` | `:n` | `_aimee_action_new` | Clears ID, sets agent `aimee`. No text → `aimee banner`. With text → `conversation new` + `-p`. |
| `:conversation [id\|-]` | `:c` | `_aimee_action_conversation` | Picker, or switch to `id`, or toggle previous (`-`, like `cd -`). Shows `conversation show` + `info`. |
| `:conversation-tree` | `:ct` | `_aimee_action_conversation_tree` | `aimee select conversation --parent $ID` |
| `:clone [id]` | | `_aimee_action_clone` | `aimee conversation clone`, then switch |
| `:rename <name>` | `:rn` | `_aimee_action_rename` | `aimee conversation rename $ID <name>` (requires active ID) |
| `:conversation-rename [id name]` | | `_aimee_action_conversation_rename` | Picker + prompt, or `id` + `name` |
| `:retry` | `:r` | `_aimee_action_retry` | `aimee conversation retry $ID` |
| `:dump [html]` | `:d` | `_aimee_action_dump` | `aimee conversation dump $ID` [`--html`] |
| `:compact` | | `_aimee_action_compact` | `aimee conversation compact $ID` |
| `:copy` | | `_aimee_action_copy` | `conversation show --md`, then `pbcopy` / `xclip` / `xsel` |

Conversation commands that need an ID error with *No active conversation* (`lib/actions/core.zsh:78-80`).

### Git / editor / suggest

| Command | Aliases | What it runs |
|---|---|---|
| `:commit [context]` | | `aimee commit --max-diff $AIMEE_MAX_COMMIT_DIFF [context]` |
| `:commit-preview [context]` | | same with `--preview`; fills `BUFFER` with `git commit -m` / `-am` |
| `:suggest <description>` | `:s` | `aimee suggest …`; replaces `BUFFER` with the command |
| `:edit [text]` | `:ed` | `$AIMEE_EDITOR` / `$EDITOR` / `nano` on `.aimee/AIMEE_EDITMSG.md`; sets `BUFFER=": $content"` |

`:commit` in the plugin **runs** the commit (it does not leave a `git commit` line). `:commit-preview` leaves a `git commit` line for you to edit. The plugin README does not document `:commit-preview`; the dispatcher does (`dispatcher.zsh:219`).

There is **no** `:doctor` arm in the dispatcher. The plugin README lists `:doctor` (`shell-plugin/README.md:246`). Run `aimee doctor` or `aimee zsh doctor` instead.

### Session (reset when the terminal closes)

Stored in hidden plugin vars (`lib/config.zsh:19-27`).

| Command | Aliases | Effect |
|---|---|---|
| `:model [query]` | `:m` | `_AIMEE_SESSION_MODEL` + `_AIMEE_SESSION_PROVIDER` via `aimee select model` |
| `:reasoning-effort [query]` | `:re` | `_AIMEE_SESSION_REASONING_EFFORT` → `AIMEE_REASONING__EFFORT` |
| `:agent [id]` | `:a` | `_AIMEE_ACTIVE_AGENT` (picker if omitted) |
| `:config-reload` | `:cr`, `:model-reset`, `:mr` | clears the three session overrides |

`:provider` is **not** a session-provider command. In the dispatcher it is an alias of `:login` (`dispatcher.zsh:259`). `_aimee_action_session_provider` exists in `lib/actions/provider.zsh` but is **never called**.

### Persistent config (writes `~/.aimee/.aimee.toml`)

| Command | Aliases | What it runs |
|---|---|---|
| `:config-model [query]` | `:cm` | `aimee config set model <provider> <model>` |
| `:config-reasoning-effort` | `:cre` | `aimee config set reasoning-effort <effort>` |
| `:config-commit-model` | `:ccm` | `aimee config set commit …` |
| `:config-suggest-model` | `:csm` | `aimee config set suggest …` |
| `:config` | `:env`, `:e` | `aimee config list` |
| `:config-edit` | `:ce` | editor on `aimee config path` |

### Auth, workspace, info

| Command | Aliases | What it runs |
|---|---|---|
| `:provider-login [query]` | `:login`, `:provider` | picker, then `aimee provider login <id>` |
| `:supergrok` | `:xai-oauth`, `:grok-oauth`, `:supergrok-heavy` | `aimee provider login xai_oauth` |
| `:logout [query]` | | picker (`--configured`), then `aimee provider logout` |
| `:workspace-sync` | `:sync` | `aimee workspace sync --init` |
| `:workspace-init` | `:sync-init` | `aimee workspace init` |
| `:workspace-status` | `:sync-status` | `aimee workspace status .` |
| `:workspace-info` | `:sync-info` | `aimee workspace info .` |
| `:info` | `:i` | `aimee info [--cid]` |
| `:help` | | `aimee list command` |
| `:tools` | `:t` | `aimee list tools <active-or-aimee>` |
| `:skill` | | `aimee list skill` |

After a successful default / `:new <prompt>` action the plugin starts a background `workspace sync` (if `AIMEE_SYNC_ENABLED` is not disabled and the workspace is already indexed) and `aimee update --no-confirm` (`lib/helpers.zsh:204-241`).

## Theme / rprompt

`aimee zsh theme` appends `RPROMPT='$(_aimee_prompt_info)'` (`shell-plugin/aimee.theme.zsh:24-28`). `_aimee_prompt_info` runs `aimee zsh rprompt` with session env forwarded (`aimee.theme.zsh:9-21`).

## Keyboard

Plugin-owned bindings (`lib/bindings.zsh:40-45`):

| Key | Widget |
|---|---|
| Enter / `^J` | `aimee-accept-line` |
| Tab / `^I` | `aimee-completion` (`@` files, `:` commands) |
| bracketed paste | `aimee-bracketed-paste` → `aimee zsh format` on `:` lines |

`aimee zsh keyboard` prints the **ZLE** sheet (emacs default or vi if `main` is `viins`/`vicmd`) — not Aimee-specific chords (`shell-plugin/keyboard.zsh`). On macOS it reminds you to run `aimee zsh doctor` if Option bindings fail.

`aimee doctor` checks zsh, the binary, plugin/theme load, completions, `fd` / `bat`, autosuggestions / syntax-highlighting, editor, PATH, Nerd Font (`shell-plugin/doctor.zsh`).

The plugin README lists `fd` as a prerequisite (`shell-plugin/README.md:18`). File picking in current code is `aimee select file`, not `fd` directly.

## Configuration variables

From `lib/config.zsh` and `lib/helpers.zsh` (plus README vars the theme still honors):

| Variable | Default | Role |
|---|---|---|
| `AIMEE_BIN` | `aimee` | Binary the plugin invokes |
| `AIMEE_EDITOR` | `$EDITOR` or `nano` | `:edit` / `:config-edit` |
| `AIMEE_MAX_COMMIT_DIFF` | `100000` | `:commit` `--max-diff` |
| `AIMEE_SYNC_ENABLED` | `true` | background `workspace sync` |
| `AIMEE_TERM` | `true` | capture recent commands for the agent |
| `AIMEE_TERM_MAX_COMMANDS` | `5` | ring-buffer size |
| `AIMEE_TERM_OSC133` | `auto` | OSC 133 markers (Ghostty reflow) |
| `AIMEE_CURRENCY_SYMBOL` | `"$"` | theme (README) |
| `AIMEE_CURRENCY_CONVERSION_RATE` | `1.0` | theme (README) |
| `NERD_FONT` / `USE_NERD_FONT` | auto | theme icons (README) |

Internal (hidden, `typeset -h`): `_AIMEE_CONVERSATION_ID`, `_AIMEE_PREVIOUS_CONVERSATION_ID`, `_AIMEE_ACTIVE_AGENT`, `_AIMEE_SESSION_MODEL`, `_AIMEE_SESSION_PROVIDER`, `_AIMEE_SESSION_REASONING_EFFORT`.

## File interactions

| Path | Role |
|---|---|
| `shell-plugin/aimee.plugin.zsh` | Module loader |
| `shell-plugin/lib/dispatcher.zsh` | `:command` table |
| `shell-plugin/lib/actions/*.zsh` | Handlers |
| `shell-plugin/lib/completion.zsh` | `@` and `:` Tab |
| `shell-plugin/lib/bindings.zsh` | ZLE maps |
| `shell-plugin/aimee.setup.zsh` | Managed `.zshrc` block |
| `shell-plugin/aimee.theme.zsh` | `RPROMPT` |
| `.aimee/AIMEE_EDITMSG.md` | `:edit` scratch file (cwd) |

## Best practices

- Install with `aimee setup`, then open a new shell.
- Use `:ask` / `:plan` / `:aimee` in zsh; do not expect `:act`.
- Tag files with `@` + Tab rather than pasting raw paths (paste wrap only runs on `:` lines).
- `:conversation -` after `:new` is the fastest way back.
- Set `AIMEE_SYNC_ENABLED=false` if you do not want background index sync.

## Anti-patterns

- Documenting `:doctor`, `:act`, or `:planner` as plugin commands. They are not in the dispatcher.
- Calling `_aimee_action_session_provider` mentally — it is dead code until wired.
- Editing the managed setup block.
- Running `:commit-preview` and then also `:commit` on the same message without looking at `BUFFER`.
- Putting secrets in `:edit` scratch files and committing `.aimee/AIMEE_EDITMSG.md`.

## Related

- [Quickstart](quickstart.md)
- [The flock](flock.md)
- [CLI reference](cli.md)
- [TUI](surfaces/tui.md)
