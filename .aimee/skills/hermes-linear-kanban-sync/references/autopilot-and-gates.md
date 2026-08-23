# Autopilot + DoD gates (session knowledge)

## Packages

| Package | Path |
|---------|------|
| Connector | `/root/src/repos/hermes-linear-connector` |
| Eng-board | `/root/src/repos/hermes-engineering-board` |

## Autopilot stages

```text
classify | create_card | ensure_issue | ensure_issue_failed
score | specify | specify_skipped | decompose | decompose_skipped
dispatch_requested | completed | skipped | partial
```

Pipeline entry: `hermes_eng_board.pipeline.run_session_pipeline(work_unit, deps, cfg)`.

### Scoring (deterministic)

Complex if any of:

- `tool_calls >= complex_min_tool_calls` (default 8)
- `user_turns >= complex_min_user_turns` (default 6)
- keywords: refactor, migrate, multi, epic, architecture, overhaul, rewrite
- eng tools combo including terminal + patch/write/etc.

Else simple → specify only; complex → specify + decompose when `decompose_complex: true`.

### Partial success

If `ensure_issue` raises: stage `ensure_issue_failed`, still run specify. Local board progress beats waiting on Linear.

### Specify skip conditions

Skip specify when task status ≠ `triage` OR body already contains `## Spec` / `## Specification`.

### Decompose skip

Skip when `has_children(task_id)` via RO `task_links` on board DB.

### show --json unwrap

```python
raw = json.loads(stdout)
task = raw["task"] if isinstance(raw.get("task"), dict) else raw
```

## DoD gates

### Eng-board pure rules (`gates.can_complete`)

- implementer: `tests_pass` + `summary`
- reviewer: `review_pass` + `summary`
- orchestrator: `summary` only
- default role if missing: implementer

### Plugin

`pre_tool_call` → `maybe_block_complete` for tools matching kanban complete.

### Connector

`sync.require_dod_for_done: true` (default):

- `apply_kanban_status(..., hermes_status="done")` requires `dod_pass` in link metadata or arg
- else return False + `[hermes-sync]` comment on Linear issue
- `mark_dod_pass` / `POST /v1/mark-dod` sets metadata; `mark_done: true` projects Done

### Poller

When event status is done, forward `tests_pass`+`summary` from payload as `dod_pass=True` when both present.

## Config knobs

```yaml
# ~/.config/hermes-eng-board/config.yaml
autopilot:
  enabled: true
  gates_enabled: true
  decompose_complex: true
  specify_simple: true
  complex_min_tool_calls: 8
  complex_min_user_turns: 6

# ~/.config/hermes-linear/config.yaml
sync:
  require_dod_for_done: true
```

## Manual / cron

```bash
python scripts/run_autopilot_once.py --session-id <id>
python scripts/run_autopilot_once.py --json-unit /path/unit.json
python scripts/run_autopilot_once.py --sweep   # specify stale autopilot triage
```

## SPE workspace (this install)

- Linear team **SPE** (Spectrum Web Co)
- Hermes board **eng**, project **engineering**
- Discovery snapshot: `~/.config/hermes-linear/linear-discovery.json` (no secrets)

## Test counts (as of 2026-07-29 session)

- eng-board offline+hermes: ~55
- connector offline: ~49 (+ live markers)

## Backlog after gates + Command Center

Standup cron (reuse `build_default_snapshot`) → workstream lanes → Linear webhook + systemd polish.

Command Center shipped: see `references/command-center.md`.
