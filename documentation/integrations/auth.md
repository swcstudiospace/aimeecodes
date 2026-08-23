# Authentication and credentials

How Aimee authenticates to providers and where secrets live.

## Provider credentials

```bash
aimee provider login     # store credentials for a provider
aimee provider logout    # remove them
```

Credentials are written to a single file under your config base:

| Path | Contents |
|---|---|
| `~/.aimee/.credentials.json` | API keys / OAuth tokens, per provider |

Rules that matter:

* **Never commit this file.** It is not tracked by the repo and should not enter any git history, dotfile sync, or backup you share.
* The config base resolves through `AIMEE_CONFIG` → legacy `OMEGA_CONFIG` → existing directories → `~/.aimee`. Wherever it lands, `.credentials.json` sits beside `.aimee.toml`.
* OAuth flows (SuperGrok `xai_oauth`, GitHub Copilot) store tokens in the same file after device login completes.

## OAuth device login

For `xai_oauth` (SuperGrok) no API key is needed: run `aimee provider login`, choose SuperGrok, follow the device-code flow. The ZSH plugin exposes the same as a dedicated action (`_aimee_action_supergrok`). An OAuth callback handler exists for providers that redirect locally.

## Environment overrides

`AIMEE_`-prefixed variables map onto `.aimee.toml` (double underscore nests), so deployment environments can configure without touching files. Legacy `OMEGA_` variables still resolve. Key variables: `AIMEE_CONFIG`, `AIMEE_SERVICES_URL`, `AIMEE_BIN`, `AIMEE_LOG`, `AIMEE_EDITOR`. Full table at [Environment variables](../reference/env-vars.md).

## What Aimee never does

* Never prints or logs tokens — output paths redact secrets.
* Never sends credentials anywhere except the provider endpoint being authenticated.
* Never embeds keys in generated code, commits, or PR text.

If you suspect a leaked key: `aimee provider logout <id>`, rotate at the provider, log back in.

## Workspace / indexing auth

The hosted services side (`services_url`) uses its own credential path managed via `aimee workspace` commands; indexing operations authenticate separately from model providers. See [Providers](providers.md) for the `aimee_services` entry point.

## Restricted mode

Auth is not authorization. With `restricted = true` in `.aimee.toml`, tool execution requires explicit permission grants regardless of which provider is signed in — see [Security model](../operations/security.md).

## See also

* [Providers and model access](providers.md)
* [Security model](../operations/security.md)
* [Data privacy](../operations/privacy.md)

<!-- sources: AIMEE.md §2,§6,§8,§15, crates/aimee_config/src/reader.rs -->
