# Pathway CLI

Commands (id **before** subcommand; clap nested `Pathway { id, command }`):

```bash
omega conversation pathway <conversation-id> list
omega conversation pathway <conversation-id> list --porcelain
omega conversation pathway <conversation-id> show <seq>
omega conversation pathway <conversation-id> rollback <seq>
```

Help is clap-only (`--help` exits before config). Do not smoke `rollback` without a real conversation id.

## Wiring

| Piece | Location |
|-------|----------|
| Clap | `omega_main/src/cli.rs` — `ConversationCommand::Pathway`, `PathwayCommand::{List,Show,Rollback}` |
| Handler | `omega_main/src/ui.rs` — `handle_pathway_command` |
| Helpers | `omega_app::list_session_pathway` / `show_session_pathway` / `rollback_session_pathway` |
| Parse tests | `omega_main` `test_conversation_pathway_list`, `test_conversation_pathway_rollback` |

Rollback: `SessionPathwayService::rollback_to` (hash-chain verify, truncate_after, Rollback marker) then `api.upsert_conversation`. Chat only — not `omega_snaps`.

`list`/`show` read `{omega_home}/pathways` even when `[anda] enabled` is false.

Porcelain list columns: `seq`, `kind`, `content_hash`, `message_count`, `created_at` (tab-separated).

## Verify

```bash
cargo test -p omega_app --lib anda_pathway
cargo test -p omega_main --lib test_conversation_pathway
cargo run -p omega_main --quiet -- conversation pathway --help
```
