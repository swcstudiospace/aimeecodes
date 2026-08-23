---
name: technical-docs-corpora
description: "Use when bulk-crawling public tech docs into agent skills."
version: 1.1.1
author: Hermes Agent
license: MIT
platforms: [linux, macos, windows]
metadata:
  hermes:
    tags: [docs, corpus, scrape, skills, sqlite, fts, batch, offline]
    related_skills: [hermes-agent-skill-authoring]
---

# Technical docs corpora → agent skills

## Overview

Turn public software documentation sites into **queryable local corpora** and **shared agent skills** (SKILL.md + `references/`). Prefer the project `docs-scraper` when present; this skill encodes the durable workflow, allowlist/robots pitfalls, and dual-app install pattern from multi-site batch builds.

**Project (this VPS):** `/root/.openclaw/workspace/projects/docs-scraper`  
**Shared install root:** `~/agent-skills/`  
**Operator skill (external):** `docs-scraper` — if it needs curator patches, run `hermes curator adopt docs-scraper` (it lives under `skills.external_dirs`).

## When to use

- Offline skills for many eng docs sites (React, Stripe, k8s, …)
- Bulk crawl + FTS/hybrid index + export skill packages
- Full-site / “unlimited pages” corpora (see §1b + `references/full-corpus-crawl.md`)
- Dual Hermes + OpenClaw sharing for doc packages
- Empty crawls (allowlist, robots, llms.txt, sitemaps)

Don't use for: private/auth docs, CAPTCHA bypass, or ignoring robots/ToS.

## Default stack

| Layer | Choice |
|---|---|
| SoR | SQLite + FTS5 |
| Vectors (optional) | fastembed + sqlite-vec, RRF hybrid |
| Fetch | httpx + markdownify (Crawl4AI optional) |
| Shared skills | `~/agent-skills/<name>/` |

## Workflow

### 1. Single site (polite slice, default)

```bash
cd /root/.openclaw/workspace/projects/docs-scraper && source .venv/bin/activate
# configs/<site>.yaml — tight allowed_prefixes, delay_ms ≥ 300, honor_robots: true
python scripts/crawl.py --config configs/<site>.yaml --max-pages 80
python scripts/embed_index.py --db data/db/<site>.sqlite   # optional; FTS works without
python scripts/export_skill.py --db data/db/<site>.sqlite --name docs-<site> --max-pages 80
ln -sfn "$PWD/data/exports/skills/docs-<site>" ~/agent-skills/docs-<site>
# optional Hermes visibility:
ln -sfn ~/agent-skills/docs-<site> ~/.hermes/skills/docs-<site>
```

**Done when:** `pages > 0`, `chunks > 0`, export has `references/*.md`, skill listed in Hermes/OpenClaw.

### 1b. Full-site / “unlimited pages” crawl

When the user wants the whole docs site (not an 80-page skill slice):

1. Set `policy.max_pages` very high (e.g. `100000`) — no true unlimited int; the scraper always caps the frontier.
2. Prefer sitemap **and** `llms.txt`; do **not** rely on llms list order alone (often alpha → API/OS first, core SDK late).
3. Run crawl in **background** with a log; multi-k Mintlify sites take tens of minutes.
4. After fetch, **index in batches**. Single end-of-run commit loses all chunks on kill/timeout. Use batch commits in `indexer.py` and/or `scripts/index_missing.py` for pages still missing chunks.
5. **Delete mega-pages before heavy FTS**: `/llms-full.txt`, giant OpenAPI YAML, or any `length(markdown) > ~200k`. One 12MB page can balloon SQLite WAL to tens of GB.
6. Skill package ≠ full DB: SQLite is SoR; export **curated** `references/` (~100–200 core pages) and point SKILL.md at `scripts/query.py` for full-corpus FTS.
7. Spot-check FTS with short queries (`create Agent tools`); long AND-of-many-terms queries often return empty under FTS5.

