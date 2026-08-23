# Omega CLI smoke argv

Used by `scripts/dev-omega.sh smoke`. Isolated `OMEGA_CONFIG` temp dir. 30s timeout each. Do not add interactive commands.

| Name | Argv | Needle (optional, case-insensitive) |
|------|------|-------------------------------------|
| version | `--version` | `0.1.0` |
| help | `--help` | `Usage: omega` |
| help provider | `provider --help` | `provider` |
| help list | `list --help` | `list` |
| banner | `banner` | (exit 0) |
| info porcelain | `info --porcelain` | (exit 0) |
| list provider | `list provider --porcelain` | `xai` |
| list agent | `list agent --porcelain` | (exit 0) |
| list config | `list config --porcelain` | (exit 0) |
| list mcp | `list mcp --porcelain` | (exit 0) |
| list conversation | `list conversation --porcelain` | (exit 0) |
| list cmd | `list cmd --porcelain` | (exit 0) |
| list skill | `list skill --porcelain` | (exit 0) |
| workspace list | `workspace list --porcelain` | (exit 0) |
| conversation list | `conversation list --porcelain` | (exit 0) |

Expect **15 passed**. Visual-only sibling: `scripts/list-all-porcelain.sh`.
