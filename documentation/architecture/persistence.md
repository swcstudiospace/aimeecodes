# Persistence

Aimee Codes persists local agent state with **Diesel + SQLite** and talks to the remote context engine over **gRPC**. The product map is `AIMEE.md` section 9 (`aimeecodes/AIMEE.md:276-296`). House rules for schema work live in `AGENTS.md` (`aimeecodes/AGENTS.md:76-78`, `aimeecodes/AGENTS.md:123-124`).

This page documents what is in the tree. It does not invent tables, RPCs, or CLI commands.

## What `aimee_repo` owns vs `aimee_infra`

| Crate | Role |
|---|---|
| `aimee_repo` | Persistence: Diesel/SQLite, proto, agent defs, providers, skills, snapshots (`aimeecodes/AIMEE.md:133`, `aimeecodes/AGENTS.md:59`) |
| `aimee_infra` | Infrastructure trait impls: FS, HTTP, auth, MCP, gRPC, env, walker (`aimeecodes/AIMEE.md:132`, `aimeecodes/AGENTS.md:58`) |

`AimeeRepo<F>` is the repository container. It stores one `Arc<F>` and owns the durable adapters (`aimeecodes/crates/aimee_repo/src/aimee_repo.rs:36-53`):

- `ConversationRepositoryImpl` — SQLite conversations
- `AimeeFileSnapshotService` — file snapshots via `aimee_snaps`
- `CacacheStorage` — MCP cache (`KVStore`)
- `AimeeProviderRepository` / `AimeeChatRepository` — providers and chat (file-based credentials)
- `AimeeContextEngineRepository` — remote workspace index over gRPC
- `AimeeAgentRepository` / `AimeeSkillRepository` — agent and skill files
- `AimeeValidationRepository` / `AimeeFuzzySearchRepository` — gRPC validate and fuzzy search

The crate root only re-exports that container (`aimeecodes/crates/aimee_repo/src/lib.rs:17-18`).

`AimeeInfra` does **not** open SQLite. It constructs the gRPC client from `config.services_url` (`aimeecodes/crates/aimee_infra/src/aimee_infra.rs:85`) and implements the ports `AimeeRepo` forwards: HTTP, FS, walker, MCP connect, command exec, env (`aimeecodes/crates/aimee_repo/src/aimee_repo.rs:211-487`). `CacacheStorage` is implemented in `aimee_infra` and re-exported for the repo (`aimeecodes/crates/aimee_infra/src/lib.rs:26`, `aimeecodes/crates/aimee_repo/src/aimee_repo.rs:20-21`).

Do not add a second ORM. The workspace playbook is Diesel in `aimee_repo`; do not add `sqlx`, `sea-orm`, or a Diesel async rewrite (`aimeecodes/AGENTS.md:346`).

## Composition

Startup is a single composition root (`aimeecodes/AIMEE.md:103-108`, `aimeecodes/crates/aimee_api/src/aimee_api.rs:44-56`):

1. `AimeeInfra::new(cwd, config)`
2. `AimeeRepo::new(infra)`
3. `AimeeServices::new(repo)`
4. `AimeeAPI::new(services, repo)`

```
AimeeInfra
    └── AimeeRepo<AimeeInfra>
            └── AimeeServices<AimeeRepo<AimeeInfra>>
                    └── AimeeAPI<AimeeServices<…>, AimeeRepo<…>>
```

The concrete type is `AimeeAPI<AimeeServices<AimeeRepo<AimeeInfra>>, AimeeRepo<AimeeInfra>>` (`aimeecodes/crates/aimee_api/src/aimee_api.rs:44`). Services take the repo as their single generic and store it as `Arc<F>` (`aimeecodes/crates/aimee_services/src/aimee_services.rs:111`). Services do not call other services; compose at this root (`aimeecodes/AIMEE.md:112`, `aimeecodes/AGENTS.md:108-110`).

`AimeeRepo::new` builds the SQLite pool from `env.database_path()` and scopes conversations to `env.workspace_hash()` (`aimeecodes/crates/aimee_repo/src/aimee_repo.rs:63-71`).

## SQLite + Diesel

Engine: SQLite via Diesel 2.3.7 (`sqlite`, `r2d2`, `chrono`) and `diesel_migrations` 2.2.0 (`aimeecodes/crates/aimee_repo/Cargo.toml:45-46`).

