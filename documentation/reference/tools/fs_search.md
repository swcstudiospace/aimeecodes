# fs_search

`fs_search` runs a regex search over file contents, backed by ripgrep. Input type: `FSSearch` (`crates/aimee_domain/src/tools/catalog.rs:240-308`). Description source: `descriptions/fs_search.md`.

## Parameters

| Parameter | Type | Required | Default | Notes |
|---|---|---|---|---|
| `pattern` | string | yes | — | Ripgrep-flavored regex |
| `path` | string | no | cwd | File or directory to search |
| `glob` | string | no | — | Filename filter, e.g. `*.rs`, `*.{ts,tsx}` |
| `output_mode` | enum | no | `files_with_matches` | `content` \| `files_with_matches` \| `count` |
| `-B` (`before_context`) | integer | no | — | Lines before each match; `content` mode only |
| `-A` (`after_context`) | integer | no | — | Lines after each match; `content` mode only |
| `-C` (`context`) | integer | no | — | Lines before *and* after; `content` mode only |
| `-n` (`show_line_numbers`) | boolean | no | — | Line numbers in output; `content` mode only |
| `-i` (`case_insensitive`) | boolean | no | — | Case-insensitive matching |
| `type` (`file_type`) | string | no | — | ripgrep type: `rust`, `py`, `js`, … |
| `head_limit` | integer | no | unlimited | First N lines/entries across all modes |
| `offset` | integer | no | — | Skip N entries before applying `head_limit` |
| `multiline` | boolean | no | `false` | `.` matches newlines; patterns may span lines |

The context/line-number flags are literally serialized as `-B`, `-A`, `-C`, `-n`, `-i`, `type` on the wire (`catalog.rs:266-291`), mirroring rg.

## Examples

Find every caller of a function, with context:

```json
{
  "name": "fs_search",
  "arguments": {
    "pattern": "validate_email\\(",
    "path": "/home/user/project/crates",
    "type": "rust",
    "output_mode": "content",
    "-n": true,
    "-C": 2,
    "head_limit": 50
  }
}
```

List files mentioning a symbol:

```json
{
  "name": "fs_search",
  "arguments": { "pattern": "ToolRegistry", "glob": "*.md" }
}
```

## Behavior

- The tool contract forbids agents from shelling out to `grep`/`rg`; this tool is the optimized path.
- Pattern syntax is ripgrep's (not grep): literal braces need escaping, e.g. `interface\{\}`.
- `files_with_matches` (the default) returns paths only — cheap discovery before targeted reads.
- For open-ended exploration needing multiple rounds, the contract routes agents to [task](task.md); for natural-language questions ("where is backoff implemented?"), [sem_search](sem_search.md).

## Errors

Invalid regex, missing path, or unsupported flags surface as tool errors; oversized outputs are truncated per config rather than failing.

## Permissions

Gated in restricted mode as a **Read** operation ("Search in directory/file … for pattern …") on the search root (`catalog.rs:952-978`). The grant message names the pattern and glob/type filter.

## Related

- [Tool catalog](catalog.md)
- [sem_search](sem_search.md) — semantic alternative
- [read](read.md) — inspect matches
- [shell](shell.md) — what *not* to use for grep
