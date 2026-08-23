# Persistence — aimee_repo

Everything that survives a restart lives here: Diesel/SQLite storage, the gRPC contract, agent prompt definitions, provider data, skills, and file snapshots.

## Database

Generated schema: `src/database/schema.rs`. The live table is `conversations` (see [Persistence concepts](../concepts/persistence.md) for columns). Migrations live in `src/database/migrations/` — **additive only**; editing a shipped migration is forbidden. Current chain:

1. `2025-09-12-065405_create_conversations_table`
2. `2025-09-12-065740_add_conversations_indexes`
3. `2025-10-16-000000_add_metrics_to_conversations`
4. `2025-11-13-054241_create_workspace_table`
5. `2025-11-15-000000_create_indexing_auth_table`
6. `2025-11-22-061212-0000_drop_indexing_auth_table`
7. `2026-02-16-130933-0000_drop_workspace_table`

(The workspace and indexing-auth tables were introduced and later removed — history stays in migrations.)

## The proto

`proto/aimee.proto` defines `package aimee.v1`, service `AimeeService` — 15 RPCs covering search, file upload/delete/list/chunk, health, workspaces, API keys, validation, skill select, fuzzy search, and text-patch building. Documented at [gRPC contract](../reference/proto.md).

## Agent definitions

`src/agents/*.md` holds the production prompts: the flock (`aimee.md`, `muse.md`, `sage.md`) and all fourteen specialists (`fe-ui.md`, `be-security.md`, `plat-k8s.md`, …). These are product surfaces — change only when the task is agent behavior.

## Also here

Provider definitions, skills content shipped to `.aimee/skills/`, and the snapshot service backing `undo` (with `aimee_snaps`).

## See also

* [Infrastructure](infra.md)
* [Persistence concepts](../concepts/persistence.md)
* [The flock](../getting-started/the-flock.md)

<!-- sources: AIMEE.md §5,§9, crates/aimee_repo/src/database/migrations/ -->
