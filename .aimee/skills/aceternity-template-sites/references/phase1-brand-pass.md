# Phase 1 brand pass — file & string checklist

Run after nav/SEO/hero shell. Goal: visitor never sees the template vendor name.

## Grep gate (must be empty under `site/`)

```bash
rg -n 'Aceternity|Chat with Alex|contact@aceternity|No Calls\. No BS|Acebuilder is now Live|Traditional Service Providers' \
  --glob '!node_modules' --glob '!.next' site/
```

## High-impact files (Productized Agency family)

| Area | Typical paths |
|------|----------------|
| Brand config | `lib/seo.ts`, **`lib/brand.ts`** (create), `app/layout.tsx` |
| Chrome | `components/navbar.tsx`, `components/footer/index.tsx`, `components/button.tsx` |
| Hero | `components/hero/index.tsx` |
| Bento | `components/bento-one/**`, `components/bento-two/cards/*` |
| Comparison | `components/comparison/index.tsx`, `comparison-tabel.tsx`, `comparison-accordion.tsx` |
| Pricing | `components/pricing/index.tsx` |
| FAQ / CTA | `components/faq/index.tsx`, `faq/cta-card.tsx` |
| About | `components/about/index.tsx` |
| Platform | `components/acebuilder/index.tsx`, `components/products/index.tsx` |
| Proof CTAs | `components/testimonials/index.tsx`, `feedbacks/index.tsx`, `blogs/content.tsx` |
| Contact | `app/contact/page.tsx` |
| Blog | `content/blog/*.mdx` — delete samples; write brand posts with full frontmatter |

## Button contract

- Default label: `Get started`
- Optional `href` for hard navigation (`/contact`, `/pricing`)
- Cal: `getCalApi` + `brand.calLink`; on failure → `/contact`
- Env: `NEXT_PUBLIC_CAL_LINK`, `NEXT_PUBLIC_SITE_URL`

## Pricing rename pattern (example)

Not “Components / Website Pages / Multi Pages” — productized agent tiers, e.g.:

- **Spark** — solo pod + project memory  
- **Nexus** — multi-agent + org memory  
- **Eternal** — dedicated + custom ICP anchors  

Adjust numbers only with user input; placeholders OK if labeled as draft in docs.

## After pass

1. `npm run build` (webpack)
2. Curl `/ /pricing /blog /contact` for brand strings
3. Note remaining real content debt: work case studies, logo/OG, real Cal event, Pro blocks per `docs/PRO-BLOCKS-ROADMAP.md`
