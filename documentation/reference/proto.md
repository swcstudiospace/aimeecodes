# gRPC contract (aimee.proto)

The wire contract for Aimee's context-engine services, defined at `crates/aimee_repo/proto/aimee.proto`. Package `aimee.v1`, one service: **`AimeeService`**. The default client target is `services_url` from config (default `https://api.aimeecodes.dev/`).

## The 15 RPCs

| RPC | Purpose |
|---|---|
| `Search` | Semantic search over indexed workspace content |
| `UploadFiles` | Upload files into the workspace index |
| `DeleteFiles` | Remove files from the index |
| `ListFiles` | List indexed files |
| `ChunkFiles` | Chunk file content for indexing |
| `HealthCheck` | Service health probe |
| `CreateWorkspace` | Provision a workspace |
| `ListWorkspaces` | Enumerate workspaces |
| `GetWorkspaceInfo` | Details for one workspace |
| `DeleteWorkspace` | Remove a workspace |
| `CreateApiKey` | Issue API keys for workspace access |
| `ValidateFiles` | Validate files against indexing rules |
| `SelectSkill` | Skill selection service-side |
| `FuzzySearch` | Fuzzy matching over indexed items |
| `BuildTextPatch` | Server-side text patch construction |

## Grouping

The RPCs cluster into four concerns: **search/retrieval** (Search, FuzzySearch, SelectSkill), **file ingestion** (UploadFiles, DeleteFiles, ListFiles, ChunkFiles, ValidateFiles), **workspace administration** (Create/List/GetInfo/Delete Workspace, CreateApiKey), and **operations** (HealthCheck, BuildTextPatch).

## Where it's implemented

Client side lives in `aimee_infra` (gRPC via Tonic); the contract is versioned with the repo. Workspace management commands (`aimee workspace`) drive these endpoints; the ZSH sync actions (init/sync/status) exercise the ingestion path.

```bash
aimee workspace --help    # management surface over this API
```

## Versioning

Package name carries the version (`aimee.v1`). Changes to RPC shapes are additive-first per house policy — public contracts get reviewed like migrations.

## See also

* [Infrastructure](../architecture/infra.md)
* [Config reference](config.md)
* [JSON schema](schema.md)

<!-- sources: crates/aimee_repo/proto/aimee.proto, AIMEE.md §9 -->
