# Filesystem tools

The read/write/edit/delete surface agents use to touch your tree. All paths are absolute; all mutations are undoable via snapshots.

## `read`

Reads a file, by default up to the configured line limit from the top; optional start/end line ranges for long files. Errors are normal for missing files — the agent treats them as information, not failure. Output carries a line-number prefix that must be stripped before using content in edits.

## `write`

Creates or overwrites a file. Guardrails from its description: overwriting an existing file requires having read it first **and** setting overwrite explicitly — otherwise the tool fails. Policy nudges toward editing existing files rather than creating new ones.

## `patch` and `multi_patch`

Exact string replacement in files — the preferred edit path:

* `patch`: one `old_string` → `new_string` replacement; errors on non-unique matches unless replace-all is intended.
* `multi_patch`: several replacements against one file in a single operation, built atop `patch`; preferred when making multiple edits to the same file.

Both require a prior `read` in the conversation and both preserve whatever indentation exists after the display prefix.

## `remove`

Deletes a file at an absolute path. Deliberate and confirmable in restricted mode; recoverable through `undo`.

## `undo`

Reverts the most recent create/modify/delete on a given file. This is the safety net under every filesystem mutation — backed by the snapshot service (`aimee_repo` + `aimee_snaps`), not by git, so it works even in non-git directories.

## Behavior summary

| Tool | Needs prior read? | Undoable? | Restricted mode |
|---|---|---|---|
| `read` | — | — | Allowed (reads) |
| `write` | If file exists | Yes | Grant required |
| `patch` / `multi_patch` | Yes | Yes | Grant required |
| `remove` | No | Yes | Grant required |
| `undo` | No | — | Grant required |

## See also

* [Tool catalog](catalog.md)
* [Execution tools](execution-tools.md)
* [Autonomy levels and guardrails](../../concepts/autonomy.md)

<!-- sources: crates/aimee_domain/src/tools/descriptions/{fs_read,fs_write,fs_patch,fs_multi_patch,fs_remove,fs_undo}.md -->
