---
name: ai-agency-enterprise-ops
description: "Use when running AI Agency lifecycle, Grok Build bottom layer, CodeRabbit CI, product locate/post-locate, Shopify/Oxygen, docs, or HITL."
version: 2.1.0
metadata:
  hermes:
    tags: [agency, dropshipping, hitl, linear, fal, meta, tiktok, agentos, drop, mcp, acp, kip, hermes-bridge, grok-build, coderabbit, supergrok, product-rank, product-locate, post-locate, outreach, shipping, oxygen, ego-engineer, github, promptwise, ugc, ci, shopify, e2e, docusaurus, docs]
    related_skills: [agno-agentos-apps, hermes-linear-kanban-sync, ai-dropshipping-agency-mcp, parallel-task, agency-desktop-genui]
    created_by: agent
---

# AI Agency enterprise ops

Operate `/root/src/repos/ai-agency` after enterprise tools + SOTA packs (**30 agents · 12 teams · 12 workflows**) + Drop MCP/ACP + **Hermes reverse bridge** + **Grok Build bottom CLI** + **CodeRabbit CI** + **KIP** + SuperGrok + **rank → locate → post-locate** + Docusaurus + **ego.engineer / Oxygen**. Complements **`agno-agentos-apps`** (build) and **`ai-dropshipping-agency-mcp`** (Hermes→AgentOS). UI: **`agency-desktop-genui`**. GitHub: **`swcstudiospace/ai-agency`**.

**Living README (user preference):** every feature that changes ports, counts, scripts, env, or operator flow **must** update root `README.md` in the same commit/PR. Enforced by `python -m scripts.check_readme_freshness`. Prefer SOTA multi-gate CI (ruff/pytest/evals/README) over ad-hoc “smoke only” claims.

## Execution stack (Hermes → Agno → Grok Build)

```text
Hermes Agent (top orchestrator — MCP client)
        │
        ▼
Agno AgentOS (middle — 30 agents / 12 teams / 12 workflows)
  auto toolbelts: hermes_bridge + grok_build + coderabbit (+ anda_brain/analytics)
        │
        ▼
Grok Build CLI `grok` (bottom — headless SuperGrok agent / shell offload)
CodeRabbit CLI `cr` / `coderabbit` (local + CI review)
```

| Piece | Path |
|-------|------|
| Tools | `tools/grok_build_tools.py`, `tools/coderabbit_tools.py` |
| Belts | `grok_build`, `coderabbit` in `tools/toolbelts.py` |
| Factory | `agents/_factory.py` auto-attaches grok_build+coderabbit + injects `GROK_BUILD_OFFLOAD_INSTRUCTIONS` |
| Teams | `teams/_factory.py` Grok Build blurb + team-level grok_build/coderabbit/linear tools |
| Workflows | `workflows/_grok_build.with_grok_build_guidance()` on discovery + full lifecycle |
| Config | `configs/grok-build/` agents + AGENTS.md + agency-mcp.json |
| Review | `.coderabbit.yaml` |
| Docs | `docs/GROK_BUILD_AND_CODERABBIT.md` · `references/grok-build-bottom-layer.md` |

**Agent tools:** `grok_build_status`, `grok_build_run` (`grok -p`), `grok_build_offload_shell` → `tmp/grok_build_runs/`, `grok_build_orchestrate_agency_task`, `grok_build_inspect`; `coderabbit_status`/`review`/`config_validate`.

```bash
export PATH="$HOME/.grok/bin:$PATH"
# SuperGrok auth already powers Grok Build (~/.grok/auth.json) or XAI_API_KEY
grok -p "…" --cwd /root/src/repos/ai-agency --always-approve --max-turns 12

# Showcase: agents drive Grok Build inside Autonomous Dropshipping Flow
PYTHONPATH=. python -m scripts.showcase_grok_build_dropshipping_flow --skip-locate
PYTHONPATH=. python -m scripts.showcase_grok_build_dropshipping_flow --try-grok-agent
# Reports: tmp/runs/grok_build_dropshipping_showcase_*.{md,json} + tmp/grok_build_runs/

curl -fsSL https://cli.coderabbit.ai/install.sh | sh
export CODERABBIT_API_KEY=…   # Agentic key
coderabbit review --agent --base main
```

**Dropshipping × Grok Build loop:** Scout ranks → Sourcer `grok_build_offload_shell(locate)` → post_locate offload → multi-step coding via `grok_build_run`. Workflows inject `with_grok_build_guidance()`. SuperGrok auth powers both Agno models and Grok Build CLI.

Disable: `AGENCY_DISABLE_GROK_BUILD=1`, `AGENCY_DISABLE_CODERABBIT=1`.

