# Warp-clean agent timeline (Aimee Codes pattern)

## Goal

Terminal run log that feels like **Warp** (quiet, scannable) while still
showing **Hermes-like** tool/skill activity and **Super Grok Heavy** agent hops.

## Visual contract

```text
│ TOOL  HH:MM:SS  Read          path:range
│ TOOL  HH:MM:SS  Shell · bash  cargo test …
│ SKIL  HH:MM:SS  Skill         skill-name
│ AGNT  HH:MM:SS  FE_RUST    →  implement the parser
│ DONE  HH:MM:SS  Finished      conv_id
```

- Gutter: dim/violet `│`
- Chip: 4 chars, high-chroma on void (TOOL cyan, SKIL violet, AGNT magenta, DONE lime)
- Clock: dim, secondary
- Title: short verb / agent id
- Subtitle: path, command, skill name, or task — dim
- Agent hop: gold `→` between title and subtitle
- Do **not** repeat severity in the title string when the chip already says ERR/WARN

## Code map (aimeecodes, 2026-08)

| Concern | Path |
|---------|------|
| Categories + helpers | `crates/aimee_domain/src/chat_response.rs` (`Category::{Tool,Skill,Agent}`, `TitleFormat::tool/skill/agent/completion`) |
| Colored/plain render | `crates/aimee_main/src/title_display.rs` |
| Tool input titles | `crates/aimee_app/src/fmt/fmt_input.rs` |
| Plan create title | `crates/aimee_app/src/fmt/fmt_output.rs` |
| Nested agent hop | `crates/aimee_app/src/agent_executor.rs` |
| MCP | `crates/aimee_app/src/mcp_executor.rs` |
| `/agent` switch | `crates/aimee_main/src/ui.rs` `on_agent_change` |
| Turn complete | `crates/aimee_main/src/ui.rs` `TaskComplete` → `TitleFormat::completion` |

## Anti-patterns (operator rejected)

- Long prose tool titles (“Search for 'x' in 'y' files at z”) — use short **Search** + subtitle
- `NAME [Agent]` stuffed into Debug chip — use AGNT lane
- Bold/dim soup for agent switch without a clear hop arrow
- Synthetic “chat bubble” screenshots sold as the live TUI (Telegram wrap-up is a different skill)

## Tests

```bash
cargo test -p aimee_main --lib title_display
cargo test -p aimee_app --lib fmt::
```

Plain mode must lock chip tokens (`[TOOL]`, `[SKIL]`, `[AGNT]`) and `→` on handoffs.
When changing category of a formatter, update fmt snapshot/assertions (Debug→Tool).

## Optional next

- Nested indent for sub-agent tool trees
- Collapse consecutive TOOL lines into a count footer (Hermes meta style)
