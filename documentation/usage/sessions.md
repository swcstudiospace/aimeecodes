# Session management

Conversations are first-class: resumable, exportable, clonable, and inspectable.

## The conversation command group

```text
aimee conversation list       List conversation history
aimee conversation new        Create a new conversation
aimee conversation dump       Export as JSON or HTML
aimee conversation compact    Compact to reduce token usage
aimee conversation retry      Retry last command without modifying context
aimee conversation resume     Resume in interactive mode
aimee conversation show       Show last assistant message
aimee conversation info       Show details
aimee conversation stats      Show statistics
aimee conversation clone      Clone with a new ID
aimee conversation delete     Delete permanently
aimee conversation rename     Rename
aimee conversation pathway    Inspect or roll back the Anda session pathway
```

`session` is an alias for `conversation`. From the top level, `--conversation-id` (short `--cid`) pins a run to a specific conversation:

```bash
aimee --cid 018f3a2b "continue the migration work"
```

The ZSH dispatcher tracks the active conversation automatically: custom commands execute with `--cid`, minting a fresh ID when none exists.

## What's stored

Persistence is SQLite (`conversations` table): `conversation_id`, `title`, `workspace_id`, `context` (the message history), `created_at`, `updated_at`, and a `metrics` column. See [Persistence and sessions](../concepts/persistence.md) for the storage model and schema history.

## Everyday patterns

**Pick up yesterday's thread**

```bash
aimee conversation list
aimee conversation resume <id>
```

**Trim a bloated context before it hurts quality**

```bash
aimee conversation compact --cid <id>
```

Compaction summarizes rather than truncating blindly; use it when long sessions start degrading or costs climb. Related config lives under `[compact]` in `.aimee.toml`.

**Re-run without polluting history**

```bash
aimee conversation retry
```

Retries the last command without appending to the context — useful after fixing a transient provider error.

**Branch an experiment**

```bash
aimee conversation clone --cid <id>   # new ID, same history; try a risky change there
```

**Export for review or records**

```bash
aimee conversation dump --cid <id>          # JSON
aimee conversation dump --format html --cid <id>
```

## Metrics

Each conversation carries metrics alongside its context. Inspect them with `stats` / `info`; the schema is versioned through migrations like any other persisted state.

## Pathways (WEB3)

When `[anda]` is enabled, each conversation gains an append-only checkpoint pathway with hash-chained snapshots and chat-only rollback:

```bash
aimee conversation pathway <id> list
```

Full treatment: [Anda / KIP pathways](../integrations/anda-kip.md).

## See also

* [Plans and todos](plans-and-todos.md)
* [Sessions and persistence](../concepts/persistence.md)
* [Config reference](../reference/config.md)

<!-- sources: crates/aimee_main/src/cli.rs (conversation group help), AIMEE.md §6,§9,§10 -->
