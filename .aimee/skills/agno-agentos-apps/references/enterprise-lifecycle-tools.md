# Enterprise lifecycle tools (ai-agency)

Canonical repo doc: `/root/src/repos/ai-agency/docs/ENTERPRISE_TOOLS.md`.

## Principle

| Layer | Autonomy |
|-------|----------|
| Research, scoring, briefs, drafts, Linear dual-write | L2 autonomous |
| Shopify draft, Meta/TikTok DRAFT | L2 autonomous |
| Ad LIVE | HITL spend vault only |
| Supplier payment / bulk PO | Human only (not automated) |
| `confirm_spend_approval` | Human only (agents blocked) |

## Stage → modules

| Stage | Module / entry |
|-------|----------------|
| Discovery | `tools/parallel_tools.py`, `scripts/autonomous_product_rank.py` |
| Economics | `tools/economics_tools.py` |
| Linear + Kanban | `tools/linear_tools.py` (`agency_track`, `ensure_kanban_card`) |
| Suppliers | `tools/supplier_tools.py` + Parallel |
| Logistics | `tools/logistics_tools.py` — DIM g = L×W×H/5 |
| UGC | `tools/fal_tools.py` — `argil/avatars/text-to-video` |
| Store | `tools/shopify_tools.py` — default `status=draft` |
| Meta | `tools/meta_ads_tools.py` — draft free; launch HITL |
| TikTok | `tools/tiktok_ads_tools.py` — same |
| Funding/HITL | `tools/spend_vault.py` |
| Full pipeline | `scripts/autonomous_lifecycle.py` / MCP `run_autonomous_lifecycle` |

## Env auto-load

`tools/envutil.py` loads (without overriding existing env):

- agency `.env`
- `~/.config/hermes-linear/connector.env` (LINEAR_API_KEY)
- `~/.config/parallel/api.env`
- `~/.hermes/.env`

## Creds checklist

- Parallel, Linear (often already via connector.env)
- Optional: FAL_KEY, Shopify, META_*, TIKTOK_*, SEVENTEENTRACK_TOKEN

## Smoke

```bash
cd /root/src/repos/ai-agency && source .venv/bin/activate && export PYTHONPATH=.
python -c "from tools.linear_tools import linear_status; print(linear_status())"
# expect mode=live, team SPE
```
