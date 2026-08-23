# todo_write

`todo_write` creates and updates the session's structured task list. Input type: `TodoWrite` (`crates/aimee_domain/src/tools/catalog.rs:708-717`). Description source: `descriptions/todo_write.md`.

## Parameters

| Parameter | Type | Required | Notes |
|---|---|---|---|
| `todos` | array | yes | Only the **changed** items — not the whole list |

Each element of `todos` (`TodoItem`, `catalog.rs:700-706`):

| Field | Type | Required | Notes |
|---|---|---|---|
| `content` | string | yes | Task description; the **unique key** for matching |
| `status` | enum | yes | `pending` \| `in_progress` \| `completed` \| `cancelled` |

## Example

```json
{
  "name": "todo_write",
  "arguments": {
    "todos": [
      { "content": "Add retry table to embedded config", "status": "in_progress" },
      { "content": "Snapshot-test merged config", "status": "pending" }
    ]
  }
}
```

## Behavior

The server matches on `content`:

- Content not seen before → item is **added**.
- Content already present → its **status is updated**.
- `status: cancelled` → the item is **removed** from the list entirely.
- Items not mentioned in this call are **left unchanged**.
- IDs are managed internally and never exposed to the model.

Item validation: content must be non-empty and at most 1000 characters (`Todo::validate`, `catalog.rs:168-178`).

## When agents should use it

Per the tool contract: complex multi-step tasks (3+ steps), non-trivial work needing planning, explicit user requests, multi-part instructions, immediately after new instructions arrive. Mark a task `in_progress` *before* starting it — ideally only one `in_progress` at a time — and mark `completed` as soon as it finishes, adding newly discovered follow-ups.

Skip it for single straightforward tasks or trivial sub-3-step work; the contract explicitly says not to use it when just doing the task is cheaper.

## Errors

Empty content, content over 1000 characters, or an invalid status value.

## Permissions

No permission gate (`catalog.rs:1013`) — session-scoped state only.

## Related

- [Tool catalog](catalog.md)
- [todo_read](todo_read.md) — inspect the current list
- [task](task.md) — delegate rather than track
