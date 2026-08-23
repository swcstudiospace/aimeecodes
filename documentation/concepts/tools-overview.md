# Tools: how agents touch your system

Tools are the only way an agent acts. Everything else — reasoning, planning — produces tool calls or text. This page explains the machinery; the per-tool reference lives in [Tool catalog](../reference/tools/catalog.md).

## The catalog

`ToolCatalog` (`crates/aimee_domain/src/tools/catalog.rs`) registers exactly sixteen tools:

`read`, `write`, `fs_search`, `sem_search`, `remove`, `patch`, `multi_patch`, `undo`, `shell`, `fetch`, `followup`, `plan`, `skill`, `todo_write`, `todo_read`, `task`

Each variant carries a description file (e.g. `fs_read.md`, `semantic_search.md`) whose text is what the model sees. Descriptions are capped at 1024 characters — a repo rule (`docs/tool-guidelines.md`) that keeps prompts lean and behavior predictable. New tools must join the catalog and the executor/registry path; unregistered variants are a defect, not an option.

## The registry

`ToolRegistry` (`crates/aimee_app/src/tool_registry.rs`) routes three kinds of tools:

1. **Catalog tools** — the sixteen above.
2. **Agent tools** — per-agent additions.
3. **MCP tools** — anything exposed by configured Model Context Protocol servers.

Every call passes through: permission check (restricted mode), timeout (`tool_timeout_secs`), execution, and structured error reporting back to the model.

## The task/sage switch

One catalog behavior worth knowing: when `subagents = true` (the default in the embedded config), Aimee gets the `task` tool for dispatching specialist subagents, and Sage-as-a-tool is removed. Set `subagents = false` and the reverse happens — `task` is disabled and `sage` becomes directly callable. You can't have both shapes at once; the config picks the topology.

## MCP servers

External tools arrive through MCP. Configure per project (`.mcp.json`) or globally (`~/.aimee/.mcp.json`); project wins:

```json
{
  "mcpServers": {
    "my-tools": { "url": "http://127.0.0.1:8800/mcp" }
  }
}
```

Manage from the CLI:

```bash
aimee mcp list      # configured servers
aimee mcp import    # from JSON
aimee mcp show      # details for one server
aimee mcp remove    # remove
aimee mcp reload    # rebuild caches
aimee mcp login     # OAuth-enabled servers
```

MCP responses are untrusted input — they pass through the same validation as any external content (see [Security model](../operations/security.md)). A cache (cacache under the system cache dir) keeps MCP traffic fast across runs.

## Safety posture

* File mutations go through `patch`/`multi_patch` with undo support — not raw writes where avoidable.
* `shell` is the sharpest tool: in restricted mode every invocation needs a grant.
* `remove` is deliberate and confirmable; `undo` exists because mistakes happen.
* External input (fetch results, MCP payloads, file contents) is treated as untrusted everywhere.

## See also

* [Tool catalog](../reference/tools/catalog.md)
* [Autonomy levels and guardrails](autonomy.md)
* [Skills, commands, and templates](skills-commands-templates.md)

<!-- sources: AIMEE.md §7, crates/aimee_domain/src/tools/catalog.rs, crates/aimee_app/src/tool_registry.rs, docs/tool-guidelines.md -->
