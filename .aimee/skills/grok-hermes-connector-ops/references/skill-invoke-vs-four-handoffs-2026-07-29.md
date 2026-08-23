# Skill invoke vs four-handoff fan-out (2026-07-29)

## Context

User wanted Grok MCP to run Parallel deep research (stocks vs SPX 2026) as **one**
Hermes job. Observed ~**4 handoffs** instead of one. Same-day skill package:
`~/agent-skills/parallel-stock-deep-research/` (external_dirs — do not autonomous-patch).

## Audit timeline (UTC, `/var/lib/grok-hermes/audit/audit.jsonl`)

| Time | Action | Detail |
|------|--------|--------|
| 07:47:55–58 | `list_skills` ×4 | Rapid thrash |
| 07:47:58–00 | `get_skill` ×6 | parallel-task / parallel-search |
| 07:48:07 | **`invoke_skill` ×1** | `skill-20260729T074807Z-2a4d40cc` → skill `parallel-task` |
| 07:48:32 | `handoff_to_hermes` | `…4c76be80` same deep-research goal |
| 07:48:35 | `handoff_to_hermes` | `…12f28815` “CRITICAL FOLLOW-UP” (prior skill “too fast”) |
| 07:48:42 | `handoff_to_hermes` | `…32b1b280` diagnose logs + relaunch |
| 07:48:58 | `handoff_to_hermes` | `…f60c4ad3` re-run Parallel core |

Earlier (~06:29): three concurrent handoffs on connector upgrade
(see `concurrent-handoff-race-2026-07-29.md`).

## False early complete (pre-v0.3.1)

`skill-…2a4d40cc` meta shortly after submit:

- `hermesCmd`: `bash …/dispatch_handoff_to_hermes.sh …`
- `hermesReturncode: 0`, `hermesRunning: false`
- `*.hermes.log` only dispatch banner (`started pid=…`) — not research output

**Mechanism:** dispatch nohups `hermes chat` and exits.
`submit_hermes_background` waited on bash PID, not worker / job `.status`.

## Resolution shipped: connector **v0.3.1** (same day)

| Fix | Where |
|-----|--------|
| Waiter polls job `.status` + worker PID | `hermes_backend.wait_for_hermes_job` |
| `dispatchPid` vs `hermesWorkerPid` | meta fields |
| Dedupe window + fingerprint | `dedupe.py`, `store.find_by_fingerprint` |
| `idempotency_key` on tools | `handoff_to_hermes`, `invoke_skill` |
| Single-job tool/prompt copy | `server.py` descriptions, `prompts.py` |
| Env | `DEDUPE_ENABLED=1`, `DEDUPE_WINDOW_SEC=600` in `/etc/grok-hermes/connector.env` |

Install/restart path used:

```bash
cd /root/src/repos/grok-hermes-connector
uv pip install -e . --python .venv/bin/python
.venv/bin/pytest -q   # 25 passed after change
systemctl restart grok-hermes-connector
curl -sS http://127.0.0.1:8788/health   # version 0.3.1
```

## Still required at the Grok prompt layer

Server dedupe does not stop Grok from thrashing discovery tools or from using
**different** goal wording that hashes differently. Always:

1. `invoke_skill` once (not + `handoff_to_hermes`)
2. Small `arguments` JSON; no giant CLI in goal
3. Stable `idempotency_key` if retries possible
4. Poll one `taskId` 10–25m for Parallel `core`

## Prefer for stock deep research

- Skill: `parallel-stock-deep-research` (not raw `parallel-task` + pasted CLI)
- Script bypass: `~/agent-skills/parallel-stock-deep-research/scripts/run_research.py`
