# Persistence and sessions

Where conversation state, snapshots, and caches live — and what survives a restart.

## The database

Diesel + SQLite. Generated schema: `crates/aimee_repo/src/database/schema.rs`. The live table today is **`conversations`**:

| Column | Purpose |
|---|---|
| `conversation_id` | Primary key; `--cid` targets it |
| `title` | Display title (auto-generated) |
| `workspace_id` | Which workspace the session belongs to |
| `context` | Message history |
| `created_at` / `updated_at` | Timestamps |
| `metrics` | Usage metrics per conversation |

Schema history (all shipped migrations, oldest first): conversations table created → indexed → metrics added → workspace table created and later dropped → indexing-auth table created and dropped. House rule: never edit a shipped migration; add a new one.

## File snapshots

Beyond conversation state, the snapshot service (`aimee_repo` + `aimee_snaps`) keeps file snapshots so `undo` can revert tool-driven changes. This pairs with Anda pathways when enabled — pathways rewind chat, git rewinds code, snapshots rewind tool edits.

## MCP cache

MCP server traffic caches under the system cache directory via cacache, keyed by content. Clearing it is safe; it rebuilds on demand (`aimee mcp reload` forces a rebuild).

## Pathways storage

With `[anda]` enabled, checkpoint metadata lands in `{aimee_home}/pathways` and local eternal receipts in `{aimee_home}/pathways/eternal` by default. See [Anda / KIP](../integrations/anda-kip.md).

## What lives where

| Data | Location | Survives restart |
|---|---|---|
| Conversations | SQLite DB under config base | Yes |
| Credentials | `.credentials.json` in config base | Yes |
| Pathway checkpoints | `{aimee_home}/pathways` | Yes |
| MCP cache | system cache dir (cacache) | Yes |
| In-flight todos/session scratch | memory + templates | No |

## Working with stored sessions

```bash
aimee conversation list
aimee conversation resume <id>
aimee conversation compact --cid <id>
aimee conversation dump --cid <id>        # JSON or HTML export
```

Full command walkthrough at [Session management](../usage/sessions.md).

## See also

* [Streaming pipeline](streaming.md)
* [Config reference](../reference/config.md)
* [Reliability and recovery](../operations/reliability.md)

<!-- sources: AIMEE.md §9, crates/aimee_repo/src/database/migrations/, crates/aimee_main/src/cli.rs -->
