# read

`read` reads one file from disk and returns its contents with line numbers. Input type: `FSRead` (`crates/aimee_domain/src/tools/catalog.rs:204-220`). Description source: `descriptions/fs_read.md`.

## Parameters

| Parameter | Type | Required | Default | Notes |
|---|---|---|---|---|
| `file_path` | string | yes | — | Absolute path. Alias: `path`. |
| `range.start_line` | integer | no | — | 1-based first line (`FSReadRange`) |
| `range.end_line` | integer | no | — | Inclusive 1-based last line |
| `show_line_numbers` | boolean | no | `true` | Prefix each line with its index |

Unknown fields are **rejected** — `FSRead` is annotated `#[schemars(deny_unknown_fields)]`.

## Example

```json
{
  "name": "read",
  "arguments": {
    "file_path": "/home/user/project/src/main.rs",
    "range": { "start_line": 40, "end_line": 120 }
  }
}
```

Omit `range` to read the whole file — the tool contract recommends whole-file reads for long files unless only a section matters.

## Behavior

- Output uses ripgrep's `-n` display format: `line_number:content`, numbering from 1.
- Whole-file reads are capped by config (`maxReadSize` lines); lines longer than `maxLineLength` are truncated.
- With a vision-capable model, images render visually and PDFs are base64-encoded page images (oversized PDFs error).
- `.ipynb` notebooks come back as plain JSON — cell structure, outputs, and embedded content included.
- Directories cannot be read; use [shell](shell.md) (`ls`) for listings.
- The contract encourages speculative **parallel reads** of several candidate files in one response.

## Errors

| Condition | Result |
|---|---|
| File does not exist | Error (safe to attempt; agents probe paths this way) |
| Relative path | Error |
| Unknown extra field | Schema validation error |

## Permissions

Gated in restricted mode as a **Read** operation ("Read file") on the target path (`catalog.rs:939-943`). [fs_search](fs_search.md) is gated the same way, so searching implies reading posture.

## Related

- [Tool catalog](catalog.md)
- [write](write.md) · [patch](patch.md) — edits require a prior `read`
- [fs_search](fs_search.md) — locate files first
