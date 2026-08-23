---
name: aceternity-template-sites
description: "Use when rebranding Aceternity/Next marketing template zips."
version: 1.1.0
author: Hermes
license: MIT
platforms: [linux, macos, windows]
metadata:
  hermes:
    tags: [aceternity, nextjs, marketing, wireframe, ia, fumadocs, template, brand]
    related_skills: [claude-design, popular-web-designs, design-md, docs-nextjs]
---

# Aceternity / Next marketing template → brand site

Use when the user drops an **Aceternity UI** (or similar Next.js + Tailwind + Motion) template zip and wants a **real multi-page brand website** with IA, wireframe, blog, and rebranded shell — not a one-off HTML mock.

Pair with:
- **`claude-design`** for standalone HTML wireframes / visual taste
- **`popular-web-designs`** only if matching a known brand look (usually skip when the template *is* the system)
- **`docs-nextjs`** for Next App Router edge cases

## Default project layout

```text
/root/<brand>-web/          # or user-chosen root
  template-src/             # untouched unzip of purchased template
  site/                     # working copy (copy of React/ Next app)
  docs/SITE-ARCHITECTURE.md # IA, sitemap, section map, phases
  wireframes/*.html         # interactive page picker + section stacks
  README.md
```

Never edit only inside the zip extract without a clean working copy.

## Workflow (do in order)

1. **Inventory the template**
   - Unzip; find `package.json`, `app/` routes, `components/`, `content/blog/`
   - List existing routes (often `/`, `/work`, `/products`, `/pricing`, `/blog`, `/blog/[id]`)
   - Note deps: `motion`, `fumadocs-mdx`, `@calcom/embed-react`, Tailwind 4, etc.

2. **Brand brief → sitemap (10–15 pages + blog)**
   - Primary nav ≤ 6 items + one CTA
   - Map pillars of the product to routes (don’t force template labels)
   - Write `docs/SITE-ARCHITECTURE.md`: audience, funnel, page goals, template→page component map, phases
   - Flag open decisions (public pricing? Cal vs form? docs in-repo?)

3. **Interactive wireframe first**
   - Single HTML file: sidebar of all pages, browser chrome mock, section blocks, component reuse notes, P0/P1 priority
   - This is the planning artifact the user can click — ship it before deep copy polish

4. **Scaffold working app**
   - `cp -a template-src/.../React site` (or the template’s Next root)
   - `npm install`
   - Prefer **`--webpack`** for dev/build when fumadocs-mdx is present (see Pitfalls)

5. **Brand shell (P0) before pretty pixels**
   - `lib/seo.ts` → name, description, url, twitter
   - Add `lib/brand.ts` early: `email`, `calLink` (`NEXT_PUBLIC_CAL_LINK`), social URLs, tagline — single source for CTAs
   - `layout.tsx` metadata from `siteConfig`
   - `navbar.tsx` → new routes + CTA label; logo links home
   - `footer` → Product / Explore / Company / Legal real hrefs
   - Hero eyebrow + H1 + sub + watermark wordmark
   - `Button`: default “Get started”; accept optional `href` (e.g. `/contact`); Cal uses `brand.calLink` and **falls back to `/contact`** if embed fails

6. **Routes**
   - Keep template pages that map cleanly (`work`, `pricing`, `blog`)
   - Add App Router stubs for new pillars with a shared `PageShell` (header + lead + section cards + optional FAQ)
   - Legacy routes (`/products`) → `redirect()` to canonical (`/platform`) when renamed

7. **Phase 1 full copy pass (do not leave agency residue)**
   - After shell is up, **same session if possible**, retheme high-visibility agency strings:
     - Bento card titles/bodies · comparison header/rows/labels · pricing tiers · FAQ Q&A · About founder strip · Products/Acebuilder hero · metrics blurb · FAQ CTA · blog sidebar CTA · testimonials/feedbacks buttons
   - Grep for `Aceternity|Chat with|No Calls|contact@aceternity` until **zero hits** in `site/`
   - Pricing: name tiers for the product (e.g. Spark / Nexus / Eternal), not “Website Components”
   - Soft social proof only — no fake celebrity quotes or invented metrics
   - Contact: mailto form OK for v1; document Cal env var on the page

8. **Blog**
   - Keep fumadocs MDX pipeline; match **full frontmatter schema** from `source.config.ts` (date, timeToRead, author*, previewImage, labels)
   - Prefer **batch-replacing** sample agency MDX once shell exists (delete samples + write brand posts) rather than leaving residue for “later”
   - 4–6 seed posts covering product pillars beat one orphan draft

9. **Pro blocks (optional parallel)**
   - While Pro session is live, skim `/blocks/*` and write `docs/PRO-BLOCKS-ROADMAP.md` (which categories map to which IA pages)
   - Do not bulk-dump the library; pull blocks page-by-page when fidelity demands
   - Browser cookies can drop between navigations — re-check session before assuming still logged in

