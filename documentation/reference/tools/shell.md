# shell

`shell` executes a shell command and returns stdout, stderr, and exit code. Input type: `Shell` (`crates/aimee_domain/src/tools/catalog.rs:590-625`). Description source: `descriptions/shell.md`.

## Parameters

| Parameter | Type | Required | Default | Notes |
|---|---|---|---|---|
| `command` | string | yes | — | The command line to run |
| `cwd` | string (path) | no | session cwd | Working directory for this command |
| `keep_ansi` | boolean | no | `false` | Preserve ANSI escape codes in output |
| `env` | string[] | no | — | Environment variable **names** to pass through (values are read by the system) |
| `description` | string | no | — | 5–10 word summary of intent, shown to the human |

## Examples

Run tests in a crate:

```json
{
  "name": "shell",
  "arguments": {
    "command": "cargo insta test --accept -p aimee_domain",
    "cwd": "/home/user/project",
    "description": "Run aimee_domain snapshot tests"
  }
}
```

Chain dependent steps:

```json
{
  "name": "shell",
  "arguments": {
    "command": "git add -A && git commit -m \"fix: clamp retry budget\"",
    "cwd": "/home/user/project"
  }
}
```

## Behavior

- **`cd` is forbidden inside `command`** — the contract routes all directory changes through the `cwd` parameter. `cd /foo && cmd` is a contract violation even though it would work.
- Output is truncated per config (prefix/suffix line caps, max line length); the full output is written to a temporary file the agent can [read](read.md) or [fs_search](fs_search.md). Because of that, the contract tells agents *not* to wrap commands in `head`/`tail`.
- The contract steers agents away from `find`, `grep`, `cat`, `sed`, `awk`, `echo` in favor of the dedicated tools ([fs_search](fs_search.md), [read](read.md), [patch](patch.md), [write](write.md)).
- Independent commands should be issued as **parallel** tool calls; dependent ones chained with `&&` in a single call. `;` runs sequentially ignoring failures. Newlines as separators are not allowed (fine inside quoted strings).
- `shell` is the only tool with `requires_stdout = true` (`catalog.rs:915-921`) — it gets direct stdout/stderr streaming.

## Errors

Non-zero exit codes are returned as part of the result (stdout + stderr + code), not necessarily as tool failures; hard failures (spawn errors, timeout) surface as tool errors. Timeouts follow `tool_timeout_secs` — see [Reliability](../../reliability.md).

## Permissions

Gated in restricted mode as an **Execute** operation carrying the command (`catalog.rs:998-1001`). This is the most sensitive gate: see [Security](../../security.md).

## Related

- [Tool catalog](catalog.md)
- [fetch](fetch.md) — for URLs, not `curl`
- [fs_search](fs_search.md) / [read](read.md) / [patch](patch.md) — dedicated file tools
- [Reliability](../../reliability.md) — timeout behavior
