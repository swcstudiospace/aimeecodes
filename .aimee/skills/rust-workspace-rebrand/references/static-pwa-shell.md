# Static installable PWA after a product rebrand

Use when the rebranded CLI should also be reachable from a browser / home screen.
No bundler. Match the TUI palette. CLI stays the source of truth.

## Minimum files

| File | Role |
|------|------|
| `pwa/index.html` | Brand shell, agent chips, compose box |
| `pwa/manifest.webmanifest` | `display: standalone`, `theme_color`, 192+512 icons |
| `pwa/sw.js` | Cache-first app shell, `skipWaiting` + `clients.claim` |
| `pwa/icons/icon-192.png` / `icon-512.png` | Real PNGs (not SVG-only) |
| `pwa/README.md` | `python3 -m http.server 4173` from `pwa/` |

## Brand rules

- Same product name, slash agents (`:omega` / `:muse` / `:sage`), void/cyan/magenta/lime.
- Drafts on-device (`localStorage`) until an API exists.
- Wallet control is labeled **soon** — no fake connect, spend stays HITL.
- Point the repo README WEB3 section at `pwa/`.

## Icons without Pillow

Write RGB PNGs with stdlib `struct` + `zlib` (IHDR/IDAT/IEND). Avoid `&` in one-liner shell (some runners treat it as background).

## Verify

```bash
python3 -c "import json; json.load(open('pwa/manifest.webmanifest'))"
# index.html contains manifest + serviceWorker + :omega
# icons start with PNG magic \x89PNG
```

Omega Loops path: `/root/src/repos/omegaloops/pwa/`.