| Concern | Path |
|---|---|
| Diesel CLI config | `aimeecodes/diesel.toml:1-5` |
| Generated schema | `crates/aimee_repo/src/database/schema.rs` (`aimeecodes/diesel.toml:2`, `aimeecodes/AGENTS.md:76`) |
| Migrations directory | `crates/aimee_repo/src/database/migrations` (`aimeecodes/diesel.toml:4-5`) |
| Embedded migrations | `embed_migrations!("src/database/migrations")` (`aimeecodes/crates/aimee_repo/src/database/pool.rs:13`) |
| On-disk database | `{base_path}/.aimee.db` (`aimeecodes/crates/aimee_domain/src/env.rs:113-115`) |

`schema.rs` is generated. Do not hand-edit it as the source of truth (`aimeecodes/crates/aimee_repo/src/database/schema.rs:1`, `aimeecodes/AIMEE.md:116`).

Pool defaults (`aimeecodes/crates/aimee_repo/src/database/pool.rs:28-38`): `max_size` 5, `min_idle` 1, connection timeout 5s, idle timeout 600s, 5 retries. Each acquired connection sets SQLite pragmas for concurrency (`aimeecodes/crates/aimee_repo/src/database/pool.rs:108-127`):

- `PRAGMA busy_timeout = 30000;`
- `PRAGMA journal_mode = WAL;`
- `PRAGMA synchronous = NORMAL;`
- `PRAGMA wal_autocheckpoint = 1000;`

Diesel is synchronous. Conversation IO runs on `tokio::task::spawn_blocking` with a connection taken from the pool for that task (`aimeecodes/crates/aimee_repo/src/conversation/conversation_repo.rs:20-41`). Do not share a mutable connection across tasks.

## Current `conversations` table

After all shipped migrations, the generated schema has **one** table (`aimeecodes/crates/aimee_repo/src/database/schema.rs:3-13`, `aimeecodes/AIMEE.md:280`):

| Column | Diesel type | Notes |
|---|---|---|
| `conversation_id` | `Text` | Primary key |
| `title` | `Nullable<Text>` | |
| `workspace_id` | `BigInt` | `WorkspaceHash` of `cwd` (`aimeecodes/crates/aimee_domain/src/env.rs:182-187`) |
| `context` | `Nullable<Text>` | JSON `ContextRecord` |
| `created_at` | `Timestamp` | |
| `updated_at` | `Nullable<Timestamp>` | |
| `metrics` | `Nullable<Text>` | JSON `MetricsRecord` |

The Diesel row type is `ConversationRecord` (`aimeecodes/crates/aimee_repo/src/conversation/conversation_record.rs:940-952`). `context` and `metrics` are serialized JSON, not extra SQL tables (`aimeecodes/crates/aimee_repo/src/conversation/conversation_record.rs:960-968`).

Writes use Diesel's query builder, not interpolated SQL (`aimeecodes/crates/aimee_repo/src/conversation/conversation_repo.rs:50-60`). Delete is scoped to the current workspace hash (`aimeecodes/crates/aimee_repo/src/conversation/conversation_repo.rs:137-141`). List/last queries also filter `context IS NOT NULL` (`aimeecodes/crates/aimee_repo/src/conversation/conversation_repo.rs:91-94`).

`workspace` and `indexing_auth` existed in earlier revisions and were dropped. They are **not** in the current generated schema.

## Shipped migrations

Never edit a shipped migration. Add a new one (`aimeecodes/AIMEE.md:292`, `aimeecodes/AGENTS.md:77-78`, `aimeecodes/AGENTS.md:226`).

Directory: `crates/aimee_repo/src/database/migrations/`. Seven revisions, listed in apply order:

### `2025-09-12-065405_create_conversations_table`

Creates `conversations` with `conversation_id` (TEXT PK), `title`, `workspace_id` (BIGINT NOT NULL), `context`, `created_at` (DEFAULT CURRENT_TIMESTAMP), `updated_at` (`aimeecodes/crates/aimee_repo/src/database/migrations/2025-09-12-065405_create_conversations_table/up.sql:2-9`). Down: `DROP TABLE IF EXISTS conversations` (`aimeecodes/crates/aimee_repo/src/database/migrations/2025-09-12-065405_create_conversations_table/down.sql:2`).

### `2025-09-12-065740_add_conversations_indexes`

Adds `idx_conversations_workspace_created` on `(workspace_id, created_at DESC)` and partial `idx_conversations_active_workspace_updated` on `(workspace_id, updated_at DESC) WHERE context IS NOT NULL` (`aimeecodes/crates/aimee_repo/src/database/migrations/2025-09-12-065740_add_conversations_indexes/up.sql:2-6`). Down drops both indexes (`aimeecodes/crates/aimee_repo/src/database/migrations/2025-09-12-065740_add_conversations_indexes/down.sql:2-3`).

### `2025-10-16-000000_add_metrics_to_conversations`