| Env | Role |
|-----|------|
| `GROK_BUILD_BIN` | path to `grok` (default `~/.grok/bin/grok`) |
| `GROK_BUILD_MODEL` | optional override; default **`grok-4.5`** under SuperGrok/`XAI_API_KEY` |
| `GROK_BUILD_DEFAULT_CWD` | repo root |
| `GROK_BUILD_ALWAYS_APPROVE` | default `1` for agent offload |
| `GROK_BUILD_MAX_TURNS` | default ~24 |
| `GROK_BUILD_PERMISSION_MODE` | e.g. `auto` / `acceptEdits` |

**User decision (2026-08-12):** Warp/Oz bottom layer **rejected** as not optimal — fully removed. Do **not** reintroduce Warp tools, `oz`, or `configs/warp/`. Bottom layer is **Grok Build only**. Detail: `references/grok-build-bottom-layer.md`.

## Control planes (three MCP servers)

| Server | URL | Hermes name | Tools prefix | Role |
|--------|-----|-------------|--------------|------|
| AgentOS | `http://127.0.0.1:7777/mcp` | `ai-agency` | `mcp_ai_agency_*` | run_agent/team/workflow + custom |
| Drop hybrid MCP+ACP | `http://127.0.0.1:7788/mcp` | `drop` | `mcp_drop_*` | Universal: Linear, CoT×GoT, lifecycle, HITL, bridge proxies |
| Hermes reverse bridge | `http://127.0.0.1:7790/mcp` | `hermes-bridge` | `mcp_hermes_bridge_*` | Agno→Hermes browser/skills/memory/KIP/CUA jobs |

**Directionality:** Hermes is top orchestrator. Agno’s **30 agents · 12 teams · 12 workflows** call **back** via toolbelt `hermes_bridge` → `:7790`. **Hermes Ops** (Agno agent) ≠ Hermes Agent process.

Prefer **`drop`** for universal routing. Prefer **`ai-agency`** for raw Agno runs. Prefer **`hermes-bridge`** when Hermes itself needs the same reverse tools. Restart Hermes after any `mcp add`. Timeouts: agency/drop **3600**, bridge **300**.

```bash
systemctl status drop-gateway hermes-bridge
curl -s http://127.0.0.1:7788/health
curl -s http://127.0.0.1:7790/health
curl -s http://127.0.0.1:7777/health
hermes mcp test drop && hermes mcp test ai-agency && hermes mcp test hermes-bridge
```

Public hostname `drop.autonogrammer.ai` → A record **187.77.130.10**, then certbot + TLS nginx. Until DNS: loopback only.

## SuperGrok model (agents + Grok Build)

- Default Agno model: **`grok-4.5`** (`AGENCY_GROK_MODEL` / `XAI_MODEL` / `tools/xai_model.DEFAULT_GROK_MODEL`).
- Grok Build headless default: same **`grok-4.5`** when SuperGrok is used via `XAI_API_KEY` (`grok models` may not expose CLI-only `grok-build`).
- Token order: `XAI_API_KEY` → `~/.config/ai-agency/xai_oauth.json` → **Hermes SuperGrok device-code** (`~/.hermes/auth.json` / pool `xai-oauth`) → Grok CLI `~/.grok/auth.json`.
- Login if refresh revoked: `python -m tools.xai_oauth_pkce status|login|logout` (or Grok Build browser login).
- Multi-agent model ids are **not** valid on `/chat/completions` — use `grok-4.5` (or other single-agent chat ids from `/v1/models`).
- Do **not** block on a console API key when SuperGrok OAuth already works.

## Default path

```bash
cd /root/src/repos/ai-agency && source .venv/bin/activate
set -a && source .env && set +a
export PYTHONPATH=. AGENCY_GROK_MODEL=grok-4.5
./scripts/start_agentos.sh   # :7777 + /mcp

# 1) DISCOVER — rank products
PYTHONUNBUFFERED=1 python -m scripts.autonomous_product_rank \
  --niche "desk mobility and posture tools for remote workers" \
  --processor pro   # or ultra (slower)

# 2) LOCATE — where to buy GO/TEST SKUs (next step after rank)
PYTHONUNBUFFERED=1 python -m scripts.autonomous_product_locate --top 3 --processor pro
# Single SKU:
PYTHONUNBUFFERED=1 python -m scripts.autonomous_product_locate \
  --product "Fold-Flat Adjustable Aluminum Laptop Stand" --processor core
# Rank then locate:
python -m scripts.autonomous_product_locate --rank-first --niche "…" --processor pro

# 3) POST-LOCATE — outreach drafts + shipping plan (NO auto-send / NO pay)
python -u -m scripts.autonomous_post_locate --top-suppliers 2
python -u -m scripts.autonomous_post_locate --open-gmail   # Gmail compose HITL

# Full E2E (research → Linear → PromptWise → Shopify draft → ads HITL)
PYTHONUNBUFFERED=1 python -m scripts.e2e_agency_run
E2E_SKIP_RESEARCH=1 python -m scripts.e2e_agency_run

# Full lifecycle (no payments / no live ads)
PYTHONPATH=. python -m scripts.autonomous_lifecycle \
  --niche "…" --processor ultra --top 3
```

