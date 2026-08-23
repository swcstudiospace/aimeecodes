# Architecture overview

Aimee Codes is a clean-architecture loop. Presentation (`aimee_main`) talks to a composition root (`aimee_api`). The root wires orchestration (`aimee_app`), application services (`aimee_services`), persistence (`aimee_repo`), and infrastructure (`aimee_infra`). Domain types live in `aimee_domain`. Config schema lives in `aimee_config`.

This page is the human map. Policy stays in `AGENTS.md`. Product discovery stays in `AIMEE.md`. When those files disagree with this GitBook, the product tree wins.

## Purpose

Explain how crates call each other so a human can:

- Find the right crate before changing a type, tool, or provider
- See where composition happens (and where it must not)
- Use the public `API` trait without inventing RPCs

## When to use

| You need to… | Start here | Then |
|---|---|---|
| Understand the loop | This page | [Domain](domain.md) |
| Add or change a use case | [Application](app.md) | [Services](services.md) |
| Add a public method | [Composition root](api.md) | `API` trait |
| Touch FS / HTTP / MCP / gRPC | [Infrastructure](infra.md) | Do not put IO in domain |
| Add a tool | [Tools](../tools.md) | Catalog + registry |
| Add or login a model vendor | [Providers](../providers.md) | `ProviderId` constants |

Do **not** use this page for ops, security, PWA, or wallet work. Those live under other `aimeecode/` sections.

## Stack

```
CLI / TUI / ZSH / PWA
        │
   aimee_main          presentation (clap, ratatui, rustyline)
        │
   aimee_api           composition root  AimeeAPI::init
        │
   aimee_app           orchestration     AimeeApp + Orchestrator + ToolRegistry
        │
   aimee_services      application services  AimeeServices<F>
        │
   aimee_repo          persistence       AimeeRepo<F>
        │
   aimee_infra         infrastructure    AimeeInfra
        │
   aimee_domain        types, tools, policies
   aimee_config        .aimee.toml schema + IO
```

Same diagram as `AIMEE.md:84-101`. Workspace members are `crates/*`. Shared versions live in root `Cargo.toml` `[workspace.dependencies]`.

## File interactions

Startup composition (`crates/aimee_api/src/aimee_api.rs:44-56`):

1. `AimeeInfra::new(cwd, config)`
2. `AimeeRepo::new(infra)`
3. `AimeeServices::new(repo)`
4. `AimeeAPI::new(services, repo)`

Who calls whom on a chat turn:

| Caller | Calls | Why |
|---|---|---|
| `aimee_main` CLI / TUI | `API` trait on `AimeeAPI` | Presentation never owns IO |
| `AimeeAPI::chat` | `AimeeApp::chat` | Composition root builds a fresh app (`crates/aimee_api/src/aimee_api.rs:138-147`) |
| `AimeeApp` | `Orchestrator`, `ToolRegistry`, `Services` ports | Prompt, tools, model, hooks (`crates/aimee_app/src/app.rs:44-56`) |
| `Orchestrator` | `AgentService` (`services.call`) | Sequential tools; `task` in parallel (`crates/aimee_app/src/orch.rs:57-99`) |
| `ToolRegistry` | `ToolExecutor` / `AgentExecutor` / `McpExecutor` | Catalog, agent, or MCP route (`crates/aimee_app/src/tool_registry.rs:93-210`) |
| `ToolExecutor` | `Services` FS / shell / fetch / plan / skill | One port per use case (`crates/aimee_app/src/tool_executor.rs:151-339`) |
| `AimeeServices<F>` | Injected `F` (repo + infra) | Services do not call each other (`crates/aimee_services/src/aimee_services.rs:111-175`) |

Built-in agent IDs are first-class: `AgentId::AIMEE`, `AgentId::MUSE`, `AgentId::SAGE` (`crates/aimee_domain/src/agent.rs:37-39`). Default agent is Aimee (`crates/aimee_domain/src/agent.rs:42-45`).

## How to use the APIs

Humans hit the CLI. The CLI hits `API`. Do not call services from presentation.

```bash
# Interactive TUI
aimee

# One-shot
aimee -p "explain crates/aimee_api/src/aimee_api.rs"

# Inspect what the composition root exposes
aimee list agent
aimee list provider
aimee list tool aimee
aimee info
```

`TopLevelCommand::List` is the inventory surface (`crates/aimee_main/src/cli.rs:99`, `crates/aimee_main/src/cli.rs:529-609`). Provider login is `aimee provider login` (`crates/aimee_main/src/cli.rs:968-1004`).

From Rust, the public surface is the `API` trait (`crates/aimee_api/src/api.rs:13-47`). Construct the production type with `AimeeAPI::init(cwd, config)` — do not new up `AimeeServices` from a binary.

Credentials live under the config base as `.credentials.json`. Use placeholders in examples:

```toml
# ~/.aimee/.aimee.toml  — do not commit real keys
session = { provider = "openai", model = "gpt-4o" }
restricted = true
tool_timeout_secs = 300
```

## Best practices

- **Compose at the root.** If two use cases must collaborate, wire them in `AimeeAPI::init` / `AimeeServices::new`, not inside a service (`AGENTS.md:108-111`, `AIMEE.md:112`).
- **One generic per service.** Store infra as `Arc<T>`. Trait bounds on methods, not `new()` (`AGENTS.md:126-131`).
- **Domain errors are `thiserror`.** Services and CLI use `anyhow`. Do not `From`-collapse distinct failures (`AIMEE.md:114`, `crates/aimee_domain/src/error.rs:9-12`).
- **Invalid states unrepresentable.** `AgentId`, `ProviderId`, `GoalProbeSet` (exactly five probes) — not string modes (`crates/aimee_domain/src/loop_autonomy.rs:39-54`).
- **Search the tree.** If a crate, tool, flag, or provider ID is not in source, it does not exist.

## Anti-patterns

| Don't | Do |
|---|---|
| Service calling another service | Compose at `AimeeAPI` / `AimeeServices::new` |
| Business rules in `aimee_main` or HTTP adapters | Domain type + application service |
| Inventing a crate or RPC | Search `crates/` first |
| Putting FS / HTTP in `aimee_domain` | Port in `aimee_app::services`, impl in `aimee_infra` |
| Leaking Diesel / tonic / reqwest errors across the `API` trait | Map to `aimee_domain::Error` or `anyhow` at the edge |
| `cargo build --release` to "check architecture" | `cargo check -p <crate>` |

## Verify

Docs-only change in this repo:

```bash
python3 scripts/verify-docs.py
```

If you later touch the loop crates (do not do that from this docs task):

```bash
cargo fmt
cargo check -p aimee_domain -p aimee_app -p aimee_services -p aimee_api -p aimee_infra
cargo clippy -p aimee_domain -p aimee_app -p aimee_services -p aimee_api -p aimee_infra --all-targets -- -D warnings
cargo insta test --accept -p aimee_domain -p aimee_app
```

Never `cargo build --release` for verification (`AGENTS.md:370-371`).

## Related

- [Domain](domain.md) — types, catalog, policies, loop autonomy
- [Application](app.md) — `AimeeApp`, orchestrator, tool routing
- [Services](services.md) — `AimeeServices<F>` and ports
- [Composition root](api.md) — `AimeeAPI` + `API`
- [Infrastructure](infra.md) — FS, HTTP, MCP, gRPC
- [Tools](../tools.md) · [Providers](../providers.md)
- Product map: `AIMEE.md` §4–5 · policy: `AGENTS.md` architecture
