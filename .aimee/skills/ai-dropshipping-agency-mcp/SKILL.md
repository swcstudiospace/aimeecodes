---
name: ai-dropshipping-agency-mcp
description: "Use when controlling AI Dropshipping Agency via AgentOS MCP."
version: 1.0.0
metadata:
  hermes:
    tags: [agno, agentos, mcp, dropshipping, agency, parallel]
---

# AI Dropshipping Agency — Hermes ↔ AgentOS MCP

Control plane for the Agno **AgentOS** at `/root/src/repos/ai-agency` from **Hermes** (top agent).

## Architecture

```
Hermes (this agent)
  └─ MCP client: mcp_servers.ai-agency
       url: http://127.0.0.1:7777/mcp
       timeout: 3600s
         │
         ▼
AgentOS (python -m app.main)  :7777
  ├─ REST: /agents /teams /workflows /health
  └─ MCP:  /mcp
       ├─ built-ins: get_agentos_config, run_agent, run_team, run_workflow,
       │             continue_run, cancel_run, get_sessions, get_session_runs
       └─ custom:    agency_health, agency_roster, run_product_rank,
                     list_product_rank_reports, read_product_rank_report
```

Hermes tool names are prefixed: `mcp_ai_agency_<tool>` (hyphens → underscores).

## Prerequisites (every session)

1. **AgentOS running** on port 7777 with MCP enabled:
   ```bash
   cd /root/src/repos/ai-agency
   source .venv/bin/activate
   export PYTHONPATH=/root/src/repos/ai-agency
   python -m app.main
   # health: curl -s http://127.0.0.1:7777/health
   # mcp:    curl -s http://127.0.0.1:7777/info | jq .mcp
   ```
2. **Hermes MCP entry** (already configured if setup ran):
   ```bash
   hermes mcp list | grep ai-agency
   hermes mcp test ai-agency
   ```
3. **New Hermes session** after MCP add/change (tools discovered at startup).
4. Env: `PARALLEL_API_KEY` (and xAI via Hermes OAuth or `XAI_API_KEY`) in agency `.env`.

## First calls (always)

1. `mcp_ai_agency_agency_health` — runtime OK?
2. `mcp_ai_agency_get_agentos_config` **or** `mcp_ai_agency_agency_roster` — IDs for run_* tools
3. Then route work (below)

## Autonomous flows

### A) Product find + rank (Parallel ultra) — preferred for research

```
mcp_ai_agency_run_product_rank(
  niche="…",
  processor="ultra",   # lite|base|core|pro|ultra
  skip_team=false,
  default_cpa=18,
  timeout_s=3600
)
```

Then:

```
mcp_ai_agency_read_product_rank_report(name="latest", format="summary")
# optional deep dive:
mcp_ai_agency_read_product_rank_report(name="latest", format="md")
```

Reports land in `/root/src/repos/ai-agency/tmp/runs/product_rank_*.{json,md}`.

### B) Research team only (Grok + Parallel tools inside Agno)

```
mcp_ai_agency_run_team(
  team_id="research-team",
  message="Score dropshipping opportunities in <niche>. Use Parallel. Return top 5 GO/TEST/NO-GO with CM."
)
```

### C) Full product lifecycle workflow

```
mcp_ai_agency_run_workflow(
  workflow_id="full-product-lifecycle",
  message="Opportunity: <product summary from rank report>. Run research→supply→creative→store→launch→retention as drafts (L2 autonomy)."
)
```

Other workflows:

| workflow_id | When |
|-------------|------|
| `marketing-launch` | Creatives + compliance gate + paid launch |
| `supplier-onboarding` | Vet suppliers / inventory policy |
| `post-purchase-ops` | Fulfillment exceptions + CX |
| `weekly-performance-review` | Growth + retention + supply → director |

### D) Single specialist agent

```
mcp_ai_agency_run_agent(agent_id="product-scout", message="…")
mcp_ai_agency_run_agent(agent_id="hermes-ops", message="Coordinate …")
```

Stable agent ids (also in `agency_roster`):  
`hermes-ops`, `product-scout`, `supplier-sourcer`, `pricing-strategist`, `brand-strategist`, `creative-director`, `listing-specialist`, `seo-content`, `store-builder`, `compliance-officer`, `growth-media-buyer`, `influencer-manager`, `email-crm`, `customer-success`, `fulfillment-ops`, `inventory-planner`, `analyst`, `finance-controller`.

