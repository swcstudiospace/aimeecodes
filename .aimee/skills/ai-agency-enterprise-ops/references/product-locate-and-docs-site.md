# Product locate + Docusaurus docs site

Session learnings (discover → locate → docs).

## Product locate (after rank)

**Why:** Rank answers *what to sell*; locate answers *where to buy* (suppliers, MOQ, landed cost). Fully automatable as research; samples/POs stay HITL.

### Commands

```bash
cd /root/src/repos/ai-agency && source .venv/bin/activate
export PYTHONPATH=. PYTHONUNBUFFERED=1

# Top GO/TEST from latest product_rank_*.json
python -m scripts.autonomous_product_locate --top 3 --processor pro

# Single product
python -m scripts.autonomous_product_locate \
  --product "Fold-Flat Adjustable Aluminum Laptop Stand" --processor core

# Rank then locate
python -m scripts.autonomous_product_locate \
  --rank-first --niche "desk mobility" --processor pro
```

### Implementation map

| Piece | Path |
|-------|------|
| Script | `scripts/autonomous_product_locate.py` |
| Tools | `locate_suppliers_for_product`, `locate_product_sources_batch` in `tools/supplier_tools.py` |
| Score | existing `score_supplier` / `compare_suppliers` |
| Workflow | `workflows/product_discovery_locate.py` → **Product Discovery & Locate** |
| Registry | `app/main.py` — first workflow; description says **11 workflows** |
| Docs | `docs/PRODUCT_LOCATE.md` + docs-site `guides/product-locate` |

### Behavior

1. Parallel Search: Alibaba / wholesale / CJ / Zendrop-style queries  
2. Parallel Task: structured supplier shortlist (unit, ship, MOQ, lead, red flags)  
3. Score 0–100 + logistics profile  
4. Linear `[Locate] …` + GitHub `ai-agency` link  
5. Artifacts: `tmp/runs/product_locate_*.{json,md}`  

**HITL:** research only — no sample payment / bulk PO without human.

### Example (Fold-Flat Laptop Stand)

- Top: Shenzhen Yocaxn (Alibaba) unit ~$2.20 + ship ~$8.50 → landed **~$10.70**  
- Also: ChengRong, Doba, CABLETIME, Movron  
- Linear: **SWC-78**  
- Artifact: `tmp/runs/product_locate_20260811T202235Z.*`

### Operating loop

```text
cron/Hermes → product_rank → product_locate
  → human picks supplier + approves sample
  → QA → PromptWise/Fal → Shopify draft → ads HITL
```

Bi-daily discovery cron currently ranks only; optionally chain locate after rank.

---

## Docusaurus docs site

| Item | Value |
|------|--------|
| Path | `docs-site/` |
| Theme | Dark violet/cyan; Syne + Instrument Sans (`src/css/custom.css`) |
| Dev | `npm start` → **:3400** |
| Build | `npm run build` → `docs-site/build/` |
| baseUrl | `/ai-agency/` (GH Pages ready) |
| CI | `.github/workflows/ci-docs.yml` called from umbrella `ci.yml` |

### Version note

Docusaurus **3.7** + Node 24 failed webpack ProgressPlugin schema validation. **3.10.x** builds cleanly — keep core/preset aligned.

### Content map (high signal)

- getting-started: intro, quickstart, **shopify-setup**
- guides: **product-discovery**, **product-locate**, lifecycle, PromptWise, HITL, E2E
- architecture: overview, control planes, agents/teams/workflows, bridge, KIP
- operations: autonomy/keys, CI, Linear-GitHub, cockpit

Monorepo `README.md` points at docs-site + locate commands.

---

## Shopify store naming (this agency)

- Display: **AI Dropshipping Agency** (`SHOPIFY_SHOP_DISPLAY_NAME`)
- Subdomain best-effort: **`aidropshipping`** (`SHOPIFY_SHOP_NAME`)
- Client credentials present; live Admin still needs **app installed** on that shop (`app_not_installed` until user installs)
- Quote env values with spaces or `source .env` breaks (`Dropshipping: command not found`)
