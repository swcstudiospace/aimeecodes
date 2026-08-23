# Warp palette + slash command menu (Aimee Codes)

Session learnings for agent CLIs that should look/feel like **Warp CLI** and expose Hermes-style tool visibility.

## Warp dark palette (lock in theme tests)

| Token | RGB | Role |
|-------|-----|------|
| Accent blue | `01 A4 FF` | chips, TOOL, cyan accents |
| Green | `00 D6 7E` | success, DONE, chevron |
| Gold | `FF CC 02` | slash commands, ACT |
| Magenta | `BF 7A F0` | AGNT, shell |
| Violet | `7C 5C FF` | gutter, SKIL, tabs |
| Body | `E6 E6 E6` | near-white text |
| Void | `0B 0D 12` | background |
| Muted | `8B 94 9E` | secondary |
| Red | `F1 4C 4C` | ERR |

**Font:** apps cannot force the host face. Document **JetBrains Mono** (Warp monospaced default) in splash footer / README. Operator preference: match Warp 1:1 including font.

Aimee Codes: `crates/aimee_main/src/theme.rs` (`WARP_FONT_FACE`).

## Slash / colon command palette (rustyline 18)

Rustyline **18 has no multi-Cmd Sequence** (`EventHandler::from(Vec<Cmd>)` fails). Use:

```rust
// Empty bol → Complete (completer treats empty as full / menu)
// Mid-line `/` → None → default SelfInsert (URLs safe)
EventHandler::Conditional(Box::new(BolCommandPalette { sentinel: '/' }))
```

`ConditionalEventHandler::handle` returns `Some(Cmd::Complete)` only when:
- `line.is_empty() && pos == 0`, or
- `line == "/" || line == ":"` (after user typed sentinel + Tab path)

Completer must accept **empty line** as “show full command menu” with default sentinel `/`:

```rust
if line.is_empty() || line.starts_with('/') || line.starts_with(':') {
    return command_completer.complete(line, pos);
}
// File picker only for @[path] — never on random mid-line /
```

Also bind `Ctrl+/` → `Cmd::Complete` as a dedicated palette hotkey.

**Do not** open workspace file picker on every Complete — gate file pick on `@` mention syntax.

## Landing agent flock

Never hardcode only 3 loop agents on splash when the tree has a full specialist roster.

- Splash: multi-row chips for **all** built-ins (loop + fe-* + be-* + plat-*).
- Prompt chips: compact (loop trio + `+N more · / for cmds`) so every turn stays Warp-quiet.
- Keep chip list in sync with `agents/*.md` (Aimee: 17 agents).

## Enterprise slash commands (XML bodies)

Built-ins live as `commands/*.md` with YAML frontmatter + **XML-tagged** prompt body (`<role>`, `<objective>`, `<process>`, `<output_format>`). Embed via `include_str!` in the command loader `init_default`.

Aimee pack (21): `review`, `harden`, `incident`, `ship`, `oncall`, `rfc`, `adr`, `migrate`, `perf`, `slo`, `threat-model`, `compliance`, `runbook`, `postmortem`, `api-contract`, `k8s-review`, `cost`, `data-privacy`, `test-plan`, `swarm`, + `github-pr-description`.

`/swarm` = explicit multi-agent fan-out policy; orchestrator agent prompt should **default to parallel task** for multi-lane work (runtime already `join_all`s task tools).

## Slow turns ≠ broken executor

If “one agent at a time” feels slow: check **policy** (orchestrator serializes) before blaming the runtime. Task tools already run in parallel when multiple task calls land in one turn.