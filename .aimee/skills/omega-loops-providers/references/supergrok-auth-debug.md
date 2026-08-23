# SuperGrok logged in but omega commands fail

## Symptoms

- `omega list provider` shows `SuperGrok  xai_oauth  api.x.ai  [yes]`
- `omega list model` / `omega -p` / `omega info` fail with Anthropic 401, `PriceValue`, `No default provider`, or ENXIO (`No such device or address`)
- `omega config set model xai_oauth grok-4.6` → `Provider 'SuperGrok' not found or returned no models`

Do **not** start with another device login. Tokens are often already in the legacy Forge dir.

## Probe (no token in output)

1. Config home = first existing of `~/omega`, `~/.omega`, `~/forge`, `~/.forge`.
2. Creds file: `<home>/.credentials.json`. Entry `id: xai_oauth`, `auth_details.o_auth.tokens`.
3. `GET https://api.x.ai/v1/models` with that Bearer. HTTP 200 + `grok-4.6` ⇒ product bug, not auth.
4. Session: `<home>/.omega.toml` `[session] provider_id` / `model_id`. Missing ⇒ picker / NoDefaultSession.

## Two product bugs (fixed 2026-08)

1. **Fail-closed model merge** — `join_all` + `collect::<Result<Vec<_>>>()` dropped SuperGrok when Anthropic 401'd. Must keep successes (`merge_provider_model_results` in `omega_app/src/app.rs`). Tests: `app::tests::test_merge_*`.
2. **xAI Imagine `pricing` array** — not OpenRouter `{prompt, completion}`. Deserialize must ignore non-object pricing (`deserialize_optional_pricing` in `omega_app/src/dto/openai/model.rs`). Fixture: `xai_oauth_models.json`. Test: `test_xai_oauth_models_response_deserializes`.

## Session for dogfood

```toml
# <config-home>/.omega.toml
[session]
provider_id = "xai_oauth"
model_id = "grok-4.6"
```

Or, after models list includes SuperGrok: `omega config set model xai_oauth grok-4.6`.

Never set `OMEGA_SESSION__PROVIDER_ID=xai` when you mean SuperGrok.

## Check

```bash
omega config get provider   # SuperGrok
omega config get model      # grok-4.6
omega list model --porcelain | rg 'xai_oauth|grok-4.6'
omega info --porcelain      # api.x.ai
# optional: omega -p 'Reply with exactly: SuperGrok auth ok'
```
