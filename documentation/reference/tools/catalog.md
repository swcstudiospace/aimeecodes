# Tool catalog

The registered tools every agent can draw on. Sixteen variants, one catalog (`ToolCatalog` in `crates/aimee_domain/src/tools/catalog.rs`), each backed by a description file under `src/tools/descriptions/`. Grouped reference pages: [Filesystem](filesystem-tools.md) · [Search & web](search-web-tools.md) · [Execution](execution-tools.md) · [Workflow](workflow-tools.md).

## The full set

| Tool | Purpose (from its description file) | Destructive? | Page |
|---|---|---|---|
| `read` | Reads files from the local filesystem, whole or by line range | No | Filesystem |
| `write` | Writes/overwrites a file; requires prior read + explicit overwrite for existing files | Yes | Filesystem |
| `fs_search` | Regex search built on ripgrep; glob/type filters | No | Search & web |
| `sem_search` | AI-powered semantic code search over the workspace index | No | Search & web |
| `remove` | Deletes a file at an absolute path | Yes (undoable) | Filesystem |
| `patch` | Exact string replacement in files; requires prior read | Yes (undoable) | Filesystem |
| `multi_patch` | Multiple find-and-replace edits to one file in one operation | Yes (undoable) | Filesystem |
| `undo` | Reverts the most recent create/modify/delete on a file | Reverting | Filesystem |
| `shell` | Executes shell commands with a working-directory parameter | Yes | Execution |
| `fetch` | Retrieves URL content as markdown/text; text-only, robots-aware | Network read | Search & web |
| `followup` | Asks the user for clarification on ambiguities | No | Workflow |
| `plan` | Creates structured plan files (name/version/content) | Plan files | Workflow |
| `skill` | Loads a skill's instructions on demand | No | Workflow |
| `todo_write` / `todo_read` | Maintain and read the in-session task list | No | Workflow |
| `task` | Dispatches specialist subagents (present when `subagents = true`) | Delegated | Workflow |

## Rules every tool follows

* **Descriptions ≤ 1024 characters** — policy from `docs/tool-guidelines.md`, kept in editable markdown files bound via `aimee_tool_macros`.
* **Read-before-write**: `patch`, `multi_patch`, and `write`-over-existing all error unless the file was read first in the conversation.
* **Timeouts** from `tool_timeout_secs`; **permissions** checked per call (restricted mode requires grants).
* **Undo support** for file mutations via snapshots.

## Template placeholders

Description files contain placeholders like `{{tool_names.patch}}` or `{{env.cwd}}`, resolved at load time — so descriptions always reference the real tool names and environment.

## Registry routing

`ToolRegistry` (`aimee_app`) routes three sources: this catalog, per-agent tools, and MCP server tools. MCP additions appear alongside these but are validated like any external input. See [Tools overview](../concepts/tools-overview.md).

## See also

* [Filesystem tools](filesystem-tools.md)
* [Search and web tools](search-web-tools.md)
* [Execution tools](execution-tools.md)
* [Workflow tools](workflow-tools.md)

<!-- sources: crates/aimee_domain/src/tools/catalog.rs, src/tools/descriptions/*.md, docs/tool-guidelines.md -->
