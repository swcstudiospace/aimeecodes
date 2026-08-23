---
name: grok-hermes-connector-ops
description: Use when Grok→Hermes MCP handoffs, invoke_skill, or multi-job fan-out on this VPS.
version: 0.3.3
metadata:
  hermes:
    tags: [grok, mcp, handoff, invoke-skill, parallel, connector, get-task-result]
    related_skills: [grok-handoff-hermes]
---

# Grok ↔ Hermes connector ops

Class-level runbook for the **Hermes** side of Grok MCP handoffs and the
`grok-hermes-connector` service on this VPS. Complements (does not replace)
user-owned `grok-handoff-hermes` receiver skill — if that skill is wrong, ask
the user to `hermes curator adopt grok-handoff-hermes`.

## When this applies

- Job id `hermes:grok-handoff:*` / EXECUTE packs from Grok
- Running a Hermes skill **through** Grok MCP (`invoke_skill`)
- Getting skill/research **results back into Grok** (`get_task_result` / `read_artifact`)
- Grok spawning **multiple** handoffs for one user goal (~3–4 is common)
- Changes under `/root/src/repos/grok-hermes-connector`
- Public MCP `https://grok.ego.engineer/mcp` → `127.0.0.1:8788`
- Storing/smoke-testing vendor API keys used by agent skills (e.g. Parallel)

## Topology

| Piece | Location |
|-------|----------|
| Hermes MCP | unit `grok-hermes-connector`, env `/etc/grok-hermes/connector.env`, data `/var/lib/grok-hermes/` |
| OpenClaw sibling | `127.0.0.1:8787`, `/etc/grok-openclaw` (separate `CONNECTOR_TOKEN`) |
| Repo | `/root/src/repos/grok-hermes-connector` |
| OAuth paste | Client ID `grok`, secret = `CONNECTOR_TOKEN`, scope `mcp`, Streamable HTTP |
| Audit | `/var/lib/grok-hermes/audit/audit.jsonl` |
| Dispatch | `/root/agent-scripts/dispatch_handoff_to_hermes.sh` |
| Work / artifacts | `/var/lib/grok-hermes/work`, `/var/lib/grok-hermes/artifacts` |

v0.3+ surface: Tasks ext `io.modelcontextprotocol/tasks`, `list_skills` /
`get_skill` / `invoke_skill` / `get_task` / `get_task_result` / **`read_artifact`**,
prompts, audit JSONL, rate-limit, `/metrics`.

**v0.3.1:** waiter tracks worker PID + job `.status` (not bash exit). Dedupe
(`DEDUPE_ENABLED`, `DEDUPE_WINDOW_SEC=600`) + `idempotency_key`. See
`references/v0.3.1-waiter-dedupe-checklist.md`.

**v0.3.2:** results return path — promote to `artifacts/<id>/result.json`;
durable work dir; dispatch `HANDOFF_ID`. See
`references/result-return-to-grok-v0.3.2.md`.

**v0.3.3 (current):** full **result loop**. `get_task` exposes `resultSummary` /
`hasStructuredResult` / `nextStep`. **`read_artifact`** for RESULT.md/json.
`submit_hermes_background` injects `HANDOFF_ID`+`ARTIFACTS_DIR` into Hermes;
waiter promotes + writes `task_store.result`. Skill briefs include RESULT
DELIVERY. Unit **`PrivateTmp=false`**. Canonical:
`references/grok-result-loop-v0.3.3.md` + repo `docs/GROK_RESULT_LOOP.md`.

Skill catalog: `/root/agent-skills` + `/root/.hermes/skills`. `/health` expect
version ≥ **0.3.3**.

## Choose the right MCP tool

| Need | Tool | Notes |
|------|------|--------|
| Run a named skill | **`invoke_skill` only** | One call; then poll only |
| Free-form implement | `handoff_to_hermes` | Still **one** per goal |
| Discover names | `list_skills` | **Once** per request |
| Confirm body/args | `get_skill` | Optional once |
| Poll status | `get_task` | Read `nextStep` / `hasStructuredResult` |
| Load payload for user | **`get_task_result`** | **Required** after completed |
| Read one file | **`read_artifact`** | `RESULT.md` or `result.json` |

**Never** combine `invoke_skill` + `handoff_to_hermes` for the same goal.

### Single-skill invoke (Grok paste pattern)

1. `invoke_skill` **exactly once** — never `handoff_to_hermes` for this request.
2. `list_skills` at most once; no tool fan-out.
3. After `taskId`, only poll that id (30–60s; up to 25m for deep Task).
4. Pass `idempotency_key` when retries are likely.
5. Forbidden: second invoke, parallel handoffs, inventing results while polling.