Team ids: `agency-director-team`, `research-team`, `supply-chain-team`, `creative-team`, `store-ops-team`, `growth-team`, `retention-team`.

### E) Session continuity / HITL

- Pass `session_id` from a prior run’s structuredContent to continue a conversation.
- If status is **PAUSED**, resolve requirements and call `mcp_ai_agency_continue_run`.
- `mcp_ai_agency_get_sessions` / `get_session_runs` for history.
- `mcp_ai_agency_cancel_run` to stop a long run.

## Autonomy & guardrails (do not skip)

- Default **L2**: drafts + human approve for spend, publish, supplier PO, public claims.
- Never scale ads without Finance/ROAS gates (`roas-guardrails` skill inside agency).
- Compliance: no medical cures / income guarantees — route copy through `compliance-officer` or Store Ops team.
- Ultra research is slow/expensive — use `processor=pro` for smoke tests; `ultra` for real ranking.
- Hermes MCP **timeout=3600** for `ai-agency` (required for ultra + long workflows).

## Smooth operator checklist

When user says “run the agency” / “find products” / “launch lifecycle”:

1. Health → roster/config  
2. If product discovery: `run_product_rank` (ultra) → read report → summarize top GO/TEST  
3. If they want end-to-end: `run_workflow` `full-product-lifecycle` with the winning candidate brief  
4. Dual-write notable decisions via Linear tools if available; else note in reply  
5. Save paths to JSON/MD reports in the user-facing summary  

## Re-setup (if MCP missing)

```bash
# Terminal A — AgentOS
cd /root/src/repos/ai-agency && source .venv/bin/activate
export PYTHONPATH=/root/src/repos/ai-agency
python -m app.main

# Terminal B — Hermes MCP
printf 'n\nY\n' | hermes mcp add ai-agency --url 'http://127.0.0.1:7777/mcp' --connect-timeout 90
hermes config set mcp_servers.ai-agency.timeout 3600
hermes config set mcp_servers.ai-agency.connect_timeout 90
hermes mcp test ai-agency
# start a NEW Hermes session so tools inject
```

## Pitfalls

- **Tools missing in chat**: MCP is discovered at Hermes process start — restart TUI/gateway after add.
- **AgentOS down**: MCP tools fail; start `python -m app.main` first.
- **DNS rebinding 400**: Host must be localhost/127.0.0.1 (see `AGENCY_MCP_ALLOWED_HOSTS`).
- **Empty authorize/JWT**: local MCP is open on loopback by design; do not expose :7777 publicly without auth.
- **run_product_rank vs run_workflow**: rank pipeline is custom (Parallel ultra + scoring); lifecycle is multi-team Agno workflow — use both in sequence for best results.

## Enterprise lifecycle (SOTA tools)

- `agency_integrations_status` — Linear/Shopify/Meta/TikTok/Fal/spend readiness
- `run_autonomous_lifecycle(niche, processor=ultra)` — full pipeline minus payments
- `attach_agency_funding_source` / `request_ad_spend_approval` — HITL spend
- Human confirms with `confirm_spend_approval` (not via agents) then launch ads
- Docs: `/root/src/repos/ai-agency/docs/ENTERPRISE_TOOLS.md`

## Drop universal MCP+ACP (`drop.autonogrammer.ai`)

Gateway package: `/root/src/repos/ai-agency/drop_server/`

- Local MCP: `http://127.0.0.1:7788/mcp` (Hermes server name: **drop** → tools `mcp_drop_*`)
- ACP HTTP: `http://127.0.0.1:7788/acp/v1`
- ACP stdio: `python -m drop_server.acp_agent`
- CoT×GoT: `reason_cot_got` + auto-trigger on lifecycle/spend/product goals
- Linear embedded: `linear_status`, `linear_create_issue`, …
- Also: Hermes catalog Linear MCP may already be enabled separately
- systemd: `drop-gateway.service`
- Docs: `drop_server/README.md`
- Public DNS: point `drop.autonogrammer.ai` A → this VPS `187.77.130.10`, then certbot + enable TLS nginx block

## Hermes reverse bridge + KIP

- Service: `hermes-bridge` → `http://127.0.0.1:7790/mcp` (Hermes MCP name `hermes-bridge`, 19 tools)
- All 18 Agno agents get toolbelt `hermes_bridge` automatically
- Shared memory: KIP Cognitive Nexus (`kip_remember`/`kip_recall`) + ICP capsules via `kip_export_icp`
- Docs: `docs/HERMES_AGNO_BRIDGE.md`
