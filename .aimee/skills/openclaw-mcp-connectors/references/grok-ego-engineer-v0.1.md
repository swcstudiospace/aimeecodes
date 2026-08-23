# Superseded by v0.2

This file was the v0.1 bearer-only deploy snapshot (2026-07-28).

**Use instead:**

- `references/grok-ego-engineer-deploy.md` — current repo path (`/root/src/repos/…`), OAuth paste fields, verify commands
- `references/oauth-pkce-checklist.md` — OAuth/PKCE implementation notes

## Still-valid v0.1 lessons

- Gateway loopback only; never public `:18789`
- Separate `CONNECTOR_TOKEN` from gateway token
- Create `options-ssl-nginx.conf` + `ssl-dhparams.pem` if certbot omitted them
- Public eth0:443 (nginx) coexists with Tailscale Serve on ts.net IP
- radicle stays `default_server` on `:80`
- OpenClaw MCP registry: prefer `Bearer ${CONNECTOR_TOKEN}` + gateway `EnvironmentFile` drop-in

## Obsolete v0.1 assumptions

- Bearer-only middleware is enough for Grok — **false**; Grok needs OAuth endpoints
- Project only under `~/.openclaw/workspace/projects/` — now **`/root/src/repos/grok-openclaw-connector`**
