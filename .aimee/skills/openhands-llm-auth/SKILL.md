---
name: openhands-llm-auth
description: "Use when adding OAuth dual-auth to OpenHands/PrayerHands."
---

# OpenHands / PrayerHands LLM authentication

Class-level guide for extending LLM credentials in OpenHands-based apps (notably **PrayerHands** at `~/src/repos/PrayerHands`). Default product path is a single `llm_api_key` + optional `llm_base_url` stored in settings and applied via `setup_llm_config`. Use this skill when the user wants OAuth, subscription login, or multiple auth methods per provider.

## When to use

- “Add SuperGrok / Grok OAuth / not just API key”
- Dual auth UI (API key **or** browser OAuth) on LLM settings
- Wiring provider tokens into LiteLLM for conversation start
- Porting Hermes-style xAI device-code login into the OpenHands stack

## Architecture map (OpenHands)

| Layer | Key paths |
|-------|-----------|
| Settings model | `openhands/storage/data_models/settings.py` |
| GET/POST settings API | `openhands/server/routes/settings.py`, `openhands/server/settings.py` |
| Apply settings → LLM | `openhands/utils/utils.py` → `setup_llm_config` |
| Session gate | `openhands/server/services/conversation_service.py` (must accept non–API-key auth) |
| Model list | `openhands/utils/llm.py` `get_supported_llm_models` |
| App routers | `openhands/server/app.py` |
| Frontend LLM screen | `frontend/src/routes/llm-settings.tsx` |
| Frontend types/defaults | `frontend/src/types/settings.ts`, `services/settings.ts`, `hooks/query/use-settings.ts`, `settings-service/settings.types.ts`, `mocks/handlers.ts` |
| Provider display names | `frontend/src/utils/map-provider.ts`, `verified-models.ts` |

## Dual-auth pattern (proven)

1. **Persist method + secrets on Settings**
   - `llm_auth_method`: `"api_key"` | `"xai_oauth"` (extensible)
   - OAuth: `*_access_token`, `*_refresh_token` as `SecretStr`; endpoint + `expires_at`
   - Serialize secrets with the same `@field_serializer` as `llm_api_key`; never return raw tokens on GET

2. **Dedicated auth routes** (don’t overload POST `/api/settings` for device polling)
   - Start device code, poll, status, refresh, logout
   - On poll success: write tokens, set `llm_auth_method`, set `llm_api_key` = access token, `llm_base_url` = provider base, normalize model to LiteLLM prefix (`xai/...`)

3. **GET `/api/settings`**
   - Expose booleans only: `llm_api_key_set`, `xai_oauth_set`, `llm_auth_method`
   - Strip secret fields before response

4. **Merge on store**
   - Preserve OAuth fields when POST omits them (same pattern as preserving `llm_api_key`)
   - If user submits a **new** API key that is not the OAuth access token, flip `llm_auth_method` back to `api_key`

5. **Inject at runtime**
   - In `setup_llm_config` / `_apply_*_oauth_credentials`: if method is OAuth, refresh (proactively near expiry) and set `llm_config.api_key` + `base_url`
   - From sync code that may already be inside an asyncio loop, run refresh via thread + `asyncio.run` (nested-loop safe)

6. **Conversation start**
   - Treat OAuth credentials as satisfying the “has API key” gate in `conversation_service`

7. **Frontend**
   - Show OAuth panel when provider/model is xAI/Grok **or** OAuth already connected
   - Hide API-key field while OAuth is active; disconnect restores key entry
   - Poll on server `interval`; handle `authorization_pending`, `slow_down`, timeout
   - Invalidate React Query `["settings"]` after login/logout

## Frontend checklist

- Extend `Settings` / `ApiSettings` / `DEFAULT_SETTINGS` / MSW mocks together
- i18n: add keys to both `declaration.ts` and `translation.json` (all langs can fall back to EN)
- Provider map: add `xai: "xAI (Grok)"`; seed models in backend list + `VERIFIED_*`

## Pitfalls

- **Don’t use only API-key fields for OAuth** — you lose refresh, method switching, and safe GET responses.
- **GET must never leak tokens** — exclude/null OAuth secrets like `llm_api_key`.
- **LiteLLM model prefix** — bare `grok-4` should become `xai/grok-4` when saving OAuth defaults.
- **Async refresh from sync** — conversation setup is often under a running loop; plain `asyncio.run` crashes. Use a one-shot thread pool.
- **Tier 403 after browser success** — not a bad token; offer API-key fallback (see xAI reference).
- **Client ID** — make `XAI_OAUTH_CLIENT_ID` env-overridable; default may be the public Grok CLI client used by Hermes.
- **Tests** — mock `httpx.AsyncClient` for device-code/poll/refresh; assert 403 maps to a distinct error code (`xai_oauth_tier_denied`) without forcing relogin.

## Verification

```bash
cd ~/src/repos/PrayerHands
poetry run pytest tests/unit/integrations/test_xai_oauth.py -q
poetry run pytest tests/unit/storage/data_models/test_settings.py \
  tests/unit/server/routes/test_settings_api.py \
  tests/unit/server/routes/test_settings_store_functions.py -q
poetry run python -c "from openhands.server.app import app; print([r.path for r in app.routes if 'xai' in getattr(r,'path','')])"
```

## References

- `references/xai-supergrok-oauth.md` — endpoints, client id, scopes, Hermes source map, PrayerHands file list
- In-repo user doc after implementation: `docs/usage/llms/supergrok-oauth.md`
- Hermes reference implementation (read-only): `/usr/local/lib/hermes-agent/hermes_cli/auth.py` (`_xai_oauth_*`, `XAI_OAUTH_*`)
- Hermes guide: https://hermes-agent.nousresearch.com/docs/guides/xai-grok-oauth
- **Standalone Agno/Python module** (no OpenHands UI): skill `agno-agentos-apps` — `tools/xai_oauth_pkce.py` device-code CLI, project token file, optional Hermes `auth.json` reuse. Use when wiring Grok into AgentOS rather than PrayerHands settings.
