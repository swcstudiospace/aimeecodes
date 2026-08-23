# API composition root — aimee_api

The layer where the world gets built. Every dependency is created and wired here; nothing below composes anything sideways.

## Startup sequence

`AimeeAPI::init(cwd, config)` in `src/aimee_api.rs`:

```rust,ignore
let infra    = AimeeInfra::new(cwd, config);   // fs, http, auth, mcp, grpc
let repo     = AimeeRepo::new(infra);          // persistence over infra
let services = AimeeServices::new(repo);       // use cases over infra
let api      = AimeeAPI::new(services, repo);  // public surface
```

All four hold their dependencies as `Arc`s. Lifetimes belong to this layer — that's what makes every other layer testable with substitutes.

## The public trait

`pub trait API: Sync + Send` (`src/api.rs`) is the contract presentation code consumes. `aimee_main` never builds services itself; it asks for an initialized API. This is why the TUI, CLI subcommands, and ZSH-driven runs behave identically: same root, same wiring.

## What crosses here

* Configuration arrives as a parsed `AimeeConfig` (from `aimee_config`).
* Services expose use cases; DTOs from `aimee_app` cross upward.
* Infra traits (filesystem, HTTP, auth, MCP, gRPC) resolve downward into `aimee_infra` implementations.

## Rules for changes

New capability? Add the service method, expose it on `API`, wire nothing extra — init already passes the whole graph. Resist adding composition logic anywhere else; a second place that builds dependencies is an architecture bug.

## See also

* [Architecture overview](overview.md)
* [Infrastructure](infra.md)
* [Terminal UI](../surfaces/tui.md)

<!-- sources: crates/aimee_api/src/{aimee_api.rs,api.rs}, AGENTS.md -->