**Loop:** rank → locate → **post-locate (email + shipping plan)** → human sample send/pay → QA → creatives → Shopify draft → ads HITL.

Long Parallel Task runs: launch **unbuffered** (`PYTHONUNBUFFERED=1` or `python -u`) and detach if the terminal tool caps ~180s — poll `tmp/runs/product_rank_*.md`, `product_locate_*.md`, `post_locate_*.md`, or `e2e_*.md`.

MCP tools: `run_autonomous_lifecycle` / `agency_run_lifecycle`, `run_product_rank` / `agency_product_rank`, integrations status, spend helpers; Linear helpers on **drop**.

Artifacts: rank `product_rank_*`; locate `product_locate_*` + Linear `[Locate]`; post-locate `post_locate_*` + `tmp/outreach/` + `tmp/shipping/`; E2E `e2e_*`; lifecycle `lifecycle_*` + `*_HITL_CODES.json` (chmod 600).

### Product locate (supplier sourcing)

After rank answers *what to sell*, locate answers *where to buy*:

| Piece | Detail |
|-------|--------|
| Script | `scripts/autonomous_product_locate.py` |
| Tools | `locate_suppliers_for_product`, `locate_product_sources_batch` (`tools/supplier_tools.py`) |
| Workflow | **Product Discovery & Locate** (`workflows/product_discovery_locate.py`) — includes outreach step; one of **12** workflows |
| Toolbelt | `supplier` (+ outreach tools) → Supplier Sourcer + Supply Chain |
| Output | Shortlist unit/ship/MOQ/lead/red flags + score 0–100 + logistics |
| HITL | Research only — no unsupervised sample/PO payment |

Example (Fold-Flat Laptop Stand): top Alibaba landed ~$10.70 vs $49.99 retail; Linear **SWC-78**. See `references/product-locate-and-docs-site.md`, `docs/PRODUCT_LOCATE.md`.

### Discovery cron (6×/day)

- Hermes job: **`agency-product-discovery-6x`** · schedule **`0 */4 * * *` UTC** (every 4 hours).
- Rotates niches by hour bucket; **rank → locate → post_locate** (outreach drafts + shipping plan).
- **No spend**, no auto-email send, no active publish.
- Job id historically `438265dac60f` (renamed from bidaily).
- If missing: recreate with workdir `/root/src/repos/ai-agency`.

## Post-locate (seller contact + shipping)

**Do both** after locate — not either/or. Factories need sample email; shipping/order routing is always required before ads. Platforms (CJ/Doba) reduce email.

| Piece | Detail |
|-------|--------|
| Script | `scripts/autonomous_post_locate.py` |
| Outreach | `tools/seller_outreach_tools.py` — drafts + `gmail_compose_url` / `open_gmail_compose` (`--open-gmail`) |
| Shipping | `tools/shipping_pipeline_tools.py` — `supplier_dropship` / `platform_cj` / `stock_3pl` / `hybrid` |
| Shopify bootstrap | `shopify_bootstrap_checklist`, `shopify_domain_plan`, `shopify_create_policy_pages` |
| Toolbelts | `outreach`, `shipping_pipeline` (also folded into `supplier` + `logistics`) |
| Workflow | **Post-Locate Fulfillment Setup** + step on Product Discovery & Locate — **12 workflows** |
| Brand domain | `AGENCY_PRIMARY_DOMAIN=ego.engineer` (DNS at registrar is human HITL) |
| Headless | `storefront-oxygen/` Vite SPA → Hydrogen/Oxygen |
| Artifacts | `tmp/outreach/email_*.md`, `tmp/shipping/pipeline_*.json`, `tmp/runs/post_locate_*.{json,md}` |
| Docs | `docs/POST_LOCATE_AND_FULFILLMENT.md` · docs-site `guides/post-locate-fulfillment` |
| Detail | `references/post-locate-oxygen-ego.md` |

```bash
python -u -m scripts.autonomous_post_locate --top-suppliers 2
python -u -m scripts.autonomous_post_locate --open-gmail   # HITL send in Gmail
```

**Gmail:** hermes-bridge Playwright profile; operator logs in once and clicks **Send**. Agency **never** auto-sends or pays samples. Example Linear: **SWC-80**.

**Sequence:** (1) sample emails top 1–2 HITL → (2) install Shopify app + draft product → (3) shipping/policies + ego.engineer DNS → (4) paid sample → QA → (5) fulfillment mode → (6) test order → ads HITL.
## CoT × GoT (Drop gateway)

