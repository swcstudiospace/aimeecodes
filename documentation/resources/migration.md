# Migrating from Omega Loops

Aimee Codes was formerly **Omega Loops** (and before that, **Forge**). Existing installs keep working: the product reads legacy config locations and environment variables, then you migrate once with a single command. This page lists every compatibility path that actually exists in the tree and how to move off it.

## What stays compatible (intentionally — do not delete)

| Legacy surface | Current | Behavior |
|---|---|---|
| `OMEGA_CONFIG` env var | `AIMEE_CONFIG` | Still read; `AIMEE_` wins |
| `~/aimee`, `~/.omega`, `~/omega`, `~/forge`, `~/.forge` dirs | `~/.aimee` | Picked up as base-path candidates in order of existence |
| `OMEGA_*` env variables | `AIMEE_*` mapping onto `.aimee.toml` | Still read |
| ZSH `:omega` alias | `:act` / `:aimee` | Still mapped where the plugin keeps it |
| "Omega Loops" trademark | Aimee Codes | LICENSE still lists it as a Spectrum Web Co trademark |

## One-command migration

```bash
aimee config migrate
```

Moves `~/aimee`, `~/.omega`, or `~/omega` → `~/.aimee`. After migrating:

```bash
aimee config path     # should print ~/.aimee
aimee info            # resolved config sanity check
```

## Step-by-step

1. **Update the binary** to an Aimee-branded release (`nix run github:swcstudiospace/omegaloops`, or `cargo install --path crates/aimee_main --locked --bin aimee`). The command is now `aimee`.
2. **Run `aimee config migrate`**. Your conversations, credentials (`.credentials.json`), agents, and skills come across with the directory.
3. **Reinstall the ZSH plugin**: `aimee setup`. Old `:` muscle memory works — including `:omega`.
4. **Update scripts/env**: prefer `AIMEE_CONFIG`, `AIMEE_SERVICES_URL`, and other `AIMEE_*` names. Legacy names still work but are deprecated for new setups.
5. **Verify**: `aimee doctor` checks shell integration end-to-end.

## Naming map

| Old | New |
|---|---|
| Omega Loops (product) | Aimee Codes |
| `omega` binary | `aimee` |
| `.omega.toml` | `.aimee.toml` (schema: `aimee.schema.json`) |
| Forge (earlier era) | folded into the same compat ladder |

The live GitHub remote is [`swcstudiospace/omegaloops`](https://github.com/swcstudiospace/omegaloops) until the repository itself is renamed — that URL is correct today; do not invent a new one.

## Troubles

- **Migrate says nothing to do** — your config already lives in `~/.aimee`, or only Forge-era paths exist and they're empty.
- **Two configs fighting** — `AIMEE_CONFIG` (or a stale `OMEGA_CONFIG`) is pinning the base path. Unset the env var and re-check `aimee config path`.
- **Plugin invokes `omega`** — reinstall with `aimee setup`; the rewrite honors `AIMEE_BIN`.

## Related

- [Configuration](../configuration.md) — resolution order details
- [Troubleshooting and FAQ](troubleshooting.md)
- [Install and Nix](../ops/install.md)
