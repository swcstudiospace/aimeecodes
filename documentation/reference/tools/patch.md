# patch

`patch` performs an exact string replacement inside one file. Input type: `FSPatch` (`crates/aimee_domain/src/tools/catalog.rs:537-556`). Description source: `descriptions/fs_patch.md`.

## Parameters

| Parameter | Type | Required | Default | Notes |
|---|---|---|---|---|
| `file_path` | string | yes | — | Absolute path. Aliases: `path` |
| `old_string` | string | yes | — | Exact text to replace. Alias: `search` |
| `new_string` | string | yes | — | Replacement (must differ). Alias: `content` |
| `replace_all` | boolean | no | `false` | Replace every occurrence |

The legacy aliases exist because older model habits sent `path` / `search` / `content`; tests pin backward compatibility for all of them (`catalog.rs:1564-1676`).

## Example

```json
{
  "name": "patch",
  "arguments": {
    "file_path": "/home/user/project/src/auth.rs",
    "old_string": "const MAX_RETRIES: u32 = 3;",
    "new_string": "const MAX_RETRIES: u32 = 5;",
    "replace_all": false
  }
}
```

## Behavior

- The agent must have called [read](read.md) on the file earlier in the conversation, otherwise the edit errors.
- Matching is literal and whitespace-exact: copy text *after* the `line_number:` display prefix, never including it.
- If `old_string` appears more than once and `replace_all` is `false`, the call **fails** — the fix is more surrounding context or `replace_all: true`.
- `replace_all: true` is the renaming primitive (variables, functions, types) across a single file; for many edits to one file prefer [multi_patch](multi_patch.md).

## Errors

| Condition | Result |
|---|---|
| File not read first | Error |
| `old_string` not found | Error |
| `old_string` not unique without `replace_all` | Error |
| `old_string == new_string` | Error |

## Permissions

Gated in restricted mode as a **Write** operation ("Modify file") on the target path (`catalog.rs:984-988`).

## Related

- [Tool catalog](catalog.md)
- [read](read.md) — prerequisite
- [multi_patch](multi_patch.md) — several edits, one atomic call
- [undo](undo.md) — revert
