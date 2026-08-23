# Tools

Tools are how agents touch the tree. The catalog is domain. Routing is app. IO is services + infra. MCP and agent-as-tool sit beside the catalog, not inside it.

Policy for descriptions: `docs/tool-guidelines.md`. Contributor digest: [Best practices](best-practices.md).

## Purpose

Give humans a map of:

- Which tools exist (`ToolCatalog`)
- Who is allowed to call them (`ToolResolver` + agent tool lists)
- How a call is executed (`ToolRegistry` → executor)
- How to add one without inventing a second registry

## When to use

| Situation | Tool | When **not** |
|---|---|---|
| Read a file (optional line range) | `read` | Semantic "where is X?" — use `sem_search` / `fs_search` |
| Create or overwrite a file | `write` | Surgical edit of an existing file — `patch` / `multi_patch` |
| Regex / glob search | `fs_search` | Conceptual search on an indexed workspace — `sem_search` |
| Indexed semantic search | `sem_search` | CWD not indexed or not authenticated (hidden) |
| Delete a file | `remove` | Undo last write — `undo` |
| Single-file exact replace | `patch` | Many hunks — `multi_patch`; you have not `read` yet |
| Sequential hunks on one file | `multi_patch` | Unrelated files — one call per file |
| Revert last FS op | `undo` | Git revert of committed work — `shell` |
| Project command / tests | `shell` | Reading a file — `read` |
| HTTP(S) URL → markdown/text | `fetch` | Repo files — `read` |
| Ask the human a question | `followup` | Logging a decision — conversation text |
| Muse plan file | `plan` | Implementing the plan — that is Aimee + patch/write |
| Load a skill body | `skill` | Defining a new skill file — see skills page |
| Session todo list | `todo_write` / `todo_read` | GitHub issues |
| Delegate to `aimee` / `muse` / `sage` | `task` | When `subagents = false` |

## File interactions

```text
LLM tool call
    │
ToolRegistry::call_inner          crates/aimee_app/src/tool_registry.rs:93-210
    ├─ ToolCatalog::contains  →  Task? AgentExecutor
    │                         →  else ToolExecutor (timed)
    ├─ agent tool name        →  AgentExecutor
    └─ MCP tool name          →  McpExecutor (timed)
    │
ToolExecutor::execute             crates/aimee_app/src/tool_executor.rs:342-387
    │  read-before-edit gate
    ▼
ToolExecutor::call_internal       crates/aimee_app/src/tool_executor.rs:151-339
    │
Services ports                    AimeeFs*, AimeeShell, AimeeFetch, …
    │
AimeeInfra / AimeeRepo
```

Catalog enum (`crates/aimee_domain/src/tools/catalog.rs:41-61`):

| Variant | Input struct | Description file |
|---|---|---|
| `Read` | `FSRead` | `tools/descriptions/fs_read.md` |
| `Write` | `FSWrite` | `fs_write.md` |
| `FsSearch` | `FSSearch` | `fs_search.md` |
| `SemSearch` | `SemanticSearch` | `semantic_search.md` |
| `Remove` | `FSRemove` | `fs_remove.md` |
| `Patch` | `FSPatch` | `fs_patch.md` |
| `MultiPatch` | `FSMultiPatch` | `fs_multi_patch.md` |
| `Undo` | `FSUndo` | `fs_undo.md` |
| `Shell` | `Shell` | `shell.md` |
| `Fetch` | `NetFetch` | `net_fetch.md` |
| `Followup` | `Followup` | `followup.md` |
| `Plan` | `PlanCreate` | `plan_create.md` |
| `Skill` | `SkillFetch` | `skill_fetch.md` (`catalog.rs:686-691`) |
| `TodoWrite` / `TodoRead` | `TodoWrite` / `TodoRead` | `todo_write.md` / `todo_read.md` |
| `Task` | `TaskInput` | `task.md` (`catalog.rs:75-95`) |

