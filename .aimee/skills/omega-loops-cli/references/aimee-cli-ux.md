# Aimee Codes CLI UX (Warp + flock + enterprise commands)

Canonical tree for this operator: **`/root/src/repos/aimeecodes`** (bin `aimee`,
crates `aimee_*`). Older docs may still say `omegaloops` / `omega` — re-read
`Cargo.toml` `[[bin]]` before every verify.

## Operator preferences

1. **Warp CLI** is the aesthetic benchmark: palette 1:1, JetBrains Mono host font.
2. Timeline must show **TOOL / SKIL / AGNT → / DONE** lanes (not truncated bubble cards).
3. Landing must list the **full specialist flock** (17), not only `:aimee/:muse/:sage`.
4. **`/` opens a command menu** of built-ins + custom packs.
5. Multi-lane work should **swarm** via parallel `task` subagents (`/swarm` + aimee.md policy).

## Key paths

| Surface | Path |
|---------|------|
| Theme (Warp RGB) | `crates/aimee_main/src/theme.rs` |
| Timeline chips | `crates/aimee_main/src/title_display.rs` |
| Tool titles | `crates/aimee_app/src/fmt/fmt_input.rs` |
| Agent handoff | `crates/aimee_app/src/agent_executor.rs` |
| Splash / chips | `crates/aimee_main/src/banner.rs` |
| Slash palette | `crates/aimee_main/src/editor.rs` + `completer/` |
| Built-in commands | `commands/*.md` → `aimee_services` `init_default` |
| Orchestrator prompt | `crates/aimee_repo/src/agents/aimee.md` |
| Agents | `crates/aimee_repo/src/agents/*.md` (17) |

## rustyline 18 slash menu

No `EventHandler::from(Vec<Cmd>)`. Use `ConditionalEventHandler`: empty bol →
`Cmd::Complete`; mid-line → `None` (SelfInsert). Completer accepts empty line as
full `/` menu. See `ratatui-agent-tui` → `references/warp-palette-and-slash-menu.md`.

## Enterprise command pack

21 built-ins with XML prompt bodies (review, harden, incident, ship, oncall,
rfc, adr, migrate, perf, slo, threat-model, compliance, runbook, postmortem,
api-contract, k8s-review, cost, data-privacy, test-plan, swarm,
github-pr-description).

## Parallel task runtime

`aimee_app` orch already `join_all`s task tool calls. Serial feel = orchestrator
prompt, not missing concurrency. Keep aimee.md swarm default + `/swarm` command.

## Verify slice

```bash
cd /root/src/repos/aimeecodes
cargo test -p aimee_main --lib -- banner:: theme:: title_display::
cargo test -p aimee_services --lib -- command::
cargo test -p aimee_app --lib -- fmt::
```