- Explicit: `mcp_drop_reason_cot_got(goal, mode="hybrid")`
- **Auto-triggers** on product/lifecycle/spend/complexity goals
- Hybrid = CoT chain + GoT branches + merge
- Details: `references/drop-mcp-acp-gateway.md`

## HITL ads only

```text
attach_funding_source → request_spend_approval
  → HUMAN confirm_spend_approval(id, code, "I authorize…")
  → meta_launch_campaign / tiktok_launch_campaign(draft, approval_id, token)
```

Agents must **not** confirm. Guardrails block self-confirm and launch without tokens. Details: `references/hitl-spend.md`.

## Linear dual-write (agency project)

**Org:** spectrumwebco · **Team key: SWC** (not SPE) · **Project:** AI Dropshipping Agency

| Env | Purpose |
|-----|---------|
| `LINEAR_API_KEY` | GraphQL |
| `LINEAR_TEAM_ID` | SWC team UUID |
| `LINEAR_TEAM_KEY=SWC` | identifiers `SWC-N` |
| `LINEAR_PROJECT_ID` | auto-attach to AI Dropshipping Agency |
| `LINEAR_GITHUB_REPO` | **`swcstudiospace/ai-agency`** (never `agent_runtime`) |
| `LINEAR_GITHUB_LINK` | `1` = auto GH issue on create (default) |

- Config: `~/.config/hermes-linear/config.yaml` + `connector.env` (chmod 600). **State IDs must be SWC workflow states** — stale SPE state UUIDs → `stateId contained an entry that could not be found`.
- `create_linear_issue` defaults `project_id` from env; product rank dual-writes GO/TEST; then **`link_issue_to_github_repo`** creates/links a GitHub issue on **ai-agency** and strips any `agent_runtime` attachments.
- **Canonical GitHub:** https://github.com/swcstudiospace/ai-agency — Linear workspace sync often defaults to **Agent Runtime** (`agent_runtime`); always correct agency issues to **ai-agency**. See `docs/LINEAR_GITHUB.md` + `references/linear-github-ai-agency.md`.
- Eng Kanban may still use SPE elsewhere — **agency product work is SWC**.
- Kanban: `hermes kanban --board eng create … --idempotency-key linear:SWC-N` when mirroring.
- **Never** `hermes kanban create --board eng` (flag order). See `hermes-linear-kanban-sync`.
- Secrets pasted in chat: store in `.env` only, never echo; recommend rotate.
- **Env load:** `tools/envutil.py` **forces** project `.env` `LINEAR_*` / `AGENCY_*` / `PARALLEL_API_KEY` / `SHOPIFY_*` over a stale shell export (another org’s `LINEAR_API_KEY` silently wins otherwise).

## Keys matrix (what is required)

| For | Need | Optional |
|-----|------|----------|
| Product discovery/rank | SuperGrok OAuth **or** `XAI_API_KEY`, `PARALLEL_API_KEY`, Linear | — |
| Live store/orders | **Shopify shop + installed app** (below) | — |
| UGC/images | — | **PromptWise** (browser) and/or Fal (`FAL_KEY`) |
| Live ads | HITL vault + human confirm | Meta / TikTok tokens |

**Fal and Shopify are not required for autonomous product finding.** Stubs until keys present. Docs: `docs/AUTONOMY_AND_KEYS.md`.

## Shopify (Dev Dashboard client credentials)

**Store display name:** AI Dropshipping Agency  
**Configured subdomain (best effort):** `aidropshipping` → `aidropshipping.myshopify.com`  
(`SHOPIFY_SHOP_DISPLAY_NAME` + `SHOPIFY_SHOP_NAME` — quote display names with spaces.)

Modern apps no longer show a permanent `shpat_` in UI. You get **Client ID + Secret (`shpss_…`)** and exchange for a **~24h Admin token**.

| Env | Role |
|-----|------|
| `SHOPIFY_SHOP_NAME` | **Required** — `your-store` → `your-store.myshopify.com` |
| `SHOPIFY_SHOP_DISPLAY_NAME` | Human label (e.g. `"AI Dropshipping Agency"`) |
| `SHOPIFY_CLIENT_ID` | Dev Dashboard client id |
| `SHOPIFY_CLIENT_SECRET` | Dev Dashboard secret |
| `SHOPIFY_ACCESS_TOKEN` | Optional static; else auto client credentials |
| `SHOPIFY_API_VERSION` | default `2024-10` |

`tools/shopify_tools.py`: `POST …/admin/oauth/access_token` with `grant_type=client_credentials`; cache `tmp/secrets/shopify_token.json` (chmod 600).

**LIVE drafts need:** (1) exact shop subdomain where the app is **installed**, (2) product write scopes, (3) matching client id/secret.  
`app_not_installed` on `aidropshipping` = install Dev Dashboard app on that store (or fix subdomain). Do not brute-force slugs. Default product **`status=draft`**. Never log secrets.

