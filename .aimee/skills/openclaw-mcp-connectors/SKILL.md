---
name: openclaw-mcp-connectors
description: >-
  Build MCP HTTP ingress that hands work into local OpenClaw.
version: 0.2.0
metadata:
  hermes:
    tags: [OpenClaw, MCP, Streamable-HTTP, OAuth, nginx, systemd, handoff, security, Grok]
    related_skills: [receive-openclaw-handoff]
---

# OpenClaw MCP ingress connectors

## When to use

- Expose an MCP endpoint for an external agent/UI (**Grok requires OAuth**, not bearer-only)
- Inject compacted work into **local** OpenClaw (`openclaw agent`)
- Keep Gateway on loopback; public surface is only the thin connector
- Deploy with systemd + nginx TLS + separate connector secret
- Bundle an OpenClaw **receiver skill** with the MCP repo

## Non-goals

- Do **not** put OpenClaw Gateway WS, `/tools/invoke`, or `/v1/*` on the public internet
- Do **not** use `openclaw mcp serve` as primary ingress (needs existing channel routes)
- Do **not** reuse the Gateway auth token as the public connector secret
- Do **not** expect “Handover to OpenClaw” chat text alone to work — client must call MCP tools after OAuth

## Reference architecture

```text
[Grok / external MCP client]
        |  OAuth (PKCE) then Streamable HTTP + Bearer access_token
        v
https://<public-host>   (/authorize /token /register /.well-known/* /mcp)
        |  nginx TLS on eth0:443 only (not Tailscale IPv6 443)
        v
[connector]  127.0.0.1:<port>
        |  compact → handoff PACK (brief + EXECUTE) → disk
        |  openclaw agent --session-key agent:<id>:<prefix>:<handoffId> --message-file …
        v
[OpenClaw Gateway]  127.0.0.1:18789  + workspace skill <prefix>
```

Canonical deploy (post domain upgrade):

- Hermes (primary on main domain): `https://grok.ego.engineer/mcp` (127.0.0.1:8788)
- OpenClaw (dedicated subdomain): `https://openclaw-ego.engineer/mcp` (127.0.0.1:8787)
- Project roots: `/root/src/repos/grok-*-connector/`
- Env files: `/etc/grok-*/connector.env` (distinct CONNECTOR_TOKEN per connector = OAuth client_secret)
- Data: `/var/lib/grok-*/handoffs/` + `/var/lib/grok-*/oauth/`
- nginx: `grok.ego.engineer` vhost proxies to Hermes; separate vhost for openclaw-ego.engineer
- Receiver skills: `grok-handoff-hermes` (Hermes) and the OpenClaw equivalent

When performing domain/role swaps between the siblings, treat both repos + live /etc as a single unit. See `references/sibling-mcp-domain-swap.md`.

## Grok UI field mapping (OAuth)

| Grok field | Value |
|------------|--------|
| MCP URL | `https://<host>/mcp` |
| Authorization endpoint | `https://<host>/authorize` |
| Token endpoint | `https://<host>/token` |
| Client ID | `grok` (static convenience) or DCR-issued |
| Client secret | **`CONNECTOR_TOKEN`** — never put the token in Client ID |
| Scope | `mcp` |
| Transport | Streamable HTTP |

Discovery:

- `/.well-known/oauth-authorization-server`
- `/.well-known/oauth-protected-resource/mcp` (RFC 9728 path includes resource suffix `/mcp`)

## Implementation checklist

1. **MCP server (Python SDK `mcp` 2.x)**  
   - `uv add "mcp[cli]"` → `MCPServer` + `streamable_http_app(path="/mcp")`  
   - Bind connector `127.0.0.1` only; systemd + uvicorn  
   - Public custom routes: `/`, `/health` (list OAuth URLs on health)

