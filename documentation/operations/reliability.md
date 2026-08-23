# Reliability and recovery

How Aimee fails gracefully and how you recover state when something goes wrong.

## Failure handling in the loop

* **Tool failures reflect, not dead-end**: errors feed back to the model as corrective context via the reflection template.
* **Retries are deliberate**: transient provider/tool failures follow the retry template rather than hammering.
* **Failure budgets**: set `max_tool_failure_per_turn` to stop loops that keep failing instead of looping forever.
* **Doom-loop interruption**: repetitive failure patterns trigger a reminder that breaks the cycle.
* **HITL stop-and-ask**: goal probes define when the agent must stop and ask a human rather than guess.

## Conversation recovery

State survives restarts — conversations live in SQLite:

```bash
aimee conversation list                 # find it
aimee conversation resume <id>          # continue where you left off
aimee conversation retry                # re-run last command without polluting context
```

After a crash mid-run, resume plus `todo_read` reconstructs where execution stood.

## Undo paths

| What went wrong | Recovery |
|---|---|
| Agent edited a file wrongly | `undo` tool / snapshots (works outside git too) |
| Code changes generally | git — your normal workflow |
| Conversation drifted badly | Anda pathway rollback (chat-only) when `[anda]` enabled |
| Context bloated/degraded | `aimee conversation compact --cid <id>` |
| Wrong model/provider mid-session | Session model switch from the ZSH dispatcher |

## Provider resilience

Provider HTTP behavior is tunable under `[retry]` and `[http]` in `.aimee.toml`. Multiple providers can be configured; switch session models without losing conversation context.

## Data durability

Conversations persist automatically. With `[anda]` enabled, checkpoints add hash-chained history with local eternal receipts by default (`eternal_mode = "local"`), so even chat state has verifiable backups. See [Anda / KIP pathways](../integrations/anda-kip.md).

## Diagnostics first

```bash
aimee doctor    # shell environment
aimee info      # config + active model + env status
aimee logs      # stream recent logs
```

Most "it's broken" reports resolve into one of: stale credentials (`provider login`), wrong working directory (`-C`), or plugin binding clobbering (re-run `aimee setup`). See [Troubleshooting](../help/troubleshooting.md).

## See also

* [Persistence concepts](../concepts/persistence.md)
* [Reliability engineering commands](../usage/commands.md)
* [Troubleshooting](../help/troubleshooting.md)

<!-- sources: templates/{tool-retry,doom-loop,tool-error-reflection}*, AIMEE.md §9,§10 -->
