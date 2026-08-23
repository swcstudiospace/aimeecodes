# Environment variables

Variables the binary reads, verified in source. `AIMEE_`-prefixed variables generally map onto `.aimee.toml` keys (double underscore nests); legacy `OMEGA_*` names still resolve for compatibility.

## Core

| Variable | Role |
|---|---|
| `AIMEE_CONFIG` | Config base directory (wins over all path candidates) |
| `OMEGA_CONFIG` | Legacy config base directory |
| `AIMEE_SERVICES_URL` | Workspace / indexing API base (maps to `services_url`) |
| `OMEGA_WORKSPACE_SERVER_URL` | Legacy form of the above |
| `AIMEE_WORKSPACE_SERVER_URL` | Alternate spelling read by workspace tooling |
| `AIMEE_BIN` | Binary name used by the ZSH plugin (default `aimee`) |
| `AIMEE_LOG` | `tracing` filter, e.g. `aimee=info` |
| `AIMEE_EDITOR` | Editor for `:edit` / `:config-edit` |

## Behavior toggles

| Variable | Role |
|---|---|
| `AIMEE_BANNER` | Suppress/alter startup banner display |
| `AIMEE_SKIP_INTERACTIVE` | Skip interactive prompts (scripting/CI) |
| `AIMEE_TRACKER` | Disable telemetry when set to a falsey value |

## Credentials

| Variable | Role |
|---|---|
| `AIMEE_API_KEY` / `OMEGA_API_KEY` | API key injection where supported — prefer `aimee provider login`; never bake into images or dotfiles |

## Session overrides

The ZSH dispatcher sets these for session-scoped model switching:

| Variable | Role |
|---|---|
| `AIMEE_SESSION__PROVIDER_ID` | Session-only provider (note the double underscore) |
| `AIMEE_SESSION__MODEL_ID` | Session-only model |

## Pods

| Variable | Role |
|---|---|
| `AIMEE_POD_BIN` | Pod runtime binary override |
| `AIMEE_POD_WORKSPACE` | Pod workspace override |

## Precedence summary

Environment variables beat file values; `AIMEE_*` beats legacy `OMEGA_*`; explicit CLI flags beat both.

## See also

* [Config reference](config.md)
* [Authentication and credentials](../integrations/auth.md)
* [CLI reference](cli.md)

<!-- sources: grep of env reads across crates/, AIMEE.md §6 -->
