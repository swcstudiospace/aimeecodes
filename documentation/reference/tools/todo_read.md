# todo_read

`todo_read` returns the current session todo list. Input type: `TodoRead` (`crates/aimee_domain/src/tools/catalog.rs:719-721`) — it takes no arguments. Description source: `descriptions/todo_read.md`.

## Parameters

None. Call with an empty arguments object.

## Example

```json
{
  "name": "todo_read",
  "arguments": {}
}
```

## Behavior

Returns all current todos with their content and status (`pending`, `in_progress`, `completed`). An empty list is returned when nothing is tracked yet.

The contract's intended uses: check for existing items before calling [todo_write](todo_write.md) (avoid duplicates), review pending/in-progress work at any point, resume after a break, or answer the user's "what's left?" questions.

## Errors

Effectively none — an empty list is a valid response, not a failure.

## Permissions

No permission gate (`catalog.rs:1014`).

## Related

- [Tool catalog](catalog.md)
- [todo_write](todo_write.md) — create / update / remove items
