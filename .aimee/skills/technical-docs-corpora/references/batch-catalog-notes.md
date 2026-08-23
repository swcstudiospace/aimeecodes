# Top-20 eng docs batch — site quirks

Catalog: `configs/catalog/top20-eng-docs.yaml`  
Runner: `scripts/batch_docs_skills.py`  
State: `data/batch/top20-state.json`  
DBs: `data/db/skills/docs-*.sqlite`  
Exports: `data/exports/skills/docs-*/`

## Successful pattern (this VPS)

- Cap **~80 pages/site**, `delay_ms: 400`, `concurrency: 2`, `honor_robots: true`
- First pass **`--skip-embed`** (FTS skills ship fast); optional later embed
- Totals observed: ~1.5k pages, ~27k chunks across 20 skills

## Per-site notes

| id | root | Notes |
|---|---|---|
| python | docs.python.org/3/ | Sitemap often sparse in discover; BFS from seed fills. Prefix `/3/` needs slash-tolerant match. |
| mdn-web | developer.mozilla.org/en-US/docs/Web/ | Allow `/en-US/docs/Web` (not only leaf topics) so the seed is allowed. |
| react | react.dev/learn | Strong `llms.txt`; many `.md` URLs work. |
| nextjs | nextjs.org/docs | llms + sitemap; allow `/docs`. |
| typescript | typescriptlang.org/docs/ | Weak sitemap; rely on BFS. |
| nodejs | **nodejs.org/api/** | robots: `Disallow: /docs/`, `Allow: /api/`. Old `/docs/latest/api/` → all `skipped_robots`. |
| django | docs.djangoproject.com/en/stable/ | Sitemap works. |
| fastapi | fastapi.tiangolo.com | Deny locale prefixes (`/es/`, `/zh/`, …). |
| postgres | postgresql.org/docs/current/ | Sitemap + allow `/docs/current`. |
| redis | redis.io/docs/ | llms + sitemap. |
| docker | docs.docker.com | llms + sitemap. |
| kubernetes | kubernetes.io/docs/home/ | Host sitemap is language index; nested EN sitemap + BFS. |
| github | docs.github.com/en | Host `llms.txt` (markdown links); allow `/en`, deny enterprise AE paths. |
| stripe | docs.stripe.com | llms + sitemap; good API coverage. |
| tailwind | tailwindcss.com/docs | No sitemap; BFS from seed. |
| vercel | vercel.com/docs | Host sitemap noisy; llms filters to `/docs`. |
| cloudflare | developers.cloudflare.com | Large; keep max_pages cap. |
| openai | **developers.openai.com/docs/** | platform.openai.com sitemap almost empty; developers host has llms. |
| anthropic | **platform.claude.com/docs** | docs.anthropic.com sitemap points at platform.claude.com — use final host. |
| golang | go.dev/doc/ | No llms/sitemap; BFS + max_depth. |

## Also built (ad-hoc, outside top-20)

- `parallel-web` — full docs.parallel.ai crawl (~176 pages) + hybrid vectors; skill under `~/agent-skills/parallel-web`
- `docs-agno` — full https://docs.agno.com/ Mintlify crawl (~5055 pages FTS); config `configs/agno-docs.yaml`; DB `data/db/skills/docs-agno.sqlite`; skill `~/agent-skills/docs-agno`. Drop `llms-full.txt` + giant OpenAPI before index. See `references/full-corpus-crawl.md`.
- Discover code fixes (project): slash-tolerant allowlist, markdown llms parse, safe normalize_url

## Refresh one site

```bash
cd /root/.openclaw/workspace/projects/docs-scraper && source .venv/bin/activate
python scripts/batch_docs_skills.py --only nodejs --max-pages 80 --skip-embed --force
```
