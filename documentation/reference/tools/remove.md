# remove

`remove` deletes a file at an absolute path. Input type: `FSRemove` (`crates/aimee_domain/src/tools/catalog.rs:450-455`). Description source: `descriptions/fs_remove.md`.

## Parameters

| Parameter | Type | Required | Notes |
|---|---|---|---|
| `path` | string | yes | Absolute path of the file to delete |

## Example

```json
{
  "name": "remove",
  "arguments": { "path": "/home/user/project/src/legacy_module.rs" }
}
```

## Behavior

- Files only — there is no directory-removal variant in the catalog.
- The deletion is snapshotted, so [undo](undo.md) can restore the file immediately afterwards.

## Errors

Missing file or invalid path surfaces as a tool error.

## Permissions

Gated in restricted mode as a **Write** operation ("Remove file") on the path (`catalog.rs:979-983`) — deletion is treated with write-level caution.

## Related

- [Tool catalog](catalog.md)
- [undo](undo.md) — restore a removed file
- [shell](shell.md) — `rm -r` for directories (restricted mode will gate it as Execute)
