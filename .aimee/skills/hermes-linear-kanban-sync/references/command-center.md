# Engineering Command Center

Read-only war room for the personal AI eng unit. Complements native Kanban UI.

## Why it exists

| Surface | Job |
|---------|-----|
| `/kanban` | Card CRUD, drag-drop, comments, dispatch visibility |
| **Eng Command** | SPE ids, Linear links, DoD chips, blocked/heartbeat ages, exception strip |

Do not replace Kanban with CC.

## Snapshot contract

Built by `hermes_eng_board.command_center.build_snapshot` / `build_default_snapshot`.

Sources (SQLite RO — preferred over connector HTTP):

- Kanban: `~/.hermes/kanban/boards/eng/kanban.db` (or default `kanban.db`)
- Sync: `~/.hermes/linear-sync.db` → `sync_links` (+ `metadata_json` for `dod_pass`)

Key JSON fields:

```text
generated_at, board, cycle{active,...}, counts{}, columns{triage|todo|ready|running|blocked|done}[],
exceptions[{kind, task_id, title, age_seconds, linear_identifier}], links_missing
```

Exception kinds:

- `blocked_stale` — age ≥ `command_center.blocked_stale_seconds` (default 4h)
- `heartbeat_stale` — running with null HB or age ≥ `heartbeat_stale_seconds` (15m)
- `missing_linear` — non-done card without `linear_identifier`

## Dashboard plugin layout

```text
~/.hermes/plugins/hermes_eng_board/
  __init__.py              # autopilot + gate hooks
  dashboard/
    manifest.json          # name: eng-command-center, path: /eng-command
    plugin_api.py          # FastAPI router → /api/plugins/eng-command-center/
    dist/index.js          # IIFE, SDK.React + SDK.fetchJSON
    dist/style.css
```

Install: `hermes-engineering-board/scripts/install_command_center.sh`

Manifest tips:

- `icon`: use mapped Lucide names (`Activity` works)
- `tab.position`: `before:kanban` places next to native board
- API name in URL = manifest `name`, not parent folder name

## Frontend fetch

```javascript
const API = "/api/plugins/eng-command-center";
SDK.fetchJSON(API + "/snapshot")  // preferred — auth handled by host
// poll every command_center.poll_seconds (default 8)
```

Register: `window.__HERMES_PLUGINS__.register("eng-command-center", Page)`.

## SPE URL

```text
https://linear.app/{command_center.linear_org_url_key}/issue/{SPE-N}
```

Default org key for Spectrum Web Co: `swcstudio`. Invalid/missing identifier → null URL.

## Config knobs

```yaml
command_center:
  board: eng
  blocked_stale_seconds: 14400
  heartbeat_stale_seconds: 900
  done_recent_limit: 10
  linear_org_url_key: swcstudio
  poll_seconds: 8
  include_cycle: true   # soft-fail if no LINEAR_API_KEY / no active cycle
  sync_db_path: ~/.hermes/linear-sync.db
```

## Standup reuse (rank 4)

```python
from hermes_eng_board.command_center import build_default_snapshot
snap = build_default_snapshot()
# format exceptions + running/blocked into markdown; [SILENT] if empty
```

Never scrape dashboard HTML.

## Tests

```bash
cd /root/src/repos/hermes-engineering-board
uv run pytest tests/unit/command_center -q
uv run python -c "from hermes_eng_board.command_center import build_default_snapshot; print(build_default_snapshot()['counts'])"
```

## v1 non-goals

Mutations (promote/assign/complete), WS live tail, desktop twin, multi-board switcher beyond config.
