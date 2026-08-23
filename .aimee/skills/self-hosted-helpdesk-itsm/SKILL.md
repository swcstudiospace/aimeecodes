---
name: self-hosted-helpdesk-itsm
description: "Use when installing self-hosted helpdesk/ITSM with Hermes."
version: 1.0.0
author: Hermes Agent
license: MIT
metadata:
  hermes:
    tags: [itsm, helpdesk, uvdesk, glpi, self-hosted, docker, hermes, support]
    related_skills: [hermes-linear-kanban-sync, tailscale-serve-host-edge, hermes-agent]
    created_by: agent
---

# Self-hosted helpdesk / ITSM + Hermes

## When to use

- Install or operate **UVdesk**, GLPI, FreeScout, Snipe-IT, iTop, Zammad, etc.
- Wire Hermes Agent to open/list/update support tickets via API
- Choose ITSM tools under **RAM-tight** single-host constraints
- Keep eng delivery (Linear + Hermes Kanban) separate from support tickets

**Don't use for:** eng board / sprint execution (`hermes-linear-kanban-sync`), Tailscale path plumbing alone (`tailscale-serve-host-edge`).

## Dual-track (hard rule)

| System | Owns |
|--------|------|
| Linear + Hermes eng board | Engineering execution, DoD, Autopilot |
| Vikunja (optional) | Human kanban UI only |
| **Helpdesk/ITSM** | Support/IT tickets, mailbox, assets/CMDB — **not** agent execution |

Never auto-file every eng session into UVdesk/GLPI. Classifier default: deny unless user asks for a support ticket.

## Tool shortlist (this host)

| Goal | Pick first | RAM tier |
|------|------------|----------|
| Full ITIL + assets one box | **GLPI** | Medium ~1.5–3 GiB |
| Strong CMDB | **iTop** | Medium |
| Shared inbox / light helpdesk | **FreeScout** | Light |
| Assets only | **Snipe-IT** | Light–medium |
| Modern UX helpdesk (needs headroom) | **Zammad** | Heavy (ES+PG+Redis) |
| Feature-rich Symfony helpdesk | **UVdesk** | Medium ~0.5–2 GiB |

Avoid stacking Zammad + UVdesk + full monitoring on the same 15 GiB box.

Details: `references/tool-shortlist.md`.

## UVdesk on this host (installed path)

| Item | Value |
|------|--------|
| Compose | `/root/src/repos/uvdesk/docker-compose.yml` |
| Image | `untraceablez/uvdesk:latest` (2026 community build) |
| Bind | `127.0.0.1:8082` → `:80` |
| Mem | `mem_limit: 2g` |
| Entrypoint fix | `uvdesk-entrypoint-fixed.sh` (MySQL 8–safe) |
| Secrets | `~/.config/uvdesk/uvdesk.env`, `WIZARD.txt`, `api.env` (mode 600) |
| Hermes client | `/root/src/repos/uvdesk/hermes/uvdesk_client.py` |

```bash
cd /root/src/repos/uvdesk
docker compose up -d
# Wizard: http://127.0.0.1:8082/  — DB fields from ~/.config/uvdesk/WIZARD.txt
# Admin after setup: /en/member/login
python3 hermes/uvdesk_client.py ping
```

### MySQL 8 pitfall (upstream entrypoint)

Upstream bootstrap uses invalid MySQL 8 syntax:

```sql
-- BROKEN on MySQL 8+
GRANT ALL … TO 'user'@'localhost' IDENTIFIED BY 'pass';
```

Use separate `CREATE USER … IDENTIFIED WITH mysql_native_password BY …` then `GRANT …`.
Mount fixed script as entrypoint (already in compose). After root password is set,
entrypoint must use `my.cnf` / `-p$MYSQL_ROOT_PASSWORD`, not passwordless root.
Avoid `service apache2 restart` loops — end with `exec apachectl -D FOREGROUND`.

### Official Dockerfile build pitfall

`community-skeleton` Dockerfile uses `ubuntu:latest` + `ppa:ondrej/php`. On
rolling Ubuntu codenames without a PPA Release file, `apt` fails (404). Prefer a
maintained prebuilt image or pin `FROM ubuntu:24.04`.

### Hermes API wiring (after wizard)

1. Admin → create agent (e.g. `hermes-bot@local`) + API token (API bundle).
2. `~/.config/uvdesk/api.env`:

```bash
UVDESK_BASE_URL=http://127.0.0.1:8082
UVDESK_API_EMAIL=...
UVDESK_API_TOKEN=...
```

3. Probe endpoints — paths vary by bundle (`/api/tickets.json`, `/en/api/…`,
   `/api/v1/ticket`). Client tries several.
4. Optional skill later wrapping the client. Do not Autopilot-spam tickets.

API: https://github.com/uvdesk/api-bundle/wiki/Ticket-Related-APIs  
Local: `/root/src/repos/uvdesk/hermes/README.md`.

## Edge exposure

Localhost first. Public/tailnet later via `tailscale-serve-host-edge`. Do not
steal Hermes TS `:443` `/`.

## Pitfalls

1. Treating helpdesk as second eng Kanban
2. Building official UVdesk Dockerfile on unsupported Ubuntu codename
3. MySQL 8 `GRANT … IDENTIFIED BY`
4. Passwordless root after first `ALTER USER` — entrypoint must use password/my.cnf
5. Stacking heavy ITSM + k3s + Hermes without mem limits
6. Putting API tokens in compose git tree

## Verification

- [ ] `docker ps` shows `uvdesk` healthy; curl installer/login 200
- [ ] DB user can `SELECT 1` inside container
- [ ] Wizard completed; admin login works
- [ ] `uvdesk_client.py ping` OK; list/create after token
- [ ] Secrets mode 600; not in git

## Support files

- `references/tool-shortlist.md` — comparison matrix
- `references/uvdesk-host-install.md` — this host paths + MySQL 8 fix

## Related

- `hermes-linear-kanban-sync` — eng dual-track; do not merge with ITSM
- `tailscale-serve-host-edge` — private HTTPS paths
