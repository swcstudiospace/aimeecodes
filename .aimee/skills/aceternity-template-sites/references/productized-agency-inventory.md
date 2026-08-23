# Aceternity Productized Agency template — inventory

Typical zip layout: `productized-agency-template/React/`

## App routes (stock)

- `app/page.tsx` — Home section stack
- `app/work/page.tsx`
- `app/products/page.tsx`
- `app/pricing/page.tsx`
- `app/blog/page.tsx`, `app/blog/[id]/page.tsx`
- `app/layout.tsx` — Navbar + Footer, Inter / Geist Mono / DM Mono

## Home stack (stock)

`Hero` → `LogoCloud` → `BentoOne` → `Projects` → `Testimonials` → `BentoTwo` → `Comparison` → `Pricing` → `AboutSection` → `Feedbacks` → `FAQ`

## Key components dirs

`components/hero`, `bento-one`, `bento-two`, `comparison`, `pricing`, `products`, `projects`, `about`, `faq`, `feedbacks`, `testimonials`, `blogs`, `resources`, `acebuilder`, `navbar`, `footer`, `button` (Cal embed)

## Blog

- `content/blog/*.mdx`
- `source.config.ts` — zod schema with authorName, authorRole, authorAvatar, previewImage, labels
- `lib/source.ts` + fumadocs

## Deps to expect

next 16, react 19, motion, fumadocs-mdx/core/ui, @calcom/embed-react, tailwind 4, tabler icons

## Rebrand touch list (minimum)

1. `lib/seo.ts`  
2. `components/navbar.tsx`  
3. `components/footer/index.tsx`  
4. `components/hero/index.tsx`  
5. `components/button.tsx` default text  
6. `app/layout.tsx`  
7. New pillar routes + optional `components/page-shell.tsx`  
8. `package.json` name + webpack scripts if MDX present  
9. `next.config.mjs` remotePatterns  