```bash
python scripts/crawl.py --config configs/agno-docs.yaml   # example: max_pages=100000
python -u scripts/index_missing.py --db data/db/skills/docs-agno.sqlite --batch 50
python scripts/query.py "create Agent tools" --db data/db/skills/docs-agno.sqlite -k 5 --mode fts
```

Details: `references/full-corpus-crawl.md` (Agno Mintlify case study).

### 2. Multi-site batch (5+ sites)

```bash
# Catalog: configs/catalog/top20-eng-docs.yaml
python scripts/batch_docs_skills.py --catalog configs/catalog/top20-eng-docs.yaml \
  --max-pages 80 --skip-embed --force
# FTS-first for speed; embed later if hybrid needed:
# for db in data/db/skills/docs-*.sqlite; do python scripts/embed_index.py --db "$db"; done
python scripts/batch_docs_skills.py --only react,fastapi --max-pages 150 --force
python scripts/batch_docs_skills.py --resume-failed
```

State: `data/batch/top20-state.json`.  
**Done when:** each source `status=ok`, pages ≥ floor, installed under `~/agent-skills/`.

### 3. Dual-app install (Hermes + OpenClaw)

1. Canonical body under `~/agent-skills/<skill>/`
2. Hermes: `skills.external_dirs: [/root/agent-skills]` as a **real YAML list** (not a quoted JSON string)
3. OpenClaw: `skills.load.extraDirs: ["/root/agent-skills"]` in `openclaw.json`
4. Optional symlinks under `~/.hermes/skills/` and `~/.openclaw/skills/`

## Discovery & allowlist pitfalls

1. **Trailing slash mismatch.** Normalized paths drop trailing `/`. Prefix `/3/` must match `/3` and `/3/library/...` via “equals stripped pref OR startswith stripped + `/`”.
2. **llms.txt is markdown.** Parse `[text](url)` and bare URLs. Skip `*`, `{templates}`, and junk that breaks `urlsplit` (Invalid IPv6 URL).
3. **llms.txt order is not importance.** Many Mintlify sites list paths alphabetically. Early cap (`frontier[:max_pages]`) then yields only `agent-os` / `api-reference` and starves `agents` / `models` / `tools`. For balanced slices: score+quota by top-level prefix, or crawl full site then curate export. `seed_frontier=` on `run_crawl` accepts a pre-ranked list.
4. **Host drift.** Redirecting roots (docs.anthropic.com → platform.claude.com, docs.gitbook.com → gitbook.com/docs — no `www`): set `root_url` to the **final** host so allowlist host checks pass.
5. **robots.txt path games.** nodejs.org `Disallow: /docs/` but `Allow: /api/`. Prefer robots-allowed paths; don't default to turning robots off.
6. **Sitemap sparse → BFS.** One seed is fine if crawl expands under allowlist; raise `max_depth` for handbook sites.
7. **discovered > 0 but fetched_ok = 0.** Check `skipped_robots` before blaming fetch/engine.
8. **Duplicate HTML + `.md` URLs.** Mintlify and GitBook often serve both; full crawls can nearly double page count. Harmless for FTS; **dedupe in skill export** (prefer the non-`.md` URL; strip `.md` as the uniqueness key). Shipping both wastes the ~100–200 reference budget.
9. **Locale sitemap mirrors.** GitBook-style sitemap indexes list `documentation/fr|zh|ja-*` copies of the same EN tree. Deny those prefixes or you triple-count. Probe `sitemap.xml` children before setting `max_pages`.
10. **llms.txt lists non-pages.** Mintlify indexes can include `/docs/bin/*.sh` and `/docs/scripts/*.sh` that 404. Deny `/docs/bin` and `/docs/scripts`, or treat those `fetched_fail` as expected.
11. **`llms-full.txt` pollutes FTS even when it is “only” ~200–300k.** The concat dump often steals another page’s title (Greptile: titled “Analytics Dashboard”) and outranks real pages. Deny `/docs/llms-full.txt` (and `/docs/llms-full.txt/1`) in config; delete matching rows **before** trusting FTS or export. This is separate from WAL-death on multi-MB dumps.

