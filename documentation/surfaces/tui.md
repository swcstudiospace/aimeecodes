# TUI

The interactive Aimee Codes shell is a **rustyline** prompt on top of **ratatui** splash/banner widgets. It is not a full-screen application that takes over every key. Type a message, or start a line with `:` / `/`, or prefix a shell command with `!`.

Start it:

```bash
aimee
aimee --agent sage
aimee --cid <id>
aimee -C /path/to/project
```

Interactive mode is: no `-p`, no piped stdin, no subcommand (`crates/aimee_main/src/cli.rs:80-86`). `UI::run_inner` prints the banner, initializes state, hydrates caches, opens a conversation, then loops on `prompt` → `on_command` (`crates/aimee_main/src/ui.rs:359-444`). Ctrl+C cancels the in-flight turn; the loop continues. EOF (Ctrl+D) is `ReadResult::Exit` (`crates/aimee_main/src/editor.rs:120`).

## How the ratatui shell works

| Piece | Path | Job |
|---|---|---|
| Process / cwd / sandbox / pod | `crates/aimee_main/src/main.rs` | Resolve worktree or DevPod, then `UI::init` |
| Event loop | `crates/aimee_main/src/ui.rs` | Banner, caches, prompt, dispatch |
| Prompt chrome | `crates/aimee_main/src/prompt.rs` | Left: chips + dir + branch + chevron. Right: agent, tokens, cost, model, effort |
| Line editor | `crates/aimee_main/src/editor.rs` | rustyline, history, palette, keys |
| Input parse | `crates/aimee_main/src/input.rs` + `model.rs` | `AppCommand` |
| Highlight | `crates/aimee_main/src/highlighter.rs` | `:cmd` gold, `@[path]` cyan, `!shell` magenta |
| Splash | `crates/aimee_main/src/banner.rs` | ratatui `Buffer` flushed as ANSI |
| Theme | `crates/aimee_main/src/theme.rs` | Warp-dark truecolor palette |
| Display markers | `crates/aimee_main/src/display_constants.rs` | `[yes]`, `[no]`, `[empty]`, `[built-in]` |

The banner is composed in a ratatui `Buffer` (void fill, rounded cyan frame, figlet art, loop `LineGauge`, agent chips, tagline) and printed as ANSI (`banner.rs:84-123`, `banner.rs:141-149`). Override art with `AIMEE_BANNER`. Tagline in source: `CLI agent flock  ·  17 specialists  ·  Warp palette` (`banner.rs:14`).

Each prompt turn reprints loop chips so `:aimee` / `:muse` / `:sage` stay visible without a full-screen redraw (`prompt.rs:89-91`, `banner.rs:351-358`).

Left prompt (`prompt.rs:64-118`):

1. Chip row
2. Folder icon + cwd basename (bold cyan) and git branch (bold lime) when it differs from the directory name
3. Lime chevron

Right prompt: agent (nerd-font + `UPPER_SNAKE`), token count, cost, short model id, reasoning effort (hidden when `Effort::None`; 3-letter form under 100 columns) (`prompt.rs:120-197`).

## Theme tokens

Org brand copy still lists rose `#ff5a7a`, cyan `#00e5ff`, void `#080612` (`org/brand.md:24`, `AIMEE.md:199`). Those hex values are **not** in `crates/aimee_main/src/theme.rs`. The TUI palette is locked to **Warp CLI dark** (`theme.rs:1-46`, test at `theme.rs:124-143`):

| Token | Hex | RGB in code | Used for |
|---|---|---|---|
| `CYAN` | `#01A4FF` | `0x01, 0xA4, 0xFF` | Accent, mentions, directory, frame |
| `MAGENTA` | `#BF7AF0` | `0xBF, 0x7A, 0xF0` | `!shell`, banner cycle |
| `VIOLET` | `#7C5CFF` | `0x7C, 0x5C, 0xFF` | Tabs / gutters |
| `LIME` | `#00D67E` | `0x00, 0xD6, 0x7E` | Branch, chevron, success |
| `GOLD` | `#FFCC02` | `0xFF, 0xCC, 0x02` | `:command` tokens |
| `NEAR_WHITE` | `#E6E6E6` | `0xE6, 0xE6, 0xE6` | Body |
| `VOID` | `#0B0D12` | `0x0B, 0x0D, 0x12` | Background |
| `MUTED` | `#8B949E` | `0x8B, 0x94, 0x9E` | Secondary |
| `RED` | `#F14C4C` | `0xF1, 0x4C, 0x4C` | Errors |

