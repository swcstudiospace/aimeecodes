---
name: agent-cli-warp-ux
description: "Use when building Warp-like multi-agent CLI UX."
version: 1.0.0
author: Hermes Agent
license: MIT
platforms: [linux, macos, windows]
metadata:
  hermes:
    tags: [warp, cli, tui, ratatui, rustyline, agents, slash-commands, aimee]
    related_skills: [ratatui-agent-tui, omega-loops-cli, hermes-telegram-remote-control]
    created_by: agent
---

# Agent CLI Warp UX

Class skill for multi-agent coding CLIs that should feel like **Warp CLI**:
quiet timeline, full agent flock on splash, slash command palette, enterprise
XML prompt packs, and parallel specialist swarm.

**Canonical operator tree:** `/root/src/repos/aimeecodes` (bin `aimee`). Also
applies to any ratatui + rustyline agent CLI with the same preferences.

## When to use

- Operator wants Warp 1:1 palette/font or “make it look like Warp”
- Landing only shows 3 agents but the tree has a full specialist roster
- `/` does not open a command menu
- Only 1 built-in slash command; need enterprise packs with XML bodies
- Turns feel serial / one-agent-at-a-time despite multi-agent design
- Timeline lacks TOOL / SKILL / AGENT hop visibility

Don't use for: Telegram stop photos (`hermes-telegram-remote-control`), pure
install/smoke without UX (`omega-loops-cli` install section only).

## Operator preferences (hard)

1. **Warp dark palette + JetBrains Mono** (font is host-side; document it).
2. Timeline lanes: **TOOL / SKIL / AGNT → / DONE** — not prose titles or bubble mockups.
3. Splash lists **all** built-in agents; prompt chips stay compact.
4. **`/` or `:` on empty line opens the full command palette.**
5. Multi-lane work **swarms** via parallel `task` subagents (`/swarm` + orchestrator policy).

## Procedure

### 1. Palette

Lock Warp dark RGB in theme module + unit tests:

| Token | RGB |
|-------|-----|
| Accent blue | `01 A4 FF` |
| Green | `00 D6 7E` |
| Gold | `FF CC 02` |
| Magenta | `BF 7A F0` |
| Violet | `7C 5C FF` |
| Body | `E6 E6 E6` |
| Void | `0B 0D 12` |
| Red | `F1 4C 4C` |

Also: `ratatui-agent-tui/references/warp-palette-and-slash-menu.md`.

### 2. Timeline

Domain categories Tool/Skill/Agent + presentation gutter/chip + short tool verbs
+ agent `NAME → task`. See `ratatui-agent-tui` Warp-clean timeline.

### 3. Splash flock

Multi-row chips for every `agents/*.md` id. Prompt: loop trio + `+N more · / for cmds`.

### 4. Slash palette (rustyline 18)

```text
Empty bol `/` or `:` → ConditionalEventHandler → Cmd::Complete
Mid-line `/` → None → SelfInsert
Completer: empty line = full menu with sentinel `/`
File pick: only @[path]
Ctrl+/ → Complete
```

**No** `EventHandler::from(Vec<Cmd>)` — rustyline 18 does not implement it.

### 5. Enterprise commands

`commands/*.md` with YAML frontmatter + XML prompt body. Embed in loader
`init_default` via `include_str!`. Pack includes review, harden, incident,
ship, oncall, rfc, adr, migrate, perf, slo, threat-model, compliance, runbook,
postmortem, api-contract, k8s-review, cost, data-privacy, test-plan, swarm.

### 6. Swarm policy

Runtime often already parallelizes task tools (`join_all`). Serial feel =
orchestrator prompt. Teach orchestrator to fan out concurrent specialists;
ship `/swarm` for explicit multi-agent runs.

## Pitfalls

1. Selling synthetic bubble cards as “the TUI”
2. Hardcoding 3 splash agents
3. Sequence bindings on rustyline 18
4. File picker on every Complete (breaks URLs)
5. Inventing a new executor when task parallel already exists
6. Forgetting host font is JetBrains Mono (operator Warp 1:1)

## Verification

```bash
cd /root/src/repos/aimeecodes
cargo test -p aimee_main --lib -- banner:: theme:: title_display::
cargo test -p aimee_services --lib -- command::
cargo test -p aimee_app --lib -- fmt::
```

Live: splash shows full flock; `/` opens menu; TOOL/SKIL/AGNT lanes in a run.

## Support files

- `ratatui-agent-tui/references/warp-palette-and-slash-menu.md`
- `omega-loops-cli/references/aimee-cli-ux.md`

## Related

- `ratatui-agent-tui` — banners, chips, timeline presentation
- `omega-loops-cli` — build/smoke for aimee/omega trees
- `omega-loops-agents` — agent prompt bodies
