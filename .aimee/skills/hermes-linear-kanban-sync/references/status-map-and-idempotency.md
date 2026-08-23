# Status map and idempotency

## Discover workspace first

GraphQL `viewer` + `teams.nodes.states` + `activeCycle`. Save fixture **without** API keys; strip emails.

Resolve by Linear **type** (`backlog|unstarted|started|completed|canceled|duplicate`); pin IDs in config after discovery.

## Hermes ↔ Linear (SPE-like)

| Hermes | Linear type | Name preference |
|--------|-------------|-----------------|
| triage | backlog | Backlog |
| todo, ready, scheduled | unstarted | Todo |
| running | started | **In Progress** (not In Review) |
| running + in_review | started | In Review |
| blocked | started + marker | In Progress + `<!-- hermes:blocked=true -->` |
| done | completed | Done |
| archived | canceled/duplicate | Canceled |

Many workspaces lack Blocked—never assume it exists.

## Priority

Linear `0..4` (1=urgent … 4=low). Kanban: higher int = higher urgency. Keep a bidirectional table in code.

## Idempotency

| Case | Key / marker |
|------|----------------|
| Session → card | `--idempotency-key sess:<session_id>` |
| Linear → card | `--idempotency-key linear:<issue_uuid>` |
| Description backlink | `<!-- hermes:task_id=t_… -->` |

`hermes kanban create` with existing key returns the existing non-archived task id.