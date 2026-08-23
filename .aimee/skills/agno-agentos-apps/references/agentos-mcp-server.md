# AgentOS as MCP server (Hermes control plane)

## Enable

```python
from agno.os.config import MCPServerConfig
AgentOS(
  ...,
  mcp_server=MCPServerConfig(
    enable_builtin_tools=True,
    tools=custom_tools,           # optional @tool callables
    allowed_hosts=["localhost", "127.0.0.1"],
    result_mode="trimmed",      # or "full"
  ),
)
```

Deps: `pip install "agno[mcp]"` → `fastmcp`. Without it, get_app/serve fails when MCP enabled.

## Endpoint

- URL: `http://127.0.0.1:7777/mcp`
- Transport: Streamable HTTP (initialize returns `text/event-stream` + `mcp-session-id`)
- Discover: `GET /info` → `{ mcp: { enabled: true, path: "/mcp" } }`

## Built-in tools (8)

`get_agentos_config`, `run_agent`, `run_team`, `run_workflow`, `continue_run`, `cancel_run`, `get_sessions`, `get_session_runs`

Run tools take `agent_id`/`team_id`/`workflow_id` + `message` (+ optional `session_id`). IDs are slugified names (e.g. `product-scout`, `research-team`, `full-product-lifecycle`).

## Hermes register (local open MCP)

```bash
printf 'n\nY\n' | hermes mcp add ai-agency --url 'http://127.0.0.1:7777/mcp' --connect-timeout 90
hermes config set mcp_servers.ai-agency.timeout 3600
hermes config set mcp_servers.ai-agency.connect_timeout 90
hermes mcp test ai-agency
# NEW Hermes session required for tool injection
```

Hermes names: `mcp_ai_agency_<tool>` (hyphens → underscores).

## Custom tools pattern

Agency example (`tools/mcp_custom.py`): `agency_health`, `agency_roster`, `run_product_rank`, `list_product_rank_reports`, `read_product_rank_report`.

Long jobs (Parallel ultra): set Hermes MCP `timeout: 3600`.

## Security

- Loopback open MCP is fine for same-host Hermes; do not expose :7777 publicly without JWT/`authorization=True` + authorize gate.
- `allowed_hosts` mitigates DNS rebinding when set.

## Operate the live agency

Skill: **ai-dropshipping-agency-mcp**