`ALTER TABLE conversations ADD COLUMN metrics TEXT` (`aimeecodes/crates/aimee_repo/src/database/migrations/2025-10-16-000000_add_metrics_to_conversations/up.sql:2`). Down drops the column (`aimeecodes/crates/aimee_repo/src/database/migrations/2025-10-16-000000_add_metrics_to_conversations/down.sql:2`).

### `2025-11-13-054241_create_workspace_table`

Creates `workspace` (`remote_workspace_id` TEXT PK, `user_id`, `path` UNIQUE, timestamps) plus `idx_workspace_path` and `idx_workspace_user_id` (`aimeecodes/crates/aimee_repo/src/database/migrations/2025-11-13-054241_create_workspace_table/up.sql:2-12`). Later dropped (see below). Down drops the indexes then the table (`aimeecodes/crates/aimee_repo/src/database/migrations/2025-11-13-054241_create_workspace_table/down.sql:2-5`).

### `2025-11-15-000000_create_indexing_auth_table`

Creates `indexing_auth` (`user_id` TEXT PK, `token` TEXT NOT NULL, `created_at`) for indexing-service auth (`aimeecodes/crates/aimee_repo/src/database/migrations/2025-11-15-000000_create_indexing_auth_table/up.sql:1-7`). Later dropped. Down: `DROP TABLE IF EXISTS indexing_auth` (`aimeecodes/crates/aimee_repo/src/database/migrations/2025-11-15-000000_create_indexing_auth_table/down.sql:2`).

### `2025-11-22-061212-0000_drop_indexing_auth_table`

`DROP TABLE IF EXISTS indexing_auth` because credentials moved to the credentials file (`aimeecodes/crates/aimee_repo/src/database/migrations/2025-11-22-061212-0000_drop_indexing_auth_table/up.sql:1-2`). Down recreates the table (`aimeecodes/crates/aimee_repo/src/database/migrations/2025-11-22-061212-0000_drop_indexing_auth_table/down.sql:2-6`).

### `2026-02-16-130933-0000_drop_workspace_table`

Drops `idx_workspace_path`, `idx_workspace_user_id`, then `workspace` (`aimeecodes/crates/aimee_repo/src/database/migrations/2026-02-16-130933-0000_drop_workspace_table/up.sql:1-6`). Down recreates the table and indexes (`aimeecodes/crates/aimee_repo/src/database/migrations/2026-02-16-130933-0000_drop_workspace_table/down.sql:2-12`).

There is an empty `.diesel_lock` next to these folders. Do not rewrite any of the seven.

## gRPC contract

Proto: `crates/aimee_repo/proto/aimee.proto`. Package `aimee.v1` (`aimeecodes/crates/aimee_repo/proto/aimee.proto:5`). Compiled at crate build (`aimeecodes/crates/aimee_repo/build.rs:2`) and included as `tonic::include_proto!("aimee.v1")` (`aimeecodes/crates/aimee_repo/src/lib.rs:13-15`).

Service name: **`AimeeService`** (`aimeecodes/crates/aimee_repo/proto/aimee.proto:8-53`). RPCs that exist in the proto:

| RPC | Request | Response |
|---|---|---|
| `Search` | `SearchRequest` | `SearchResponse` |
| `UploadFiles` | `UploadFilesRequest` | `UploadFilesResponse` |
| `DeleteFiles` | `DeleteFilesRequest` | `DeleteFilesResponse` |
| `ListFiles` | `ListFilesRequest` | `ListFilesResponse` |
| `ChunkFiles` | `ChunkFilesRequest` | `ChunkFilesResponse` |
| `HealthCheck` | `HealthCheckRequest` | `HealthCheckResponse` |
| `CreateWorkspace` | `CreateWorkspaceRequest` | `CreateWorkspaceResponse` |
| `ListWorkspaces` | `ListWorkspacesRequest` | `ListWorkspacesResponse` |
| `GetWorkspaceInfo` | `GetWorkspaceInfoRequest` | `GetWorkspaceInfoResponse` |
| `DeleteWorkspace` | `DeleteWorkspaceRequest` | `DeleteWorkspaceResponse` |
| `CreateApiKey` | `CreateApiKeyRequest` | `CreateApiKeyResponse` |
| `ValidateFiles` | `ValidateFilesRequest` | `ValidateFilesResponse` |
| `SelectSkill` | `SelectSkillRequest` | `SelectSkillResponse` |
| `FuzzySearch` | `FuzzySearchRequest` | `FuzzySearchResponse` |
| `BuildTextPatch` | `BuildTextPatchRequest` | `BuildTextPatchResponse` |

