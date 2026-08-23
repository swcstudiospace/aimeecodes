# KIP dual-write + analytics split

## Two stores + one metrics DB

| Store | Role | Path / service |
|-------|------|----------------|
| Local SQLite Brain | Hot formation heuristics, local FIND | `kip_memory/data/` via `kip_memory/nexus.py` |
| **Anda DB Cognitive Nexus** | Authoritative graph when configured | `anda-nexus.service` → `:8091` |
| Analytics SQLite/Postgres | SKU/ads metrics only — **not** preferences/identity | `kip_memory/data/analytics.db` or `AGENCY_ANALYTICS_DSN` |

## Dual-write API

Module: `kip_memory/dual_write.py`

- `ensure_agency_schema_remote()` — registers concept types (Product, Supplier, …) and **$PropositionType**s (`derived_from`, `mentioned_in`, `consolidated_from`, `improves`, …)
- `dual_upsert` / `dual_remember` — local + remote KML UPSERT
- Brain paths using dual-write: `formation`, `maintenance` (sleep), `learn_from_skill_proposal`, `learn_from_hermes_memory_entry`
- Recall: `engine=dual_recall` merges remote **`SEARCH CONCEPT "q" LIMIT n`** + local graph

Expect formation `remote_ok_ratio ≈ 1.0` and `event_remote_ok=True` when nexus is up.

## Official nexus protocol

```bash
curl -sX POST http://127.0.0.1:8091/kip \
  -H 'content-type: application/json' \
  -d '{"jsonrpc":"2.0","id":1,"method":"execute_kip","params":{"command":"DESCRIBE PRIMER"}}'
```

- `params` must be `{"command":"..."}` not a bare string
- Remote search that works: `SEARCH CONCEPT "Desk Reset" LIMIT 5`
- Bare `SEARCH "q"` may fail depending on server version — prefer `SEARCH CONCEPT`

## Proposition UPSERTs

Remote UPSERT with `SET PROPOSITIONS` fails if the predicate is not a registered `$PropositionType`. Always `ensure_remote_proposition_type` (or full `ensure_agency_schema_remote`) before props.

## Daily maintenance

```bash
systemctl is-active daily-brain-maintenance.timer   # or agency-brain-daily.timer if renamed
PYTHONPATH=/root/src/repos/ai-agency \
  /root/src/repos/ai-agency/.venv/bin/python -m scripts.daily_brain_maintenance
```

Job: schema ensure → sleep/consolidate → `kip_export_icp` → analytics heartbeat. Reports under `tmp/brain_maintenance/run_*.json`.

## Env

```bash
ANDA_NEXUS_URL=http://127.0.0.1:8091
KIP_ICP_MODE=local          # or ic_oss | s3 | canister
# AGENCY_ANALYTICS_DSN=postgresql://…   # optional
# AGENCY_DISABLE_ANDA_BRAIN=1
# AGENCY_DISABLE_ANALYTICS=1
```

## Do not

- Put order/ads time-series only in KIP (use analytics)
- Put identity/preferences only in analytics (use KIP)
- Claim live ICP canister write without `KIP_ICP_MODE` + endpoint configured
