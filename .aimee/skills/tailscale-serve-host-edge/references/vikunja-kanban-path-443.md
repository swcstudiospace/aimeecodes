# Vikunja on Tailscale :443 `/kanban` (this host)

Preferred user-facing URL (no custom port):

**https://srv1778002.hedgehog-mooneye.ts.net/kanban/**

## Architecture

```text
Tailscale Serve :443
  /        → 127.0.0.1:9119     Hermes
  /kanban  → 127.0.0.1:3457     nginx (path STRIPPED by serve)
               location / → 127.0.0.1:3456 Vikunja
               sub_filter absolute URLs → /kanban/…
```

## Commands

```bash
# nginx site: /etc/nginx/sites-available/vikunja-kanban-path
nginx -t && systemctl reload nginx

tailscale serve --bg --https=443 --set-path=/kanban http://127.0.0.1:3457

# compose PUBLICURL (trailing slash) then recreate
VIKUNJA_SERVICE_PUBLICURL=https://srv1778002.hedgehog-mooneye.ts.net/kanban/
cd /root/src/repos/vikunja && docker compose up -d --force-recreate
```

## sub_filter rules that work

Allow: `href="/`, `src="/`, `"/api/`, `"/assets/`, `"/images/`, `"/favicon`, `url(/`  
Deny: blanket `"/` (breaks `content="…"/>` → `…"/kanban/>`)  
Undo: `sub_filter '"/kanban/>' '"/>';`

## Why not `kanban.` hostname

- `tailscale cert kanban.srv….ts.net` → invalid domain (only machine FQDN)
- `tailscale serve --service=svc:kanban` → requires tagged node
- Second `tailscaled --hostname=kanban` → interactive auth (blocked unattended)

Dual-track ownership + connector ports: skill `hermes-linear-kanban-sync` / `references/vikunja-human-ui.md` (update that file when write gate allows — preferred access is `/kanban` on 443, not `:9456`).
