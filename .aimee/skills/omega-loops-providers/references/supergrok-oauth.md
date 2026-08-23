# SuperGrok / xAI OAuth in Omega Loops

## Protocol (shared with Hermes)

| Item | Value |
|------|--------|
| Auth | OAuth 2.0 **device code** (RFC 8628), not browser PKCE |
| Issuer | `https://auth.x.ai` |
| Discovery | `https://auth.x.ai/.well-known/openid-configuration` |
| Device code | `https://auth.x.ai/oauth2/device/code` |
| Token | `https://auth.x.ai/oauth2/token` |
| Client id | `b1a00492-073a-47ea-816f-4c329264a828` (public Grok CLI / Hermes) |
| Scope | `openid profile email offline_access grok-cli:access api:access` |
| Inference | `https://api.x.ai/v1` (chat completions path in Omega) |
| Omega provider id | `xai_oauth` (display **SuperGrok**) |
| API-key sibling | `xai` + `XAI_API_KEY` |

## provider.json shape (excerpt)

```json
{
  "id": "xai_oauth",
  "api_key_vars": "XAI_OAUTH_ACCESS_TOKEN",
  "response_type": "OpenAI",
  "url": "https://api.x.ai/v1/chat/completions",
  "models": "https://api.x.ai/v1/models",
  "auth_methods": [
    {
      "oauth_device": {
        "auth_url": "https://auth.x.ai/oauth2/device/code",
        "token_url": "https://auth.x.ai/oauth2/token",
        "client_id": "b1a00492-073a-47ea-816f-4c329264a828",
        "scopes": [
          "openid", "profile", "email", "offline_access",
          "grok-cli:access", "api:access"
        ],
        "use_pkce": false
      }
    }
  ]
}
```

Do **not** set `token_refresh_url` — that switches factory to GitHub-style OAuthWithApiKey exchange.

## Files touched (2026-08 SuperGrok land)

| File | Change |
|------|--------|
| `crates/omega_domain/src/provider.rs` | `XAI_OAUTH`, aliases, display SuperGrok, tests |
| `crates/omega_repo/src/provider/provider.json` | `xai_oauth` entry |
| `crates/omega_main/src/model.rs` | `AppCommand::SuperGrok` + reserved names |
| `crates/omega_main/src/ui.rs` | slash handler, SuperGrok device UX labels |
| `shell-plugin/lib/actions/auth.zsh` | `_omega_action_supergrok` |
| `shell-plugin/lib/dispatcher.zsh` | `:supergrok` aliases |
| `shell-plugin/README.md` | docs |

## Hermes source map (read-only)

- `/usr/local/lib/hermes-agent/hermes_cli/auth.py` — `XAI_OAUTH_*`, `_xai_oauth_*`, `refresh_xai_oauth_pure`
- Docs: https://hermes-agent.nousresearch.com/docs/guides/xai-grok-oauth
- Provider id there: `xai-oauth` (hyphen); Omega uses `xai_oauth` (snake) + hyphen aliases in FromStr

## Tier / 403

Browser login can succeed while refresh/inference returns **HTTP 403** (“caller does not have permission…”). That is entitlement gating (often SuperGrok Heavy vs standard SuperGrok), not a dead refresh token. Fix path: `XAI_API_KEY` + provider `xai`, or upgrade tier — **never** re-run device login in a loop.

## Related implementations

| Product | Skill / path |
|---------|----------------|
| Hermes | bundled hermes-agent + auth.py |
| PrayerHands / OpenHands | `openhands-llm-auth` |
| Agno agencies | `agno-agentos-apps` → `tools/xai_oauth_pkce.py` |
| Omega Loops | this skill |
