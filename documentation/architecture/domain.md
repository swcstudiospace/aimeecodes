# Domain

`aimee_domain` is the innermost crate. It owns types, errors, the tool catalog, provider IDs, policies, teams, and loop-autonomy probes. It does **not** own IO. Other crates depend on it. It does not depend on `aimee_app`, `aimee_services`, `aimee_api`, or `aimee_infra`.

Crate root re-exports every module (`crates/aimee_domain/src/lib.rs:1-133`).

## Purpose

Make invalid states unrepresentable and keep the loop vocabulary in one place:

- Branded IDs (`AgentId`, `ProviderId`, `ConversationId`)
- Tool input schemas (`ToolCatalog`)
- Permission operations and `Permission` outcomes
- Typed domain errors (`thiserror`)
- HITL goal probes (`GoalProbeSet`)

## When to use

| Change | Lives here | Does not live here |
|---|---|---|
| New tool input / description | `tools/catalog.rs` + `tools/descriptions/` | Executor, HTTP client |
| New provider ID | `provider.rs` constants | Config TOML only (unless it is a custom id) |
| New agent id constant | `agent.rs` | Prompt markdown (that is `aimee_repo`) |
| New permission kind | `policies/` | UI confirm prompt (that is `aimee_services::policy`) |
| New domain error | `error.rs` | `anyhow` at the CLI edge |

Use this crate when the type must be shared by app, services, API, and infra. If the type is an adapter (reqwest client, Diesel row), it is not domain.

## File interactions

Who reads domain types:

| Module | Callers | Role |
|---|---|---|
| `agent.rs` | App, services, CLI | `AgentId::AIMEE` / `MUSE` / `SAGE` (`crates/aimee_domain/src/agent.rs:37-39`) |
| `tools/catalog.rs` | App registry + executor | `ToolCatalog` variants (`crates/aimee_domain/src/tools/catalog.rs:41-61`) |
| `provider.rs` | Config merge, login, chat | Built-in `ProviderId` list (`crates/aimee_domain/src/provider.rs:48-141`) |
| `policies/` | `AimeePolicyService`, `ToolRegistry` | `PermissionOperation`, `Permission` (`crates/aimee_domain/src/policies/operation.rs:5-26`, `types.rs:9-16`) |
| `loop_autonomy.rs` | Goal / HITL flow | Exactly five probes (`crates/aimee_domain/src/loop_autonomy.rs:10-27`) |
| `team.rs` | Workflow roster | `Team::engineering()` Muse / Aimee / Sage (`crates/aimee_domain/src/team.rs:71-77`) |
| `error.rs` | All layers at the edge | Typed `thiserror` enum (`crates/aimee_domain/src/error.rs:9-14`) |

`aimee_app` re-exports the whole crate as `aimee_app::domain` (`crates/aimee_app/src/lib.rs:63-65`) so services can depend on app ports without taking a second path to the same types.

### Tool catalog

`ToolCatalog` is a tagged enum (`name` + `arguments`, snake_case). Variants (`crates/aimee_domain/src/tools/catalog.rs:41-61`):

`Read`, `Write`, `FsSearch`, `SemSearch`, `Remove`, `Patch`, `MultiPatch`, `Undo`, `Shell`, `Fetch`, `Followup`, `Plan`, `Skill`, `TodoWrite`, `TodoRead`, `Task`.

Each input struct points at a markdown description via `#[tool_description_file = "…"]` (example: `SkillFetch` at `crates/aimee_domain/src/tools/catalog.rs:686-691`). Descriptions must stay under **1024 characters** (`docs/tool-guidelines.md:22`).

### Providers

`ProviderType` is `Llm` (default) or `ContextEngine` (`crates/aimee_domain/src/provider.rs:17-23`). `ProviderResponse` wire protocols are `OpenAI`, `OpenAIResponses`, `Anthropic`, `Bedrock`, `Google`, `OpenCode` (`crates/aimee_domain/src/provider.rs:259-267`). Config mirrors those as `ProviderResponseType` (`crates/aimee_config/src/config.rs:16-25`).

`ProviderId::built_in_providers()` is the authoritative ID list (`crates/aimee_domain/src/provider.rs:97-141`). Aliases such as `omega` → `aimee` and SuperGrok OAuth names are parsed in `FromStr` (`crates/aimee_domain/src/provider.rs:196-250`). Do not invent IDs that are not in that list unless you are adding a **custom** provider (owned `Cow`).

### Policies and loop autonomy

Operations the engine can decide (`crates/aimee_domain/src/policies/operation.rs:5-26`): `Write`, `Read`, `Execute`, `Fetch`. Outcomes (`crates/aimee_domain/src/policies/types.rs:9-16`): `Allow`, `Deny`, `Confirm`.

Goal loops require five non-empty answers (`GOAL_PROBE_COUNT = 5`). `GoalProbeSet::try_new` rejects the wrong count or a blank answer (`crates/aimee_domain/src/loop_autonomy.rs:56-73`). Default tool-failure budget is `usize::MAX` (`crates/aimee_domain/src/loop_autonomy.rs:13-17`).

## How to use

From another crate, take domain types — not infra structs:

```rust
use aimee_domain::{AgentId, ProviderId, ToolCatalog, PermissionOperation};
```

Inspect catalog and providers from the CLI (presentation → `API`, not domain directly):

```bash
aimee list tool aimee
aimee list provider
aimee list agent
```

When adding a tool:

1. Add a `ToolCatalog` variant and input struct in `crates/aimee_domain/src/tools/catalog.rs`.
2. Write `crates/aimee_domain/src/tools/descriptions/<name>.md` (≤ 1024 characters).
3. Route it in `ToolRegistry` / `ToolExecutor` (see [Tools](../tools.md)). An unregistered variant is an anti-pattern (`docs/tool-guidelines.md:29-31`).

## Best practices

- Newtypes and enums over string modes. `AgentId::new` / constants, not `"aimee"` scattered through services.
- Domain errors stay typed. The comment on `Error` is policy: do not `From`-collapse every serde failure into one variant (`crates/aimee_domain/src/error.rs:9-12`).
- Tool descriptions explain what, when, when **not**, parameters, and limits — examples last (`docs/tool-guidelines.md:8-27`).
- Keep IO, Diesel schema, and HTTP clients out of this crate.
- `///` docs describe behavior for agents. Do not put tutorials or code samples in rustdoc (`AGENTS.md:318-323`).

## Anti-patterns

| Don't | Do |
|---|---|
| `aimee_domain` calling `reqwest` / Diesel | Port in app, impl in infra / repo |
| Stringly tool names in match arms | `ToolCatalog` / `ToolKind` |
| New `ProviderId` that is not a constant or a documented custom id | Add the constant + `built_in_providers` + `FromStr` arm |
| Tool description over 1024 characters | Trim; tests enforce the cap |
| `From<serde_json::Error> for Error` that hides the call site | Explicit variant with context |
| Editing domain to "make the TUI compile" | Fix the mapping at the edge |

## Verify

```bash
cargo fmt
cargo check -p aimee_domain
cargo clippy -p aimee_domain --all-targets -- -D warnings
cargo insta test --accept -p aimee_domain
```

Optional coverage gate (defaults to this crate): `./scripts/tdd-gate.sh` or `./scripts/tdd-gate.sh aimee_domain`.

Never `cargo build --release` for this.

## Related

- [Architecture overview](overview.md)
- [Tools](../tools.md) · [Providers](../providers.md)
- [Application](app.md) consumes these types
- `docs/tool-guidelines.md` · `AIMEE.md` §4
