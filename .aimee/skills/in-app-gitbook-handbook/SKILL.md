---
name: in-app-gitbook-handbook
description: "Use when cloning GitBook into a product repo or writing that product's handbook. Facts come only from the target tree."
version: 1.1.0
metadata:
  hermes:
    tags: [docs, gitbook, tanstack, landing, handbook]
    related_skills: [docs-gitbook, aceternity-template-sites]
---

# In-app GitBook handbook

Ship product docs as a **GitBook-compatible space in the repo** plus an **in-app reader**. Pair with `docs-gitbook` for hosted GitBook.com / Git Sync product questions.

## When to use

- “Clone GitBook into this repository”
- “Create a GitBook of our documentation” — hosted publishing via site-wide Git Sync; see `references/gitbook-monorepo-gitsync.md` for the validated monorepo recipe (root `docs.yaml` + self-contained `documentation/` space + verifier + porting audit)
- Replace a `/docs` placeholder that says docs will publish later
- Handbook + landing page, Magic UI / Aceternity, product fonts/colors

## Do not

- Clone `GitBookIO/gitbook` or vendor the GitBook app.
- Leave `/docs` as a “publishes on GitBook when we ship” stub.
- Invent a second visual system (Inter, purple). Use the product `@theme` fonts and accent.
- Turn on auth/db for a public handbook.
- Pull facts from a **sibling repo** (agency SOUL, another product’s agents). If the user says the project is standalone, the target folder is the only source. Cross-repo color is a defect.

## Depth bar (thin handbook = not done)

A first pass of one-paragraph pages is a stub. Include instructions, copy-pasteable code, best practices, and common errors grounded in this tree: `.env.example` + real `process.env` reads, `curl` for health/cron/`/api/v1`/`/api/mcp`, thrown codes from `userFacingErrorMessage`, playbook/scope/nav identifiers that exist as source.

## License (Spectrum Web Co)

When the user asks for **AGL 3.0** for Spectrum Web Co / `swcstudiospace` / `@swcstudio`, write `LICENSE` as **Autonogrammer General License 3.0** (`LicenseRef-AGL-3.0`). Do not substitute Apache-2.0 (Aimee) or GNU AGPL unless they name those. Point README + `/docs/legal/agl-3.0` + landing footer at it. Licensor: Spectrum Web Co LLC · ovesheng@spectrumweb.co.

## Steps

1. **Read only the target product tree** — README, nav, integration guides, `errors.ts`, playbooks, API routes. Document real routes and test-connection contracts. Do not invent endpoints or metrics. Do not open a neighbouring repo “for context.”
2. **Create the Git Sync space** under `documentation/`:
   - `.gitbook.yaml` (`readme` + `summary`)
   - `SUMMARY.md` (`## Group` then `* [Title](path.md)`)
   - `README.md` as welcome (slug `""`)
   - **Monorepo (site-wide Git Sync)**: also write `docs.yaml` at the REPOSITORY ROOT declaring the space mapping (`$schema: https://api.gitbook.com/openapi.yaml#/components/schemas/GitSyncSiteConfig`; `site.structure[].content.directory`). Each mapped directory must be fully self-contained — GitBook does not share assets between spaces. Confirmed working layout (aimeecodes, 2026-08): root `docs.yaml` + `documentation/{.gitbook.yaml,README.md,SUMMARY.md}` with groups Getting started / Usage / Surfaces / WEB3 / Ops / Help / Architecture / Tool reference / API reference / Development. Details: [references/gitbook-monorepo-gitsync.md](references/gitbook-monorepo-gitsync.md).
   - **Ship a verifier**: a script inside the space that fails on broken relative `.md` links, pages unreachable from SUMMARY/README, and secret-shaped filenames (template: [scripts/verify-docs.py](scripts/verify-docs.py), expects `<repo>/documentation/scripts/`). Run it right after scaffolding — while pages are still missing it doubles as the TODO list; run again before claiming done.
   - **Porting an existing docs space? Copy first, then audit.** Relative links break silently when the hierarchy changes (old `runtime/streaming.md` → new `architecture/streaming.md`, dropped siblings like `pwa-delivery.md`). Run the verifier immediately after the bulk copy and fix every hit before writing new pages; every SUMMARY entry must exist on disk or GitBook sync errors.
3. **Markdown is source of truth** — after the first scaffold, edit `documentation/**/*.md` and run `python3 scripts/sync-docs-catalog.py` to rebuild `src/lib/docs/generated.ts`. A one-shot `generate-docs.py` is fine for the initial dump only. Do not `fs.read` markdown at runtime. Do not rely on `import.meta.glob` of `.md` unless that Vite setup already does it.
4. **Reader routes** (TanStack Start):
   - `docs.tsx` — layout + `<Outlet />`
   - `docs.index.tsx` — `/docs`
   - `docs.$.tsx` — splat
   - Resolve the page from `pathname`, not splat param names (they differ across Start versions).
   - Dynamic links: `to={hrefFor(slug) as "/docs"}` until `/docs/$` is in the typed tree.
5. **Shell** — left groups + pages, ⌘K search, article, right TOC, prev/next. Magic UI / Aceternity on the **docs home hero**, not on every paragraph.
6. **Landing links (all of them)** — header Docs, hero Documentation, handbook card row, FAQ pointer, footer. One hero link is not enough.
7. **Verify** — `/docs` and one nested slug show article body (not sidebar-only). Landing **Docs** lands on `/docs`. Screenshot after BlurFade has settled.

## Markdown dialect

Keep the renderer small: ATX headings, lists, tables, fences, `[text](/docs/…)`, `:::hint info|warning`, `:::cards` (`title|href|body` per line).

## Grouping (clipping / agency OS)

Getting started (local setup, env, deploy, quickstart) · Product (one page per nav surface) · Social Machine · Integrations (one page per card) · Autonomy (Hermes, playbooks, scopes — **only if those files exist in-tree**) · Storage · API · Operators (best practices + common errors) · Safety · Reference · Legal.

## Support

- [references/in-app-product-handbook.md](references/in-app-product-handbook.md) — ClippyOS layout, sync script, AGL-3.0 pointer
- [references/gitbook-monorepo-gitsync.md](references/gitbook-monorepo-gitsync.md) — site-wide Git Sync `docs.yaml` facts + working aimeecodes layout
- [scripts/verify-docs.py](scripts/verify-docs.py) — link/reachability/secret verifier template for a `documentation/` space
