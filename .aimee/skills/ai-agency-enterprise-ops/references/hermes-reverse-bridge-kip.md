# Hermes reverse bridge + KIP shared memory

## Topology

```text
Hermes Agent (top)
  native tools: browser, computer_use, skills, memory, terminal
  MCP clients → ai-agency :7777 | drop :7788 | hermes-bridge :7790 | linear

Agno agents (18) ──HTTP MCP──► hermes-bridge :7790
  Playwright browser
  ~/.hermes/skills (+ /root/agent-skills, repo skills)
  ~/.hermes/memories MEMORY.md
  kip_memory Cognitive Nexus → capsules + ICP receipts
  computer_use job files → Hermes polls/completes
```

**Hermes Ops** = Agno director agent. **Hermes Agent** = this orchestrator process. Do not conflate.

## Ports / systemd

| Service | Port | Unit |
|---------|------|------|
| AgentOS | 7777 | `python -m app.main` / start_agentos.sh |
| Drop MCP+ACP | 7788 | `drop-gateway` |
| Reverse bridge | 7790 | `hermes-bridge` |

```bash
systemctl status hermes-bridge
curl -s http://127.0.0.1:7790/health
# skills_indexed should be ~100+
```

Env: `HERMES_BRIDGE_URL=http://127.0.0.1:7790/mcp`, `HERMES_BRIDGE_ALLOW_LOCALHOST=1`, optional `HERMES_BRIDGE_TOKEN`.

## Agno wiring

- Toolbelt key: `hermes_bridge` in `tools/toolbelts.py` → `tools/hermes_bridge_tools.py`
- Factory always appends belt unless `AGENCY_DISABLE_HERMES_BRIDGE=1`
- Profiles include `"hermes_bridge"` on all 18 agents
- Client uses Streamable HTTP: initialize → `mcp-session-id` → `tools/call`
- Drop also proxies: `bridge_browser_navigate`, `bridge_skill_search`, `bridge_kip_*`

## Browser

- Implementation: Playwright Chromium headless on the bridge process
- Install once: `/root/src/repos/ai-agency/.venv/bin/python -m playwright install chromium`
- **Must** run sync Playwright in a worker thread when invoked from FastMCP async handlers
- Good smoke: navigate `https://example.com` → title `Example Domain`

## Skills self-improve

- List/search/read from Hermes skills roots
- `hermes_skill_propose` writes `skills/_proposals/prop_*.md` + Linear dual-write
- Does **not** auto-merge into production skills — curator/Hermes reviews

## Computer-use handoff

1. Agent: `hermes_computer_use_request(goal=…)`
2. Job JSON under `tmp/bridge_jobs/cu_*.json` + Linear `[Ops]` issue
3. Hermes: `hermes_computer_use_list_jobs` → run CUA → `hermes_computer_use_complete`

## KIP (ldclabs) subset

Commands via `kip_execute` / helpers:

- `DESCRIBE PRIMER` — `$self` + type counts
- `SEARCH "q"` / `FIND` with type/name filters
- `UPSERT` CONCEPT type/name + ATTRIBUTES + PROP links
- `EXPORT` → capsule JSON + ICP receipt

Natural language:

- `kip_remember(text, kind, name, link="Type/Name")`
- `kip_recall(query)`
- `kip_export_icp(label)`

Store path: `kip_memory/data/nexus.db` · capsules: `kip_memory/data/capsules/`.

### ICP

```bash
KIP_ICP_MODE=local          # default: local_icp_ready receipt (sha256)
# KIP_ICP_MODE=canister
# IC_OSS_ENDPOINT=https://gateway/upload
# KIP_ICP_CANISTER_ID=…
```

No private keys in agency code. Host already has Anda Bot at `/root/src/repos/anda-bot` and `~/.anda` for optional deeper Brain integration later.

## FastMCP hosting checklist (drop + bridge)

1. `mcp = FastMCP(...)` then `mcp_app = mcp.streamable_http_app()` (creates session manager)
2. Parent Starlette `lifespan`: `async with mcp.session_manager.run(): yield`
3. Routes: app routes first; `Mount("/", mcp_app)` last (app already has `/mcp`)
4. After code change: `systemctl restart <unit>`

## MCP client pitfall

```python
def _call(tool_name: str, **kwargs):  # NOT name=
    return session.call_tool(tool_name, kwargs)
```

## Verify (ad-hoc)

```bash
curl -s http://127.0.0.1:7790/health
PYTHONPATH=/root/src/repos/ai-agency .venv/bin/python - <<'PY'
from tools.hermes_bridge_tools import hermes_browser_navigate, kip_remember, hermes_skill_list
print(hermes_skill_list(limit=3).get('count'))
print(kip_remember(text='ping', kind='Insight', name='Ping'))
print(hermes_browser_navigate('https://example.com').get('title'))
PY
# expect: skills 100+, kip ok, title Example Domain
# 18/18 agents expose hermes_* / kip_* tools
```

Repo docs: `docs/HERMES_AGNO_BRIDGE.md`, `hermes_bridge/README.md`.