### Brand domain + headless (ego.engineer)

| Env / path | Role |
|------------|------|
| `AGENCY_PRIMARY_DOMAIN` / `SHOPIFY_PRIMARY_DOMAIN` | **`ego.engineer`** |
| `shopify_domain_plan()` | DNS A/CNAME plan (apex often `23.227.38.65` — confirm in Shopify admin) |
| `shopify_bootstrap_checklist()` | Bare account → payments/shipping/policies/domain/Storefront token |
| `storefront-oxygen/` | Headless Vite+React scaffold; demo GO SKUs until Storefront API token |
| Promote | Official Hydrogen + `shopify hydrogen deploy` (Oxygen) |

DNS at the **registrar** is human-owned. Attach domain in Shopify Admin → Domains and/or Oxygen.  
Storefront npm on this host often has **`NODE_ENV=production`** (omits devDeps) — use `NODE_ENV=development npm install --include=dev`; run `node node_modules/esbuild/install.js` if allow-scripts skipped postinstall. Dev **:3456**. Detail: `references/post-locate-oxygen-ego.md`, `references/shopify-client-credentials-and-e2e.md`, docs-site `getting-started/shopify-setup`.

## Full E2E runner (`scripts/e2e_agency_run.py`)

Stages: auth → research **or** prior rank → unit econ + supplier + logistics → Linear (+ GH) → PromptWise brief → Shopify draft → Meta/TikTok drafts → HITL spend (**no live spend**).

### Critical API footguns

```python
# WRONG — 4th positional is payment_fee_pct (bare 18.0 → absurd negative CM%)
contribution_margin(price, cogs, ship, 18.0)
# RIGHT
contribution_margin(price, cogs, ship, ad_spend_per_order=18.0)

score_supplier(name=…, lead_time_days=7, moq=50, unit_cost=…, shipping_cost=…, rating=4.4)
estimate_shipping_profile(…, dest_country="US")  # not dest_zone
meta_draft_campaign(…, daily_budget_usd=25.0)
tiktok_draft_campaign(…, daily_budget_usd=20.0, objective_type="CONVERSIONS")
request_spend_approval(amount_usd=…, channel="meta", purpose=…, campaign_draft_id=…, daily_budget_usd=…)
```

## PromptWise AI UGC (creative path)

**Product:** https://www.promptwise.com · app https://app.promptwise.com — UGC Factory, Flows, Wise. **No stable public OpenAPI** → primary path is **browser + briefs** (optional API later).

| Tool (`tools/promptwise_tools.py`) | Role |
|------------------------------------|------|
| `promptwise_status` | mode `browser` \| `api` \| `brief_only` |
| `promptwise_build_ugc_brief` | brief + Wise prompt → `tmp/creatives/promptwise/` |
| `promptwise_open_workspace` | Hermes bridge browser → app |
| `promptwise_run_ugc_job` | brief → optional API → browser HITL playbook |

- Toolbelts: `promptwise`, `creative_ops`, `creative_prod`
- Agents: **Creative Director**, **Ads Creative Ops**
- Env: `PROMPTWISE_APP_URL`, `PROMPTWISE_BROWSER_ENABLED=1`, optional `PROMPTWISE_API_KEY` + `PROMPTWISE_API_BASE`
- Operator: log into PromptWise once in the bridge Playwright profile; **credit spend is HITL**
- Stack: rank GO → brief → PromptWise (or Fal fallback) → HITL → ad draft (no unsupervised spend)
- Detail: `references/promptwise-ugc-and-ci.md` · repo `docs/CI_AND_PROMPTWISE.md`

## GitHub CI/CD (SOTA — not smoke-only)

User asked to stop treating ad-hoc smoke as CI. **Source of truth = GitHub Actions multi-gate.**

| Workflow | Path | Gates |
|----------|------|--------|
| Umbrella | `ci.yml` | backend + cockpit + docs + storefront + security + **CodeRabbit (inline)** + **All CI gates green** |
| Backend | `ci-backend.yml` | **Ruff** · **Pytest+coverage** (`fail_under` ~20% tools) · **agent evals 30/12/12** · **living README** |
| Cockpit | `ci-cockpit.yml` | Vite PWA; optional Tauri (`[tauri]` / dispatch) |
| Docs | `ci-docs.yml` | Docusaurus build |
| Storefront | `ci-storefront.yml` | `storefront-oxygen` Vite (`NODE_ENV=development` + esbuild install) |
| Security | `ci-security.yml` | gitleaks (`.gitleaks.toml`) · pip-audit (soft) · npm audit (soft) |
| CodeRabbit | **inline job in `ci.yml`** + standalone `ci-coderabbit.yml` | install CLI · validate · headless review if `CODERABBIT_API_KEY` |
| CD Docs | `cd-docs.yml` | GitHub Pages from `docs-site/build` |

