# Multi-target packaging (Agency Cockpit)

Identifier: `ai.autonogrammer.agency-cockpit`  
Root: `/root/src/repos/ai-agency/agency-cockpit`

## Surfaces

| Target | Command | Artifact |
|--------|---------|----------|
| Web PWA | `npm run build:web` | `dist/` + `sw.js` + `manifest.webmanifest` |
| Web Docker | build web then `docker build -t agency-cockpit-web .` | nginx SPA on :80 |
| Desktop Linux | `npm run desktop:build` | `bundle/deb/*.deb`, `bundle/rpm/*.rpm`, `bundle/appimage/*.AppImage` |
| Desktop binary | (part of tauri build) | `src-tauri/target/release/agency-cockpit` |
| Android | `npm run android:init` then `android:build` / `android:build:aab` | `src-tauri/gen/android` + APK/AAB |
| iOS | `npm run ios:init` then `ios:build` (macOS) | `src-tauri/gen/apple` + IPA |

## Linux desktop system packages

```bash
sudo apt-get install -y libwebkit2gtk-4.1-dev build-essential curl wget file \
  libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev \
  libglib2.0-dev pkg-config xdg-utils
```

- Missing **webkit/glib** → compile fails early in `glib-sys` / `webkit2gtk`.
- Missing **xdg-utils** → deb/rpm may succeed; **AppImage fails** with `xdg-open binary not found`.

## Icons

```bash
npm run icons   # scripts/generate-icons.mjs → src-tauri/icons + public/icons + favicon
```

PNG-based ico/icns placeholders are OK for internal builds; replace for store polish.

## Runtime shell detection

`src/lib/platform.ts` → `detectShell()`: `web` | `tauri-desktop` | `tauri-mobile`.  
UI: `<PlatformBadge />`. External links: `openExternal()` (Tauri shell plugin vs `window.open`).

## Config pointers

- `src-tauri/tauri.conf.json` — `bundle.{linux,macOS,windows,iOS,android}`, CSP, `frontendDist: ../dist`
- `vite.config.ts` — `vite-plugin-pwa`, base `./` for static web / `/` under Tauri
- `capabilities/default.json` — core + shell:allow-open
- Mobile gen trees gitignored: `src-tauri/gen/`

## Verify checklist

1. `npm run build:web` exit 0; dist has PWA files; manifest `display: standalone`
2. Desktop: binary >1MB; deb/rpm/AppImage present under `bundle/`
3. Mobile: scripts exist; full device build only where SDK/Xcode present
