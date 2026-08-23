# Environment variables

Every environment variable Aimee Codes reads, grouped by concern. `AIMEE_`-prefixed variables map onto `.aimee.toml` keys (prefix `AIMEE_`, double underscore `__` for nesting); legacy `OMEGA_` variables are still read for compatibility.

## Core

| Variable | Role |
|---|---|
| `AIMEE_CONFIG` | Config base directory. Wins over everything (then legacy `OMEGA_CONFIG`). |
| `AIMEE_SERVICES_URL` | Workspace / indexing API target. Overrides `services_url`. Default `https://api.aimeecodes.dev/`. |
| `AIMEE_BIN` | Binary name the ZSH plugin invokes (default `aimee`) |
| `AIMEE_LOG` | `tracing` filter, e.g. `aimee=info` |
| `AIMEE_EDITOR` | Editor for `:edit` / `:config-edit` |

Base-path resolution order ([Configuration](../configuration.md)):

1. `AIMEE_CONFIG`
2. `OMEGA_CONFIG`
3. First existing of `~/aimee`, `~/.aimee`, `~/omega`, `~/.omega`
4. Forge-legacy `~/forge` / `~/.forge` if present
5. Otherwise `~/.aimee`

## Generic mapping

Any `.aimee.toml` key can be set via env: `AIMEE_TOOL_TIMEOUT_SECS=60`, or nested with `__`: `AIMEE_REASONING__EFFORT=high`, `AIMEE_ANDA__ENABLED=true`. Precedence: explicit env beats file values.

## Provider credentials

Provider API keys are **not** read from arbitrary env vars; they are stored by `aimee provider login` in `.credentials.json` under the config base and are never committed to git. See [Providers](../providers.md) and [Security](../security.md).

## Shell integration

The ZSH plugin honors `AIMEE_BIN` when rewriting `:` lines, so wrapper scripts can point at a specific build (`~/.local/bin/aimee → target/debug` is a common local setup). See [ZSH plugin](../zsh.md).

## Related

- [Configuration](../configuration.md)
- [Config JSON schema](schema.md)
- [gRPC contract](proto.md) — what talks to `AIMEE_SERVICES_URL`
