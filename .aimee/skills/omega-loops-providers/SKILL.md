---
name: omega-loops-providers
description: "Use when wiring LLM providers/OAuth into Omega Loops."
version: 1.0.0
author: Hermes Agent
license: MIT
platforms: [linux, macos, windows]
metadata:
  hermes:
    tags: [omega, omegaloops, forgecode, providers, oauth, supergrok, xai, tui, slash-commands, rust]
    related_skills: [openhands-llm-auth, agno-agentos-apps, omega-anda-pathways, ratatui-agent-tui, omega-loops-cli]
---

# Omega Loops — providers & SuperGrok OAuth

Omega Loops (`/root/src/repos/omegaloops`) is a **forgecode/forge fork** rebranded toward loop engineering (subagents, workflows, teams, goals). Crates are `omega_*` (binary `omega`). Provider auth is already multi-method (API key, OAuth device, OAuth code, Codex device, ADC, AWS). Prefer **config + existing strategies** over new auth stacks.

## When to use

- Add/change LLM providers in Omega Loops
- SuperGrok / SuperGrok Heavy / xAI OAuth login (no API key)
- TUI slash commands or shell-plugin aliases for provider login
- Port Hermes `xai-oauth` device-code flow into the Rust agent

Don't use for: Agno AgentOS Grok wiring (`agno-agentos-apps`), OpenHands/PrayerHands settings UI (`openhands-llm-auth`), day-to-day Hermes SuperGrok use (bundled `hermes-agent`).

## Architecture map

| Layer | Path |
|-------|------|
| Provider IDs | `crates/omega_domain/src/provider.rs` |
| Auth methods | `crates/omega_domain/src/auth/` (`AuthMethod::OAuthDevice`, …) |
| Built-in providers | `crates/omega_repo/src/provider/provider.json` |
| Strategy factory | `crates/omega_infra/src/auth/strategy.rs` (`OAuthDeviceStrategy`) |
| Auth service | `crates/omega_services/src/provider_auth.rs` |
| TUI login UX | `crates/omega_main/src/ui.rs` (`handle_provider_login`, `handle_device_flow`) |
| Slash commands | `crates/omega_main/src/model.rs` (`AppCommand`) |
| Shell plugin | `shell-plugin/lib/actions/auth.zsh`, `dispatcher.zsh` |
| CLI | `omega provider login <ProviderId>` |

OpenAI chat adapters already use OAuth **access_token as Bearer** (`AuthDetails::OAuth { tokens, .. }`).

## SuperGrok pattern (shipped)

**Separate providers** (Hermes-style), not dual methods on one id:

| Id | Display | Auth |
|----|---------|------|
| `xai` | XAI | API key (`XAI_API_KEY`) |
| `xai_oauth` | SuperGrok | OAuth device only |

`provider.json` `xai_oauth` entry: device code URL + token URL under `auth.x.ai`, public Grok CLI client id, scopes including `grok-cli:access` + `api:access`, chat URL `https://api.x.ai/v1/chat/completions`. Full constants: `references/supergrok-oauth.md`.

### FromStr aliases (ProviderId)

`xai_oauth`, `xai-oauth`, `grok-oauth`, `x-ai-oauth`, `xai-grok-oauth`, `supergrok`, `supergrok_heavy`, `supergrok-heavy`

### User surfaces

| Surface | Command |
|---------|---------|
| TUI slash | `/supergrok` (aliases `/xai-oauth`, `/grok-oauth`, `/supergrok-heavy`) |
| Generic login | `/login` or `/provider` → pick **SuperGrok** |
| CLI | `omega provider login xai_oauth` (or any alias) |
| Shell plugin | `:supergrok` (+ same aliases) |

Slash handler calls `handle_provider_login(Some(&ProviderId::XAI_OAUTH))` — single auth method → device flow immediately (no method picker).

### UX labels

When `auth_url` host is `auth.x.ai` or provider is `XAI_OAUTH`, label method **SuperGrok / SuperGrok Heavy (device login)** and print SuperGrok-specific device-code copy before the verification URL.

