# undo

`undo` reverts the most recent Aimee file operation (create / modify / delete) on one path. Input type: `FSUndo` (`crates/aimee_domain/src/tools/catalog.rs:583-588`). Description source: `descriptions/fs_undo.md`.

## Parameters

| Parameter | Type | Required | Notes |
|---|---|---|---|
| `path` | string | yes | Absolute path previously touched by an Aimee file tool |

## Example

```json
{
  "name": "undo",
  "arguments": { "path": "/home/user/project/src/auth.rs" }
}
```

## Behavior

- Requires a prior **snapshot** for that path — snapshots are taken automatically by Aimee's own file operations ([remove](remove.md), [write](write.md), [patch](patch.md), [multi_patch](multi_patch.md)). If the file was deleted, pass its original path.
- Reverts exactly one step (the latest operation), so repeated calls walk backwards through history.
- External edits (your editor, git checkout) are not Aimee operations and have no snapshot; `git restore` remains the right tool there.

## Errors

No prior snapshot for the path is the main failure ("the system requires a prior snapshot").

## Permissions

No permission gate — undo is treated as safe bookkeeping (`catalog.rs:1009`). It can only restore prior state, not destroy it.

## Related

- [Tool catalog](catalog.md)
- [Snapshot service (`aimee_snaps`)](../../architecture/overview.md)
- [remove](remove.md) · [write](write.md) · [patch](patch.md) · [multi_patch](multi_patch.md)
