# task

`task` delegates work to another agent, running it as an autonomous subprocess. Input type: `TaskInput` (`crates/aimee_domain/src/tools/catalog.rs:76-95`). Description source: `descriptions/task.md`. This is the tool behind Aimee's orchestrator role — the specialist dispatch described in [The flock](../../flock.md).

## Parameters

| Parameter | Type | Required | Notes |
|---|---|---|---|
| `tasks` | string[] | yes | Clear, detailed task descriptions; executed in parallel |
| `agent_id` | string | yes | Target agent ID: `aimee`, `muse`, `sage`, or a specialist (`fe-ui`, `be-api`, `plat-k8s`, …) |
| `session_id` | string | no | Resume a prior agent session; omit for a fresh stateless session |

## Example

```json
{
  "name": "task",
  "arguments": {
    "agent_id": "sage",
    "tasks": [
      "Review the retry configuration added to crates/aimee_config for correctness and document any edge cases in the merged table behavior."
    ]
  }
}
```

Multiple entries in `tasks` run concurrently with the named agent.

## Behavior

- The launched agent gets its **own context** and returns a single summary message; the delegating agent must relay results to the user itself.
- Resuming via `session_id` preserves the child's full previous context; fresh invocations need self-contained task descriptions (the child knows nothing of this conversation unless it is a context-sharing agent type).
- The contract tells agents when **not** to delegate: known file paths → [read](read.md); exact symbol searches → [fs_search](fs_search.md); small scopes don't justify a subprocess.
- Parallelism is explicit: several `task` calls in one response launch concurrently.
- Availability is config-dependent: when `subagents = true` (default), Aimee has `task`; when disabled, `task` is removed and Sage-as-a-tool replaces it. See [Tools overview](../../tools.md).

## Errors

Unknown `agent_id`, or delegation while `subagents = false`.

## Permissions

No permission gate on the dispatch itself (`catalog.rs:1015`) — the child agent's own tool calls are gated normally under restricted mode.

## Related

- [Tool catalog](catalog.md)
- [The flock](../../flock.md) — Sage / Muse / Aimee and the specialist roster
- [Application architecture](../../architecture/app.md) — how the orchestrator routes subagents