Recommended face: **JetBrains Mono**. The CLI cannot load a font into the host emulator (`theme.rs:48-50`).

PWA pages still use `#ff5a7a`. Do not document the TUI as rose/cyan/void unless `theme.rs` changes.

Styles (`theme.rs:52-96`):

- Command / slash tokens → gold bold
- `@[path]` mentions → cyan bold
- `!shell` → magenta
- Directory → cyan bold; branch / chevron → lime bold
- Key chips → void on cyan, bold

## Key interactions (from code)

rustyline bindings in `AimeeEditor::new` (`editor.rs:54-90`):

| Key | Effect | Evidence |
|---|---|---|
| Enter | Submit the line | rustyline default |
| Alt+Enter | Insert a newline (multi-line) | `editor.rs:64-67` |
| Ctrl+K / Ctrl+Shift+K | Clear screen | `editor.rs:68-75` |
| `/` at beginning of line | Open command palette (`Cmd::Complete`) | `editor.rs:76-81`, `136-158` |
| `:` at beginning of line | Same palette | `editor.rs:82-85` |
| Ctrl+`/` | Complete / palette from empty-ish state | `editor.rs:86-90` |
| Mid-line `/` or `:` | Normal insert (URLs, prose) | `BolCommandPalette` returns `None` (`editor.rs:156-157`) |
| Ctrl+C | `ReadResult::Continue` — new prompt, no exit | `editor.rs:119` |
| Ctrl+D | `ReadResult::Exit` | `editor.rs:120` |
| Ctrl+C during a turn | Cancel the in-flight `on_command` / `-p` | `ui.rs:382-386`, `ui.rs:403-406` |

History: capacity `1024 * 1024` entries, list completion, forced color, signals enabled (`editor.rs:26`, `editor.rs:54-61`). Successful lines are saved to the env history path (`editor.rs:96-101`).

Highlighter (`highlighter.rs:21-56`): `:foo` / `/foo` gold bold; `@[path]` cyan bold (must be closed `]`); `!cmd` magenta.

Parser (`model.rs:300-325`):

- `!…` → `AppCommand::Shell` (native shell, bypasses clap)
- `:name` or `/name` → clap `AppCommand`
- anything else → `AppCommand::Message`

## Slash / colon commands (`AppCommand`)

Canonical prefix is `:`. `/` is compat (`model.rs:316-325`, `banner.rs:92-93`). Names from `AppCommand` (`model.rs:422-771`) and `name()` (`model.rs:773-829`). Internal variants (`Message`, `Custom`, `Shell`, `AgentSwitch`, `Rename`) are hidden from `aimee list command` (`model.rs:837-847`).

### Loop / agents

| Command | Aliases | Effect |
|---|---|---|
| `:act` | `:aimee`, `:omega` | Switch to Aimee |
| `:plan` | `:muse` | Switch to Muse |
| `:sage` | | Switch to Sage |
| `:agent` | `:a` | Interactive agent picker |
| `:help` | | Help / command list |

