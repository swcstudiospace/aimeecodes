# Anda Bot / Autonogrammer marketing site (reference)

Repo package: `website/` (SvelteKit, pnpm, adapter-cloudflare).

## Key paths

| Path | Role |
|------|------|
| `website/src/routes/+page.svelte` | Thin orchestrator (locale, OS, sticky, copy) |
| `website/src/routes/layout.css` | Landing styles + mobile nav + sticky + unstuck |
| `website/src/lib/content/landing.ts` | Multi-locale section copy |
| `website/src/lib/content/branding.ts` | Dual brand, mobile/sticky, unstuck, SEO, site URLs |
| `website/src/lib/content/info.ts` | Privacy/terms/support |
| `website/src/lib/components/landing/*` | Section components |

## Dual brand (surface)

- **Product:** Autonogrammer
- **Engine:** Anda Bot (+ Brain)
- **Tagline (EN):** Autonomous intelligence, composed.
- Binaries/installers/store IDs remain Anda Bot unless a full rebrand is requested.

## Site URLs (stable)

- Site: `https://anda.bot`
- Docs: `https://docs.anda.bot`
- GitHub: `https://github.com/ldclabs/anda-bot`
- Chrome / Edge extension store IDs live in `branding.ts` `siteConstants`

## Verify

```bash
pnpm --dir website check
pnpm --dir website lint
pnpm --dir website build
```
