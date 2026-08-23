# Operating /root/src/repos/ai-agency from Hermes

Canonical app for this VPS. Prefer Hermes skill **`ai-dropshipping-agency-mcp`** once adopted (`hermes curator adopt ai-dropshipping-agency-mcp`). Until then, use this note + MCP tools + `agno-agentos-apps` refs.

## Start

```bash
cd /root/src/repos/ai-agency && ./scripts/start_agentos.sh
# MCP: http://127.0.0.1:7777/mcp
```

Hermes config (typical):

```yaml
mcp_servers:
  ai-agency:
    url: http://127.0.0.1:7777/mcp
    enabled: true
    timeout: 3600
    connect_timeout: 90
```

Add: `printf 'n\nY\n' | hermes mcp add ai-agency --url 'http://127.0.0.1:7777/mcp'` then **new session**.

## Tool prefix

`mcp_ai_agency_*` — health, roster, integrations_status, run_product_rank, run_autonomous_lifecycle, spend HITL helpers, run_agent/team/workflow, …

## Default paths

### Research only

1. `agency_health` / `agency_integrations_status`
2. `run_product_rank(niche, processor=ultra, timeout_s=3600)`
3. `read_product_rank_report(latest, summary)`

Reports: `tmp/runs/product_rank_*.{json,md}`

### Full lifecycle (everything except paying) — preferred for “run the agency”

1. `agency_integrations_status` (Linear should be live)
2. `run_autonomous_lifecycle(niche, processor=ultra, top=3)`
3. Summarize SPE issues + drafts + path to `tmp/runs/lifecycle_*_HITL_CODES.json`
4. Human confirms spend → launch ads (see `hitl-spend-and-ads.md`)

CLI equivalent:

```bash
PYTHONPATH=. python -m scripts.autonomous_lifecycle --niche "…" --processor ultra --top 3
```

### Agno multi-team workflow (optional after rank)

`run_workflow(full-product-lifecycle, message=winner brief)`

## Stable ids (slug)

Agents: hermes-ops, product-scout, supplier-sourcer, pricing-strategist, brand-strategist, creative-director, listing-specialist, seo-content, store-builder, compliance-officer, growth-media-buyer, influencer-manager, email-crm, customer-success, fulfillment-ops, inventory-planner, analyst, finance-controller  

Teams: agency-director-team, research-team, supply-chain-team, creative-team, store-ops-team, growth-team, retention-team  

Workflows: full-product-lifecycle, marketing-launch, supplier-onboarding, post-purchase-ops, weekly-performance-review  

## L2 + HITL

- Drafts OK; human approve spend/publish/PO/claims
- Agents **must not** call `confirm_spend_approval` — surface codes to operator
- Linear dual-write: SPE + `hermes kanban --board eng create` (board flag global)
- See `enterprise-lifecycle-tools.md`, `hitl-spend-and-ads.md`
