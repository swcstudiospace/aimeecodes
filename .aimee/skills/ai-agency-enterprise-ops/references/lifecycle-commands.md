# Lifecycle commands cheat sheet

```bash
cd /root/src/repos/ai-agency
source .venv/bin/activate
export PYTHONPATH=.

# AgentOS + MCP
./scripts/start_agentos.sh

# Structural agent evals
python -m evals.run_agent_evals

# Product rank only (Parallel ultra)
python -m scripts.autonomous_product_rank --niche "…" --processor ultra

# Full lifecycle (Linear dual-write, drafts, HITL request)
python -m scripts.autonomous_lifecycle --niche "…" --processor ultra --top 3
# optional Fal render:
python -m scripts.autonomous_lifecycle --niche "…" --processor ultra --render-ugc

# Linear smoke
python -c "from tools.linear_tools import linear_status, agency_track; print(linear_status())"

# Kanban mirror (board flag GLOBAL)
hermes kanban --board eng create "title" --triage --idempotency-key linear:SPE-N --json
```

Reports under `tmp/runs/`. HITL codes: `lifecycle_*_HITL_CODES.json` mode 600.

Hermes MCP tools: `mcp_ai_agency_run_autonomous_lifecycle`, `run_product_rank`, `agency_integrations_status`, `request_ad_spend_approval`, `attach_agency_funding_source`.
