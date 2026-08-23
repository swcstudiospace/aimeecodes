# Full-corpus / unlimited docs crawls

## When

User says “scrape the whole docs”, “unlimited pages”, or names a large Mintlify/Docusaurus site and wants offline FTS — not an 80-page skill slice.

## Config knobs

| Knob | Full-corpus guidance |
|------|----------------------|
| `policy.max_pages` | `100000` (no true unlimited; int cap only) |
| `delay_ms` | 250–400; stay polite |
| `concurrency` | 3–4 |
| `honor_robots` | true |
| `index.vectors` | false on first pass |
| `denied_prefixes` | `/cdn-cgi`, `/_next`; optionally `/examples` if you want a *balanced slice*, not true full |

Config lives under project `configs/<site>.yaml`. Example full Agno: `configs/agno-docs.yaml`.

## Pipeline order

1. `discover.py` — print frontier count + top-level path buckets.
2. `crawl.py` — background + log; pages land in SQLite even if index dies later.
3. **Cull mega-pages** before heavy FTS:
   ```sql
   SELECT canonical_url, length(markdown) FROM pages ORDER BY 2 DESC LIMIT 20;
   -- drop llms-full, giant openapi.yaml, anything >> 200k chars
   ```
4. Index:
   - Prefer `scripts/chunk_index.py` only if batch-commit is in `indexer.py`.
   - Else `python -u scripts/index_missing.py --db ... --batch 50` (pages with zero chunks only; keeps existing code_blocks).
5. `PRAGMA wal_checkpoint(TRUNCATE)` after interrupted runs; watch `*.sqlite-wal` size.
6. Export skill: curated references (core prefixes first) + SKILL.md pointing at full DB + `query.py`.
7. Install: `ln -sfn $PWD/data/exports/skills/docs-<x> ~/agent-skills/docs-<x>` and optional `~/.hermes/skills/`.

## Discovery skew

`discover` builds frontier from llms then sitemap, then **truncates to max_pages in list order**. Alpha llms → poor coverage under small caps. Mitigations:

- Raise max_pages to site size (true full crawl).
- Or build `seed_frontier` ranked by prefix quotas (core SDK / OS / reference / other) and pass into `run_crawl(..., seed_frontier=...)`.

## Agno case study (2026-08)

| Item | Value |
|------|--------|
| Root | https://docs.agno.com/ (Mintlify) |
| robots | Allow all docs; disallow `/_next/`, `/cdn-cgi/` |
| Sitemap | ~3.9k URLs; llms ~1k links; combined frontier ~5k with HTML+.md dupes |
| DB | `data/db/skills/docs-agno.sqlite` |
| Outcome | ~5055 pages, ~17k FTS chunks, ~32k code blocks |
| Skill | `docs-agno` @ `~/agent-skills/docs-agno` (200 curated refs) |
| Dropped | `llms-full.txt` (~12MB), `reference-api/openapi.yaml` (~356k) |
| Failure mode | Indexing mega-page → WAL ~30GB → “database or disk is full”; fixed by delete + `index_missing.py` |

Query smoke:

```bash
cd /root/.openclaw/workspace/projects/docs-scraper && source .venv/bin/activate
python scripts/query.py "create Agent tools" --db data/db/skills/docs-agno.sqlite -k 5 --mode fts
```

## Do not

- Treat empty multi-term FTS as “corpus empty” — retry with 2–4 keywords.
- Re-run full `index_pages` wipe+rewrite of all code_blocks on a 5k-page DB without batch commits.
- Patch the external `docs-scraper` skill package for code fixes — edit the project repo instead.