2. **Auth — prefer OAuth AS when client is Grok**  
   - Implement `OAuthAuthorizationServerProvider` (simple single-tenant is fine)  
   - Wire `AuthSettings(issuer_url=…, resource_server_url=…/mcp, client_registration_options=enabled)`  
   - Pass **`auth_server_provider` only** — cannot also pass `token_verifier` (SDK raises)  
   - `load_access_token` must accept **issued tokens AND** raw `CONNECTOR_TOKEN` (legacy scripts)  
   - `authorize()` auto-approves + redirects with PKCE `code` (S256)  
   - DCR `/register`: force `client_secret = CONNECTOR_TOKEN`  
   - Static clients: `grok` / `grok-mcp` / `openclaw` with broad redirect allowlist  
   - Persist clients/tokens under `OAUTH_DIR` so restarts keep sessions  
   - Bearer-only middleware is **not enough** for Grok’s OAuth form

## Handoff pack (not bare transcript)  
   - Compact messages (window + size caps)  
   - Structured brief + **EXECUTE (mandatory)** block so OpenClaw cannot treat as FYI  
   - Include `session_key`, skill name, trust boundary footer  
   - Persist `<id>.md` + `<id>.json` + `.openclaw.log`  
   - Reject oversize (`MAX_BODY_BYTES` ~512KiB)

**High-quality / full-session mode** (see `references/high-quality-full-session-handoffs.md`):
- Connector now defaults to 120 recent messages (was 40) with much gentler truncation that preserves code blocks and message tails.
- MAX_MESSAGES raised to 300.
- Extraction surfaces more artifacts from the broader transcript.
- For best results, the Grok client should pass the entire relevant session history in `messages`. The brief will contain a "Full Recent Transcript (high fidelity)" section plus richer extracted decisions/artifacts.

4. **OpenClaw submit**  

   ```bash
   openclaw agent \
     --agent <agent_id> \
     --session-key "agent:<agent_id>:<prefix>:<handoffId>" \
     --message-file <packPath> \
     --timeout <sec> \
     --json
   ```

   - **Background** `Popen` + waiter; MCP returns `status=submitted` immediately  
   - Status tool: metadata + log tail + pid liveness

5. **Bundled receiver skill**  
   - Ship under repo `skills/<name>/` with `references/`, `templates/`, optional `scripts/verify_brief.py`  
   - Install copy to `~/.openclaw/workspace/skills/<name>/` on deploy  
   - Trust boundary, session key pattern, completion status template

6. **nginx (critical on hosts with Tailscale)**  
   - `server_name` only; never steal `default_server`  
   - **Bind TLS to eth0 IPv4 only**: `listen <PUBLIC_IP>:443 ssl http2`  
   - **Do not** `listen [::]:443` if Tailscale already owns IPv6 `:443`  
   - Proxy **`location /`** to connector (OAuth + well-known + UI), not only `/mcp`  
   - `/mcp`: HTTP/1.1, `proxy_buffering off`, long timeouts, pass `Authorization`  
   - Write **full** site file (HTTP + SSL) — do not rely on failed `reload` after append  
   - If `nginx -t` ok but workers stale: **`systemctl restart nginx`** (reload can no-op on bind errors)  
   - Ensure `options-ssl-nginx.conf` + `ssl-dhparams.pem` before start

7. **OpenClaw MCP client registry (optional)**  

   ```bash
   openclaw mcp add <name> \
     --url http://127.0.0.1:<port>/mcp \
     --transport streamable-http \
     --header 'Authorization=Bearer ${CONNECTOR_TOKEN}' \
     --no-probe
   ```

   - Gateway user unit drop-in: `EnvironmentFile=-/etc/…/connector.env`  
   - CLI: `set -a; source …/connector.env; set +a` before `mcp probe`

8. **Tests (TDD)**  
   - Unit: OAuth metadata, PKCE code→token, unauth 401, pack sections  
   - ASGI tests need **`asgi-lifespan.LifespanManager`** or StreamableHTTP task group errors  
   - Client smoke: **`httpx2.AsyncClient`** + `streamable_http_client`  
   - Live: public well-known 200, full PKCE over HTTPS, agent session appears