Default client target is `config.services_url` (`aimeecodes/AIMEE.md:294`, `aimeecodes/crates/aimee_infra/src/aimee_infra.rs:85`). Embedded default is `https://api.aimeecodes.dev/` (`aimeecodes/crates/aimee_config/.aimee.toml:23`, `aimeecodes/crates/aimee_config/src/config.rs:189-193`). HTTPS uses tonic TLS with webpki roots (`aimeecodes/crates/aimee_infra/src/grpc.rs:41-46`). Do not weaken TLS to make a call work.

Repo call sites that actually invoke the client:

- Context engine: `CreateApiKey`, `CreateWorkspace`, `UploadFiles`, `Search`, `ListWorkspaces`, `GetWorkspaceInfo`, `ListFiles`, `DeleteFiles`, `DeleteWorkspace` (`aimeecodes/crates/aimee_repo/src/context_engine.rs:117-383`)
- Validation: `ValidateFiles` (`aimeecodes/crates/aimee_repo/src/validation.rs:47-50`)
- Fuzzy search: `FuzzySearch` (`aimeecodes/crates/aimee_repo/src/fuzzy_search.rs:44-46`)
- Text patch: `BuildTextPatch` (`aimeecodes/crates/aimee_repo/src/aimee_repo.rs:639-648`)

`ChunkFiles`, `HealthCheck`, and `SelectSkill` are on the service in proto. Do not invent additional RPCs.

Workspace-index RPCs attach `authorization: Bearer <token>` (`aimeecodes/crates/aimee_repo/src/context_engine.rs:99-111`). AuthN is not AuthZ (`aimeecodes/AGENTS.md:198`).

## File snapshots

File undo is **not** SQLite. `AimeeFileSnapshotService` wraps `aimee_snaps::SnapshotService` and stores under `env.snapshot_path()` = `{base_path}/snapshots` (`aimeecodes/crates/aimee_repo/src/fs_snap.rs:7-16`, `aimeecodes/crates/aimee_domain/src/env.rs:78-80`).

`SnapshotService::create_snapshot` reads the live file and writes `{snapshots}/{path_hash}/{YYYY-MM-DD_HH-MM-SS-nnnnnnnnn}.snap` (`aimeecodes/crates/aimee_snaps/src/service.rs:22-34`, `aimeecodes/crates/aimee_domain/src/snapshot.rs:94-110`). `undo_snapshot` restores the newest `.snap` for that path hash and deletes that snap (`aimeecodes/crates/aimee_snaps/src/service.rs:57-80`). `AimeeRepo` implements `SnapshotRepository` by forwarding those two methods (`aimeecodes/crates/aimee_repo/src/aimee_repo.rs:102-111`).

This is workspace file history for the `undo` tool. It is not conversation rollback. Anda/KIP conversation pathways are a different store (see the WEB3 page when present).

## MCP cache

MCP cache is cacache under the env cache dir (`aimeecodes/AIMEE.md:296`). Construction:

```
CacacheStorage::new(env.cache_dir().join("mcp_cache"), Some(3600))
```

(`aimeecodes/crates/aimee_repo/src/aimee_repo.rs:73-76`). `cache_dir()` is `{base_path}/cache` (`aimeecodes/crates/aimee_domain/src/env.rs:117-120`). TTL is 3600 seconds.

`CacacheStorage` hashes keys, stores JSON `CachedEntry { value, timestamp }`, and treats expired entries as misses (`aimeecodes/crates/aimee_infra/src/kv_storage.rs:16-71`). `AimeeRepo` implements `KVStore` by forwarding `cache_get` / `cache_set` / `cache_clear` (`aimeecodes/crates/aimee_repo/src/aimee_repo.rs:241-262`).

Do not log cache payloads. They can contain untrusted MCP output (`aimeecodes/AGENTS.md:193-199`).

## How to run migrations / verify

**Runtime (what the product actually runs).** Opening the pool applies pending embedded migrations:

- On-disk: `DatabasePool::try_from(PoolConfig)` → `build_pool` → `run_pending_migrations(MIGRATIONS)` (`aimeecodes/crates/aimee_repo/src/database/pool.rs:176-184`)
- Tests: `DatabasePool::in_memory()` does the same on `:memory:` (`aimeecodes/crates/aimee_repo/src/database/pool.rs:59-66`)

There is no Makefile, script, or documented `diesel migration run` in this tree. `diesel.toml` only tells Diesel CLI where schema and migrations live (`aimeecodes/diesel.toml:1-5`). Do not invent a CLI workflow that is not checked in.

