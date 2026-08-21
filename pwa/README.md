# Aimee Codes PWA

Installable browser/mobile shell for Aimee Codes. Same brand as the `aimee` TUI. Drafts stay on-device; the CLI remains the source of truth until `aimee_*` is wired as an API.

## Run locally

```bash
cd pwa
python3 -m http.server 4173
```

Open http://localhost:4173 — Chrome/Edge/Safari can then **Install app** / Add to Home Screen.

## Files

- `index.html` — branded shell, agent chips (`:aimee` / `:muse` / `:sage`)
- `manifest.webmanifest` — standalone display, theme `#ff5a7a`
- `sw.js` — cache-first app shell
- `icons/` — 192 and 512 PNG marks
