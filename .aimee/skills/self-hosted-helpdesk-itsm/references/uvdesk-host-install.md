# UVdesk install on this VPS

## Layout

```text
/root/src/repos/uvdesk/
  docker-compose.yml
  uvdesk-entrypoint-fixed.sh   # MySQL 8–safe; apache FOREGROUND
  community-skeleton/          # upstream clone (build failed — image used instead)
  hermes/
    uvdesk_client.py
    config.example.yaml
    README.md
~/.config/uvdesk/
  uvdesk.env      # MYSQL_*  mode 600
  WIZARD.txt      # installer DB fields
  api.env         # UVDESK_BASE_URL + email + token (fill after wizard)
```

## Image choice

| Attempt | Result |
|---------|--------|
| Build `community-skeleton` Dockerfile | Fail: `ubuntu:latest` + ondrej PHP PPA missing Release for rolling codename |
| `webkul/uvdesk:latest` | Works but stale (2023) |
| **`untraceablez/uvdesk:latest`** | Preferred — 2026 build, smaller, same all-in-one Apache+MySQL shape |

## Compose essentials

- Port: `127.0.0.1:8082:80`
- Volumes: `uvdesk_app`, `uvdesk_db`
- `mem_limit: 2g`
- Entrypoint override mounts fixed script

```bash
cd /root/src/repos/uvdesk
docker compose up -d
docker compose ps   # healthy
curl -fsS -o /dev/null -w '%{http_code}\n' http://127.0.0.1:8082/
```

## Wizard DB fields

From inside the all-in-one container:

- Host: `127.0.0.1`
- Port: `3306`
- Database / user / password: `WIZARD.txt`

After wizard: admin `/en/member/login`, portal `/en/`.

## MySQL 8 bootstrap (fixed entrypoint)

Do **not** use:

```sql
GRANT ALL ON db.* TO 'u'@'localhost' IDENTIFIED BY 'p';
```

Use:

```sql
CREATE USER IF NOT EXISTS 'u'@'localhost' IDENTIFIED WITH mysql_native_password BY 'p';
GRANT ALL ON db.* TO 'u'@'localhost';
```

After root has a password, never assume passwordless `mysql -uroot`. Write `/etc/mysql/my.cnf` client section.

Keep container alive with `exec apachectl -D FOREGROUND` — avoid restart loops of `service apache2 restart` + bash exit.

## Hermes client

```bash
# after API token exists
python3 /root/src/repos/uvdesk/hermes/uvdesk_client.py ping
python3 /root/src/repos/uvdesk/hermes/uvdesk_client.py list-tickets
```

Endpoint paths differ by API bundle version — client tries several.

## Dual-track reminder

UVdesk ≠ eng board. Linear + Hermes Kanban remain execution truth.
