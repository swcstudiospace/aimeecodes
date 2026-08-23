# Vikunja as human UI (optional third surface)

Vikunja is a **human-facing** self-hosted board. It does **not** replace Hermes Kanban execution or Linear planning.

## Ownership (this host)

| Surface | Owns |
|---------|------|
| Linear (SWC team) | Backlog, priority, cycles |
| Hermes Kanban `eng` | Agent `running`/`blocked`, Autopilot, DoD, decompose |
| hermes-linear-connector | `linear_issue_id` ↔ `kanban_task_id` ↔ `session_id` + DoD |
| Vikunja | Optional human view / personal ops only |

**Never** make Vikunja drive Hermes `done` or Linear Done without DoD gates.

## Install layout (this host)

```text
/root/src/repos/vikunja/          # docker compose + README
  docker-compose.yml              # SQLite, mem_limit 512m, 127.0.0.1:3456
  docs/LINEAR_METADATA_LINKING.md
  scripts/link_task.py            # metadata-only link helper
~/.config/vikunja/admin.env       # mode 600 login
~/.config/vikunja/config.yaml     # engineering_project_id, public_url, connector URL
```

- Image: `vikunja/vikunja` (verified v2.5.0)
- Healthcheck: `["CMD", "/app/vikunja/vikunja", "healthcheck"]` — image has **no wget/curl/shell tools**
- DB: SQLite at `./db/vikunja.db` (prefer over Postgres when host RAM is tight ~15Gi)
- Registration: create first user via CLI, then `VIKUNJA_SERVICE_ENABLEREGISTRATION=false`
- First user is enough for local admin; Pro `user set-admin` needs license (may refuse)

```bash
cd /root/src/repos/vikunja && docker compose up -d
docker exec vikunja /app/vikunja/vikunja user create -u USER -e EMAIL -p PASS
# lock registration in compose, then:
docker compose up -d --force-recreate   # restart alone does NOT apply env changes
```

## Access

- Local only bind: `127.0.0.1:3456`
- Tailnet (this host): `tailscale serve --bg --https=9456 http://127.0.0.1:3456`
  - URL: `https://srv1778002.hedgehog-mooneye.ts.net:9456/`
- Set `VIKUNJA_SERVICE_PUBLICURL` to the public URL (trailing slash) **before** recreate
- nginx stub: `/etc/nginx/sites-enabled/vikunja.ego.engineer` (ACME HTTP only until DNS+cert)

Do **not** clobber existing Tailscale serve on `:443` (Hermes) or `:8443`.

## Metadata linking

Canonical store remains connector SQLite `~/.hermes/linear-sync.db`:

```text
sync_links.metadata_json += {
  "vikunja_task_id": N,
  "vikunja_project_id": 2,
  "vikunja_url": "https://…/tasks/N"
}
```

```bash
python3 /root/src/repos/vikunja/scripts/link_task.py \
  --title "…" --kanban-task-id <id> --linear-identifier SWC-123
# --dry-run first; updates metadata only when a sync_links row exists
```

Optional later: `POST /v1/ensure-vikunja` on connector; Vikunja webhooks → metadata only (signature + localhost).

## Connector port (critical on this host)

| Port | Service |
|------|---------|
| **8788** | **Grok→Hermes MCP** — do not bind Linear here |
| **8792** | **hermes-linear-connector** ensure API (`hermes-linear-api.service`) |
| poller | `hermes-linear-poller.service` |

```bash
# config
~/.config/hermes-linear/config.yaml   # server.port: 8792
~/.config/hermes-eng-board/config.yaml # connector.base_url: http://127.0.0.1:8792

curl -fsS http://127.0.0.1:8792/v1/health
systemctl status hermes-linear-api hermes-linear-poller
```

`connector.env`: quote values with spaces (`LINEAR_PROJECT_NAME="AI Dropshipping Agency"`). Mode 600. Never `source` unquoted multi-word values.

## Pitfalls

1. Second agent Kanban product (Vikunja/WeKan/etc.) instead of Hermes native `eng`
2. Binding Linear connector to `:8788` → collides with Grok MCP; health looks “wrong service”
3. Vikunja healthcheck with `wget` → image has no wget; use `vikunja healthcheck`
4. `docker restart` after compose env edit → stale env; use `compose up -d --force-recreate`
5. Letting Vikunja status mirror Linear Done without DoD → Done theater
6. Pasting API keys into chat/logs — rotate if leaked; store only in env 600
7. Public expose without updating `VIKUNJA_SERVICE_PUBLICURL` → broken frontend API origin

## Verification

- [ ] `docker inspect` health healthy; `/api/v1/info` 200
- [ ] registration_enabled false after bootstrap
- [ ] Tailscale or local UI loads
- [ ] `curl 127.0.0.1:8792/v1/health` ok (Linear viewer)
- [ ] eng board still current: `hermes kanban boards list`
- [ ] link script dry-run + real create under Engineering project
