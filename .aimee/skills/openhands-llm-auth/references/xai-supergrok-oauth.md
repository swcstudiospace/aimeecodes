# xAI SuperGrok OAuth (device code) — reference

## Protocol

| Item | Value |
|------|--------|
| Auth type | OAuth 2.0 **device code** (RFC 8628) |
| Issuer | `https://auth.x.ai` |
| Discovery | `https://auth.x.ai/.well-known/openid-configuration` |
| Device code URL | `https://auth.x.ai/oauth2/device/code` |
| Default client id | `b1a00492-073a-47ea-816f-4c329264a828` (public Grok CLI / Hermes client; override with `XAI_OAUTH_CLIENT_ID`) |
| Scope | `openid profile email offline_access grok-cli:access api:access` |
| Inference base | `https://api.x.ai/v1` (override `XAI_BASE_URL` / `PRAYERHANDS_XAI_BASE_URL`) |
| LiteLLM provider prefix | `xai/` |

### Flow

1. POST device code: `client_id` + `scope` → `device_code`, `user_code`, `verification_uri`, `verification_uri_complete`, `expires_in`, `interval`
2. User opens `verification_uri_complete` (or enters code at `verification_uri`)
3. Poll token endpoint with `grant_type=urn:ietf:params:oauth:grant-type:device_code` until 200 or error
4. Soft poll errors: `authorization_pending`, `slow_down` (bump interval, cap ~30s)
5. Hard errors: `expired_token`, `access_denied`, missing refresh_token
6. Refresh: `grant_type=refresh_token` + `client_id` + `refresh_token`
7. Use **access_token** as Bearer / LiteLLM `api_key`

### Security checks

- Only accept token/authorize endpoints under `*.x.ai` / `x.ai` over HTTPS
- Never return access/refresh tokens on GET settings — only `xai_oauth_set` / `llm_auth_method`

## Tier gating (HTTP 403)

Browser login can succeed while refresh/inference returns **403** (“caller does not have permission…”). That is usually **tier/entitlement**, not a stale token. Do **not** loop re-login as the fix.

- Map to code like `xai_oauth_tier_denied`, `relogin_required=False`
- UI/docs: fall back to `XAI_API_KEY` / console.x.ai API key path
- SuperGrok Heavy is the intended subscription tier for this surface; standard SuperGrok has been seen rejected (Hermes issue #26847)

## Hermes source map (read-only reference)

- `/usr/local/lib/hermes-agent/hermes_cli/auth.py`
  - Constants ~`XAI_OAUTH_*`
  - `_xai_oauth_discovery`, `refresh_xai_oauth_pure`, `_xai_oauth_request_device_code`, `_xai_oauth_poll_device_token`, `_xai_oauth_device_code_login`, `resolve_xai_oauth_runtime_credentials`
- Docs: https://hermes-agent.nousresearch.com/docs/guides/xai-grok-oauth

Do not copy Hermes auth store (`~/.hermes/auth.json`) into PrayerHands; implement OpenHands Settings + dedicated routes instead.

## PrayerHands implementation (2026-08)

### Backend

- `openhands/integrations/xai/oauth.py` — async httpx client
- `openhands/integrations/xai/__init__.py`
- `openhands/server/routes/xai_oauth.py` — `/api/llm-auth/xai/*`
- Settings fields: `llm_auth_method`, `xai_oauth_access_token`, `xai_oauth_refresh_token`, `xai_oauth_token_endpoint`, `xai_oauth_expires_at`
- `openhands/utils/utils.py` — `_apply_xai_oauth_credentials` + nested-loop-safe `_run_coroutine_sync`
- `openhands/server/services/conversation_service.py` — OAuth satisfies credential gate
- `openhands/utils/llm.py` — seed `xai/grok-*` models
- Tests: `tests/unit/integrations/test_xai_oauth.py`
- User doc: `docs/usage/llms/supergrok-oauth.md`

### Frontend

- `frontend/src/api/xai-oauth-service.ts`
- `frontend/src/components/features/settings/llm-settings/supergrok-oauth-panel.tsx`
- Wired in `frontend/src/routes/llm-settings.tsx` (basic + advanced)
- Types/defaults/mocks/i18n/map-provider/verified-models updated together

### API

| Method | Path |
|--------|------|
| GET | `/api/llm-auth/xai/status` |
| POST | `/api/llm-auth/xai/device-code` |
| POST | `/api/llm-auth/xai/poll` |
| POST | `/api/llm-auth/xai/refresh` |
| DELETE | `/api/llm-auth/xai` |

Default OAuth model when unset: `xai/grok-4`.

## Suggested models to seed

- `xai/grok-4`, `xai/grok-4-0709`, `xai/grok-4.5`
- `xai/grok-code-fast-1`
- `xai/grok-3`, `xai/grok-3-mini`
- `xai/grok-2-1212`, `xai/grok-2-vision-1212`
