# Built-in loop roster

Lock these when rewriting prompts. Parse test: `agent::tests::test_builtin_agents_parse`.

| id | title | description must contain | tools (frontmatter) |
|----|-------|--------------------------|---------------------|
| omega | Implement and verify | `Loop` | task, sem_search, fs_search, read, write, undo, remove, patch, multi_patch, shell, fetch, skill, todo_write, todo_read, mcp_* |
| muse | Plan the loop | `Loop` | sem_search, sage, search, read, fetch, plan, mcp_* |
| sage | Research and review | `Loop` | sem_search, search, read, fetch |

Body needles:

- omega: `You are Omega, the implementer` and `Sage (research) → Muse (plan) → Omega`
- muse: `You are Muse, the planner` and `plan tool`
- sage: `You are Sage, the researcher` and `Read-only`

Keep `user_prompt` as the event/date/command_trace handlebars block. Do not bake a static date.

`apply_subagent_tool_config` only mutates omega: drops `task`/`sage`, then inserts `task` before `mcp_*` when `OmegaConfig.subagents` is true.

Wrapper `templates/omega-custom-agent-template.md` must include `<loop_engineering>` and the Muse `plans/` plan-tool exception. orch_system_spec snapshots must contain `<loop_engineering>` after a wrapper edit.
