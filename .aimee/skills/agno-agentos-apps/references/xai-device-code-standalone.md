# Standalone xAI device-code module (Agno apps)

Filename often `tools/xai_oauth_pkce.py` for historical reasons. **Protocol is device code**, not PKCE.

## Constants (public Grok CLI client)

| Item | Value |
|------|--------|
| Issuer | `https://auth.x.ai` |
| Discovery | `https://auth.x.ai/.well-known/openid-configuration` |
| Device code | `https://auth.x.ai/oauth2/device/code` |
| Default client_id | `b1a00492-073a-47ea-816f-4c329264a828` (`XAI_OAUTH_CLIENT_ID`) |
| Scope | `openid profile email offline_access grok-cli:access api:access` |
| Inference | `https://api.x.ai/v1` |

## Required API surface

```text
get_xai_token_or_fallback(*, force_refresh=False) -> str
status() -> dict  # booleans/paths only, no secrets
device_code_login(*, open_browser=True)
# CLI: python -m tools.xai_oauth_pkce login|status|logout|token
```

## Resolve order

1. `XAI_API_KEY`  
2. Project JSON `~/.config/ai-agency/xai_oauth.json` (chmod 600)  
3. Hermes `~/.hermes/auth.json` → providers/`xai-oauth` tokens (read-only reuse)

Refresh: `grant_type=refresh_token` + `client_id` + `refresh_token` against discovered `token_endpoint` (must be `*.x.ai` HTTPS). Proactive refresh when JWT `exp` within skew (~120s).

## 403 handling

Map token refresh HTTP 403 → `xai_oauth_tier_denied`, `relogin_required=False`. SuperGrok Heavy / entitlement; API key path is the fix.

## Agno wiring

```python
OpenAIChat(id="grok-4.5", api_key=get_xai_token_or_fallback(), base_url="https://api.x.ai/v1")
```

Hermes reference implementation (read-only): `/usr/local/lib/hermes-agent/hermes_cli/auth.py` (`_xai_oauth_*`). Full dual-auth UI pattern: skill `openhands-llm-auth`.
