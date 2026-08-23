# multi_patch

`multi_patch` applies several find-and-replace edits to a single file in one atomic operation. Input type: `FSMultiPatch` (`crates/aimee_domain/src/tools/catalog.rs:558-581`). Description source: `descriptions/fs_multi_patch.md`.

## Parameters

| Parameter | Type | Required | Notes |
|---|---|---|---|
| `file_path` | string | yes | Absolute path |
| `edits` | array | yes | Sequential edit operations |

Each element of `edits` (`PatchEdit`, `catalog.rs:559-571`):

| Field | Type | Required | Default | Notes |
|---|---|---|---|---|
| `old_string` | string | yes | — | Exact text to replace |
| `new_string` | string | yes | — | Replacement (must differ) |
| `replace_all` | boolean | no | `false` | Replace every occurrence |

## Example

```json
{
  "name": "multi_patch",
  "arguments": {
    "file_path": "/home/user/project/src/config.rs",
    "edits": [
      {
        "old_string": "const DEFAULT_PORT: u16 = 8080;",
        "new_string": "const DEFAULT_PORT: u16 = 8443;"
      },
      {
        "old_string": "fn load() -> Config {",
        "new_string": "pub fn load() -> Config {"
      },
      {
        "old_string": "old_endpoint",
        "new_string": "new_endpoint",
        "replace_all": true
      }
    ]
  }
}
```

## Behavior

- Edits apply **in sequence**: each edit operates on the result of the previous one, so an earlier edit can set up text a later edit matches.
- The operation is **atomic** — if any edit fails (no match, non-unique match), *none* of the edits are applied.
- Creating a new file with `multi_patch` is supported: first edit uses an empty `old_string` with the full new content as `new_string`; subsequent edits then patch that fresh content. For a plain new file, [write](write.md) is simpler.
- Same prerequisites as [patch](patch.md): the file must have been [read](read.md) earlier in the conversation.

## Errors

| Condition | Result |
|---|---|
| Any `old_string` not found / not unique | Error; **no edits applied** |
| `old_string == new_string` in an edit | Error |
| Earlier edit changes text a later edit expected | Error — plan edit order carefully |

## Permissions

Gated in restricted mode as a **Write** operation ("Modify file with N edits") naming the edit count and path (`catalog.rs:989-997`).

## Related

- [Tool catalog](catalog.md)
- [patch](patch.md) — single-edit variant
- [read](read.md) — prerequisite
- [undo](undo.md) — revert
