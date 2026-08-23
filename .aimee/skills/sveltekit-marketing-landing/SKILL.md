---
name: sveltekit-marketing-landing
description: "Use when shipping multi-locale SvelteKit landing sites."
version: 1.0.0
author: Hermes Agent
license: MIT
platforms: [linux, macos, windows]
metadata:
  hermes:
    tags: [sveltekit, marketing, landing, seo, i18n, dual-brand, mobile-nav, monorepo]
    related_skills: [hermes-project-soul, ratatui-agent-tui, dogfood, popular-web-designs, claude-design]
---

# SvelteKit multi-locale marketing landings

## Overview

Ship and evolve **product marketing sites** on **SvelteKit + Tailwind** (often inside a monorepo next to the product): dual-brand identity, mobile nav + sticky CTAs, sectioned components, multi-locale copy, SEO/JSON-LD, and a first-run “unstuck” strip. Pattern crystallized on the anda-bot `website/` package (Autonogrammer × Anda Bot).

## When to Use

- User wants improvements to a SvelteKit marketing/`website/` package
- Dual-brand (new product name + existing engine/runtime brand)
- Mobile navigation / sticky install CTAs missing on small screens
- Monolith `+page.svelte` needs splitting
- Multi-locale landing SEO (hreflang, og:locale, JSON-LD)
- Post-install help / troubleshooting strip near Install

Don't use for: Aceternity/Next template zips → `aceternity-template-sites`; one-off HTML mocks → `claude-design` / `sketch`; agent TUI brand → `ratatui-agent-tui`.

## Prerequisites

- `pnpm` workspace or package under `website/` (or similar)
- Match existing Svelte 5 runes style (`$state` / `$derived` / `$props`) when present
- Prefer package scripts over inventing deploy

## Default verify gates (this class)

```bash
pnpm --dir website check    # svelte-check — must be 0/0
pnpm --dir website lint     # prettier --check
pnpm --dir website build    # production + adapter
```

Do **not** treat root `make test` / `make lint` as the primary gate for website-only slices (those are often Rust-oriented in monorepos).

If `pnpm test` exits 1 with “No test files found”, that is an empty suite — not a functional regression. Note it; don’t block on inventing tests unless asked.

After edits: `pnpm --dir website exec prettier --write <paths>` then full `pnpm --dir website format` if lint still fails on pre-existing drift.

## Five high-leverage improvements (implement together when asked for “all”)

### 1. Mobile nav + sticky CTAs

- Desktop: full section nav. Mobile: hamburger → drawer with nav + primary/secondary CTAs.
- Sticky bottom bar (`Install` | `Extension` / second CTA) for `max-width < lg`; hide on desktop.
- **Pitfall:** some landings hide `.header-actions` entirely on small screens — that also kills language + menu. Keep actions visible; only hide desktop-only GitHub if needed.
- Pad `main` bottom on mobile so sticky bar doesn’t cover footer.

### 2. Dual brand without full product rename

Separate content module (e.g. `lib/content/branding.ts`):

| Field | Role |
|-------|------|
| `productName` | User-facing product (e.g. Autonogrammer) |
| `engineName` | Runtime/engine (e.g. Anda Bot) |
| `tagline` | Short kicker |
| `dualLine` | Header subtitle: “Product · powered by Engine & X” |
| `logoAlt`, launcher mock labels, store chrome, GitHub label | Localized chrome |

Header lockup = logo + productName + dualLine. Footer shows product + engine. Launcher mock uses product strings. Meta titles: `Product — … (Engine)`.

Do **not** rename binaries, installers, or store IDs unless asked — dual-brand the **surface**, keep package paths stable.

### 3. Split the monolith landing

Thin `+page.svelte` orchestrator (locale/OS state, copy handlers, mount detection). Sections as components under `lib/components/landing/`:

`SeoHead`, `SiteHeader`, `StickyCtaBar`, `HeroSection`, `ProofBand`, `WhySection`, `InstallSection`, `UnstuckSection`, `BrowserSection`, `LauncherSection`, `MemorySection`, `WorkSection`, `FinalCta`, `SiteFooter`, plus `icons.ts`.

Shared icon helpers live in `icons.ts`, not duplicated per section.

### 4. SEO / social / structured data

In `SeoHead` (or equivalent):

- `<title>`, description, **canonical**
- `hreflang` alternates for every locale + `x-default`
- `og:locale` + `og:locale:alternate`, image + **image:alt**, site_name
- Twitter card fields completed (title, description, image, alt)
- JSON-LD `SoftwareApplication` (name, OS list, description, offers price 0 if free)
- Escape `<` in JSON-LD when injecting via `{@html}`

Localize previously hardcoded English chrome (“Chrome Web Store / Edge…”, “GitHub”) via brand copy.

### 5. First-run unstuck strip

Place **after Install**, `id="unstuck"`:

- Three cards: daemon won’t start · extension won’t pair · no model/empty chat
- Each: short detail + deep link to docs
- Primary CTA → GitHub issue templates (`/issues/new/choose`)
- Nav label in all locales (`nav.unstuck` optional on type with fallback)

Content per locale in `branding.ts` / parallel module so you don’t balloon the giant `landing.ts` for every string.

## Content architecture

| Module | Owns |
|--------|------|
| `landing.ts` | Large per-locale marketing sections (hero, install, why, …) |
| `branding.ts` (or split) | Brand dual-identity, mobile/sticky strings, unstuck, SEO constants, site URLs |
| `info.ts` | Privacy/terms/support pages |

Extend `LandingCopy.nav` with optional `unstuck?: string`. Inject locale nav labels carefully (regex on `memory`+`docs` pairs if bulk-editing a huge file).

## Workflow

1. Survey `website/` routes, content modules, layout CSS mobile rules.
2. Add `branding.ts` (or equivalent) for dual-brand + unstuck + SEO + sticky/mobile strings.
3. Extract sections; keep state in page orchestrator.
4. Implement header drawer + sticky bar; fix CSS that hides header actions.
5. Wire SeoHead; update EN (+ primary secondary locales) meta/hero for dual brand.
6. Add unstuck section + nav labels for all locales.
7. `prettier --write` touched paths → `check` → `lint` → `build`.

Done when: mobile can open full nav and sticky CTAs; dual brand visible in header/footer/meta; page is sectioned; SEO tags present; unstuck lives under Install; verify triad green.

## Pitfalls

1. **Hiding header-actions on mobile** — breaks language + hamburger.
2. **Full rename by accident** — store IDs, binary names, install scripts stay unless scoped.
3. **JSON-LD via raw `<script>` in Svelte** — prefer `{@html}` with escaped `<`.
4. **Only EN unstuck/nav** — multi-locale sites need all locale keys or explicit fallbacks.
5. **Giant landing.ts only** — new chrome strings belong in a side module.
6. **`pnpm test` empty suite** — don’t treat as product failure.
7. **Prettier fail on unrelated files** — format whole package once so `lint` is trustworthy.
8. **RTL** — mirror drawer side and sticky layout under `[dir='rtl']`.

## Verification checklist

- [ ] `pnpm --dir website check` → 0/0
- [ ] `pnpm --dir website lint` → clean
- [ ] `pnpm --dir website build` → success
- [ ] Mobile: menu opens, sticky CTAs appear after scroll, footer not covered
- [ ] View-source/head: canonical, hreflang, og:locale, JSON-LD
- [ ] Dual brand in header, footer, title
- [ ] `#unstuck` after install with issue CTA

## References

- `references/anda-bot-website.md` — paths and dual-brand constants for the Autonogrammer/Anda Bot landing
