# Result return to Grok (v0.3.2) — 2026-07-29

## Problem

`invoke_skill` / handoff can **succeed on the VPS** while Grok still has nothing useful:

1. Hermes writes JSON under **systemd `PrivateTmp`** (`/tmp/systemd-private-…/tmp/…`).
2. Connector `get_task_result` historically returned only `{goal, status}` metadata.
3. Grok polls `get_task` → `completed` with empty payload → invents or re-fires jobs.

Real incident: task `skill-20260729T084101Z-bbd4c43b` (`parallel-stock-deep-research`) produced 10 picks in PrivateTmp; Grok never saw them until artifacts were promoted.

## Fixed shape (v0.3.2+)

| Path | Role |
|------|------|
| `/var/lib/grok-hermes/artifacts/<taskId>/result.json` | Canonical structured output |
| `…/summary.json` | tickers, pick_count, methodology preview |
| `…/RESULT.md` | Human table for mobile |
| `…/hermes-worker.log` | Worker log copy |
| `/var/lib/grok-hermes/handoffs/<taskId>.result.json` | Mirror beside meta |
| `/var/lib/grok-hermes/work/jobs/` | Durable job pid/status/log (not `/tmp`) |
| `/var/lib/grok-hermes/work/results/` | Skill mirrors |

Env (`/etc/grok-hermes/connector.env`):

```bash
HANDOFF_SHARED_DIR=/var/lib/grok-hermes/work
ARTIFACTS_DIR=/var/lib/grok-hermes/artifacts
DEDUPE_ENABLED=1
DEDUPE_WINDOW_SEC=600
```

Dispatch (`dispatch_handoff_to_hermes.sh`) exports `HANDOFF_ID` / `GROK_HANDOFF_ID` from job name `grok-hermes-<id>` so skills can write into `artifacts/$HANDOFF_ID/`.

Code:

- `results.py` — `load_result_for_task`, `promote_result_to_artifacts`, unwrap Parallel envelopes
- `server.get_task_result` — returns `result`, `summary`, `resultMarkdown`, paths
- `hermes_backend` waiter — on completed, calls promote
- Stock skill `run_research.py` — default `--out` under artifacts when `HANDOFF_ID` set; writes unwrapped `result.json`

## Grok workflow (after any long skill)

```text
1) get_task(taskId) until completed|failed
2) get_task_result(taskId)   # REQUIRED — this is how results enter Grok
3) optional list_artifacts(taskId)
4) Present from structured fields; do not invent
```

Pull-only prompt (existing job):

```text
Do NOT start a new job.
get_task + get_task_result taskId="<id>"
Present executive summary + ranked table + sources + non-advice disclaimer.
```

## Diagnose missing results

```bash
# meta
cat /var/lib/grok-hermes/handoffs/<id>.json | head
# public artifacts
ls -la /var/lib/grok-hermes/artifacts/<id>/
# trapped under PrivateTmp (legacy / still if something writes /tmp)
find /tmp/systemd-private-*-grok-hermes-connector* -name '*result*' 2>/dev/null | head
# promote manually if needed
python3 -c "from grok_hermes_connector.results import promote_result_to_artifacts; ..."
```

## Pitfalls

- **`completed` ≠ payload present** — always call `get_task_result`.
- **PrivateTmp** — anything under service `/tmp` is not host `/tmp`; prefer `/var/lib/grok-hermes/*`.
- **Early complete** — if status flips completed in seconds with no result.json, check waiter + job `.status` under `HANDOFF_SHARED_DIR/jobs`.
- **external skill packages** — `~/agent-skills/parallel-stock-deep-research` is external; extend via that package’s scripts, don’t silent-edit umbrella Parallel skills.
