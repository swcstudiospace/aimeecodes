# Hermes MCP connector v0.3+ (this VPS)

Repo: `/root/src/repos/grok-hermes-connector`  
Public: `https://grok.ego.engineer/mcp` → `127.0.0.1:8788`  
Unit: `grok-hermes-connector.service`  
Env: `/etc/grok-hermes/connector.env` (never print CONNECTOR_TOKEN)  
Data: `/var/lib/grok-hermes/{handoffs,tasks,oauth,artifacts,audit}`  
Receiver skill (Hermes): `grok-handoff-hermes` (user-owned / bundled with repo —
ask operator to `hermes curator adopt grok-handoff-hermes` if curator should own it)

## Tools

| Tool | Role |
|------|------|
| `handoff_to_hermes` | Compact + EXECUTE; returns `taskId` (== handoffId) |
| `get_handoff_status` / `list_recent_handoffs` | Status + recent list |
| `list_skills` / `get_skill` / `invoke_skill` | First-class Hermes skills |
| `get_task` / `get_task_result` / `cancel_task` | Task lifecycle wrappers |
| `list_artifacts` | Per-task artifact index |

## Prompts

`brainstorm_feature`, `design_component`, `handoff_to_hermes`,
`invoke_hermes_skill`, `review_handoff`, `workflow_brainstorm_to_ship`,
`mobile_quick_skill`.

## Resources

- `skill://{name}/SKILL.md`
- `skills://catalog.json`

## Tasks extension

- Id: `io.modelcontextprotocol/tasks`
- Methods: `tasks/get`, `tasks/list`, `tasks/cancel`, `tasks/result`
- CreateTaskResult when client sends `task` on tools/call
- HandoffStore = durability SoT; TaskStore optional attribute side-index
- `TasksExtension` accepts HandoffStore **or** TaskStore + `handoff_lookup`
  (+ `ttl_sec` / `default_ttl`, `poll_interval_ms`)

## Module map (single SoT)

```text
server.py  tasks.py  skills.py  prompts.py
audit.py   rate_limit.py  telemetry.py  artifacts.py
config.py  oauth.py  store.py  hermes_backend.py  handoff_pack.py  compact.py
```

Do **not** introduce parallel stacks (`prompts_lib`, second TasksExtension API)
without deleting/merging the old one first.

## Concurrent-handoff race

Grok can fire multiple near-identical handoffs at once. Before writing:

1. `git status` + list `src/grok_hermes_connector/`
2. Import-smoke: `from grok_hermes_connector.server import build_app`
3. Align every module to what **current** `server.py` imports
4. Dual-compatible constructors > blind overwrites
5. pytest green → restart unit → `/health`

## Verify

```bash
cd /root/src/repos/grok-hermes-connector
.venv/bin/pytest -q
uv pip install -e . -q   # if entrypoint package changed
systemctl restart grok-hermes-connector
curl -sS http://127.0.0.1:8788/health
curl -sS http://127.0.0.1:8788/metrics
```

Expect: `version` 0.3.x+, `extensions["io.modelcontextprotocol/tasks"]`,
`skillsIndexed` > 0, `features.tasks/prompts/audit/rateLimit`.

## Deferred (unless asked)

Redis store, full OIDC IdP, gVisor sandbox, client `tasks/update` RPC.
