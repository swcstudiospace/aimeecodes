# Infrastructure

`aimee_infra` implements the IO adapters: filesystem, HTTP, gRPC, MCP client/server, env, walker, command execution, inquire, KV cache, and auth strategies. Domain and app depend on **traits**. This crate depends on those traits and on `reqwest` / `tonic` / `rmcp`.

Crate root (`crates/aimee_infra/src/lib.rs:1-27`). Facade: `AimeeInfra` (`crates/aimee_infra/src/aimee_infra.rs:36-108`).

## Purpose

Give `AimeeRepo` / `AimeeServices` a single `F` that can read files, run commands, speak HTTP/2, connect lazily to the workspace gRPC server, and spawn MCP clients — without those concerns leaking into domain types.

## When to use

| Need | Type | File |
|---|---|---|
| Production IO graph | `AimeeInfra::new(cwd, config)` | `aimee_infra.rs` |
| HTTP + SSE | `AimeeHttpInfra` | `http.rs` |
| Workspace gRPC channel | `AimeeGrpcClient` | `grpc.rs` |
| MCP child / HTTP transport | `AimeeMcpClient` | `mcp_client.rs` |
| Read / write / remove / dirs | `AimeeFile*Service` | `fs_*.rs` |
| Shell | `AimeeCommandExecutorService` | `executor.rs` |
| Env + cached config | `AimeeEnvironmentInfra` | `env.rs` |
| Header redaction | `sanitize_headers` | re-exported from `lib.rs:25` |

Do not add a second HTTP client, gRPC stack, or MCP crate. Workspace choices are `reqwest`, `tonic`, `rmcp` (`AGENTS.md:341-352`).

## File interactions

`AimeeAPI::init` is the only production constructor path (`crates/aimee_api/src/aimee_api.rs:51-55`):

```text
AimeeInfra::new(cwd, config)
        │
   AimeeRepo::new(infra)
        │
   AimeeServices::new(repo)
```

`AimeeInfra::new` (`crates/aimee_infra/src/aimee_infra.rs:67-108`) builds:

- `AimeeEnvironmentInfra` from `cwd` + config
- `AimeeHttpInfra` (config HTTP block + file writer for debug dumps)
- FS family + directory reader (`max_parallel_file_reads`, default 4)
- `AimeeGrpcClient::new(config.services_url)`
- command executor, inquire, walker, MCP server stub, auth strategy factory

Services call **infra traits** defined in `aimee_app` (`HttpInfra`, `FileReaderInfra`, `CommandInfra`, `McpServerInfra`, `GrpcInfra`, …). They never name `reqwest::Client`.

### HTTP

`AimeeHttpInfra::new` reads `AimeeConfig.http` with defaults: 30s connect, 900s read, 10 redirects, `accept_invalid_certs: false` (`crates/aimee_infra/src/http.rs:37-56`). TLS versions map from config (`:27-35`). Do not flip `accept_invalid_certs` to make a call work.

### gRPC

`AimeeGrpcClient` is a lazily connected, cloneable `tonic` channel (`crates/aimee_infra/src/grpc.rs:6-24`). HTTPS URLs get `ClientTlsConfig::new().with_webpki_roots()` (`:41-47`). `hydrate()` drops the cached channel (`:54-59`). `API::hydrate_channel` forwards to it (`crates/aimee_api/src/aimee_api.rs:442-445`). Default `services_url` is `https://api.aimeecodes.dev/api` (`crates/aimee_config/src/config.rs:189-193`).

### MCP

`AimeeMcpClient` uses `rmcp` over streamable HTTP or `TokioChildProcess` (`crates/aimee_infra/src/mcp_client.rs:13-39`). Connections are created on first use. Trust gating is **not** here — `McpService::init_mcp` in services owns that.

## How to use

Operators configure IO; they do not construct `AimeeInfra`.

```bash
aimee info                 # environment + config the infra cached
aimee list mcp
aimee mcp --help           # MCP command group (TopLevelCommand::Mcp)
```

```toml
# ~/.aimee/.aimee.toml
services_url = "https://api.aimeecodes.dev/api"
tool_timeout_secs = 300

[http]
connect_timeout_secs = 30
read_timeout_secs = 900
accept_invalid_certs = false
```

Debug request dumps (optional, never commit the directory):

```toml
debug_requests = "/tmp/aimee-debug-requests"
```

`AimeeHttpInfra` stores that path (`crates/aimee_infra/src/http.rs:21-24`). Treat dumps as secret-bearing.

## Best practices

- Inject `AimeeInfra` at the composition root only.
- Honor timeouts and cancellation on every IO entry (HTTP read timeout, tool timeout, gRPC lazy connect).
- Keep TLS on. `accept_invalid_certs` defaults to `false` — leave it.
- Use `sanitize_headers` before logging request metadata.
- One client per concern: the `reqwest::Client` inside `AimeeHttpInfra` is shared. Do not `Client::new()` in a tool service.
- Filesystem helpers live next to the existing `fs_*` modules. Match their error context style (`aimee_fs` / anyhow).

## Anti-patterns

| Don't | Do |
|---|---|
| `ureq` / `native-tls` second client | Workspace `reqwest` |
| `grpcio` next to `tonic` | `AimeeGrpcClient` |
| Hand-rolled MCP JSON-RPC | `rmcp` via `AimeeMcpClient` |
| Domain crate opening files | `FileReaderInfra` |
| Logging `Authorization` | `sanitize_headers` |
| Weakening CORS / TLS to unblock local MCP | Fix the URL or cert |
| Holding `std::sync::Mutex` across `.await` | `AimeeGrpcClient` uses a short lock around channel clone only |

## Verify

```bash
cargo fmt
cargo check -p aimee_infra
cargo clippy -p aimee_infra --all-targets -- -D warnings
cargo insta test --accept -p aimee_infra
```

Never `cargo build --release` for this.

## Related

- [Composition root](api.md) constructs this crate
- [Services](services.md) consume the traits
- [Providers](../providers.md) — HTTP wire protocols
- Persistence (Diesel) is `aimee_repo`, documented separately