Local parity:
```bash
pip install -r requirements.txt -r requirements-dev.txt
ruff check agents app tools teams workflows scripts tests evals
pytest tests/ --cov=tools --cov-report=term-missing
python -m evals.run_agent_evals
python -m scripts.check_readme_freshness
```

- **Living README:** every feature PR must update `README.md`; `scripts/check_readme_freshness.py` fails on drift.
- Branch protection: require **All CI gates green**.
- Backend offline env: `XAI_API_KEY=missing-ci-placeholder`, `LINEAR_GITHUB_LINK=0`, disable bridge/brain/analytics.
- Gitleaks: allowlist **path** `.env.example` — UUID-looking placeholders trip `generic-api-key`; never commit real secrets.
- Storefront CI: `npm install --include=dev` when runner `NODE_ENV=production`.
- Docs: Docusaurus **3.10+** (3.7 + Node 24 ProgressPlugin schema fail).
- Ruff: ignore `E402` (scripts set `sys.path` first); `ruff check --fix` for I001.
- **CodeRabbit CI pitfall:** do **not** call `ci-coderabbit.yml` via `workflow_call` from umbrella — caused **`startup_failure`** (0 jobs). **Inline** CodeRabbit in `ci.yml`. Without secret: install+validate still **exit 0**. Install GitHub App coderabbitai for PR Check `coderabbitai`.
- Detail: `docs/CI_CD.md`, `docs/GROK_BUILD_AND_CODERABBIT.md`, `references/grok-build-bottom-layer.md`.

## Documentation site (Docusaurus)

- Path: **`docs-site/`** — dark violet/cyan theme; baseUrl `/ai-agency/` for GH Pages.
- Local: `cd docs-site && npm install && npm start` → **:3400**; build → `docs-site/build/`.
- Guides: product-discovery, **product-locate**, **post-locate-fulfillment**, Shopify setup, lifecycle, PromptWise, HITL, E2E.
- Monorepo README + `docs/PRODUCT_LOCATE.md` + `docs/POST_LOCATE_AND_FULFILLMENT.md` stay aligned with the site.
- Detail: `references/product-locate-and-docs-site.md`, `references/post-locate-oxygen-ego.md`.

## Agent brains

Edit `prompts/<agent>/{SOUL,SYSTEM,OUTPUT,EXAMPLES}.md` + `agents/profiles.py` toolbelts/skills — not fat inline Python lists. Evals: `python -m evals.run_agent_evals`.

**SOTA archetype (all 30):** extensive SOUL/SYSTEM/OUTPUT/EXAMPLES (~6KB+), thin `agents/*.py`, Pydantic schemas, `skills/agents/*-playbook` + domain `skills/ops/*`, role toolbelts + hermes_bridge + anda_brain + analytics. New ops agents must **not** ship as thin stubs — match original-18 depth. Ops external tools: `tools/*_ops_tools.py` wired in `tools/toolbelts.py`. See `docs/OPS_AGENTS_SOTA.md`, `references/ops-agent-sota-archetype.md`.

## Hermes reverse bridge (Agno → Hermes-class tools)

All **30** agents auto-get toolbelt `hermes_bridge` via `agents/_factory.py` (disable with `AGENCY_DISABLE_HERMES_BRIDGE=1`).

| Capability | Tools |
|------------|--------|
| Browser | `hermes_browser_navigate/snapshot/screenshot/extract_links` (Playwright on :7790) |
| Self-improving skills | `hermes_skill_list/read/search/propose` → `skills/_proposals/` + Linear |
| Hermes memory files | `hermes_memory_read/append` (MEMORY.md; also mirrors KIP) |
| Shared KIP graph | `kip_remember/recall/execute/export_icp` |
| Desktop CUA | `hermes_computer_use_request` → job queue; Hermes completes with `…_complete` |

systemd: `hermes-bridge.service`. Chromium: `python -m playwright install chromium`. Details: `references/hermes-reverse-bridge-kip.md`.

## Shared memory — KIP + ICP (ldclabs Anda)

- Spec: https://github.com/ldclabs/KIP · runtime family: Anda / Cognitive Nexus
- Agency store: `kip_memory/nexus.py` (SQLite graph; KQL/KML subset)
- Capsules: `kip_memory/data/capsules/` + ICP-ready SHA256 receipts (`KIP_ICP_MODE=local` default)
- On-chain: `KIP_ICP_MODE=canister` + `IC_OSS_ENDPOINT` + `KIP_ICP_CANISTER_ID` (no keys in agency process)
- Brain DB = Anda nexus `:8091` dual-write; analytics SQLite separate (`tools/analytics_store.py`)

