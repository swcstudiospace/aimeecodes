# Workflow tools

The meta-tools: how agents ask questions, structure plans, load skills, track progress, and delegate to specialists.

## `followup`

Asks you a clarifying question when ambiguity blocks progress. Its description stresses judicious use — balance gathering needed information against excessive back-and-forth. This is the tool behind "quick check before I proceed" moments.

## `plan`

Creates plan files with name, version, and content — structured project plans and task breakdowns that persist under `plans/` and can be tracked across sessions. This is the machinery under Muse's planning; see [Plans and todos](../../usage/plans-and-todos.md).

## `skill`

Loads a skill's instructions on demand — the agent pulls in task-type expertise (`resolve-conflicts`, `test-reasoning`, custom skills you've added) only when relevant, keeping base prompts lean.

## `todo_write` / `todo_read`

Maintains the in-session task list: short-lived items the agent tracks while executing. Pending todos resurface via the reminder template mid-loop, so work isn't quietly dropped. Session-scoped; they don't persist across restarts.

## `task`

Dispatches specialist subagents — present only when `subagents = true` (the default). Each call targets a specialist ID (`fe-ui`, `be-security`, `plat-k8s`, …) with bounded scope and its own verify step. Orchestrators never nest: flock agents are invalid targets. The counterpart behavior: with `subagents = false`, this tool is disabled and `sage` becomes directly callable instead.

```text
aimee (orchestrator)
  ├── task → fe-ui      "implement the settings drawer"
  ├── task → be-api     "add the endpoint"          (concurrent where independent)
  └── integrates results, verifies, reports evidence
```

## How the group fits together

| Moment | Tool |
|---|---|
| Intent unclear | `followup` |
| Work needs structure | `plan` |
| Task type has known pitfalls | `skill` |
| Multi-step run in progress | `todo_write`/`todo_read` |
| Work belongs to a specialty | `task` |

## See also

* [Tool catalog](catalog.md)
* [Swarm runs](../../usage/swarm.md)
* [Skills, commands, and templates](../../concepts/skills-commands-templates.md)

<!-- sources: crates/aimee_domain/src/tools/descriptions/{followup,plan_create,skill_fetch,todo_write,todo_read,task}.md -->
