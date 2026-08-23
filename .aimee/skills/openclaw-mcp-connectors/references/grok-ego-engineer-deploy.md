# grok.ego.engineer connector — deploy snapshot (v0.2)

## Paths

| Item | Path |
|------|------|
| Repo | `/root/src/repos/grok-openclaw-connector/` |
| Symlink | `~/.openclaw/workspace/projects/grok-openclaw-connector` → repo |
| systemd | `grok-openclaw-connector.service` |
| Env | `/etc/grok-openclaw/connector.env` |
| Handoffs | `/var/lib/grok-openclaw/handoffs/` |
| OAuth state | `/var/lib/grok-openclaw/oauth/` |
| OpenClaw skill | `~/.openclaw/workspace/skills/grok-handoff/` |
| nginx site | `/etc/nginx/sites-enabled/grok.ego.engineer` |
| TLS bind | `187.77.130.10:443` (eth0 only) |

## Grok paste fields

```text
MCP URL:                 https://grok.ego.engineer/mcp
Authorization endpoint:  https://grok.ego.engineer/authorize
Token endpoint:          https://grok.ego.engineer/token
Client ID:               grok
Client secret:           $(grep ^CONNECTOR_TOKEN= /etc/grok-openclaw/connector.env | cut -d= -f2-)
Scope:                   mcp
```

## Commands

```bash
systemctl status grok-openclaw-connector nginx
curl -fsS https://grok.ego.engineer/health
curl -fsS https://grok.ego.engineer/.well-known/oauth-authorization-server | head
curl -fsS https://grok.ego.engineer/.well-known/oauth-protected-resource/mcp | head
cd /root/src/repos/grok-openclaw-connector && .venv/bin/pytest -q
bash scripts/smoke_test.sh   # if present
openclaw status | rg grok-handoff || true
```

## Handoff after Grok connects

User/Grok: call tool `handoff_to_openclaw` with `goal` + `messages`.  
OpenClaw session: `agent:main:grok-handoff:<handoffId>`.  
Pack on disk: `/var/lib/grok-openclaw/handoffs/<id>.md` (includes EXECUTE block).