## Minimum tools

| Tool | Role |
|------|------|
| `handoff_to_*` | goal + messages → pack → submit |
| `get_handoff_status` | id → metadata / agent state |
| `list_recent_handoffs` | recent summaries |

## Security rules

- Separate connector secret vs gateway token; never log either  
- Transcript = untrusted (prompt injection)  
- DNS rebinding allowlist: loopback + public host + Grok origins if needed  
- systemd hardening + limited `ReadWritePaths`  
- Gateway must not listen on public NIC

## Verify before done

```bash
systemctl is-active <connector.service> nginx
curl -fsS http://127.0.0.1:<port>/health
curl -fsS https://<host>/.well-known/oauth-authorization-server
curl -fsS https://<host>/.well-known/oauth-protected-resource/mcp
# unauth /mcp → 401
# PKCE: register → authorize → token → POST /mcp initialize ≠ 401
openclaw status   # agent:<id>:<prefix>:… session
ss -lntp | rg ':443|:18789'
tailscale serve status
```

## Pitfalls

- **Grok OAuth form ≠ Client ID field for the secret** — secret goes in client_secret; ID is `grok`  
- **MCPServer forbids both `auth_server_provider` and `token_verifier`** — dual accept in `load_access_token`  
- **Protected-resource metadata path includes `/mcp` suffix**  
- **`listen [::]:443` fights Tailscale** — use eth0 IPv4 only  
- **nginx reload success ≠ new config live** if bind failed earlier; restart and re-check `ss`  
- **Old workers serving pre-OAuth vhost** look like “well-known 404” while localhost:8787 is 200  
- **StreamableHTTP ASGI tests need lifespan** (`Task group is not initialized`)  
- **mcp 2.x client = httpx2**  
- **Background agent** — never await full OpenClaw run inside the tool  
- **Pack without EXECUTE** → agent may only summarize  
- **k8s/ingress-nginx on host** can also claim ports; confirm which nginx owns eth0:443  
- **Concurrent Grok handoffs race the same connector repo** — Grok often fires 2–3 near-identical
  handoffs in parallel. Agents then invent overlapping modules (`prompts.py` vs `prompts_lib.py`,
  divergent `TasksExtension.__init__`). **Inventory imports + `git status` first**; pick one
  `server.py` wire-up as SoT; dual-compatible constructors beat blind deletes. Prefer one handoff
  per goal.

## Hermes sibling (enterprise surface v0.3+)

Hermes connector is primary on `grok.ego.engineer` (8788). Beyond handoff tools it may expose:

- Tasks extension `io.modelcontextprotocol/tasks` (`tasks/get|list|cancel|result`;
  task-augmented `tools/call` → `CreateTaskResult`)
- Skill surface: `list_skills` / `get_skill` / `invoke_skill` + `skill://{name}/SKILL.md`
  (`list_skills` → dict `{count,total,skills}`; body via `read_skill_md`)
- Prompt templates (composition = multi-message + resource embed)
- Audit JSONL, rate-limit ASGI middleware, `/metrics`, artifacts index
- Durability: HandoffStore SoT with `taskId == handoffId`; optional TaskStore side index

Live verify after code changes:

```bash
cd /root/src/repos/grok-hermes-connector && .venv/bin/pytest -q
systemctl restart grok-hermes-connector
curl -sS http://127.0.0.1:8788/health   # version, extensions, skillsIndexed
```

Details: `references/hermes-connector-v0.3.md`.

## Support files

- `references/grok-ego-engineer-deploy.md` — live paths, Grok paste fields, verify commands on this VPS  
- `references/oauth-pkce-checklist.md` — PKCE + DCR implementer notes for MCP Python SDK 2.x  
- `references/hermes-connector-v0.3.md` — Hermes enterprise Tasks/skills/prompts surface + race cleanup  
- `references/sibling-mcp-domain-swap.md` — Hermes↔OpenClaw domain/role swaps  
- `references/high-quality-full-session-handoffs.md` — large-window compaction