## Workflow — new OAuth provider

1. Add `ProviderId` constant + `built_in_providers` + `display_name` + `FromStr` (and Hermes-style aliases if useful).
2. Add `provider.json` entry with `auth_methods: [{ "oauth_device": { … } }]` (or `oauth_code` / `codex_device`). Standard RFC 8628 → existing `OAuthDeviceStrategy` (no custom strategy unless nonstandard like Codex).
3. If `token_refresh_url` is set on device config, factory selects **OAuthWithApiKey** (GitHub Copilot pattern) — omit it for pure Bearer OAuth (xAI).
4. Wire TUI: optional dedicated `AppCommand` + reserved names in `is_reserved_command` + `name()` match + `on_command` arm.
5. Shell plugin: action + dispatcher aliases if users live in zsh plugin.
6. Tests: FromStr aliases, display name, `built_in` membership; strategy factory already covers generic OAuth device.
7. Verify: `cargo check -p omega_domain -p omega_repo -p omega_main -p omega_infra` and focused `cargo test -p omega_domain --lib provider::tests::…`.

## Pitfalls

- **Reuse infrastructure** — do not reimplement device polling; `OAuthDeviceStrategy` + `poll_for_tokens` already handle pending/slow_down.
- **Tier HTTP 403 after browser OK** — SuperGrok entitlement gate, not stale token. Do **not** loop re-login; document API-key fallback (`xai` + `XAI_API_KEY`). Map clearly in UX if adding error handling.
- **Client id** — default public Grok CLI id used by Hermes; allow env override only if product needs a private client.
- **HTTPS host check** — Hermes validates `*.x.ai` endpoints; keep token/auth URLs on `auth.x.ai` / `api.x.ai`.
- **Crate rename** — workspace is `omega_*` / binary `omega`; older docs may say `forge_*` / `forge`.
- **Incomplete workspace members** — empty `crates/*` dirs without `Cargo.toml` break `members = ["crates/*"]`; add a real manifest or `exclude` until ready.
- **IDE/async “Rust 2015” lints** on large TUI files are often false; trust `cargo check`.
- **Cargo.lock thrash** — concurrent cargo + ratatui/nucleo-picker `unicode-width` pins can block resolution; prefer sequential `cargo check -p …` and `--offline --locked` when the lock is healthy.
- **Ad-hoc verify** — `/tmp/hermes-verify-*.sh`, static + focused packages/tests, delete after; do not claim full suite green.
- **`omega` not on PATH** — do not `cargo install` or `--release`. Load `omega-loops-cli` and run `scripts/dev-omega.sh` (debug symlink `~/.local/bin/omega` → `target/debug/omega`).

## Verification

```bash
cd /root/src/repos/omegaloops
# Need a working `omega` on PATH? scripts/dev-omega.sh  (skill omega-loops-cli)
cargo check -p omega_domain -p omega_repo -p omega_main -p omega_infra -p omega_services
cargo test -p omega_domain --lib provider::tests::test_xai_oauth -- --nocapture
cargo test -p omega_domain --lib provider::tests::test_provider_id_display_name -- --nocapture
cargo test -p omega_infra --lib test_create_auth_strategy -- --nocapture
# Manual login (needs browser / SuperGrok tier):
# omega provider login supergrok
# or TUI: /supergrok
```

## References

- `references/supergrok-oauth.md` — endpoints, client id, scopes, file touch list, Hermes map
- Protocol twin (OpenHands UI): skill `openhands-llm-auth` → `references/xai-supergrok-oauth.md`
- Standalone Python device-code: skill `agno-agentos-apps` → `tools/xai_oauth_pkce.py` / `references/xai-device-code-standalone.md`
- Hermes guide: https://hermes-agent.nousresearch.com/docs/guides/xai-grok-oauth
- Hermes source (read-only): `/usr/local/lib/hermes-agent/hermes_cli/auth.py`
- Anda pathways on Omega: skill `omega-anda-pathways`
