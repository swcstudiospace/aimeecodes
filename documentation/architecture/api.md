# Composition root

`aimee_api` is the composition root and the public `API` trait. Presentation (`aimee_main` TUI/CLI) depends on this crate, not on `AimeeServices` internals.

Crate root (`crates/aimee_api/src/lib.rs:1-9`) re-exports `AimeeAPI`, `API`, domain types, selected app DTOs, and `AimeeConfig`.

## Purpose

Own process lifetimes:

1. Build infra → repo → services → `AimeeAPI`
2. Expose one async trait for chat, tools, providers, MCP, workspaces, config
3. Map domain / service errors to `anyhow::Result` at the edge

## When to use

| You are… | Use |
|---|---|
| Adding a CLI / TUI command that needs product behavior | Add a method on `API` + impl on `AimeeAPI` |
| Writing a test that needs the real graph | `AimeeAPI::init(cwd, config)` |
| Wiring a new service | Construct it in `AimeeServices::new`, expose through `API` only if a surface needs it |

Do not use this crate for clap structs (`aimee_main`) or for FS implementations (`aimee_infra`).

## File interactions

Production type (`crates/aimee_api/src/aimee_api.rs:44-56`):

```text
AimeeAPI<AimeeServices<AimeeRepo<AimeeInfra>>, AimeeRepo<AimeeInfra>>
```

`init(cwd, config)`:

1. `Arc::new(AimeeInfra::new(cwd, config))`
2. `Arc::new(AimeeRepo::new(infra.clone()))`
3. `Arc::new(AimeeServices::new(repo.clone()))`
4. `AimeeAPI::new(app, repo)` — services in `self.services`, repo/infra in `self.infra`

`AimeeAPI` is generic (`crates/aimee_api/src/aimee_api.rs:24-32`) so tests can substitute fakes. `app()` builds `AimeeApp::new(self.services.clone())` (`crates/aimee_api/src/aimee_api.rs:34-41`).

The `API` impl (`crates/aimee_api/src/aimee_api.rs:64-446`) is the call map:

| `API` method | Delegates to |
|---|---|
| `chat` | `AimeeApp::chat` after resolving active agent (`:138-147`) |
| `get_tools` / `get_models` / `get_all_provider_models` | `AimeeApp` (`:79-88`) |
| `get_agents` / conversations / config | `self.services` |
| `commit` | `GitApp` + `use_aimee_committer` (`:103-127`) |
| `execute_shell_command` | `self.infra.execute_command` (`:207-214`) |
| `init_mcp` / `reload_mcp` | `services.mcp_service()` (`:306-311`) |
| `init_provider_auth` / `complete_provider_auth` / `remove_provider` | provider auth + credentials (`:326-351`) |
| `sync_workspace` / `query_workspace` / … | `WorkspaceService` (`:353-397`) |
| `mcp_auth` / `mcp_logout` / `mcp_auth_status` | `aimee_infra` helpers (`:424-439`) |
| `hydrate_channel` | `self.infra.hydrate()` (`:442-445`) |
| `generate_data` | `DataGenerationApp` (`:403-409`) |

`update_config` writes via `AppConfigService` and reloads agents when `SetSessionConfig` is present (`crates/aimee_api/src/aimee_api.rs:243-253`).

`API` trait documentation for models: if every provider fails, return the **first** error, not an empty list (`crates/aimee_api/src/api.rs:26-31`).

`AimeeAPI` also implements `ConsoleWriter` by forwarding to infra (`crates/aimee_api/src/aimee_api.rs:448-464`).

## How to use

Humans never construct this type. The `aimee` binary does, then calls `API` methods.

```bash
aimee                    # TUI → API::chat
aimee -p "…"             # one-shot chat
aimee commit --preview   # API::commit
aimee provider login     # API::init_provider_auth + complete
aimee list tool aimee    # API::get_tools (via list)
aimee info
```

Rust (binary / integration only):

```rust
let config = aimee_config::ConfigReader::default()
    .read_defaults()
    .read_global()
    .read_env()
    .build()?;
let api = aimee_api::AimeeAPI::init(std::env::current_dir()?, config);
// api.chat(chat_request).await?
```

Do not pass secrets into `init`. Credentials are file-based under the config base (`.credentials.json`). Config env is `AIMEE_CONFIG` (`AIMEE.md:32`).

## Best practices

- New endpoints inherit the existing auth scheme. No anonymous write path on `API`.
- Keep `init` as the only production constructor. Extra `new_with_*` helpers rot.
- Prefer adding a method to `API` over letting `aimee_main` reach into `AimeeServices`.
- `chat` returns `MpscStream` — honor cancellation on that stream.
- After session-config writes, reload agents (already done in `update_config`).

## Anti-patterns

| Don't | Do |
|---|---|
| `aimee_main` constructing `AimeeFsWrite` | Call `API` |
| Service-to-service call "because init is crowded" | Keep wiring in `AimeeServices::new` |
| Swallowing provider errors as `vec![]` | Return the first error (`api.rs:26-31`) |
| Logging `provider.api_key()` | Use the key, never print it (`aimee_api.rs:268-290` already branches on presence only) |
| Duplicating `init` in the PWA or a second binary | Share `AimeeAPI::init` |
| Inventing REST paths that are not on `API` | Search `api.rs` |

## Verify

```bash
cargo fmt
cargo check -p aimee_api
cargo clippy -p aimee_api --all-targets -- -D warnings
cargo insta test --accept -p aimee_api
```

`cargo check -p aimee_api` is the allowed type-check for this crate. Do not `cargo build --release`.

## Related

- [Architecture overview](overview.md)
- [Services](services.md) · [Infrastructure](infra.md)
- [Providers](../providers.md) for login methods
- `AIMEE.md:103-109`
