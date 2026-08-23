# Tool catalog

The **tool catalog** is the set of built-in tools an Aimee Codes agent can call during a turn. It is defined as the `ToolCatalog` enum in [`crates/aimee_domain/src/tools/catalog.rs`](https://github.com/swcstudiospace/omegaloops/blob/main/crates/aimee_domain/src/tools/catalog.rs) — one variant per tool, each carrying its input struct and LLM-facing description. The catalog is the first half of tool routing; [ToolRegistry](../architecture/app.md#tool-registry) (`crates/aimee_app/src/tool_registry.rs`) is the second.

## The 16 tools

| Tool | Input type | One-line role | Permission gate | Reference |
|---|---|---|---|---|
| `read` | `FSRead` | Read a file (text, images, PDFs, notebooks) | Yes — Read | [read](read.md) |
| `write` | `FSWrite` | Create or overwrite a file | Yes — Write | [write](write.md) |
| `fs_search` | `FSSearch` | Regex search over files (ripgrep-backed) | Yes — Read | [fs_search](fs_search.md) |
| `sem_search` | `SemanticSearch` | Natural-language semantic code search | No | [sem_search](sem_search.md) |
| `remove` | `FSRemove` | Delete a file | Yes — Write | [remove](remove.md) |
| `patch` | `FSPatch` | Exact string replacement in one file | Yes — Write | [patch](patch.md) |
| `multi_patch` | `FSMultiPatch` | Atomic sequence of edits to one file | Yes — Write | [multi_patch](multi_patch.md) |
| `undo` | `FSUndo` | Revert last file operation on a path | No | [undo](undo.md) |
| `shell` | `Shell` | Execute a shell command | Yes — Execute | [shell](shell.md) |
| `fetch` | `NetFetch` | Retrieve a URL as markdown or raw text | Yes — Fetch | [fetch](fetch.md) |
| `followup` | `Followup` | Ask the human a clarifying question | No | [followup](followup.md) |
| `plan` | `PlanCreate` | Write a Muse plan file under `plans/` | No | [plan](plan.md) |
| `skill` | `SkillFetch` | Load a skill by name | No | [skill](skill.md) |
| `todo_write` | `TodoWrite` | Create / update / remove todo items | No | [todo_write](todo_write.md) |
| `todo_read` | `TodoRead` | Read the current session todos | No | [todo_read](todo_read.md) |
| `task` | `TaskInput` | Delegate work to another agent | No | [task](task.md) |

Source of truth for this table: the `ToolCatalog` enum (`catalog.rs:41-61`) and `to_policy_operation` (`catalog.rs:926-1017`). Run `aimee list tool aimee` to print the live catalog.

## How names are resolved

Model output is normalized before matching: surrounding whitespace is trimmed and lookup is case-insensitive, so `Read`, `READ`, and `" read "` all resolve to the canonical `read`. Legacy aliases `Read` → `read`, `Write` → `write`, and `Task` → `task` remain accepted (`catalog.rs:42-60`). Unknown names pass through unchanged and then fail catalog matching downstream.

Arguments are also repaired before parsing: when a model sends `"10"` where the schema wants `10` (string vs integer), `aimee_json_repair::coerce_to_schema` coerces values against each tool's generated JSON schema (`catalog.rs:1167-1173`). This is why slightly malformed calls often succeed instead of erroring.

## Schemas

Each input struct derives `JsonSchema` (schemars), so every tool has a machine-generated JSON Schema served to the model alongside its description. Optional fields are nullable; unknown fields are rejected on tools marked `deny_unknown_fields` (currently `read`). Snapshot tests pin every schema: changing a field updates `catalog.rs`'s insta snapshot, which CI treats as an error if not accepted deliberately.

Descriptions themselves come from markdown files under [`crates/aimee_domain/src/tools/descriptions/`](https://github.com/swcstudiospace/omegaloops/tree/main/crates/aimee_domain/src/tools/descriptions), attached via the `#[tool_description_file = "..."]` attribute. They support Handlebars templating (`{{config.*}}`, `{{tool_names.*}}`, `{{env.cwd}}`) resolved at render time.

## Rules for new tools

1. Add the input struct and `ToolCatalog` variant in `aimee_domain`.
2. Add the description `.md` under `descriptions/` and wire it with `#[tool_description_file]`.
3. Register routing in `ToolRegistry` (`aimee_app`). Do not leave an unregistered variant.
4. Decide the permission posture in `to_policy_operation`: file/network-touching tools must return a `PermissionOperation`; internal bookkeeping tools return `None`.
5. Keep descriptions under **1024 characters** (the repo's `docs/tool-guidelines.md`).
6. Update the snapshots (`cargo insta test --accept -p aimee_domain`) and this reference section.

## Behavior shared by all tools

- **Timeout**: every tool call is wrapped in `tokio::time::timeout` using `tool_timeout_secs` (default `300`s). On expiry the registry returns `Error::CallTimeout`. See [Reliability](../../reliability.md).
- **Restricted mode**: permission-gated tools prompt for a grant before execution; the check runs *before* the timeout window starts. See [Security](../../security.md).
- **Yield**: `followup` yields control back to the human — the turn ends after it runs (`ToolCatalog::should_yield`).
- **stdout**: only `shell` requires direct stdout/stderr access (`ToolCatalog::requires_stdout`).

## Related

- [Tools overview](../../tools.md) — how the registry routes catalog, agent, and MCP tools
- Per-tool pages: [read](read.md) · [write](write.md) · [fs_search](fs_search.md) · [sem_search](sem_search.md) · [remove](remove.md) · [patch](patch.md) · [multi_patch](multi_patch.md) · [undo](undo.md) · [shell](shell.md) · [fetch](fetch.md) · [followup](followup.md) · [plan](plan.md) · [skill](skill.md) · [todo_write](todo_write.md) · [todo_read](todo_read.md) · [task](task.md)
- [Application architecture](../../architecture/app.md) — `ToolRegistry` and executors
- [Security](../../security.md) — restricted mode and permission grants