There is **no** `:ask` in `AppCommand`. Research in the TUI with `:sage`. See [The flock](../flock.md#aliases-ask-plan-act).

Splash chips also advertise `:fe-ui`, `:fe-web3`, `:fe-realtime`, `:fe-edge`, `:fe-qa`, `:be-api`, `:be-web3`, `:be-data`, `:be-security`, `:be-reliability`, `:plat-k8s`, `:plat-cloud`, `:plat-compliance`, `:plat-sre` (`banner.rs:18-40`). Those IDs are built-in agents, not dedicated `AppCommand` variants. The command manager registers them as `agent-<id>` (`model.rs:220-239`). Switch with `:agent` / `:a`, or `:agent-fe-ui`. Typing `:fe-ui` is not the same as `:agent-fe-ui`.

### Session / conversation

| Command | Aliases | Notes |
|---|---|---|
| `:new` | | New conversation, keep history |
| `:conversation [id]` | `:conversations`, `:c` | Picker, or switch to `id` |
| `:conversation-tree` | `:ct` | Children of the current conversation |
| `:clone [id]` | | Clone current or selected |
| `:rename <name>` | `:rn` | Current conversation; name required |
| `:conversation-rename` | | Interactive rename |
| `:retry` | `:r` | Retry last without editing context |
| `:dump [--html]` | | JSON (default) or HTML |
| `:compact` | | Compact context |
| `:copy` | | Last assistant reply to clipboard |
| `:info` | | System / session info |
| `:usage` | | Tokens and requests |
| `:exit` | | Leave the TUI |

### Config / model / auth

| Command | Aliases | Notes |
|---|---|---|
| `:config` | `:env`, `:e` | Effective resolved config |
| `:config-model` | `:cm` | Set **global** model |
| `:model` | `:m` | Session model |
| `:config-reload` | `:cr`, `:model-reset`, `:mr` | Drop session overrides |
| `:reasoning-effort` | `:re` | Session effort |
| `:config-reasoning-effort` | `:cre` | Persist effort |
| `:config-commit-model` | `:ccm` | Commit-message model |
| `:config-suggest-model` | `:csm` | Suggest model |
| `:config-edit` | `:ce` | Open global config in `$EDITOR` |
| `:provider` | `:login`, `:provider-login` | Provider auth |
| `:supergrok` | `:xai-oauth`, `:grok-oauth`, `:supergrok-heavy` | `xai_oauth` device login |
| `:logout` | | Drop provider credentials |

### Workspace / git / tools

| Command | Aliases | Notes |
|---|---|---|
| `:workspace-sync` | `:sync` | Semantic-search sync |
| `:workspace-status` | `:sync-status` | File sync status |
| `:workspace-info` | `:sync-info` | Workspace details |
| `:workspace-init` | `:sync-init` | Init without syncing |
| `:index` | | Index cwd for semantic search |
| `:commit [n\|preview]` | | AI commit; numeric arg = max-diff bytes |
| `:commit-preview` | | Preview only |
| `:suggest <text>` | `:s` | Natural language → shell command |
| `:edit [text]` | `:ed` | `$EDITOR` for a multi-line prompt |
| `:tools` | `:t` | Tools + schema for the active agent |
| `:skill` | | List skills |
| `:update` | | Self-update |

### Standing loop / teams (TUI-only)

These exist on `AppCommand` and are **not** ZSH dispatcher arms.

| Command | Usage string |
|---|---|
| `:goal [text\|status\|pause\|resume\|clear]` | Standing `/goal` loop |
| `:subgoal <criterion>` | Add a criterion to the active goal |
| `:soul` | Show loaded `SOUL.md` docs |
| `:team [list\|name lead=muse impl=aimee]` | Teams / workflows |
| `:learn <name> \| <description>` | Draft a reusable skill |
| `:channel [kind address\|list]` | Delivery channels (`:telegram` alias) |

Banner command-sheet rows also mention `:tpl-*` and `/review` `/harden` `/ship` `/oncall` (`banner.rs:269-276`). Those are prompt-pack / custom-command labels on the splash, not extra `AppCommand` variants. Custom workflow commands resolve through `AimeeCommandManager::find` (`model.rs:381-398`).

## File interactions

| Path | Role |
|---|---|
| `crates/aimee_main/src/ui.rs:359-444` | Interactive loop |
| `crates/aimee_main/src/editor.rs:54-90` | Keys |
| `crates/aimee_main/src/model.rs:422-771` | Command enum |
| `crates/aimee_main/src/theme.rs:14-46` | Palette |
| `crates/aimee_main/src/banner.rs:18-40` | Agent chips |
| `crates/aimee_main/src/prompt.rs:64-197` | Prompt geometry |
| `crates/aimee_main/src/highlighter.rs:21-56` | Input colors |
| History file | `Environment::history_path` (under the config base) |

## Best practices

- Prefer `:` over `/`. Both work; `:` is canonical.
- Open the palette with `:` or `/` on an empty line, then Tab — do not memorize every verb.
- Use Alt+Enter for multi-line prompts; or `:edit`.
- Switch agents with `:sage` / `:plan` / `:act` instead of restarting the process.
- `!` is a real shell. Do not paste secrets into `!` lines.

## Anti-patterns

- Documenting TUI colors as `#ff5a7a` / `#00e5ff` / `#080612`. The running palette is Warp-dark in `theme.rs`.
- Expecting `:ask` to switch Sage in the TUI. That alias is ZSH-only.
- Treating the TUI as a full-screen ratatui app. After the splash, input is rustyline.
- Assuming banner labels (`:tpl-*`, `/ship`, `:fe-ui`) are built-in `AppCommand` variants without checking `model.rs`. Specialists switch via `:agent` / `:agent-<id>`.

## Related

- [Quickstart](../quickstart.md)
- [The flock](../flock.md)
- [How to use](../howto.md)
- [CLI reference](../cli.md)
- [ZSH plugin](../zsh.md)
- [PWA](pwa.md) — different surface, different theme tokens
