# Dual-track architecture

## Why both

Linear: planning, cycles, stakeholder visibility.  
Hermes Kanban: agent decomposition, multi-profile workers, durable handoffs (`delegate_task` is not durable across process death).

```text
[Chat TUI/CLI/Telegram]
        → classifier (balanced)
        → hermes kanban create --triage --idempotency-key sess:…
        → connector POST /v1/ensure-issue → Linear
        → sync_map SQLite
kanban task_events poller → Linear state
Linear webhooks → kanban complete/block/archive
```

## Surfaces

| Surface | Role |
|---------|------|
| Connector HTTP ensure API | Plugin/skill automation |
| Poller on `task_events` | Kanban → Linear projection |
| Linear webhooks | Linear → Kanban |
| Linear MCP | Human/agent ad-hoc cockpit |

## Conflict rules

1. Kanban owns execution status while `running` / agent-`blocked`.
2. Linear owns cycle membership and priority.
3. Linear Done → complete Kanban.
4. Linear Canceled while running → block with reason; archive after safe reclaim.
5. Archive/cancel only—never hard-delete production work.

## Minimal HTTP API

- `POST /v1/ensure-issue`
- `POST /v1/ensure-task`
- `GET /v1/link?…`
- `GET /v1/health`

Skills must not embed Linear GraphQL; call connector or MCP tools.

## Loop-safe comments

Prefix mirrors `[hermes-sync]` / `[linear-sync]`. Skip bodies that already match before re-posting.