# gRPC contract — aimee.proto

The workspace/context-engine contract between the CLI and the services backend is defined in [`crates/aimee_repo/proto/aimee.proto`](https://github.com/swcstudiospace/omegaloops/blob/main/crates/aimee_repo/proto/aimee.proto) — `package aimee.v1`, service `AimeeService`, generated with tonic (the workspace-canonical gRPC stack). The default client target is `config.services_url` (`https://api.aimeecodes.dev/`; override with `AIMEE_SERVICES_URL` — see [Environment variables](env.md)).

## Service: AimeeService

| RPC | Purpose |
|---|---|
| `Search(SearchRequest) → SearchResponse` | Search nodes matching a query (the engine behind [`sem_search`](tools/sem_search.md)) |
| `UploadFiles(UploadFilesRequest) → UploadFilesResponse` | Upload files to the context engine ([`aimee workspace sync`](../cli.md)) |
| `DeleteFiles(DeleteFilesRequest) → DeleteFilesResponse` | Delete files from a workspace |
| `ListFiles(ListFilesRequest) → ListFilesResponse` | List all files in a workspace |
| `ChunkFiles(ChunkFilesRequest) → ChunkFilesResponse` | Split files into chunks **without** uploading (dry-run chunking) |
| `HealthCheck(HealthCheckRequest) → HealthCheckResponse` | Liveness probe |
| `CreateWorkspace(CreateWorkspaceRequest) → CreateWorkspaceResponse` | Create a workspace |
| `ListWorkspaces(ListWorkspacesRequest) → ListWorkspacesResponse` | List a user's workspaces |
| `GetWorkspaceInfo(GetWorkspaceInfoRequest) → GetWorkspaceInfoResponse` | Fetch one workspace's info |
| `DeleteWorkspace(DeleteWorkspaceRequest) → DeleteWorkspaceResponse` | Delete a workspace |
| `CreateApiKey(CreateApiKeyRequest) → CreateApiKeyResponse` | Create a user API key |
| `ValidateFiles(ValidateFilesRequest) → ValidateFilesResponse` | Batch syntax validation of files |
| `SelectSkill(SelectSkillRequest) → SelectSkillResponse` | Pick relevant skills for a prompt (feeds [`skill`](tools/skill.md) suggestions) |
| `FuzzySearch(FuzzySearchRequest) → FuzzySearchResponse` | Needle-in-haystack fuzzy search |
| `BuildTextPatch(BuildTextPatchRequest) → BuildTextPatchResponse` | Build a serialized text patch for fuzzy replacement |

## Domain model

The proto models a knowledge graph over code:

- **Node kinds** (`NodeKind`): `FILE`, `FILE_CHUNK`, `FILE_REF`, `NOTE`, `TASK`.
- **Relations** (`RelationType`): `CALLS`, `EXTENDS`, `IMPLEMENTS`, `USES`, `DEFINES`, `REFERENCES`, `CONTAINS`, `DEPENDS_ON`, `RELATED_TO`, plus `INVERSE`.
- Core messages: `Node`, `NodeData`, `File`, `FileChunk`, `FileRef`, `Note`, `Task`, `Query` / `QueryItem` / `QueryResult`, `Workspace` + `WorkspaceDefinition` + `GitInfo`, and upload results (`UploadResult`, `RelationCreateResult`).
- Identity types are first-class wrappers (`NodeId`, `WorkspaceId`, `RelationId`, `UserId`) rather than bare strings.

Read the full message definitions in [aimee.proto](https://github.com/swcstudiospace/omegaloops/blob/main/crates/aimee_repo/proto/aimee.proto).

## Who uses this contract

| Caller | Path |
|---|---|
| Workspace sync / indexing | `aimee workspace sync` ([CLI](../cli.md), [Cloud and services](../ops/cloud.md)) |
| Semantic search | [`sem_search`](tools/sem_search.md) → `Search` |
| Skill selection | [`skill`](tools/skill.md) pipeline → `SelectSkill` |
| Fuzzy patching fallback | `use_text_patch_fallback` config → `FuzzySearch` + `BuildTextPatch` |

The contract is additive-first in practice: new RPCs are added, existing ones keep wire compatibility. Treat `.proto` changes like schema migrations — reviewed artifacts.

## Related

- [Persistence — aimee_repo](../architecture/persistence.md) — where the proto lives
- [Cloud and services](../ops/cloud.md) — the backend that serves this API
- [Configuration](../configuration.md) — `services_url`
- [Environment variables](env.md) — `AIMEE_SERVICES_URL`
