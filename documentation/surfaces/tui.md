# Terminal UI

The default way to work with the flock: launch `aimee` inside your terminal and get an interactive session backed by ratatui.

## Launching

```bash
aimee
```

Without arguments you get the interactive session in the current directory. Variants:

```bash
aimee -p "one-shot prompt"     # run and exit, no TUI
aimee --agent sage             # start addressed to a specific agent
aimee --cid <id>               # resume a stored conversation
aimee -C path/to/repo          # set the working directory
```

## The look

The interface uses a Warp-style dark terminal palette defined in `crates/aimee_main/src/theme.rs`:

| Token | Value | Used for |
|---|---|---|
| Cyan | `#01A4FF` | Primary accent |
| Magenta | `#BF7AF0` | Secondary accent |
| Violet | `#7C5CFF` | Tabs / gutters |
| Lime | `#00D67E` | Success |
| Gold | `#FFCC02` | Commands |
| Near white | `#E6E6E6` | Body text |
| Void | `#0B0D12` | Background |
| Muted | `#8B949E` | Secondary text |

Session output renders as a readable timeline: agent titles with model info, tool activity as it happens, streaming markdown for responses (rendered by the `aimee_markdown_stream` crate).

## Working in a session

Address agents by name or alias; switch mid-session without restarting. Tool calls are visible — you see reads, patches, and shell commands as they execute rather than waiting for a final answer. Conversation state persists automatically (see [Sessions](../usage/sessions.md)); resume any of them later with `aimee conversation resume`.

A conversation selector lets you pick among stored sessions at startup. Input supports completion through the built-in completer; an inline editor handles multiline prompts.

## One-shot vs interactive

| | Interactive (`aimee`) | One-shot (`aimee -p "…"`) |
|---|---|---|
| Multi-turn context | Full session | Single exchange (or piped input) |
| Best for | Working sessions | Scripts, CI, quick questions |
| Piped input | No | Yes — `cat file \| aimee` |

One-shot runs still persist as conversations and can be resumed interactively afterward.

## Diagnostics

```bash
aimee doctor    # environment diagnostics
aimee info      # config, active model, environment status
aimee logs      # stream log output
```

Exact keybindings live in [Keyboard shortcuts](../reference/keybindings.md).

## See also

* [Web and mobile PWA](pwa.md)
* [Pods and sandboxes](pods.md)
* [Your first flock session](../getting-started/first-session.md)

<!-- sources: crates/aimee_main/src/ui.rs, src/theme.rs, src/cli.rs -->
