# Drop hybrid MCP + ACP gateway

Package: `/root/src/repos/ai-agency/drop_server/`  
Service: `drop-gateway.service` → `127.0.0.1:7788`  
Public target: `https://drop.autonogrammer.ai` (DNS A → **187.77.130.10**)

## Protocols

| Surface | Path / command |
|---------|----------------|
| MCP Streamable HTTP | `POST /mcp` (Python `mcp` FastMCP) |
| ACP HTTP bridge | `POST /acp/v1/session`, `…/prompt`, `GET …/session/{id}` |
| ACP stdio | `cd ai-agency && PYTHONPATH=. .venv/bin/python -m drop_server.acp_agent` |
| Health / card | `GET /health`, `GET /` |
| CoT×GoT HTTP | `POST /acp/v1/reason` |

Auth: Bearer `DROP_MCP_TOKEN` (or `X-Drop-Token`). Localhost exempt when `DROP_MCP_ALLOW_LOCALHOST=1`.

## Hermes

```bash
printf 'n\nY\n' | hermes mcp add drop --url 'http://127.0.0.1:7788/mcp' --connect-timeout 60
hermes config set mcp_servers.drop.timeout 3600
hermes mcp test drop
# NEW Hermes session required for mcp_drop_* injection
```

Tool highlights: `reason_cot_got`, `linear_*`, `agency_run_lifecycle`, `agency_product_rank`, `spend_request_approval`, `attach_funding_source`, `drop_health`, `drop_roster`.

Also keep `ai-agency` → `:7777/mcp` for AgentOS built-ins (`run_agent` / `run_team` / `run_workflow`). Catalog **Linear** MCP may be enabled separately; Drop embeds Linear so one URL is enough for external clients.

## CoT × GoT

Implementation: `drop_server/reasoning/cot_got.py`

- **cot**: linear root → decompose → research → economics → risk → decision  
- **got**: parallel branches (demand, margin, supply, creative, growth, compliance) → merge  
- **hybrid** (default): both  
- Auto-trigger heuristics: launch/spend/budget, product/niche/rank, roas/cpa/margin, compliance, lifecycle, multi-clause goals  
- Auto attaches on ACP session create and selected MCP tools; trivial chats skip  

## FastMCP mount pitfalls (verified)

```python
mcp = FastMCP("drop-autonogrammer")
mcp_app = mcp.streamable_http_app()  # route is already /mcp

@asynccontextmanager
async def lifespan(app):
    async with mcp.session_manager.run():  # REQUIRED
        yield

Starlette(routes=[..., Mount("/", app=mcp_app)], lifespan=lifespan)
```

Without `session_manager.run()` → `RuntimeError: Task group is not initialized`.  
Mounting at `/mcp` again → clients hit `/mcp/mcp`.

## DNS / TLS

1. A record `drop.autonogrammer.ai` → this VPS `187.77.130.10`  
2. nginx site: `drop_server/deploy/nginx-drop.autonogrammer.ai.conf` (HTTP ACME first)  
3. `certbot certonly --webroot -w /var/www/letsencrypt -d drop.autonogrammer.ai`  
4. Enable TLS server block; `nginx -t && systemctl reload nginx`  

Until DNS exists, operate on loopback only.

## Ops

```bash
systemctl restart drop-gateway
journalctl -u drop-gateway -n 50 --no-pager
curl -s http://127.0.0.1:7788/health
```
