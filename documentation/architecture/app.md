# Application

`aimee_app` is the orchestration crate. It owns `AimeeApp`, `Orchestrator`, `ToolRegistry` / `ToolExecutor` / `ToolResolver`, the `Services` port bundle, git/commit helpers, and optional Anda pathway hooks. It does **not** implement FS, HTTP, or Diesel. Those arrive as injected traits.

Crate root (`crates/aimee_app/src/lib.rs:1-65`).

## Purpose

Turn a `ChatRequest` into a stream of `ChatResponse` values: load conversation, resolve agent + provider + tools, build prompts, run the tool loop, persist side effects through ports.

## When to use

| Task | Module |
|---|---|
| Chat / compact / list tools | `app.rs` (`AimeeApp`) |
| Tool-call loop, `task` parallelism | `orch.rs` |
| Route catalog / agent / MCP tools | `tool_registry.rs` |
| Execute a `ToolCatalog` variant | `tool_executor.rs` |
| Filter tools for an agent (globs, aliases) | `tool_resolver.rs` |
| Declare a new port | `services.rs` |
| Session pathway checkpoints | `anda_pathway.rs` |

Do not put clap / ratatui here (`aimee_main`). Do not put reqwest / tonic impls here (`aimee_infra`). Do not compose two services by calling each other — the composition root owns that.

## File interactions

`AimeeApp` holds `Arc<S>` and a `ToolRegistry<S>` (`crates/aimee_app/src/app.rs:47-56`). `chat` (`crates/aimee_app/src/app.rs:60-110`):

1. `ConversationService::find_conversation` — missing ID is `Error::ConversationNotFound`
2. `list_current_directory`, `get_custom_instructions`, `get_agent`
3. `AgentProviderResolver` + `ProviderAuthService::refresh_provider_credential`
4. `ToolRegistry::list` → `ToolResolver::resolve(&agent)`
5. System prompt + user prompt + changed-files + metrics
6. Optional Anda hooks when `aimee_config.anda` is enabled (`crates/aimee_app/src/app.rs:153-160`)
7. `Orchestrator` runs the turn

`AimeeAPI::chat` is the only production caller of `AimeeApp::chat` (`crates/aimee_api/src/aimee_api.rs:138-147`). The API builds a fresh `AimeeApp` via `app()` (`crates/aimee_api/src/aimee_api.rs:34-41`).

### Orchestrator

`Orchestrator<S>` (`crates/aimee_app/src/orch.rs:19-49`) is bounded on `AgentService`. It partitions tool calls: `task` / `Task` run in parallel with `join_all`; every other tool runs sequentially so UI notifiers and hooks stay ordered (`crates/aimee_app/src/orch.rs:57-99`).

### Tool routing

`ToolRegistry::call_inner` (`crates/aimee_app/src/tool_registry.rs:93-210`):

1. `ToolCatalog::contains` → catalog tool. `Task` goes to `AgentExecutor` (no timeout). Other catalog tools: restricted-mode policy check, then `ToolExecutor` under `tool_timeout_secs`.
2. Else agent-as-tool → `AgentExecutor`.
3. Else MCP → `McpExecutor` (timed).
4. Else `Error::NotFound`.

Restricted mode reads `AimeeConfig.restricted`. Permission denials return `ToolOutput` text `permission_denied`, they do not panic (`crates/aimee_app/src/tool_registry.rs:140-153`). Timeouts use `tool_timeout_secs` (`crates/aimee_app/src/tool_registry.rs:45-60`). Policy checks run **before** the timeout.

`ToolExecutor::execute` enforces read-before-edit for `Patch` / `MultiPatch` and overwrite `Write` (`crates/aimee_app/src/tool_executor.rs:351-367`). `Task` is `unreachable!` here — registry must have consumed it (`crates/aimee_app/src/tool_executor.rs:335-338`).

`ToolResolver` matches the agent's tool list with globs (`fs_*`) and aliases `search`→`fs_search`, `Read`→`read`, `Write`→`write`, `Task`→`task` (`crates/aimee_app/src/tool_resolver.rs:12-43`).

### Services trait

`Services` is the port bundle (`crates/aimee_app/src/services.rs:543-599`): provider, config, conversation, template, attachment, discovery, MCP, FS family, shell, fetch, follow-up, policy, workspace, skill, auth, agent registry. `AimeeServices<F>` implements it. Orchestration never names `AimeeFsRead` — it names `FsReadService`.

## How to use

```bash
# Drive orchestration through the public API
aimee -p "list the tools this agent can call"
aimee list tool aimee
```

Enable / disable the `task` tool via config (applied when agents are loaded in `aimee_repo`, not inside the executor):

```toml
# ~/.aimee/.aimee.toml
subagents = true           # aimee agent gets `task`
research_subagent = false  # hide sage from Task description unless you want it
restricted = true
tool_timeout_secs = 300
verify_todos = true
```

`subagents` inserts `task` on the `aimee` agent (`crates/aimee_repo/src/agent.rs:165-187`). `research_subagent` controls whether `sage` / `agent` appear in the Task description (`crates/aimee_app/src/tool_registry.rs:262-268`). `verify_todos` adds `PendingTodosHandler` on turn end (`crates/aimee_app/src/app.rs:164-171`).

Anda pathway hooks are off unless `anda.enabled` (`crates/aimee_app/src/anda_pathway.rs:63-70`). Details: WEB3 pages, not this file.

## Best practices

- Depend on `Services` / individual ports, not on `AimeeServices` field names.
- Keep new use-case methods on `AimeeApp` or a dedicated `*App` (see `GitApp`, `DataGenerationApp`) and expose them through `API`.
- Honor cancellation and `tool_timeout_secs`. Do not time out permission prompts.
- Parallelize only what the orchestrator already treats as independent (`task`). Sequential tools exist for handshake reasons.
- Map domain errors at this layer with context. Do not swallow `ConversationNotFound`.

## Anti-patterns

| Don't | Do |
|---|---|
| Call `AimeeFsWrite` from `AimeeApp` | Call `FsWriteService` |
| Execute `Task` inside `ToolExecutor` | Leave it on `ToolRegistry` → `AgentExecutor` |
| Skip `require_prior_read` for patches | Keep the read-before-edit gate |
| Add a second HTTP client in this crate | Use `HttpInfra` / `NetFetchService` |
| Nest Sage/Muse/Aimee as orchestrators | Aimee dispatches `task`; specialists stay in lane |
| Re-plan inside `AimeeApp` | Muse writes `plans/`; Aimee executes |

## Verify

```bash
cargo fmt
cargo check -p aimee_app
cargo clippy -p aimee_app --all-targets -- -D warnings
cargo insta test --accept -p aimee_app
```

`orch_spec` is in-crate (`crates/aimee_app/src/lib.rs:22-23`). Prefer `cargo check` over `cargo build`. Never `--release` for verification.

## Related

- [Domain](domain.md) types this crate orchestrates
- [Services](services.md) implementations of the ports
- [Tools](../tools.md) catalog routing table
- [Tool catalog reference](../reference/tools/catalog.md) — per-tool pages
- [Composition root](api.md)
