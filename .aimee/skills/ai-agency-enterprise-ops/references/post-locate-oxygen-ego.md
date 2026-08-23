# Post-locate, outreach, shipping, ego.engineer, Oxygen

Session learnings after product locate landed (SWC-78 laptop stand).

## Decision: contact seller AND shipping pipeline

| Track | When | Automation |
|-------|------|------------|
| Contact seller | Alibaba/factory/direct mfr | Sample + dropship inquiry email (HITL send) |
| Shipping pipeline | Always before ads | Order routing, SLA, Shopify profiles, webhooks |
| Platform apps | CJ/Doba/Zendrop | Reduces email; still need Shopify shipping/policies |

Do **not** treat “we emailed the supplier” as fulfillment-ready.

## Commands

```bash
cd /root/src/repos/ai-agency && source .venv/bin/activate
export PYTHONPATH=. PYTHONUNBUFFERED=1
set -a && source .env && set +a

# After product_locate_*.json exists
python -u -m scripts.autonomous_post_locate --top-suppliers 2

# Open first draft in Gmail compose (operator sends)
python -u -m scripts.autonomous_post_locate --open-gmail
```

Artifacts:
- `tmp/outreach/email_<ts>_<uuid>.md` — ready-to-send sample inquiries
- `tmp/shipping/pipeline_*.json` — lanes, SLA, steps, webhooks
- `tmp/runs/post_locate_*.{json,md}`
- Linear `[Post-locate] …` (example **SWC-80**)

## Implementation map

| Piece | Path |
|-------|------|
| Script | `scripts/autonomous_post_locate.py` |
| Outreach tools | `tools/seller_outreach_tools.py` |
| Shipping tools | `tools/shipping_pipeline_tools.py` |
| Shopify helpers | `shopify_domain_plan`, `shopify_bootstrap_checklist`, `shopify_create_policy_pages` |
| Workflow | `workflows/post_locate_fulfillment.py` |
| Discover WF step | `workflows/product_discovery_locate.py` → “Outreach + Shipping Plan” |
| Toolbelts | `outreach`, `shipping_pipeline`; merged into `supplier` + `logistics` |
| Docs | `docs/POST_LOCATE_AND_FULFILLMENT.md` |
| Docs-site | `docs-site/docs/guides/post-locate-fulfillment.md` |

### Shipping modes

- `supplier_dropship` (default) — order → supplier ships  
- `platform_cj` — CJ/Doba app auto-fulfill  
- `stock_3pl` — bulk inbound later  
- `hybrid` — soft-launch DS then 3PL  

### Gmail HITL

1. hermes-bridge up (`:7790`)
2. `open_gmail_compose` / `--open-gmail` opens Gmail web compose deep link
3. Operator logs into Gmail **once** on the bridge Playwright profile
4. Review → click **Send** — agency never auto-sends

Email draft filenames: use `f"{int(time.time())}_{uuid.uuid4().hex[:6]}"` — same-second drafts otherwise overwrite.

## Brand domain ego.engineer

Env: `AGENCY_PRIMARY_DOMAIN=ego.engineer`, `SHOPIFY_PRIMARY_DOMAIN=ego.engineer`.

Typical DNS (confirm live values in Shopify Admin → Domains):

| Type | Host | Value |
|------|------|--------|
| A | `@` | `23.227.38.65` (Shopify apex — verify) |
| CNAME | `www` | `shops.myshopify.com` |

Registrar credentials are human-owned. Tools only emit plans via `shopify_domain_plan()`.

## Headless storefront (`storefront-oxygen/`)

- Vite + React SPA, Storefront GraphQL client, demo GO SKUs until token set
- Env: `PUBLIC_STORE_DOMAIN`, `PUBLIC_STOREFRONT_API_TOKEN`, `PUBLIC_PRIMARY_DOMAIN=ego.engineer`
- Dev: `:3456` · Build: `vite build` → `dist/`
- Path to production: official Hydrogen template + Oxygen deploy + custom domain

### npm pitfall on this host

Shell often has **`NODE_ENV=production`**, so bare `npm install` installs **deps only** (~8 packages, no vite):

```bash
cd storefront-oxygen
NODE_ENV=development npm install --include=dev --legacy-peer-deps
node node_modules/esbuild/install.js   # if allow-scripts skipped postinstall
node_modules/.bin/vite build
```

## Discovery cron (6×/day)

- Name: **`agency-product-discovery-6x`**
- Schedule: **`0 */4 * * *` UTC**
- Job id (historical): `438265dac60f`
- Prompt chain: rank → locate → post_locate
- No spend / no auto-email / no active publish
- workdir `/root/src/repos/ai-agency`; deliver local

## Shopify bare-account status

Client id/secret may be present; Admin still **stub** until Dev Dashboard app is **installed** on the shop (`app_not_installed` on `aidropshipping`). After install:

```bash
python -c "from tools.shopify_tools import shopify_status; print(shopify_status())"
# expect mode=live ok=True
E2E_SKIP_RESEARCH=1 python -m scripts.e2e_agency_run
```

## Scale reminder

**30 agents · 12 teams · 12 workflows** (post_locate fulfillment registered in `app/main.py`).
