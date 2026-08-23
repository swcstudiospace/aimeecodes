# Autonogrammer.AI — session reference

**Brand:** WEB3 Autonomous Programmers · unified eternal memory · ICP · Knowledge Nexus  
**Root:** `/root/autonogrammer-web`

## Paths

| Path | Role |
|------|------|
| `template-src/productized-agency-template/React` | Pristine Aceternity Productized Agency extract |
| `site/` | Working Next.js app (`autonogrammer-web`) |
| `site/lib/brand.ts` | email, calLink, social, tagline |
| `docs/SITE-ARCHITECTURE.md` | Full IA |
| `docs/PRO-BLOCKS-ROADMAP.md` | Which Pro blocks map to which pages |
| `wireframes/autonogrammer-site-wireframe.html` | Interactive 14-page wireframe |
| `README.md` | Dev entry |

## Sitemap (v1)

`/` · `/platform` · `/programmers` · `/memory` · `/knowledge-nexus` · `/technology` · `/how-it-works` · `/use-cases` · `/work` · `/pricing` · `/about` · `/blog` + `/blog/[id]` · `/contact`  
`/products` → redirect `/platform`

Nav: Platform · Memory · Nexus · Pricing · Work · Blog · CTA Get started

## Build

- Scripts: `next dev --webpack` / `next build --webpack` (fumadocs-mdx + Next 16)
- Dev often on **:3010**
- If build hangs with dev running: stop dev, rebuild

## Phase status

- [x] IA + wireframe
- [x] Shell + all routes + PageShell stubs
- [x] Phase 1 copy (bento, comparison, pricing Spark/Nexus/Eternal, FAQ, about, products, CTAs)
- [x] 6 brand MDX posts (samples removed)
- [x] Contact mailto + Cal via `NEXT_PUBLIC_CAL_LINK` (default `autonogrammer/get-started`)
- [ ] Real logo / OG
- [ ] Real Cal.com event username
- [ ] Work case studies content
- [ ] Pro block pulls (timeline, beams, contact) when logged in
- [ ] Deploy

## Pro portal

- Account used: **oveshen.govender@gmail.com** (All-Access · Lifetime · PERSONAL when session healthy)
- Login: `references/aceternity-pro-login.md` — provider **`nodemailer`**; user pastes magic-link callback
- Cookies can drop — re-check `/api/auth/session` before assuming Pro still open
