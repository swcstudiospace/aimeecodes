---
name: hermes-linear-kanban-sync
description: "Use when syncing Linear issues with Hermes Kanban/Scrum."
version: 1.3.0
author: Hermes Agent
license: MIT
metadata:
  hermes:
    tags: [hermes, linear, kanban, scrum, sync, engineering, tdd, autopilot, dod]
    related_skills: [enterprise-engineering, hermes-agent, plan, test-driven-development, openclaw-mcp-connectors]
    created_by: agent
---

# Hermes ↔ Linear Kanban Sync

## Overview

Build and operate **dual-track** engineering: Linear for stakeholder issues/cycles, Hermes native Kanban for agent execution. Prefer a headless connector + process pack over reinventing boards or relying only on Linear MCP. Ship **Autopilot** (session → triage → specify → decompose) and **DoD gates** before UI chrome.

## When to Use

- Linear + Hermes Kanban / Scrum / sprint automation
- Session auto-file into issues + cards (Autopilot)
- Definition-of-Done gates on complete / Linear Done
- Designing two-repo connector + board process packs
- Status mapping, webhooks, pollers, ensure-issue APIs
- TDD for issue-tracker sync daemons
- Self-hosted **human** board UIs (e.g. Vikunja) that must stay metadata-linked, not become a second agent board

**Don't use for:** ad-hoc single Linear edits (use MCP), pure OpenClaw MCP ingress (`openclaw-mcp-connectors`), general Hermes CLI help alone (`hermes-agent`), worker day-to-day card etiquette alone (`enterprise-engineering`).

## Dual-track rules

| System | Owns |
|--------|------|
| Linear | Backlog, priority, cycles, human visibility |
| Hermes Kanban | `running`/`blocked` execution, decompose, workers, durable handoffs |
| Sync map SQLite | `linear_issue_id` ↔ `kanban_task_id` ↔ `session_id` + DoD metadata |
| Optional Vikunja (etc.) | Human UI only — ids in `metadata_json`, never execution truth |

- Connector daemon (API key + poller + webhooks) = control plane
- Linear MCP = interactive cockpit only
- Never invent ticket ids; never claim filed without returned identifier/url
- Do not install a second agent Kanban (Vikunja/WeKan/…) as execution board — see `references/vikunja-human-ui.md`

See `references/dual-track-architecture.md`.

## Use native Hermes (do not rebuild Kanban)

```bash
hermes kanban boards create eng --name Engineering
hermes project create Engineering --slug engineering --board eng --use
hermes kanban create "title" --triage --idempotency-key "sess:…" --json
hermes kanban specify <id> --json
hermes kanban decompose <id> --json
# DB: ~/.hermes/kanban.db | boards/<slug>/kanban.db
# Dispatcher needs gateway: kanban.dispatch_in_gateway
```

Statuses: `triage|todo|ready|running|blocked|scheduled|done|archived`.

**CLI JSON pitfall:** `show --json` often returns `{"task": {...}, "parents": [], "children": []}`. Always unwrap `result["task"]` before reading `status`/`body` or Autopilot falsely `specify_skipped`.

Auto-ingest hooks: plugin `on_session_*` / `post_tool_call` / `pre_tool_call` (CLI+gateway). Gateway-only hooks miss TUI — keep a triage **sweep** backup.

## Two-repo pattern

1. **`hermes-linear-connector`** — GraphQL, sync store, engine, poller, FastAPI ensure API, DoD projection, webhooks
2. **`hermes-engineering-board`** — bootstrap, classifier, scoring, pipeline, plugin, skill, gates, Scrum crons

Host layout (when present): `/root/src/repos/hermes-linear-connector`, `…/hermes-engineering-board`.

| Config | Path |
|--------|------|
| Connector | `~/.config/hermes-linear/config.yaml` + `connector.env` (600) |
| Eng-board | `~/.config/hermes-eng-board/config.yaml` |
| Sync DB | `~/.hermes/linear-sync.db` |
| Work units | `~/.hermes/eng-board/sessions/<id>.json` |