```text
invoke_skill
  skill: parallel-stock-deep-research
  goal: Run scripts/run_research.py for SPX-outperform 2026; structured picks; non-advice.
  arguments: {"year":2026,"count":10,"processor":"core","timeout":1200,"wait":true}
  idempotency_key: stocks-spx-2026-v1
  priority: high
```

Prefer **skill name + arguments JSON** over a giant CLI in `goal`.

Bypass Grok when reliability > UX: run
`~/agent-skills/parallel-stock-deep-research/scripts/run_research.py` on the VPS.

### Getting results back into Grok (mandatory)

Status polling alone is **not enough**:

```text
invoke_skill once → poll get_task → get_task_result → present
                     (optional read_artifact)
```

1. When `get_task` is `completed` (or `nextStep` says so): call **`get_task_result`**.
2. Optional **`read_artifact(name="RESULT.md")`**.
3. Present from fields only; never invent tickers; market content = non-advice.

Durable layout:

```text
/var/lib/grok-hermes/artifacts/<taskId>/result.json
/var/lib/grok-hermes/artifacts/<taskId>/RESULT.md
/var/lib/grok-hermes/work/jobs/
/var/lib/grok-hermes/work/results/
```

Env: `HANDOFF_SHARED_DIR`, `ARTIFACTS_DIR`, `DEDUPE_*`. Unit: **PrivateTmp=false**.

**Pull-only** (job already done): `get_task` + `get_task_result` only — no new
invoke. Example id: `skill-20260729T084101Z-bbd4c43b`.

## Handoff execution (Hermes receiving a job)

1. ACK id + goal + plan.
2. **Inventory first** — Grok often races parallel duplicate handoffs.
3. Align to current `server.py` surface; no third redesign.
4. `pytest -q` → restart unit → `/health` ≥ 0.3.3.
5. Structured output must land at `artifacts/<id>/result.json` (RESULT DELIVERY).
6. Report done|blocked|partial + paths. Never print secrets.

Multiple jobs same goal: keep **first** `taskId`; ignore siblings unless disjoint
paths. `deduped: true` → reuse that id only.

## Pitfalls

### Four-handoff fan-out (Grok client)

One ask → audit: `list_skills`×N, then **1× invoke_skill + 3–4× handoff_to_hermes**.
Prompt still must forbid handoff; code dedupe helps same fingerprint only.
See `references/skill-invoke-vs-four-handoffs-2026-07-29.md`.

### False early complete — fixed v0.3.1

Waiter no longer treats bash dispatch exit as job done. Deep Parallel still
needs 10–25 minutes of real polling.

### Results trapped / empty Grok — fixed v0.3.2–0.3.3

PrivateTmp + no artifact promote + Grok stopping at `get_task`. Now: durable
dirs, promote on complete, `get_task_result` + `read_artifact`, PrivateTmp=false.
Old stuck PrivateTmp files: copy into `artifacts/<id>/result.json` then re-pull.

### Concurrent implementation races

Inventory first; one pytest + one restart; one handoff per goal.

### Dual Task models

`taskId == handoffId` in HandoffStore is the durable model.

### Secrets

Keys in profile env / `/root/.config/<vendor>/api.env` mode 600. Parallel REST:
`x-api-key` + `/root/.config/parallel/api.env`.

### external_dirs skills

`~/agent-skills/*` — no autonomous patch. User-requested new packages OK.

## Verify

```bash
cd /root/src/repos/grok-hermes-connector && .venv/bin/pytest -q
systemctl daemon-reload && systemctl restart grok-hermes-connector
curl -sS http://127.0.0.1:8788/health   # >= 0.3.3
grep PrivateTmp /etc/systemd/system/grok-hermes-connector.service  # false
ls /var/lib/grok-hermes/artifacts/<taskId>/
```

## Related paths

- `references/concurrent-handoff-race-2026-07-29.md`
- `references/skill-invoke-vs-four-handoffs-2026-07-29.md`
- `references/v0.3.1-waiter-dedupe-checklist.md`
- `references/result-return-to-grok-v0.3.2.md`
- `references/grok-result-loop-v0.3.3.md` — **primary result-loop recipe**
- Repo: `docs/GROK_RESULT_LOOP.md`, `docs/UPGRADE_v0.3.md`
- External (no autonomous patch): `~/agent-skills/parallel-stock-deep-research/`, `parallel-web`
