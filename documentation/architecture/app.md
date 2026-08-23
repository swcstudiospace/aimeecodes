# Application — aimee_app

The orchestration layer between the composition root and application services: it runs conversations, routes tools, and adapts domain behavior to runtime needs.

## Key pieces

| Component | File | Job |
|---|---|---|
| `AimeeApp` | `src/app.rs` (entry) | Drives the turn loop against a session |
| Orchestrator | `src/orchestrator*` | Agent selection, specialist dispatch decisions |
| `ToolRegistry` | `src/tool_registry.rs` | Routes catalog + agent + MCP tools; applies timeouts and permissions |
| DTOs | `src/dto.rs` | Transfer shapes (e.g. `ToolsOverview`) consumed upstream |
| Git app | `src/git*` | Commit generation plumbing |
| Anda pathway hooks | pathway modules | Checkpoint emission when `[anda]` is enabled |

## The registry in detail

Three tool sources converge here:

1. **Catalog tools** from `aimee_domain::ToolCatalog`.
2. **Agent tools** attached per agent definition.
3. **MCP tools** discovered from configured servers.

Every invocation passes through permission checking (restricted mode requires explicit grants), timeout enforcement from `tool_timeout_secs`, and structured error reporting. The task/sage switch is implemented at this layer: `subagents = true` exposes `task`; false exposes `sage`.

## Boundaries

`aimee_app` calls services and infra abstractions — never another app-layer module sideways, never the database directly. It knows nothing about clap or ratatui; presentation stays in `aimee_main`.

## See also

* [Services](services.md)
* [Tool macros](tool-macros.md)
* [Autonomy levels and guardrails](../concepts/autonomy.md)

<!-- sources: AIMEE.md §5, crates/aimee_app/src/tool_registry.rs -->
