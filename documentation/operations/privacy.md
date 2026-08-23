# Data privacy

What Aimee sends where, what it stores locally, and the controls you have.

## The short version

* Your code is read and processed **locally** by the CLI/TUI.
* Model requests go **only** to the provider you configured, carrying only the context needed for the turn.
* Conversation state persists in a **local SQLite database** under your config base.
* Hosted services (workspace indexing) receive content **only** when you explicitly sync a workspace.

## Local storage inventory

| Data | Location | Notes |
|---|---|---|
| Conversations + metrics | SQLite in config base | Delete anytime per-conversation |
| Credentials | `.credentials.json` | Never leave your machine except to providers |
| Pathway checkpoints | `{aimee_home}/pathways` | Only when `[anda]` enabled |
| Eternal receipts | `{aimee_home}/pathways/eternal` | Local mode keeps everything on-device |
| MCP cache | system cache dir | Rebuildable, safe to clear |

## What leaves the machine

| Destination | What | When |
|---|---|---|
| Model provider | Prompts + tool results as context | Every turn |
| Workspace service | Files/content you sync for indexing | Only via explicit workspace sync |
| Telemetry endpoint | Product telemetry | Governed by tracker settings (`AIMEE_TRACKER`) |

Nothing else has a network path. The PWA keeps drafts on-device entirely until agent wiring lands.

## Controls

* **Restricted mode** gates what tools may do before any data flows.
* **Per-provider logout**: `aimee provider logout <id>` removes stored tokens.
* **Conversation deletion**: `aimee conversation delete` removes history from local storage permanently.
* **Pathway opt-out**: don't enable `[anda]`, or set `log_responses = false` for sparser checkpoints.
* **Telemetry toggle**: environment-level control via the tracker variable (see [Environment variables](../reference/env-vars.md)).

## Privacy reviews for your own projects

When building features *with* Aimee, the `/data-privacy` command runs a structured PII-flow/retention/access review over your change set — useful before shipping anything that touches user data.

## See also

* [Security model](security.md)
* [Anda / KIP pathways](../integrations/anda-kip.md)
* [Persistence concepts](../concepts/persistence.md)

<!-- sources: AIMEE.md §6,§10,§15, pwa/README.md, commands/data-privacy.md -->