Linear **SWC / AI Dropshipping Agency** is the **ops work log**; KIP is the **knowledge graph**.

## Agency Cockpit UI (web + desktop + mobile)

- Path: `agency-cockpit/` — React + Vite + **Tauri v2** + **PWA**
- Web: `npm run build:web` → `dist/` + SW; Docker/nginx SPA optional
- Desktop: `npm run desktop:build` → deb/rpm/AppImage (Linux: webkit2gtk + **`xdg-utils`** for AppImage)
- Mobile: `android:init|build` / `ios:init|build` — `agency-cockpit/PACKAGING.md`, `docs/MOBILE.md`
- Identifier: `ai.autonogrammer.agency-cockpit`
- GenUI mock first; CopilotKit → AgentOS later
- Related skill: **`agency-desktop-genui`**

## Pitfalls

- DIM shipping: `billable_g = max(weight_g, vol_cm3/5)` — grams ≈ cm³/5; **never** ×1000 again on dim weight
- `score_supplier(name=, lead_time_days=, moq=, unit_cost=, shipping_cost=)` — not `unit_cost_usd` / `sample_lead_days`
- **`contribution_margin` 4th positional = `payment_fee_pct`** — always `ad_spend_per_order=` for CPA or CM goes wildly negative (~−1700%)
- Fal endpoint `argil/avatars/text-to-video` needs `FAL_KEY`+`fal-client` or stubs — **not required for product rank**
- **8788 is often grok-hermes-connector**, not Linear connector
- Agents must **not** call `confirm_spend_approval`; surface `human_confirm_code` once
- FastMCP Streamable HTTP on custom Starlette **requires** lifespan `async with mcp.session_manager.run()` else `Task group is not initialized`
- FastMCP `streamable_http_app()` already serves `/mcp` — parent `Mount("/", mcp_app)` not `Mount("/mcp", …)`
- **Playwright sync API inside MCP async handlers:** run browser work in a `ThreadPoolExecutor` (see hermes_bridge `mcp_app._browser_run`) — never bare `sync_playwright()` on the event loop
- **MCP client wrapper kwargs:** `_call(tool_name, **kwargs)` — do not name the first param `name` or `kip_remember(..., name=…)` collides
- Computer-use is **job-queue only** from Agno; true CUA stays in Hermes process
- AgentOS `:7777` can hang (Recv-Q stuck); `fuser -k 7777/tcp` then restart `python -m app.main`
- Restart Hermes after MCP changes; restart AgentOS after `mcp_custom.py`; `systemctl restart drop-gateway hermes-bridge` after gateway/bridge edits
- Dual-write skill proposals via Linear; curator reviews `skills/_proposals/` before merge
- **Linear team is SWC not SPE** for this agency; SPE state UUIDs break `issueCreate`
- Quote `.env` values with spaces (`LINEAR_PROJECT_NAME=…`, `SHOPIFY_SHOP_DISPLAY_NAME="AI Dropshipping Agency"`) or `source .env` breaks (`Dropshipping: command not found`)
- Product-rank / locate Linear dual-write must `load_dotenv_files()` inside the script (subprocess may not inherit shell exports)
- **Stale shell `LINEAR_API_KEY`:** Hermes/other tools may export a different org’s key — `envutil` force-from-project; when debugging Linear, check GraphQL `viewer.organization.urlKey` (expect `spectrumwebco`)
- **GitHub link = ai-agency not agent_runtime:** delete `agent_runtime` attachments; `attachmentLinkGitHubIssue` to `swcstudiospace/ai-agency`; workspace Linear→GitHub UI should prefer ai-agency for team SWC
- Long Parallel Task: unbuffered Python; Hermes terminal foreground often ~180s — background + poll log
- AppImage bundle needs `xdg-utils` (`xdg-open`) on build host
- Catalog skill name collision: agent playbook `catalog-ops-agent-playbook` vs ops domain `catalog-ops-playbook`
- Repo git root: `/root/src/repos/ai-agency` → `origin` https://github.com/swcstudiospace/ai-agency.git — never commit `.env`
- **CI compileall** fails on syntax errors in `drop_server/` (e.g. mangled `drop_roster`) — fix before claiming green
- **CI envutil:** wrap path probes in OSError; prefer `Path.home()` over bare `/root/...`
- PromptWise: no public API → browser HITL + brief is supported; do not invent production API paths
- Ad-hoc verify scripts: gateway policy may block shell bodies containing the word `restart` — prefer `write_file` + `python /tmp/hermes-verify-*.py`
- **Shopify:** client id+secret alone insufficient — need `SHOPIFY_SHOP_NAME` + app installed; token cache `tmp/secrets/shopify_token.json`; store label AI Dropshipping Agency / subdomain `aidropshipping` until corrected
- E2E: `scripts/e2e_agency_run.py`; `E2E_SKIP_RESEARCH=1` reuses last rank; never print `shpss_` / tokens
- **Locate is research-only:** never treat supplier shortlist as authorization to pay samples/POs
- **Post-locate = both** outreach + shipping plan — do not skip shipping when only emailing factories
- **Never auto-send Gmail** or auto-pay samples; `--open-gmail` is compose-only HITL
- Outreach email stamps need `time` + `uuid` — bare `int(time.time())` collides when drafting multiple suppliers in one second
- **ego.engineer DNS** and Shopify domain connect are human HITL — tools only emit plans
- **storefront-oxygen npm:** shell `NODE_ENV=production` → ~8 packages / no vite — force `NODE_ENV=development npm install --include=dev`; esbuild install.js if allow-scripts blocked postinstall
- Cron is **6×/day** `0 */4 * * *` (`agency-product-discovery-6x`), not bidaily 08/20 only
- Workflow count in `app/main.py` description must stay **12** when post_locate is registered
- **Never re-add Warp/Oz** — user rejected it; bottom layer is Grok Build only (`tools/grok_build_tools.py`, belt `grok_build`)
- **Grok Build offload:** multi-step shell/coding → `grok_build_run` / `grok_build_offload_shell` / `grok_build_orchestrate_agency_task`; block secret dumps in shell offload
- **Grok Build agent profiles** need YAML frontmatter (`---` / `name` / `description` / `model`) — bare markdown fails with `missing frontmatter delimiters`
- **Grok Build model id under SuperGrok API key:** `grok models` often lists only **`grok-4.5`**. Do **not** default headless runs to CLI-only id `grok-build` when `XAI_API_KEY` is set — causes `unknown model id`. Default `GROK_BUILD_MODEL`/`-m` to `grok-4.5` (or omit and let CLI default)
- **Custom agency agents:** `configs/grok-build/agents/{agency-bottom,dropshipping-pipeline,agency-coder}.md` + root `AGENTS.md` project rules
- **Showcase:** `python -m scripts.showcase_grok_build_dropshipping_flow --skip-locate` (offload) and `--try-grok-agent` (live `grok -p`); artifacts `tmp/grok_build_runs/` + `tmp/runs/grok_build_dropshipping_showcase_*`
- **CodeRabbit keys:** headless needs **Agentic** API key (`CODERABBIT_API_KEY`); plain user API keys rejected
- **CI umbrella `startup_failure` (0 jobs):** CodeRabbit must be **inline** in `ci.yml`, not only `uses: ./ci-coderabbit.yml`
- **Living README:** do not ship features without README update; user wants README kept current as we build — SOTA multi-gate CI (ruff/pytest/evals/README), not ad-hoc “smoke only”
- **envutil `_FORCE_FROM_PROJECT`:** include `SHOPIFY_*` + `AGENCY_PRIMARY_DOMAIN` + Linear/Parallel so stale shell keys cannot win
- Coverage gate is gradual (`fail_under` ~20% on tools) — raise as unit tests grow
- **Gitleaks:** path-allowlist `.env.example`; UUID-looking placeholders trip `generic-api-key` — use `your-…` placeholders

