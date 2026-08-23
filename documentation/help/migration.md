# Migrating from Omega Loops

Aimee Codes is the continuation of Omega Loops — renamed crates, binaries, config, and templates, with deliberate compatibility so existing installs keep working.

## What carries over automatically

| Legacy | Still works |
|---|---|
| `~/.omega` directory | Detected and used when `~/.aimee` doesn't exist |
| `OMEGA_CONFIG` env var | Read after `AIMEE_CONFIG` |
| `OMEGA_*` variable mappings | Resolve like their `AIMEE_` counterparts |
| `:omega` ZSH alias | Documented alias in the plugin |
| Base-path candidates | `omega`, `.omega`, plus Forge-legacy `forge`, `.forge` |

## The one-command migration

```bash
aimee config migrate
```

Moves `~/aimee`, `~/.omega`, or `~/omega` to `~/.aimee`. After migrating, the legacy directory is no longer consulted.

## What changed names

| Old (Omega/Forge) | New (Aimee) |
|---|---|
| `omega` binary | `aimee` binary (`crates/aimee_main`) |
| `~/.omega/.omega.toml` | `~/.aimee/.aimee.toml` |
| `OMEGA_CONFIG`, `OMEGA_API_KEY` | `AIMEE_CONFIG`, `AIMEE_API_KEY` |
| GitHub origin naming | Repo remote remains `swcstudiospace/omegaloops` until renamed |

Config keys themselves kept their shape — your `.omega.toml` values map onto `.aimee.toml` sections unchanged; only location and prefix moved.

## Checklist

1. Install the new binary ([Installing](../getting-started/install.md)).
2. Run `aimee config migrate`.
3. Verify with `aimee info` — check active config path, provider, and model.
4. Re-authenticate if any provider tokens need refresh: `aimee provider login`.
5. Re-run `aimee setup` to refresh the ZSH plugin wiring.
6. Move any custom agents/skills/commands into `~/.aimee/agents|skills|commands`.

## Trademarks note

Both "Aimee Codes" and "Omega Loops" remain trademarks of Spectrum Web Co LLC (LICENSE §6) — old references you find in the wild are still valid brand history.

## See also

* [Troubleshooting](troubleshooting.md)
* [Configuration model](../concepts/configuration.md)
* [Installing](../getting-started/install.md)

<!-- sources: AIMEE.md §2,§6, crates/aimee_config/src/reader.rs -->
