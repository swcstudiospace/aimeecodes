# Grok result loop (v0.3.3)

Canonical end-to-end path so research/skill output lands **back in Grok chat**.

## Happy path (new job)

```text
1. list_skills (optional, once)
2. invoke_skill EXACTLY ONCE
     + idempotency_key if retries likely
     + structured arguments (not a giant CLI in goal)
3. Save taskId
4. Poll get_task every 30–60s until completed|failed
     - Respect nextStep / hasStructuredResult / resultSummary
     - Deep Parallel core: 10–25+ minutes normal
     - Do NOT handoff_to_hermes or second invoke
5. get_task_result(taskId)
     - Use result, summary, resultMarkdown
6. Optional: read_artifact name=RESULT.md | result.json
7. Present table + disclaimer (non-advice if markets)
```

## Pull-only (job already finished)

```text
get_task taskId="<id>"
get_task_result taskId="<id>"
# optional
read_artifact taskId="<id>" name="RESULT.md"
# FORBIDDEN: invoke_skill, handoff_to_hermes
```

Example finished stock research id from 2026-07-29:
`skill-20260729T084101Z-bbd4c43b`
Artifacts: `/var/lib/grok-hermes/artifacts/skill-20260729T084101Z-bbd4c43b/`
Tickers (non-advice): NVDA AVGO TSM AMZN META ETN GE LLY JPM GS

## Server guarantees

| Piece | Behavior |
|-------|----------|
| Dedupe | Same skill+goal+args within DEDUPE_WINDOW_SEC → same taskId, `deduped:true` |
| Waiter | Worker PID + `work/jobs/<job>.status` (not bash exit) |
| Env into Hermes | HANDOFF_ID, ARTIFACTS_DIR, HANDOFF_SHARED_DIR |
| Promote on complete | artifacts + task_store.result |
| get_task | nextStep, hasStructuredResult, resultSummary |
| get_task_result | Full structured payload |
| read_artifact | File body for presentation |
| PrivateTmp | false on unit |

## Skill authoring note

External skills under `~/agent-skills` must write:

```text
$ARTIFACTS_DIR/$HANDOFF_ID/result.json
$ARTIFACTS_DIR/$HANDOFF_ID/RESULT.md   # optional human table
$HANDOFF_SHARED_DIR/results/$HANDOFF_ID.json
```

`parallel-stock-deep-research` `run_research.py` does this when HANDOFF_ID is set.
Do not autonomous-patch external Parallel umbrella packages unless the user asks.

## Ops smoke

```bash
curl -sS http://127.0.0.1:8788/health   # >= 0.3.3
grep PrivateTmp /etc/systemd/system/grok-hermes-connector.service
ls /var/lib/grok-hermes/artifacts/<taskId>/
pytest -q   # from connector repo
```

Repo mirror: `/root/src/repos/grok-hermes-connector/docs/GROK_RESULT_LOOP.md`