10. **Verify**
   - `npm run build` (webpack scripts) after contact/form changes too
   - `npm run dev -p <port>` and curl every route for 200
   - Grep HTML for brand strings; grep source for leftover agency vendor name

11. **Report**
   - Paths table, sitemap, what’s stub vs polished, blockers (Pro login, assets, Cal)

## Aceternity Pro / codeblocks

- Template **zip = full page source** for that template; enough to rebrand and ship structure
- Pro portal login needs **user credentials or an already-authenticated browser session** — never invent passwords
- If logged out: continue from zip; list which extra blocks would help later
- Do not block the IA/wireframe/shell on Pro access

### Magic-link login (NextAuth on ui.aceternity.com)

UI form typing is flaky in headless/automation browsers (value clears, no toast). Prefer the **API path in the same browser session**, then have the user paste the email link:

1. Open `https://ui.aceternity.com/login` (establishes cookies).
2. In-page JS:
   - `GET /api/auth/csrf` → `csrfToken`
   - `GET /api/auth/providers` → email provider id is **`nodemailer`** (not `email`)
   - `POST /api/auth/signin/nodemailer` as `application/x-www-form-urlencoded` with:
     - `email`, `csrfToken`, `callbackUrl=https://ui.aceternity.com/`, `json=true`
     - `credentials: 'include'`
3. Success redirect: `/api/auth/verify-request?provider=nodemailer&type=email` (“Check your email”).
4. User pastes the full callback URL from their inbox into chat; **navigate that URL in the same browser** (token is single-use).
5. Confirm session: `GET /api/auth/session` → look for `user.email`, `user.isAllAccess`, `user.isLifetime`, `user.licenseType`.
6. Header should show **Orders** + email (not Login).

Details: `references/aceternity-pro-login.md`

## Component reuse map (Productized Agency pattern)

| Template piece | Typical brand use |
|----------------|-------------------|
| `Hero` + globe/stars | Home brand moment |
| `LogoCloud` | Ecosystem / partners |
| `BentoOne` / `BentoTwo` | Pillars + metrics |
| `Projects` | Case studies / work |
| `Comparison` | vs incumbents / chat agents |
| `Pricing` + Cal `Button` | Packages + book |
| `AboutSection` | Home strip + seed `/about` |
| `FAQ` / `Feedbacks` / `Testimonials` | Shared objections + proof |
| `Products` + `Acebuilder` | Platform / product umbrella |
| `content/blog` + fumadocs | SEO blog |

## Copy discipline

- Rewrite template agency copy on shell surfaces first (hero, nav, footer, SEO), then **same pass** for comparison/pricing/FAQ/bento — users experience residue as “still Aceternity”
- Centralize conversion config in `lib/brand.ts` so Cal/email don’t stay hardcoded to the template author
- Don’t invent metrics, logos, or customer quotes; soft role-based blurbs are fine until real proof exists
- CTA language: prefer **Get started** → `/contact` over template “Chat with Alex”

## Pitfalls

- **fumadocs-mdx + Next 16 Turbopack**: default `next build` / `next dev` can panic on `turbopack.rules.*.json` `query` conditions. Fix: set scripts to `next dev --webpack` and `next build --webpack`. Do not claim Next is broken globally.
- **`images.domains` deprecated**: use `images.remotePatterns` in `next.config.mjs`.
- **Blog MDX schema**: new posts must include every required zod field or collect-phase fails.
- **Don’t skip IA**: jumping straight to restyling leaves orphan routes and wrong nav.
- **Don’t wait on Pro login** when the zip is present.
- **Working copy vs template-src**: keep pristine extract for diffs/license reference.
- **Pro session drop**: header may flip back to Login after idle/navigation; re-run session check before copying blocks.
- **Cal embed without account**: point `NEXT_PUBLIC_CAL_LINK` at a real event; until then Button should degrade to `/contact`, not a dead Manu demo link.
- **Long `npm run build` with dev server up**: if build hangs/times out, stop `next dev` first; retry is usually clean.

## Support files

- `references/autonogrammer-ai.md` — live Autonogrammer.AI paths, sitemap, webpack note, Pro account state
- `references/productized-agency-inventory.md` — routes/components checklist for that template family
- `references/aceternity-pro-login.md` — NextAuth nodemailer magic-link API flow + session checks
- `references/phase1-brand-pass.md` — checklist of files/strings to retheme + grep gate

## Done checklist

- [ ] Architecture doc + interactive wireframe on disk
- [ ] Working `site/` with brand SEO/nav/footer/hero
- [ ] Phase 1 copy pass: zero agency-vendor greps in `site/`
- [ ] `lib/brand.ts` + Cal/contact wired
- [ ] Blog samples replaced or explicitly deferred
- [ ] All planned routes 200 (or explicit redirect)
- [ ] Build succeeds (webpack if needed)
- [ ] README with dev commands and next phases
