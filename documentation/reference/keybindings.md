# Keyboard shortcuts

Bindings verified in source: the ZSH plugin's `lib/bindings.zsh` and the TUI input path. Where a binding is context-dependent, it says so.

## ZSH plugin (shell prompt)

| Key | Widget | Behavior |
|---|---|---|
| `Enter` (`^M`) | `aimee-accept-line` | Submits lines; `:` lines route through the dispatcher |
| `Ctrl+J` (`^J`) | `aimee-accept-line` | Same submit path for terminals that distinguish |
| `Tab` (`^I`) | `aimee-completion` | Fuzzy completion; file picking via `aimee select file` |
| Paste (bracketed) | `aimee-bracketed-paste` | On `:` lines, pasted paths auto-wrap as `@[...]`; other lines untouched |

Notes from source:

* Bindings are re-applied after zsh-vi-mode rebuilds keymaps (`zvm_after_init_commands`), so vi-mode plugins don't clobber them.
* Path wrapping is decided by `aimee zsh format --buffer`, keeping parsing in one tested place.
* Regenerate the canonical help text anytime: `aimee zsh keyboard`.

## TUI

The interactive session accepts typed prompts with completion support and renders tool activity inline. Key handling lives in `crates/aimee_main/src/ui.rs` and its input modules; bindings follow standard terminal conventions (Enter to submit, Ctrl+C to cancel/interrupt).

For exact in-TUI chords, run a session and consult the built-in help; the interface keeps interactive affordances visible rather than hidden behind memorized keys.

## File tagging syntax

```zsh
: sage review @[src/main.rs] and @[crates/importer/src/lib.rs]
```

`@` + Tab completes paths; pasting a path into a `:` line wraps it automatically.

## See also

* [The : prefix (ZSH)](../usage/zsh-prefix.md)
* [Terminal UI](../surfaces/tui.md)
* [CLI reference](cli.md)

<!-- sources: shell-plugin/lib/bindings.zsh, shell-plugin/README.md, crates/aimee_main/src/cli.rs -->
