# Configuration model

One TOML file, layered defaults, environment overrides. This page explains how configuration resolves; every key is documented at [Config reference](../reference/config.md) and machine-published in `aimee.schema.json`.

## Resolution order

The config base directory is the first decision:

1. `$AIMEE_CONFIG` if set
2. `$OMEGA_CONFIG` (legacy) if set
3. Whichever exists: `~/aimee`, `~/.aimee`, `~/omega`, `~/.omega`
4. Forge-legacy `~/forge` or `~/.forge`
5. Default: `~/.aimee`

Inside the base, `~/.aimee/.aimee.toml` is the primary file. Defaults are embedded from `crates/aimee_config/.aimee.toml` at build time, so a missing file still yields a working config.

## Layering

```text
embedded defaults
      ↓
~/.aimee/.aimee.toml          (user)
      ↓
.aimee.toml in project?       (per-repo overrides where supported)
      ↓
AIMEE_* environment variables  (highest wins)
```

Environment mapping: prefix `AIMEE_`, double underscore nests — `AIMEE_SERVICES_URL` → `services_url`, `AIMEE_FOO__BAR` → `[foo] bar`. Legacy `OMEGA_*` variables resolve identically.

## Reading and writing values

```bash
aimee config get services_url
aimee config set commit xai grok-4
aimee config list
```

Inspect everything at once:

```bash
aimee info    # config, active model, environment status
```

## Key groups

| Section | Controls |
|---|---|
| top-level | `services_url`, `restricted`, `tool_timeout_secs`, `subagents`, `research_subagent`, `use_aimee_committer`, `max_tool_failure_per_turn` |
| `[reasoning]` | Reasoning-effort behavior for capable models |
| `[anda]` | Session pathways / KIP (see [Anda](../integrations/anda-kip.md)) |
| `[retry]` | Provider retry behavior |
| `[http]` | HTTP client settings |
| `[compact]` | Conversation compaction |
| `[updates]` | Self-update checks |
| `[[providers]]` | Inline provider definitions merged over built-ins |

## Project policy files

Configuration isn't just TOML. Aimee reads policy from the tree:

* `AGENTS.md` (project root) or `~/.aimee/AGENTS.md` (global) — house rules for agents
* `.mcp.json` (project) beats `~/.aimee/.mcp.json` (global) for MCP servers
* `.aimee/skills/`, `.aimee/commands/`, `.aimee/agents/` — extensions

## Validation

The full JSON Schema (`aimee.schema.json`) publishes every key, type, and default — editors can validate `.aimee.toml` against it, and CI can too. See [JSON schema reference](../reference/schema.md).

## See also

* [Config reference (.aimee.toml)](../reference/config.md)
* [Environment variables](../reference/env-vars.md)
* [Providers and model access](../integrations/providers.md)

<!-- sources: crates/aimee_config/src/reader.rs, aimee.schema.json, AIMEE.md §6 -->