`tools_overview` builds system + agent + MCP lists (`crates/aimee_app/src/tool_registry.rs:243-288`). `sem_search` is omitted unless the CWD is indexed **and** authenticated (`:250-254`, `:332-339`). `research_subagent = false` drops `sage` / `agent` from the Task description (`:262-268`).

Agent allow-lists are globs. Aliases: `search`→`fs_search`, `Read`→`read`, `Write`→`write`, `Task`→`task` (`crates/aimee_app/src/tool_resolver.rs:12-19`).

`subagents` (config) is applied in `aimee_repo` when loading the `aimee` agent: strip `task`/`sage`, then re-insert `task` if enabled (`crates/aimee_repo/src/agent.rs:165-187`).

Orchestrator runs `task` calls in parallel and everything else sequentially (`crates/aimee_app/src/orch.rs:57-99`).

Restricted mode: `ToolRegistry` asks `PolicyService` **before** the timeout (`crates/aimee_app/src/tool_registry.rs:140-153`). Operations map to `PermissionOperation` `{Read,Write,Execute,Fetch}`.

## How to use

```bash
# Tools visible to a given agent (after resolver + config)
aimee list tool aimee
aimee list tool muse
aimee list tool sage

aimee list mcp
aimee list skill
```

`ListCommand::Tool` takes an `AgentId` (`crates/aimee_main/src/cli.rs:573-578`).

```toml
# ~/.aimee/.aimee.toml
subagents = true
research_subagent = false
restricted = true
tool_supported = true
tool_timeout_secs = 300
max_read_lines = 2000
max_fetch_chars = 40000
max_stdout_prefix_lines = 200
max_stdout_suffix_lines = 200
```

Adding a tool (do not skip a step):

1. Variant + input + `#[tool_description_file]` on `ToolCatalog`.
2. Description markdown ≤ **1024 characters** (`docs/tool-guidelines.md:22`).
3. Arm in `ToolExecutor::call_internal` (or registry, if it is `Task`-like).
4. Service port + `AimeeServices` impl if IO is new.
5. Policy mapping if it reads/writes/executes/fetches.

## Best practices

- Descriptions: what, when, when not, parameters, limits. Examples last (`docs/tool-guidelines.md:8-27`).
- Prefer `patch` / `multi_patch` over rewriting a whole file.
- Always `read` before overwrite or patch — the executor enforces it (`crates/aimee_app/src/tool_executor.rs:351-367`).
- `shell` is for project verify commands (`cargo check`, `cargo clippy`, …), not for implementing `rg` (that is `fs_search`).
- Truncation dumps (`aimee_shell_*`, `aimee_fetch_*`) are temp files — do not commit them (`crates/aimee_app/src/tool_executor.rs:68-112`).
- MCP servers are trusted at startup, connected lazily (`McpService::init_mcp`).

## Anti-patterns

| Don't | Do |
|---|---|
| Unregistered `ToolCatalog` variant | Route it in the registry / executor |
| Description > 1024 characters | Trim; tests fail otherwise |
| Second tool registry | `ToolRegistry` is the only one |
| `shell` to read a file | `read` |
| `write` overwrite without a prior `read` | Executor will error |
| Handling `Task` in `ToolExecutor` | `unreachable!` on purpose |
| Inventing tool names (`grep`, `bash`, `python`) | Use catalog names |
| Timing out the permission prompt | Policy runs before `timeout` |

## Verify

```bash
cargo fmt
cargo check -p aimee_domain -p aimee_app -p aimee_services
cargo clippy -p aimee_domain -p aimee_app -p aimee_services --all-targets -- -D warnings
cargo insta test --accept -p aimee_domain -p aimee_app
```

Never `cargo build --release`.

## Related

- [Tool catalog](reference/tools/catalog.md) — per-tool reference: parameters, examples, errors, permissions
- [Domain](architecture/domain.md) · [Application](architecture/app.md)
- [Skills and commands](skills.md)
- `docs/tool-guidelines.md` · `AGENTS.md` tool section
