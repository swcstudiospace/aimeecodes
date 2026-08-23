# Sibling Grok MCP connector domain swap (Hermes primary)

## Context from production upgrade
Hermes connector promoted to the main public hostname; OpenClaw moved to its own subdomain. This is a common follow-up after handoffs when the user wants one connector as the "primary" for Grok.

## Post-swap mapping
- **Hermes (primary)**: https://grok.ego.engineer/mcp → 127.0.0.1:8788
- **OpenClaw**: https://openclaw-ego.engineer/mcp → 127.0.0.1:8787

Separate OAuth endpoints, separate CONNECTOR_TOKEN per connector (client_secret), shared Client ID "grok".

## Required synchronized changes

**Both source repos** (`grok-hermes-connector` and `grok-openclaw-connector`):
- `src/.../config.py`: public_host and public_base_url (defaults + from_env fallback)
- All docs/README/TEST_RESULTS/deploy templates/scripts: update domain strings
- Tests asserting on issuer/resource URLs

**Live /etc configuration**:
- `/etc/grok-hermes/connector.env` and `/etc/grok-openclaw/connector.env`: PUBLIC_HOST + PUBLIC_BASE_URL
- `/etc/nginx/sites-available/grok.ego.engineer`: change proxy_pass targets from 8787 to 8788 (both / and /mcp locations)
- New site: `/etc/nginx/sites-available/openclaw-ego.engineer` (HTTP redirect + ACME + commented full SSL block)
- `ln -sf ... /etc/nginx/sites-enabled/`

**Always**:
- Backup before touching live files: `cp file file.bak.$(date +%s)`
- After nginx edits: `nginx -t`
- After any Python change: `cd <repo> && ./.venv/bin/python -m pytest -q`

## Remaining manual steps (always document)
- DNS A record for the new subdomain
- `certbot --nginx -d openclaw-ego.engineer` (then enable the SSL block in the site file)
- Restart nginx if reload does not pick up bind changes
- Re-register the affected connector(s) in Grok UI (grok.com/connectors) using the new MCP URL + the matching CONNECTOR_TOKEN as client secret
- Optional: retire or redirect the old grok-hermes.ego.engineer vhost

## Verification checklist (run fresh after edits)
```bash
nginx -t
systemctl is-active grok-*-connector nginx
curl -fsS http://127.0.0.1:8788/health   # or 8787
curl -fsS http://127.0.0.1:8788/.well-known/oauth-authorization-server
# (after DNS + cert) public equivalents
```

## Observed pitfalls
- `nginx -s reload` frequently does nothing for bind/port changes — use restart + `ss -lntp | grep 443` + live curl.
- Old nginx workers continue serving the previous vhost until restart.
- The TLS cert for `grok.ego.engineer` "belongs" to whichever connector is now primary.
- Grok UI registrations are not auto-updated; handoffs will hit the wrong backend until refreshed.
- Always keep the two connectors' docs and code in lockstep when swapping roles.

See the main SKILL.md "Implementation checklist" and "nginx (critical...)" sections for the general pattern this swap follows.
