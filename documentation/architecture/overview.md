# Architecture overview

Aimee Codes is a Rust 2024 Cargo workspace organized as clean architecture: dependencies point inward, infrastructure is injected, and the composition root owns lifetimes. This page gives the map; each layer has its own page in this section.

## The stack

```text
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
   aimee_repo          persistence       AimeeRepo<F> (Diesel/SQLite)
        │
   aimee_infra         infrastructure    AimeeInfra
        │
   aimee_domain        types, tools, policies
   aimee_config        .aimee.toml schema + IO
```

## Composition at startup

`AimeeAPI::init(cwd, config)` (`crates/aimee_api/src/aimee_api.rs`) wires the world in four steps:

1. `AimeeInfra::new(cwd, config)` — filesystem, HTTP, auth, MCP, gRPC implementations
2. `AimeeRepo::new(infra)` — persistence over those implementations
3. `AimeeServices::new(repo)` — application services generic over infra
4. `AimeeAPI::new(services, repo)` — the public `API` trait surface

Everything is held in `Arc`s; nothing reaches across layers sideways.

## The invariants

These are enforced by house policy and code review, not just convention:

* **No service-to-service calls.** If two use cases must collaborate, compose them at the composition root.
* **Services take at most one generic parameter**, store infra as `Arc<T>`, put trait bounds on methods rather than constructors, and never hold `Box<dyn>` in fields.
* **Domain errors are typed** with `thiserror`; services and CLI use `anyhow`. No `From` impls that collapse distinct failures.
* **Invalid states are unrepresentable** — newtypes, enums, branded IDs over strings.
* **Migrations are append-only artifacts.**

## The supporting cast

Beyond the core loop: WEB3 crates (`aimee_anda`, `aimee_anda_icp`), presentation helpers (`aimee_display`, `aimee_markdown_stream`, `aimee_spinner`, `aimee_select`, `aimee_tracker`), file/tooling crates (`aimee_fs`, `aimee_walker`, `aimee_embed`, `aimee_template`, `aimee_tool_macros`, `aimee_json_repair`, `aimee_snaps`), streaming (`aimee_eventsource`, `aimee_eventsource_stream`, `aimee_stream`), and CI/tests (`aimee_ci`, `aimee_test_kit`). Full inventory at [Crate map](crates.md).

## Where to look for what

| You want to change... | Look in |
|---|---|
| A tool's behavior | `aimee_domain` (catalog) + `aimee_app` (registry) |
| How a command works | `aimee_main/src/cli.rs` → its handler |
| Provider wire behavior | `aimee_infra` (HTTP/auth) + `aimee_config` (provider defs) |
| Storage/schema | `aimee_repo` (migrations are additive) |
| Agent prompts | `aimee_repo/src/agents/*.md`, `templates/` |
| CI workflows | `aimee_ci` generator (not YAML) |

## See also

* [Crate map](crates.md)
* [Domain](domain.md)
* [API composition root](api.md)

<!-- sources: AIMEE.md §4,§5, crates/aimee_api/src/aimee_api.rs, AGENTS.md -->