HTTP ensure API: `GET /v1/health`, `POST /v1/ensure-issue`, `POST /v1/ensure-task`, `POST /v1/mark-dod`, `GET /v1/link`.

**This host port map (do not guess):**

| Port | Service |
|------|---------|
| `127.0.0.1:8792` | hermes-linear-connector (`hermes-linear-api` + poller) |
| `127.0.0.1:8788` | **Grok→Hermes MCP** — never bind Linear here |
| `127.0.0.1:3456` | Vikunja (optional human UI); Tailscale serve often `:9456` |

Example YAML may still say `8788`; on this host set `server.port: 8792` and eng-board `connector.base_url: http://127.0.0.1:8792`. Systemd: `hermes-linear-api.service`, `hermes-linear-poller.service`.

Commands: `references/repos-and-bootstrap.md`. Autopilot + gates: `references/autopilot-and-gates.md`. Command Center: `references/command-center.md`. Vikunja human UI: `references/vikunja-human-ui.md`.

## Status + idempotency

Resolve Linear by **state type**; pin IDs after discovery. Prefer "In Progress" over "In Review" for `running`. No Blocked column → started + `<!-- hermes:blocked=true -->`.

Keys: `sess:<id>`, `linear:<uuid>`, marker `<!-- hermes:task_id=… -->`.
Comment stamps: `[hermes-sync]` / `[linear-sync]` — never re-mirror.

Details: `references/status-map-and-idempotency.md`.

## Classifier (balanced default)

Skip deny phrases and too-short chitchat. Create on explicit track phrases, eng tools, keywords+turns, or thresholds. Modes: aggressive|balanced|strict.

## Autopilot pipeline (eng-board)

```text
classify → create_card (sess:<session_id>) → ensure_issue
  → score simple|complex → specify → [complex] decompose → gateway dispatch
```

- Default trigger: **session finalize only** (`eager_after_tools: false`)
- Titles/bodies: deterministic (no LLM) in v1
- Ensure-issue failure: **partial** — still specify (local execution > mirror)
- Simple → specify only; complex → specify + decompose
- Idempotent re-runs via `--idempotency-key sess:…`

Details: `references/autopilot-and-gates.md`.

## Definition of Done gates

When `autopilot.gates_enabled` / `sync.require_dod_for_done`:

| Actor | Required |
|-------|----------|
| Implementer complete | `tests_pass` + non-empty `summary` |
| Reviewer complete | `review_pass` + `summary` |
| Linear Done projection | `dod_pass` (metadata or `POST /v1/mark-dod`) |

Refuse Linear Done without DoD; leave `[hermes-sync]` breadcrumb comment. Plugin `pre_tool_call` blocks `kanban_complete` when metadata fails.

```json
{"tests_pass": true, "summary": "what changed + test evidence", "role": "implementer"}
```

## Initiative order (personal AI eng unit)

1. Autopilot (throughput) → 2. DoD gates (trust) → 3. Command Center UI → 4. Standup brief → 5. Lanes (product/platform/firefight). Do not ship UI before Autopilot fills the board.

**Shipped through rank 3 on this host:** Autopilot + gates + Eng Command tab. Next: standup (reuse snapshot; do not scrape UI).

## Engineering Command Center (dashboard)

Read-only ops cockpit — keep native `/kanban` for CRUD. CC joins SPE ids, DoD chips, blocked/heartbeat ages, exception strip.

