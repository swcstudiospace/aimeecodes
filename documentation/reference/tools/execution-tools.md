# Execution tools

The tools that run things: shell command execution and the subagent dispatch surface.

## `shell`

Executes shell commands. Key behaviors from its description:

* **`cwd` parameter** sets the working directory for the command; defaults to the session's working directory.
* **No `cd` in command strings** — the description forbids it explicitly, since changing directories mid-command breaks the working-directory contract and auditability.
* Output streams back as tool results the model can act on.
* In restricted mode, every invocation needs an explicit permission grant.

This is the sharpest tool in the set: builds, tests, git operations, curl for binaries (`fetch` refuses binary downloads by design). It's also where most guardrails concentrate — timeouts, budgets, and permission gates all apply.

## `task`

Dispatches specialist subagents. Present only when `subagents = true` (the default); when false, `task` is disabled and `sage` becomes directly callable instead.

Each task call targets a specialist ID with bounded scope:

```text
aimee (orchestrator)
  ├── task → be-security    "threat-model the webhook handler"
  ├── task → fe-ui          "add error states to settings"   (concurrent if independent)
  └── integrates evidence, verifies, reports
```

Policy: prefer concurrent calls for independent workstreams; flock agents (aimee/muse/sage) are never valid targets; specialists don't spawn further orchestrators.

## Safety summary

| Tool | Blast radius | Contained by |
|---|---|---|
| `shell` | Machine-level | Timeout, failure budget, restricted-mode grants, pod isolation |
| `task` | Bounded workstreams | Orchestrator discipline, per-task verify commands |

For untrusted code, combine: run the whole session inside a pod so `shell` executes on a disposable machine. See [Pods](../../surfaces/pods.md).

## See also

* [Tool catalog](catalog.md)
* [Filesystem tools](filesystem-tools.md)
* [Workflow tools](workflow-tools.md)

<!-- sources: crates/aimee_domain/src/tools/descriptions/{shell,task}.md, templates/aimee-partial-swarm-policy.md -->