## Skill package shape

- Frontmatter `name` + trigger-focused `description`
- Runbook: cite `references/` Source URLs; don't invent APIs
- Optional SQLite path for hybrid/FTS query of the full corpus
- **No secrets in SKILL.md.** Avoid OpenClaw `requires.env` for optional keys (marks skill “needs setup”)

## Query

```bash
python scripts/query.py "question" --db data/db/skills/docs-<x>.sqlite -k 8 --mode auto
# modes: auto | fts | vector | hybrid
# Prefer short queries (2–4 tokens). Long AND chains often return zero under FTS5.
```

## Common Pitfalls

1. **Vectors on first batch pass** — slow; ship FTS with `--skip-embed`, embed offline later.
2. **Forking skills per app** — drift; always symlink from `~/agent-skills`.
3. **external_dirs as a string** — tree never loads; fix to YAML sequence.
4. **Patching external_dirs skills via agent** — refused; edit project + re-export, or `hermes curator adopt <name>`.
5. **Loose allowlists on MDN-scale sites** — keep caps polite (≤100 unless asked for full corpus).
6. **Single end-of-run FTS commit on multi-k pages** — kill/timeout → `chunks=0` despite pages stored. Batch commits or `index_missing.py`; `PRAGMA wal_checkpoint(TRUNCATE)` after kills.
7. **Mega-page WAL death** — indexing `llms-full.txt` / huge OpenAPI as one page can grow WAL to 30GB+ and hit “database or disk is full”. Delete those rows before indexing; monitor `*.sqlite-wal` size.
8. **`llms-full.txt` FTS title theft** — a 200–300k concat dump can stay under the WAL threshold and still dominate FTS with a wrong title. After crawl, `DELETE` rows where `url LIKE '%llms-full%'` or `length(markdown) > 200000`, then spot-check a short FTS query before export.
9. **Export `--query` with many AND terms** — FTS5 may return 1 page or none; export without query or with a short query, or build curated references via SQL prefix quotas. Default `export_skill.py` is recency-ordered and will include `.md` dupes + `llms.txt` unless you filter.
10. **API-heavy frontiers** — GitBook `developers/gitbook-api` can be ~60% of discovered URLs. Crawl full EN if asked; curate the skill package with prefix quotas (getting-started / create-content / publish / git-sync first). Point SKILL.md at `scripts/query.py` for the API tail.
11. **`docs-scraper` is external** — durable project code fixes go in the docs-scraper repo; this skill only carries operator workflow.
12. **Probe robots / llms / sitemap with curl** (plain-text endpoints). Do not use the browser stack or a paid extract API for `robots.txt`, `llms.txt`, or `sitemap.xml`.

## Verification Checklist

- [ ] Dry-run discover counts before full crawl
- [ ] robots honored; empty runs checked for skipped_robots
- [ ] Each skill: pages > 0, chunks > 0, references + SKILL.md
- [ ] Full corpus: `indexed_pages == pages`, no `length(markdown) > 200k`, WAL not multi-GB
- [ ] Under `~/agent-skills` and visible to Hermes + OpenClaw
- [ ] Spot FTS query returns on-host citations (short query); first hits are not `llms-full.txt` or a stolen title
- [ ] Skill `references/` are unique HTML pages (no `.md` duplicate, no `llms*.txt`)

## Support files

- `references/batch-catalog-notes.md` — site quirks from the top-20 eng docs batch
- `references/full-corpus-crawl.md` — unlimited/full-site crawl runbook + Agno Mintlify case study
- `references/gitbook-greptile-crawl.md` — Greptile Mintlify + GitBook EN corpus recipe (locale deny, llms-full, unique-HTML export)
