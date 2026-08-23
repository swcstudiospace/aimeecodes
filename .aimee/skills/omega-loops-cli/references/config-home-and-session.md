# Config home and session (dogfood)

Smoke uses a temp `OMEGA_CONFIG` and does **not** prove SuperGrok or a real session.

## Config home

`ConfigReader::resolve_base_path` uses the first existing of `~/omega`, `~/.omega`, `~/forge`, `~/.forge`. On this machine that is often **`~/.forge`**:

- creds: `~/.forge/.credentials.json` (`id: xai_oauth`)
- session: `~/.forge/.omega.toml`

## Session required for real commands

Without `[session]`, `omega info` / `-p` call `init_state` → interactive provider picker → **ENXIO** on non-TTY.

```toml
[session]
provider_id = "xai_oauth"
model_id = "grok-4.6"
```

`xai` is the API-key sibling. SuperGrok is `xai_oauth`.

`scripts/dev-omega.sh install` does not rebuild a present `target/debug/omega`. After crate edits: `cargo build -p omega_main` (link can exceed 180s) then `scripts/dev-omega.sh install`.

Logged-in-but-unusable SuperGrok: skill `omega-loops-providers` → `references/supergrok-auth-debug.md`.
