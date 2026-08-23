# Shopify client credentials + agency E2E

Session learnings (2026-08-11) for `/root/src/repos/ai-agency`.

## Auth model (Dev Dashboard apps)

| Legacy custom app | Dev Dashboard app |
|-------------------|-------------------|
| Static `shpat_…` Admin token in UI | **Client ID** + **Secret (`shpss_…`)** only |
| Long-lived paste token | Exchange → **~24h** `access_token` via client credentials |

```http
POST https://{shop}.myshopify.com/admin/oauth/access_token
Content-Type: application/x-www-form-urlencoded

grant_type=client_credentials
&client_id={SHOPIFY_CLIENT_ID}
&client_secret={SHOPIFY_CLIENT_SECRET}
```

Response includes `access_token`, `scope`, `expires_in` (~86399). Agency caches at `tmp/secrets/shopify_token.json` (gitignored, chmod 600).

Code: `tools/shopify_tools.py` — `_client_credentials_token`, `_shop_config` (static token wins if set).

## Required env

```bash
SHOPIFY_SHOP_NAME=your-store          # NOT optional for live API
SHOPIFY_CLIENT_ID=…
SHOPIFY_CLIENT_SECRET=shpss_…
# SHOPIFY_ACCESS_TOKEN=…             # optional override
# SHOPIFY_API_VERSION=2024-10
```

Without `SHOPIFY_SHOP_NAME`, `shopify_status()` returns stub reason and `draft_product` stubs.

## Failure modes

| Symptom | Meaning | Action |
|---------|---------|--------|
| HTML 404 Store unavailable | Shop slug does not exist | Ask user for real subdomain |
| 400 `app_not_installed` / OAuth error | Shop exists; app not installed there | Install Dev Dashboard app on **that** shop |
| `shop_not_permitted` | Client credentials not allowed for this install type | Use auth code / offline token path per Shopify docs |
| 401/403 on products | Token OK but scopes missing | Add `write_products` (etc.) and reinstall/redeploy |

Do **not** brute-force dozens of guessed store names as the primary strategy — one correct `SHOPIFY_SHOP_NAME` from the user is faster and safer.

## Product safety

- Default create path: **`status=draft`**
- Publish/active is L3/HITL (`tools/guardrails.py`)
- Never log client secret, access token, or full auth responses in chat/Telegram

## E2E runner

```bash
cd /root/src/repos/ai-agency && source .venv/bin/activate
set -a && source .env && set +a
export PYTHONPATH=. AGENCY_GROK_MODEL=grok-4.5 PYTHONUNBUFFERED=1

python -m scripts.e2e_agency_run
# or reuse last rank:
E2E_SKIP_RESEARCH=1 python -m scripts.e2e_agency_run
```

### Stages

1. Auth probes (SuperGrok, Linear, Shopify, Meta, TikTok, PromptWise)
2. Parallel search + task **or** load `tmp/runs/product_rank_*.json`
3. Unit econ + `score_supplier` + `estimate_shipping_profile`
4. Linear dual-write (+ GitHub `swcstudiospace/ai-agency` link)
5. PromptWise UGC brief → `tmp/creatives/promptwise/`
6. Shopify **draft** product
7. Meta/TikTok **drafts** + `request_spend_approval` (pending HITL — no live spend)

Artifacts: `tmp/runs/e2e_<stamp>.{json,md}` with `pass_core`, `shopify_gate`, scorecard.

### Footguns (must-use kwargs)

```python
# WRONG — 4th positional is payment_fee_pct (18.0 → absurd negative CM)
contribution_margin(price, cogs, ship, 18.0)

# RIGHT
contribution_margin(price, cogs, ship, ad_spend_per_order=18.0)

score_supplier(name=…, lead_time_days=7, moq=50, unit_cost=…, shipping_cost=…, rating=4.4)
estimate_shipping_profile(…, dest_country="US")  # not dest_zone
meta_draft_campaign(name=…, daily_budget_usd=25.0, …)
tiktok_draft_campaign(name=…, daily_budget_usd=20.0, objective_type="CONVERSIONS")
request_spend_approval(amount_usd=75.0, channel="meta", purpose="…", campaign_draft_id=…, daily_budget_usd=25.0)
```

## Verified E2E snapshot (clean run)

- Core pass with SuperGrok + Parallel prior rank + Linear SWC + PromptWise brief + HITL spend id
- Hero example: Fold-Flat Adjustable Aluminum Laptop Stand ~$49.99 CM%~34.5 (from product rank)
- Shopify gate without shop name: `BLOCKED_NEED_SHOP_NAME_OR_INSTALL` (expected)

## Related

- Repo docs: `docs/AUTONOMY_AND_KEYS.md`, `docs/CI_AND_PROMPTWISE.md`
- Shopify grant docs: https://shopify.dev/docs/apps/build/authentication-authorization/access-tokens/client-credentials-grant
