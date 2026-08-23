---
name: anda-ecosystem
description: "Use LDC Labs Anda/KIP memory stack end-to-end."
version: 1.0.0
author: Autonogrammer + Hermes Agent
license: MIT
platforms: [linux, macos, windows]
metadata:
  hermes:
    tags: [anda, kip, memory, icp, brain, anda-db, ldclabs]
    related_skills: [ai-dropshipping-agency-mcp, docs-scraper]
---

# Anda Ecosystem Skill

LDC Labs / ICPanda stack for **sovereign agent memory**: KIP protocol, Anda runtime,
Anda DB (embedded graph+BM25+HNSW), Anda Brain (formation/recall/sleep), Anda Bot,
ic-oss object storage, Agent Protocols. Use this when wiring shared memory across
**Hermes self-improvement** and **Agno AgentOS**.

## When to Use

- Shared memory between Hermes and Agno agents
- KIP commands, capsules, Genesis bootstrap
- Formation / recall / sleep maintenance cycles
- ICP / ic-oss / S3 capsule export
- Querying offline Anda docs under `knowledge/anda/`
- Don't use for: generic vector RAG without graph semantics

## Stack map

```text
Products:     Anda Brain · Anda Bot
Infrastructure: Anda engine · Anda DB · Cognitive Nexus server
Protocols:    KIP (KQL/KML/META) · Agent Protocols
Foundation:   ICP identity · TEE · object_store (local/S3/GCS/Azure/ic-oss)
```

Canonical upstream: https://github.com/ldclabs · https://anda.ai/

## This VPS layout

| Piece | Path / port |
|-------|-------------|
| Docs corpus (scraped/cloned) | `ai-agency/knowledge/anda/` |
| Local KIP SQLite + Brain | `ai-agency/kip_memory/` |
| Genesis capsules | `kip_memory/capsules_seed/*.kip` |
| Official Rust Nexus | `anda-nexus.service` → `127.0.0.1:8091` |
| Hermes reverse bridge | `hermes-bridge` → `:7790` (brain+KIP tools) |
| Agno knowledge | `FileSystemKnowledge(knowledge/anda)` on all agents |
| Agno brain tools | toolbelt `anda_brain` |

## Prerequisites

- Agency venv + `PYTHONPATH=/root/src/repos/ai-agency`
- Services: `systemctl start anda-nexus hermes-bridge`
- Env (see `.env.example`):
  - `ANDA_NEXUS_URL=http://127.0.0.1:8091`
  - `KIP_ICP_MODE=local` (or `canister`/`s3`/`ic_oss`)
  - Optional: `IC_OSS_ENDPOINT`, `KIP_ICP_CANISTER_ID`, `KIP_S3_BUCKET`

## How to Run

```bash
# Official Cognitive Nexus health
curl -s http://127.0.0.1:8091/

# KIP via JSON-RPC
curl -sX POST http://127.0.0.1:8091/kip \
  -H 'content-type: application/json' \
  -d '{"jsonrpc":"2.0","id":1,"method":"execute_kip","params":{"command":"DESCRIBE PRIMER"}}'

# Local Brain (formation → recall → sleep)
cd /root/src/repos/ai-agency && source .venv/bin/activate
PYTHONPATH=. python - <<'PY'
from kip_memory.brain import bootstrap_genesis, formation, recall, maintenance
print(bootstrap_genesis()["loaded"])
print(formation("Prefer dark UI. Learned kill CPA > $24.", agent="hermes")["extracted"])
print(recall("CPA")["context_markdown"][:400])
print(maintenance())
PY

# Hermes bridge tools (from Agno or Hermes MCP hermes-bridge)
# anda_brain_formation / anda_brain_recall / anda_brain_sleep / kip_export_icp
```

## Procedure — Hermes self-improve loop

1. Agent or Hermes proposes skill change via `hermes_skill_propose`.
2. Proposal lands in `skills/_proposals/` + Linear issue.
3. Bridge auto-writes KIP **Insight** + **Commitment** (`learn_from_skill_proposal`).
4. Curator merges skill; formation a confirmation message; optional `anda_brain_sleep`.
5. Completion: proposal file + KIP commitment status updated in graph.

