# Services

`aimee_services` holds application services. Each service is generic over **one** infra/repo parameter `F`, stores `Arc<F>`, implements `new` without trait bounds, and applies bounds on methods. Services do **not** call other services.

The container is `AimeeServices<F>` (`crates/aimee_services/src/aimee_services.rs:43-87`). `new` wires every child from the same `Arc<F>` (`crates/aimee_services/src/aimee_services.rs:111-175`).

Crate root (`crates/aimee_services/src/lib.rs:1-56`).

## Purpose

Implement the ports declared in `aimee_app::services` (`ProviderService`, `FsReadService`, `PolicyService`, …) so `AimeeApp` can run without knowing about Diesel, reqwest, or the terminal.

Also owns `IntoDomain` / `FromDomain` for mapping external DTOs (`crates/aimee_services/src/lib.rs:38-56`).

## When to use

| Use case | Type | Module |
|---|---|---|
| Chat + model list + credentials | `AimeeProviderService<F>` | `provider_service.rs` |
| Provider login / refresh | `AimeeProviderAuthService<F>` | `provider_auth.rs` |
| Restricted-mode grants | `AimeePolicyService<F>` | `policy.rs` |
| Conversation persistence | `AimeeConversationService<F>` | `conversation.rs` |
| MCP merge + trust gate | `AimeeMcpManager` / `AimeeMcpService` (associated type `McpService`) | `mcp/` |
| Semantic workspace | `AimeeWorkspaceService` | `context_engine` |
| Tool IO (read/write/patch/…) | `AimeeFs*` / `AimeeShell` / `AimeeFetch` | `tool_services/` |
| Session / commit / suggest config | `AimeeAppConfigService<F>` | `app_config.rs` |
| Agent roster | `AimeeAgentRegistryService<F>` | `agent_registry.rs` |

Add a **new** service only when there is a new use case. Do not add a "utils" service.

## File interactions

`AimeeAPI::init` constructs `AimeeServices::new(repo)` (`crates/aimee_api/src/aimee_api.rs:51-55`). `F` in production is `AimeeRepo<AimeeInfra>` — persistence plus infra, still one generic.

Who calls whom:

| Caller | Service | Port |
|---|---|---|
| `AimeeApp` / `ToolExecutor` | `AimeeFsRead`, `AimeeFsWrite`, `AimeeFsPatch`, `AimeeFsSearch`, `AimeeFsRemove`, `AimeeFsUndo`, `AimeeShell`, `AimeeFetch`, `AimeeFollowup`, `AimeePlanCreate`, `AimeeSkillFetch`, `AimeeImageRead` | matching `*Service` traits |
| `ToolRegistry` | `AimeePolicyService` | `PolicyService::check_operation_permission` |
| `AimeeApp` chat | `AimeeProviderService`, `AimeeProviderAuthService` | `ProviderService`, `ProviderAuthService` |
| `AimeeAPI` | same container via `Services` accessors | `API` methods |

`tool_services/mod.rs` is the FS/shell/skill family (`crates/aimee_services/src/tool_services/mod.rs:1-25`). They talk to infra traits (`FileReaderInfra`, `CommandInfra`, …), not to each other.

### Policy

`AimeePolicyService` loads embedded defaults from `permissions.default.yaml` (`crates/aimee_services/src/policy.rs:34-40`) and user grants from the environment permissions path (`crates/aimee_services/src/policy.rs:50-52`). Interactive outcomes are `Accept`, `Reject`, `AcceptAndRemember` (`crates/aimee_services/src/policy.rs:18-28`). Domain still owns `Permission` / `PermissionOperation`.

### Providers

`AimeeProviderService` renders Handlebars URL templates (including optional `null` params) then calls `ChatRepository` / `ProviderRepository` (`crates/aimee_services/src/provider_service.rs:15-75`). `AimeeProviderAuthService` builds an auth strategy from `StrategyFactory` and completes it (`crates/aimee_services/src/provider_auth.rs:22-56`).

MCP: `init_mcp` is the startup trust gate; connections stay lazy until first tool use (`crates/aimee_app/src/services.rs:225-233`).

## How to use

Do not instantiate a single child service from the CLI. Go through `AimeeAPI`:

```bash
aimee provider login openai
aimee provider list --type llm
aimee list mcp
aimee workspace sync .
```

Config flags these services honor (placeholders only):

```toml
restricted = true
tool_supported = true
tool_timeout_secs = 300
max_read_lines = 2000
max_fetch_chars = 40000
services_url = "https://api.aimeecodes.dev/api"
```

`AimeeConfig` field docs: `crates/aimee_config/src/config.rs:122-337`.

Rust shape to copy when adding a service (`AGENTS.md:141-151`):

```rust
pub struct ExampleService<F> {
    infra: Arc<F>,
}

impl<F> ExampleService<F> {
    pub fn new(infra: Arc<F>) -> Self { Self { infra } }
}

impl<F: SomeInfra> ExampleService<F> {
    pub async fn run(&self) -> anyhow::Result<()> { /* bounds on the method */ }
}
```

Wire the `Arc` in `AimeeServices::new` only.

## Best practices

- **No service-to-service calls.** If `AimeeFsWrite` needs a snapshot, take a repository port on `F`, or compose at `AimeeServices::new`.
- One generic. No `Box<dyn …>` fields. No trait bounds on `new()`.
- Map driver errors to `anyhow` / domain `Error` here. Do not leak Diesel or tonic types through a port.
- `AimeeFetch::new()` is the exception that takes no infra — keep that rare.
- Prefer tuple structs when there is a single dependency.

## Anti-patterns

| Don't | Do |
|---|---|
| `AimeeShell` calling `AimeePolicyService` | Registry checks policy, then calls shell |
| Second generic "for the logger" | Use `tracing` |
| `Box<dyn FsReadService>` on the struct | `Arc<F>` + bounds on methods |
| Catch-all `anyhow!("failed")` | Context + typed domain error when the caller must branch |
| New HTTP client inside a tool service | `HttpInfra` already on `F` |
| God `AimeeServices` methods that bypass children | Add a child + port |

## Verify

```bash
cargo fmt
cargo check -p aimee_services
cargo clippy -p aimee_services --all-targets -- -D warnings
cargo insta test --accept -p aimee_services
```

A colocated fixture → actual → expected example is `AimeeSkillFetch` (`crates/aimee_services/src/tool_services/skill.rs`).

Never `cargo build --release` for this.

## Related

- [Application](app.md) — ports this crate implements
- [Composition root](api.md) — who constructs `AimeeServices`
- [Infrastructure](infra.md) — `F` adapters
- [Best practices](../best-practices.md) — Rust service shape
