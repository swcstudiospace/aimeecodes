# Config reference (.aimee.toml)

The primary configuration file, normally `~/.aimee/.aimee.toml`. Defaults are embedded at build time from `crates/aimee_config/.aimee.toml`, so the file is optional — everything here overrides those defaults. Machine-readable contract: `aimee.schema.json` (see [JSON schema](schema.md)).

## Reading and writing

```bash
aimee config get services_url
aimee config set commit xai grok-4       # dedicated commit-message model
aimee config list
```

## Top-level keys

| Key | Type | Meaning |
|---|---|---|
| `services_url` | URL | Workspace/indexing API base. Default `https://api.aimeecodes.dev/` |
| `restricted` | bool | Require explicit permission grants for tool execution |
| `tool_timeout_secs` | int | Per-tool-call timeout used by the registry |
| `subagents` | bool | `true` (default): expose `task`; `false`: expose `sage` instead |
| `research_subagent` | — | Research dispatch tuning |
| `use_aimee_committer` | bool | Route commits through Aimee's committer |
| `max_tool_failure_per_turn` | int | Failure budget per turn; default unlimited (`usize::MAX`) |

## Sections

### `[reasoning]`
Reasoning-effort behavior for models that support it.

### `[anda]`

Session pathways / KIP. Full key table at [Anda / KIP pathways](../integrations/anda-kip.md). Minimum:

```toml
[anda]
enabled = true
kip_enabled = true
```

### `[retry]`, `[http]`
Provider retry behavior and HTTP client settings.

### `[compact]`
Conversation compaction thresholds and behavior.

### `[updates]`
Self-update check behavior (`aimee update`).

### `[[providers]]`
Inline provider definitions merged over built-ins; same `id` overrides field-by-field:

```toml
[[providers]]
id = "my_gateway"
response_type = "openai"
base_url = "https://llm.internal.example.com/v1"
```

## Where the file lives

Base directory resolution: `$AIMEE_CONFIG` → `$OMEGA_CONFIG` → existing `~/aimee`, `~/.aimee`, `~/omega`, `~/.omega` → Forge-legacy → default `~/.aimee`. Credentials sit beside it as `.credentials.json`.

## Validation

Validate any `.aimee.toml` against the published JSON Schema in CI or your editor — every key, type, and default is specified there.

## See also

* [Configuration model](../concepts/configuration.md)
* [Environment variables](env-vars.md)
* [JSON schema](schema.md)

<!-- sources: aimee.schema.json, crates/aimee_config/src/{config.rs,reader.rs,anda.rs}, AIMEE.md §6 -->
