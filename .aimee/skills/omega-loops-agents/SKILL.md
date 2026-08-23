---
name: omega-loops-agents
description: Use when editing Omega built-in agent prompts.
version: 1.1.0
author: Hermes Agent
license: MIT
platforms: [linux, macos, windows]
metadata:
  hermes:
    tags: [omega, omegaloops, agents, templates, prompts, loop-engineering, muse, sage]
    related_skills: [omega-loops-cli, omega-loops-providers, omega-anda-pathways, hermes-project-soul, ratatui-agent-tui]
---

# Omega Loops — built-in agents & templates

Production prompt surfaces for the **Sage → Muse → Omega** loop. Repo: `/root/src/repos/omegaloops`.

Don't use for: CLI install/smoke (`omega-loops-cli`), provider OAuth (`omega-loops-providers`), Anda/KIP wiring (`omega-anda-pathways`), TUI chrome (`ratatui-agent-tui`).

## When to use

- User wants agents more enterprise / loop-engineering / SOTA
- Edit `crates/omega_repo/src/agents/{omega,muse,sage}.md`
- Edit `templates/omega-*.md` (wrapper, doom-loop, todos, retry, skills)
- Agent picker titles/descriptions look stale after a prompt change

## Loop contract

| Agent | Title | Role | Mutates |
|-------|-------|------|---------|
| `sage` (`:ask`) | Research and review | Grounded truth + critique | No |
| `muse` (`:plan`) | Plan the loop | Team lead; plan tool → `plans/` | Plan file only |
| `omega` | Implement and verify | Close the loop; verify before done | Yes |

Default team in `ui.rs`: lead=muse, impl=omega, review=sage. Workflow: plan → build → review.

Keep that contract. Do not turn Sage into a planner or Muse into an implementer.

## File map

| Path | Role |
|------|------|
| `crates/omega_repo/src/agents/omega.md` | Implementer system prompt + tools |
| `crates/omega_repo/src/agents/muse.md` | Planner; `plan` tool |
| `crates/omega_repo/src/agents/sage.md` | Researcher; read-only tools |
| `templates/omega-custom-agent-template.md` | Injected for **every** agent (incl. custom) |
| `templates/omega-partial-skill-instructions.md` | Skill tool, omega only when skills exist |
| `templates/omega-doom-loop-reminder.md` | Repeated similar calls |
| `templates/omega-pending-todos-reminder.md` | Open todos at turn end |
| `templates/omega-tool-retry-message.md` | Failed tool retry |
| `templates/omega-partial-summary-frame.md` | Compaction memory — stay in role, do not “proceed with implementation” |
| `templates/omega-system-prompt-title-generation.md` | Conversation titles; prefer Research/Plan/Implement/Review |
| `templates/omega-partial-tool-error-reflection.md` | Tool-error reflection; no identical retry; stay in role |
| `crates/omega_repo/src/agent.rs` | `include_str!` + `parse_agent_file` + `apply_subagent_tool_config` |
| `crates/omega_app/src/system_prompt.rs` | Renders agent body, then `{{> omega-custom-agent-template.md }}` |

Body after YAML `---` becomes `system_prompt`. Frontmatter holds `id`, `title`, `description`, `tools`, `user_prompt`, `reasoning`. Do not invent tools; match `ToolCatalog`.

`apply_subagent_tool_config` (omega only): strips `task`/`sage`, re-inserts `task` before `mcp_*` when `config.subagents`.

Muse plan tool writes `{cwd}/plans/{YYYY-MM-DD}-{plan_name}-{version}.md` and refuses overwrite — bump version.

Roster + expected needles: `references/loop-roster.md`.

## Workflow

1. Read the three agent files and the custom wrapper before rewriting. Preserve handlebars (`{{tool_names.*}}`, `{{#if skills}}`, user_prompt event tags).
2. Keep YAML quoted; `id` must stay `omega` / `muse` / `sage`.
3. Put **shared** loop rules in the custom wrapper (`<loop_engineering>`). Put **role** rules in the agent body.
4. After edits: `cargo test -p omega_repo --lib agent::tests` (includes `test_builtin_agents_parse`).
5. Wrapper changes update `omega_app` orch_system_spec snapshots. Accept with `INSTA_UPDATE=1 cargo test -p omega_app --lib orch_spec::orch_system_spec`. To re-check, **unset** `INSTA_UPDATE` — `INSTA_UPDATE=0` panics on insta 1.48.
6. Rebuild so `include_str!` lands in the binary: `scripts/dev-omega.sh` (skill `omega-loops-cli`). `install` alone does **not** rebuild a present debug binary.
7. Confirm titles: `OMEGA_CONFIG=$(mktemp -d) OMEGA_SESSION__PROVIDER_ID=xai_oauth OMEGA_SESSION__MODEL_ID=grok-4.6 omega list agent --porcelain`. (`xai` is the API-key sibling, not SuperGrok.)
8. Summary-frame edits also update `omega_app` compact snapshots (`INSTA_UPDATE=1 cargo test -p omega_app --lib compact::tests`).

## Pitfalls

- **Generic “coding assistant” copy** — product is loop engineering. Each agent must name its place in Sage → Muse → Omega.
- **Docs ban vs Muse** — wrapper forbids new `*.md` unless asked; Muse may write `plans/` **only** via the plan tool (already noted in the wrapper).
- **Smoke `list agent` exit 0** — can still print `No default provider and model configured` and hide titles. Set `OMEGA_SESSION__PROVIDER_ID=xai_oauth` (not `xai`) as above.
- **Compaction “implement” closer** — `omega-partial-summary-frame.md` must stay role-neutral. “Proceed with implementation” pulls Sage/Muse out of role.
- **Stale binary** — agent markdown is compiled in. `dev-omega.sh install` only builds if `target/debug/omega` is missing.
- **`cargo insta` CLI** — may be absent; `INSTA_UPDATE=1 cargo test …` accepts. Never `INSTA_UPDATE=0`.
- **IDE “Rust 2015 async”** on `agent.rs` is false; trust `cargo test`.
- **Historical fixtures** (`omega_domain` conversation snapshots) quote old prompt text; do not “fix” them unless those tests fail.
- **Don't use** for adding wallet/Anda tools to agents unless those tools exist in `ToolCatalog`.

## Verification

```bash
cd /root/src/repos/omegaloops
cargo test -p omega_repo --lib agent::tests -- --nocapture
unset INSTA_UPDATE
cargo test -p omega_app --lib orch_spec::orch_system_spec -- --nocapture
scripts/dev-omega.sh
OMEGA_CONFIG=$(mktemp -d) OMEGA_SESSION__PROVIDER_ID=xai_oauth OMEGA_SESSION__MODEL_ID=grok-4.6 \\\n  omega list agent --porcelain
  omega list agent --porcelain
```

Expect titles: Implement and verify / Plan the loop / Research and review.

## References

- `references/loop-roster.md` — titles, tools, body needles
- CLI dogfood: skill `omega-loops-cli` (`scripts/dev-omega.sh`)