| Piece | Notes |
|-------|--------|
| Snapshot | `hermes_eng_board.command_center` — pure builder + SQLite RO (kanban + `linear-sync.db`) |
| Prefer RO SQL | Do not depend on connector HTTP for the war room (API may be down) |
| Plugin | `plugin/hermes_eng_board/dashboard/` → tab **Eng Command** `/eng-command` |
| API | `GET /api/plugins/eng-command-center/snapshot` |
| Install | `scripts/install_command_center.sh` → `~/.hermes/plugins/hermes_eng_board` |
| UI fetch | `SDK.fetchJSON` + ~8s poll (same pattern as kanban plugin) |
| SPE URL | `https://linear.app/{linear_org_url_key}/issue/{SPE-N}` (default org `swcstudio`) |
| Standup later | `build_default_snapshot()` / snapshot route only |

Details: `references/command-center.md`.

## TDD spine

Offline: `pytest -m "not live and not hermes"`.
Live tests prefix `[hermes-sync-test]` and cancel/archive in finalizers.
Vertical RED→GREEN: map → keys → store → clients → engine → poller → pipeline slices → gates → live e2e. Expect ~50 offline tests/repo when mature.

Checklist: `references/tdd-connector-spine.md`.

## Scrum compose

Linear Cycles = sprints; Hermes columns = execution board; cron for standup/planning(propose-only)/review/retro. Do not fork a separate Scrum product.

## Security

Secrets only in env files mode 600—never git, plans, skills, fixtures. Summaries outbound, not full transcripts. Webhooks on localhost + verified signatures. If a key is pasted in chat, store to env immediately and note rotation.

## Conflict rules

1. Kanban owns status while running/agent-blocked
2. Linear owns cycle + priority
3. Linear Done → complete Kanban
4. Hermes Done → Linear Done **only with** `dod_pass` when gates on
5. No hard deletes

## Common pitfalls

1. Second Kanban product instead of Hermes native (includes treating Vikunja as agent board)
2. MCP-only automation for hooks/webhooks
3. Wrong board DB path (default vs `boards/<slug>`)
4. Mapping by state name only (breaks on rename)
5. Ticket spam without classifier
6. Gateway down → ready tasks never dispatch
7. Pasting API keys into plans/chat without env storage + rotation note
8. Reading `show --json` without unwrapping `task` → false `specify_skipped`
9. Projecting Linear Done without DoD → “Done theater”
10. Relying on gateway hooks alone for TUI finalizes — add sweep
11. Replacing native Kanban with Command Center — CC is ops view only
12. Snapshot via connector HTTP only — prefer RO SQLite join
13. Standup scraping dashboard HTML — reuse snapshot builder/route
14. Binding Linear connector to `:8788` on this host — that is Grok→Hermes; use **`:8792`**
15. Unquoted multi-word values in `connector.env` (e.g. project name) break `source`/systemd EnvironmentFile
16. Vikunja healthcheck via `wget` — image is distroless-ish; use `vikunja healthcheck`
17. `docker restart` after compose env edit — stale env; `compose up -d --force-recreate`
18. Vikunja status → Linear/Hermes Done without DoD metadata

## Verification

- [ ] eng board + project; gateway up
- [ ] connector `http://127.0.0.1:8792/v1/health` + offline pytest green both repos
- [ ] ensure-issue returns SPE/SWC-like identifier + link row
- [ ] Autopilot creates `sess:` card and runs specify
- [ ] Done without DoD refused; with mark-dod/metadata succeeds
- [ ] Eng Command tab loads; snapshot shows columns + exceptions without CLI
- [ ] no API key material in repos
- [ ] if Vikunja installed: healthy on `:3456`, registration off, link script dry-run OK; still not used as agent board

## Related skill split

- **`hermes-linear-kanban-sync`** (this) — build/operate connector + Autopilot/gates
- **`enterprise-engineering`** — worker/orchestrator behavior on live cards
- **`self-hosted-helpdesk-itsm`** — UVdesk/GLPI/etc. support desk; never merge into eng board
- **`tailscale-serve-host-edge`** — private HTTPS paths for human UIs (Vikunja, helpdesk)

If both drift, update this umbrella first then align `enterprise-engineering`. User may `hermes curator adopt enterprise-engineering` if that copy is user-owned and diverges.
