# write

`write` creates a new file or overwrites an existing one with the given content. Input type: `FSWrite` (`crates/aimee_domain/src/tools/catalog.rs:222-238`). Description source: `descriptions/fs_write.md`.

## Parameters

| Parameter | Type | Required | Default | Notes |
|---|---|---|---|---|
| `file_path` | string | yes | — | Absolute path. Alias: `path`. |
| `content` | string | yes | — | Full file body |
| `overwrite` | boolean | no | `false` | Must be `true` to replace an existing file |

## Example

```json
{
  "name": "write",
  "arguments": {
    "file_path": "/home/user/project/src/new_module.rs",
    "content": "pub fn hello() -> &'static str {\n    \"hello\"\n}\n",
    "overwrite": false
  }
}
```

## Behavior

- If the file exists and was **not** read earlier in the conversation, or `overwrite` is not `true`, the call fails and returns the existing content so the agent can compare before replacing.
- Intended for **new files**. For existing files in a codebase, the tool contract tells the agent to prefer [patch](patch.md); wholesale rewrites of already-read files are the legitimate `overwrite: true` case.
- The description template instructs agents never to create docs/README files unless explicitly requested, and to avoid emojis unless asked.

## Errors

| Condition | Result |
|---|---|
| Relative path | Error |
| Existing file + `overwrite: false` (or unread) | Error including existing content |
| Parent directory missing | Error |

## Permissions

Gated in restricted mode as a **Write** operation ("Create/overwrite file") on the target path (`catalog.rs:944-951`).

## Related

- [Tool catalog](catalog.md)
- [read](read.md) — required first for existing files
- [patch](patch.md) · [multi_patch](multi_patch.md) — preferred for edits
- [undo](undo.md) — revert a bad write