## Procedure — Agno agent memory

1. Before research/answer: `anda_brain_recall(query=...)` or `kip_recall`.
2. After decisions: `anda_brain_formation(text=summary, agent=agent_id)`.
3. Docs: `anda_docs_search` or knowledge tools `grep_file` / `get_file` on Anda corpus.
4. Nightly/cron: `anda_brain_sleep` then `kip_export_icp(label=daily)`.
5. Completion: recall returns `context_markdown`; export has `cloud` receipt.

## Procedure — Cloud / ICP

1. Keep `KIP_ICP_MODE=local` until canister ready (writes sha256 receipts).
2. For ic-oss: set `KIP_ICP_MODE=ic_oss`, `IC_OSS_ENDPOINT`, optional token.
3. For S3-compatible: `KIP_ICP_MODE=s3`, `KIP_S3_BUCKET`, AWS keys.
4. Call `kip_export_icp` — capsule under `kip_memory/data/capsules/` + push.
5. Completion: response `cloud.ok` true or local receipt path present.

## KIP cheat sheet

| Goal | Command / tool |
|------|----------------|
| Who am I / schema | `DESCRIBE PRIMER` |
| Search | `SEARCH "mobility"` |
| Remember | `kip_remember` / UPSERT CONCEPT |
| Conversation encode | `anda_brain_formation` |
| Pre-answer context | `anda_brain_recall` |
| Consolidate | `anda_brain_sleep` |
| Portable backup | `EXPORT` / `kip_export_icp` |

Instruction sets: **KQL** FIND · **KML** UPSERT/UPDATE/MERGE/DELETE · **META** DESCRIBE/SEARCH/EXPORT.

## Offline docs corpus

Cloned + curated at `knowledge/anda/`:

- `KIP_README.md`, `KIP_SPECIFICATION.md`, `KIP_BRAIN.md`
- `ANDA_*.md`, `ANDADB_*.md`, `ANDABRAIN_*.md`, `ANDABOT_*.md`
- `anda_db_docs/*`, `brain_posts/*`, `ICOSS_README.md`, `AGENT_PROTOCOLS_README.md`

Re-scrape: docs-scraper configs `anda-ecosystem.yaml` / `anda-ai-site.yaml` under
`/root/.openclaw/workspace/projects/docs-scraper/configs/`.

## Pitfalls

- Official nexus `params` must be `{"command":"..."}` not a bare string.
- Local SQLite Brain and Rust AndaDB are **two stores** unless you route all KIP through `ANDA_NEXUS_URL` (formation still local-first for speed; execute prefers remote when URL set).
- `cargo install anda_cognitive_nexus_server` from crates.io fails (`publish=false`); build from ldclabs/anda-db path (already installed on this host).
- Don't put wallet seeds in KIP attributes; ICP identity is separate from ad-spend vault.
- FileSystemKnowledge needs `search_knowledge=True` (factory sets this).

## Dual-write + daily maintenance (improvements)

- Formation/remember/sleep/skill-propose **dual-write** local SQLite + remote Anda DB (`ANDA_NEXUS_URL`)
- Recall merges remote `SEARCH CONCEPT` + local graph
- Timer: `daily-brain-maintenance.timer` @ **04:15 UTC** → sleep + capsule export
- Manual: `PYTHONPATH=. python -m scripts.daily_brain_maintenance`
- Ops analytics (not brain): toolbelt `analytics` / SQLite `kip_memory/data/analytics.db` or `AGENCY_ANALYTICS_DSN`

## Verification

```bash
systemctl is-active anda-nexus hermes-bridge agency-brain-daily.timer
curl -s http://127.0.0.1:8091/ | jq .
curl -s http://127.0.0.1:7790/health | jq .
PYTHONPATH=/root/src/repos/ai-agency .venv/bin/python -c "from kip_memory.brain import formation; print(formation('prefer kit', agent='v')['remote_ok_ratio'])"
```

Expect: nexus up, bridge ok, formation `remote_ok_ratio` ≈ 1.0, timer active.
