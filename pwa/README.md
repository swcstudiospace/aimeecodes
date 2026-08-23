# Aimee Codes PWA + landing

Installable browser/mobile shell and marketing landing for Aimee Codes.

## Design

- **Landing** (`index.html`) — ClippyOS-style Decide/Learn surface
  ([os.swcstudio.space](https://os.swcstudio.space/)): sticky mono nav,
  hero + CTAs, product shots, feature grid, install, FAQ.
- **Accent** — bubblegum pink `#FF5AC8` (not ClippyOS emerald).
- **Fonts** — IBM Plex Mono (UI, same family as ClippyOS) + JetBrains Mono
  (terminal chrome / CLI screenshots).
- **Shots** — `shots/cli-*.png` captured from the packaged `aimee` binary.

## Run locally

```bash
cd pwa
python3 -m http.server 4173
```

Open:

- http://localhost:4173/ — landing
- http://localhost:4173/app.html — PWA app shell (Install app)

## Files

| Path | Role |
|------|------|
| `index.html` | Marketing landing |
| `app.html` | Branded PWA chat shell (`:aimee` / `:muse` / `:sage`) |
| `manifest.webmanifest` | `start_url` → `app.html`, theme `#FF5AC8` |
| `sw.js` | Cache-first app shell |
| `icons/` | 192 / 512 PNG marks |
| `shots/` | CLI banner, commands, agents PNGs |

## Refresh CLI shots

```bash
export PATH="$HOME/.local/bin:$PATH"
aimee banner > /tmp/aimee-banner.txt
aimee list cmd --porcelain | head -45 > /tmp/aimee-cmds.txt
aimee list agent --porcelain | head -25 > /tmp/aimee-agents.txt
# then re-run the SVG→PNG generator used in packaging, or replace PNGs in shots/
```
