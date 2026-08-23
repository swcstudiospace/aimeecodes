# Greptile + GitBook EN corpus (2026-08)

Configs live in the docs-scraper project: `configs/greptile-docs.yaml`, `configs/gitbook-docs.yaml`.
Probe `robots.txt`, `llms.txt`, and `sitemap.xml` with curl — they are plain text.

## Greptile (Mintlify)

- Final host: `https://www.greptile.com/docs` (robots Allow `/`; sitemap + `/docs/llms.txt`).
- Size: ~48 sitemap URLs, ~51 llms links, ~109 discovered (HTML + `.md`). Full crawl is polite.
- `llms-full.txt` ~277k chars, titled **Analytics Dashboard** — pollutes FTS. Deny `/docs/llms-full.txt` and delete the row before export.
- llms.txt also lists `/docs/bin/*.sh` and `/docs/scripts/*.sh` (404). Deny those prefixes.
- Export unique HTML only (~51 pages). DB after cleanup: ~100 pages / ~956 chunks (HTML+`.md`).
- Skill: `~/agent-skills/docs-greptile` → `data/exports/skills/docs-greptile`.

## GitBook (GitBook-hosted)

- Final host: `https://gitbook.com/docs` (not `www`, not `docs.gitbook.com`). robots Allow `/`; `Content-Signal: ai-train=yes`.
- Sitemap index children: EN pages, `documentation/fr|zh|ja-*` mirrors, `developers`, `changelog`, `policies`, `guides`.
- EN sitemap ~675 URLs; with `.md` dupes frontier ~1356. Locales 0 if denied.
- API dominance: `developers/gitbook-api` can be ~60% of the frontier. Crawl full EN; curate skill refs with prefix quotas.
- Also deny `/docs/llms-full.txt` and `/docs/llms-full.txt/1`.
- `.md` URLs return real markdown via httpx (not empty JS). Shortest product pages still ~1.4k chars.
- Skill: `~/agent-skills/docs-gitbook` (export after crawl+index; chunks stay 0 until end-of-run index).

## Post-crawl cleanup (both)

```sql
DELETE FROM pages WHERE url LIKE '%llms-full%' OR length(markdown) > 200000;
-- also delete matching chunks + chunks_fts rows first if FKs are not enough
```

Export uniqueness key: strip trailing `.md` and `?…`; prefer the non-`.md` URL. Skip `llms*.txt`.
Default `export_skill.py` is recency-ordered and will ship dupes unless filtered.