**To add a schema change:** add a **new** folder under `crates/aimee_repo/src/database/migrations/` with `up.sql` and `down.sql`. Then regenerate `schema.rs` with Diesel CLI pointed at that `diesel.toml`. Never rewrite a shipped revision (`aimeecodes/AGENTS.md:77-78`).

**Verify the crate** (workspace contract, `aimeecodes/AGENTS.md:361-368`):

```bash
# from the aimeecodes repo root
cargo fmt
cargo check -p aimee_repo
cargo clippy -p aimee_repo --all-targets -- -D warnings
cargo insta test --accept -p aimee_repo
```

Conversation tests construct `DatabasePool::in_memory()` so they exercise the full migration set (`aimeecodes/crates/aimee_repo/src/conversation/conversation_repo.rs:162-164`). Do not `cargo build --release` for this check (`aimeecodes/AGENTS.md:370-371`).

**Verify this docs page** (from the repo root):

```bash
python3 documentation/scripts/verify-docs.py
```

## On-disk layout (config base)

Paths are relative to `Environment.base_path` (default `~/.aimee` for new installs, `aimeecodes/AIMEE.md:30`):

| Path | Owner |
|---|---|
| `.aimee.db` | Diesel/SQLite conversations |
| `snapshots/` | `aimee_snaps` file snapshots |
| `cache/mcp_cache/` | cacache MCP `KVStore` |
| `.credentials.json` | Provider / indexing credentials (not SQLite) |

Credentials are a file, not a table. That is why `indexing_auth` was dropped (`aimeecodes/crates/aimee_repo/src/database/migrations/2025-11-22-061212-0000_drop_indexing_auth_table/up.sql:1`). Never commit `.credentials.json` or print its contents (`aimeecodes/AIMEE.md:229`, `aimeecodes/AGENTS.md:229`).

## Best practices

- Additive schema first. Compatibility defaults for new persisted fields (`aimeecodes/AGENTS.md:123-124`).
- Parameterized queries only. Use Diesel's query builder; no interpolated SQL (`aimeecodes/AGENTS.md:195`).
- One connection per `spawn_blocking` task from the r2d2 pool. No shared mutable SQLite session across tasks (`aimeecodes/crates/aimee_repo/src/conversation/conversation_repo.rs:20-41`).
- Scope mutations by `workspace_id`. Delete already does (`aimeecodes/crates/aimee_repo/src/conversation/conversation_repo.rs:137-141`).
- Keep domain types out of the SQL row. `ConversationRecord` plus JSON `ContextRecord` / `MetricsRecord` isolate storage from domain churn (`aimeecodes/crates/aimee_repo/src/conversation/conversation_record.rs:1-5`).
- File snapshots and MCP cache stay on the filesystem. Do not fold them into SQLite unless a new migration exists.
- gRPC stays on `services_url` with TLS for `https`. Auth header on workspace RPCs; still authorize every mutation.
- Treat conversation JSON, MCP cache, and snapshot bytes as untrusted input.

## Anti-patterns

| Don't | Do |
|---|---|
| Edit a shipped migration | Add a new migration (`aimeecodes/AGENTS.md:226`) |
| Hand-edit `schema.rs` as source of truth | Regenerate from Diesel CLI + `diesel.toml` |
| Add `sqlx` / `sea-orm` / a second Diesel | Use Diesel in `aimee_repo` (`aimeecodes/AGENTS.md:346`) |
| Interpolated SQL or f-string queries | Diesel `filter` / `values` / `eq` |
| Hold `std::sync::Mutex` across `.await` on the pool | `spawn_blocking` + short-lived pooled connection |
| Recreate `workspace` / `indexing_auth` in place | They were dropped; a new need is a new migration |
| Store API tokens in SQLite | `.credentials.json` (modeled after the `indexing_auth` drop) |
| Invent proto RPCs or tables | Search `aimee.proto` and `schema.rs` first |
| Log credentials, conversation payloads, or cache values | Redact; never commit dumps (`aimeecodes/AGENTS.md:199`) |
| Service-to-service calls to reach the DB | Compose at `AimeeAPI::init` |

## Residual risk

- WAL + `synchronous = NORMAL` is a local-laptop durability tradeoff (`aimeecodes/crates/aimee_repo/src/database/pool.rs:117-122`). Point-in-time restore of `.aimee.db` is a file copy of that SQLite file (and its `-wal`/`-shm` companions if present). The tree does not ship a backup job.
- `metrics` deserialize failures fall back to `Metrics::default()` (`aimeecodes/crates/aimee_repo/src/conversation/conversation_record.rs:1011-1018`). Corrupt JSON is not a hard error.
- gRPC `CreateApiKeyResponse.key` is an API token. Do not print it in docs, logs, or tests.