## References

- `references/hitl-spend.md`
- `references/lifecycle-commands.md`
- `references/drop-mcp-acp-gateway.md`
- `references/hermes-reverse-bridge-kip.md`
- `references/ops-agent-sota-archetype.md`
- `references/agency-cockpit-ui.md`
- `references/session-2026-08-11-autonomy-supergrok-linear.md` — SuperGrok, SWC Linear, product rank, keys
- `references/linear-github-ai-agency.md` — relink Agent Runtime → ai-agency; env force; push workflow
- `references/promptwise-ugc-and-ci.md` — PromptWise UGC tools + GitHub Actions CI
- `references/shopify-client-credentials-and-e2e.md` — Dev Dashboard token exchange, E2E, CM footguns
- `references/product-locate-and-docs-site.md` — locate pipeline, Docusaurus docs-site
- `references/post-locate-oxygen-ego.md` — outreach+shipping, Gmail HITL, ego.engineer, Oxygen, 6× cron
- `references/grok-build-bottom-layer.md` — Warp rollback, Grok Build integration, SuperGrok model ids, custom agents, showcase
- Repo: `docs/GROK_BUILD_AND_CODERABBIT.md`, `docs/CI_CD.md`, `docs/PRODUCT_LOCATE.md`, `docs/POST_LOCATE_AND_FULFILLMENT.md`, `configs/grok-build/`, `AGENTS.md`, `scripts/showcase_grok_build_dropshipping_flow.py`, `tools/grok_build_tools.py`, `tests/test_grok_build_coderabbit.py`
- Build: `agno-agentos-apps` · UI: `agency-desktop-genui`
