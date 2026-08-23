# PWA delivery

How the Aimee Codes installable shell is **served, cached, and installed**. Feature UI (agent chips, drafts, wallet stub) lives on [PWA](pwa.md). This page is delivery only.

Source of truth is the static tree under `pwa/` in the product repository. There is no bundler, no SSR, no CDN, and no edge cache config in the tree.

| File | Role |
|---|---|
| `pwa/index.html` | Single-file app shell (inline CSS + JS) |
| `pwa/manifest.webmanifest` | Install metadata: display, theme, icons |
| `pwa/sw.js` | Cache-first service worker |
| `pwa/icons/icon-192.png` | 192×192 PNG (`purpose: any`) |
| `pwa/icons/icon-512.png` | 512×512 PNG (`purpose: any maskable`) |
| `pwa/README.md` | Local serve command |

`flake.nix` builds the `aimee` CLI (`aimee_main` binary) only (`flake.nix:37-58`). It does not package or publish `pwa/`. `nix run github:swcstudiospace/omegaloops` is not a PWA delivery path.

## Serve locally

From a product checkout (`pwa/README.md:7-12`, also `README.md:323-325`):

```bash
cd pwa
python3 -m http.server 4173
```

Open [http://localhost:4173](http://localhost:4173). The working directory **must** be `pwa/` so relative URLs (`./index.html`, `./sw.js`, `./manifest.webmanifest`) resolve. Serving the repo root will 404 the shell and fail service-worker registration.

The shell registers `./sw.js` and reports status in `#sw-status` (`pwa/index.html:224-232`):

- success: `Service worker: registered — installable offline shell.`
- register failure: `Service worker: failed (serve over http://localhost).`
- no API: `Service worker: not supported in this browser.`

Service workers require a [secure context](https://developer.mozilla.org/en-US/docs/Web/Security/Secure_Contexts). `http://localhost` qualifies. Opening `index.html` as a `file://` URL will not register the worker.

There is no production host, CDN origin, or Cache-Control policy in this tree. Do not invent one.

## What “Install app” means

The shell is a Web App Manifest + service worker installable PWA, not an app-store binary.

1. Chrome / Edge fire `beforeinstallprompt`. The page `preventDefault()`s it, stores the event, and unhides `#install` (`pwa/index.html:211-215`).
2. Clicking **Install app** calls `deferredPrompt.prompt()`, waits for `userChoice`, then hides the button (`pwa/index.html:216-222`).
3. Safari / iOS has no `beforeinstallprompt`. Use the browser **Add to Home Screen** control (`pwa/README.md:12`). The document also sets `apple-mobile-web-app-capable` and `apple-touch-icon` (`pwa/index.html:8-13`).

After install, `display: standalone` (`pwa/manifest.webmanifest:7`) opens the shell without browser chrome. `start_url` is `./index.html` and `scope` is `./` (`pwa/manifest.webmanifest:5-6`), so the installed app is scoped to the directory that served it (locally, `http://localhost:4173/`).

Install does **not** ship the `aimee` CLI. The lede states the CLI remains the source of truth (`pwa/index.html:139-141`). Drafts stay in `localStorage` under `aimee-codes-pwa-v1` (`pwa/index.html:152-163`).

## Manifest: display, theme, icons

`pwa/manifest.webmanifest` is linked from the document (`pwa/index.html:11`). Theme is also duplicated as `<meta name="theme-color">` (`pwa/index.html:6`).

| Field | Value | Source |
|---|---|---|
| `name` | `Aimee Codes` | `pwa/manifest.webmanifest:2` |
| `short_name` | `Aimee` | `pwa/manifest.webmanifest:3` |
| `start_url` | `./index.html` | `pwa/manifest.webmanifest:5` |
| `scope` | `./` | `pwa/manifest.webmanifest:6` |
| `display` | `standalone` | `pwa/manifest.webmanifest:7` |
| `orientation` | `portrait-primary` | `pwa/manifest.webmanifest:8` |
| `background_color` | `#080612` | `pwa/manifest.webmanifest:9` |
| `theme_color` | `#ff5a7a` | `pwa/manifest.webmanifest:10` |
| `lang` | `en` | `pwa/manifest.webmanifest:11` |
| icon 192 | `icons/icon-192.png`, `any` | `pwa/manifest.webmanifest:13-18` |
| icon 512 | `icons/icon-512.png`, `any maskable` | `pwa/manifest.webmanifest:19-24` |

Favicon and Apple touch icon both point at the 192 PNG (`pwa/index.html:12-13`).

## Cache-first service worker

`pwa/sw.js` is a 34-line Cache API worker. `pwa/README.md:18` names the strategy **cache-first app shell**.

### Precache

```1:2:pwa/sw.js
const CACHE = "aimee-codes-v1";
const SHELL = ["./index.html", "./manifest.webmanifest", "./icons/icon-192.png", "./icons/icon-512.png"];
```

On `install`, the worker opens that cache, `addAll(SHELL)`, then `skipWaiting()` so the new worker does not wait for leftover tabs (`pwa/sw.js:4-8`).

On `activate`, every Cache Storage key **other than** `aimee-codes-v1` is deleted, then `clients.claim()` takes control of open pages (`pwa/sw.js:10-16`).

`sw.js` itself is not in `SHELL`. The browser caches the worker script separately. Changing `sw.js` bytes triggers a new install; changing shell files without bumping `CACHE` does not evict the old HTML/icons.

### Fetch

```18:33:pwa/sw.js
self.addEventListener("fetch", (event) => {
  if (event.request.method !== "GET") {
    return;
  }
  event.respondWith(
    caches.match(event.request).then((hit) => {
      if (hit) {
        return hit;
      }
      return fetch(event.request).then((response) => {
        const copy = response.clone();
        caches.open(CACHE).then((cache) => cache.put(event.request, copy));
        return response;
      }).catch(() => caches.match("./index.html"));
    })
  );
});
```

Behavior, in order:

1. Non-GET requests fall through to the network (no `respondWith`).
2. GET: `caches.match` first. A hit is returned **without** revalidation. That is cache-first, not stale-while-revalidate.
3. Miss: network `fetch`, `clone()`, `cache.put` of that response, then return the network body.
4. Network failure: fall back to the precached `./index.html` (offline navigation shell).

There is no allowlist beyond GET. There is no `response.ok` check before `cache.put`. There is no bypass for auth, wallet, or API URLs — none of those routes exist in this static tree today.

## Current limitations

Documented in the shell and README; do not treat the PWA as a second runtime.

| Limitation | Evidence |
|---|---|
| Static shell only | One `index.html` with inline `<style>` and `<script>`. No framework, no bundler, no `package.json` under `pwa/`. |
| No SSR / no edge render | Nothing in `pwa/` or `flake.nix` renders HTML on a server. |
| No CDN / no edge cache | Search of the product tree finds no CloudFront, Cloudflare, Fastly, or `Cache-Control` policy for this shell. |
| CLI is source of truth | `pwa/README.md:3`, `pwa/index.html:139-141`. Agent runs happen in `aimee`, not in the browser. |
| No agent API | Send appends a canned local reply (`pwa/index.html:183-188`). `aimee_*` is not wired. |
| Drafts are device-local | `localStorage` key `aimee-codes-pwa-v1` (`pwa/index.html:152-163`). Not a server cache. |
| Wallet is not a route | `#wallet` alerts only (`pwa/index.html:207-209`). No wallet URL for the worker to exclude yet. |
| Install scope is the origin that served `pwa/` | Manifest `scope` is `./` (`pwa/manifest.webmanifest:6`). |

Because the worker is cache-first on **every GET**, any future authenticated HTML or wallet/API GET added under this origin would be stored in `aimee-codes-v1` unless `sw.js` is changed first. Cache policy is a security control: do not cache authenticated HTML; do not intercept auth or wallet routes unsafely.

## Performance notes (measured from files)

There is **no** performance-budget file, Lighthouse config, or bundle analyzer in the tree. Do not claim a budget or a runtime win. These numbers are on-disk sizes only (`wc -c` on the files listed):

| Asset | Bytes |
|---|---|
| `pwa/index.html` | 9 207 |
| `pwa/sw.js` | 1 018 |
| `pwa/manifest.webmanifest` | 600 |
| `pwa/icons/icon-192.png` | 22 718 (192×192 RGB PNG) |
| `pwa/icons/icon-512.png` | 124 996 (512×512 RGB PNG) |
| **Sum of those files** | **159 122** |

Delivery-relevant facts that follow from the files, not from a benchmark:

- CSS and JS are inline in `index.html`. First paint does not wait on extra stylesheet or module requests.
- `"IBM Plex Mono"` is first in `font-family` but is not loaded via `@font-face` or a `<link>` (`pwa/index.html:29`). No webfont request is issued; the stack falls through to system monospace.
- The 512 icon is the largest payload (~125 KiB). It is in `SHELL`, so the first install precaches it.
- `python3 -m http.server` does not set long-lived `Cache-Control`. Repeat visits after the worker is active are served from Cache Storage, not from HTTP freshness.

Do not quote these bytes as a SLA.

## Best practices for changing the SW cache

The activate handler only deletes keys **not equal to** `CACHE` (`pwa/sw.js:12-13`). Cache-first means a hit is never revalidated (`pwa/sw.js:23-25`). Treat the cache name as the bust token.

1. **Bump `CACHE` when shell bytes change.** Rename `aimee-codes-v1` → `aimee-codes-v2` (or later) in `pwa/sw.js:1` whenever `index.html`, the manifest, or an icon in `SHELL` changes. Leaving the name in place leaves users on stale HTML until they clear site data.
2. **Keep `SHELL` in lockstep with files that must work offline.** Today that list is exactly four URLs (`pwa/sw.js:2`). Adding a CSS/JS file later without adding it to `SHELL` means first-offline after a miss can fail; adding a file that no longer exists makes `cache.addAll` reject and the install event fail.
3. **Expect immediate takeover.** `skipWaiting()` (`pwa/sw.js:6`) plus `clients.claim()` (`pwa/sw.js:14`) activates the new worker on this load, not on the next navigation. Test that a cache-name bump really drops `aimee-codes-v1`.
4. **Do not `cache.put` authenticated or error responses.** The current miss path stores whatever `fetch` returned (`pwa/sw.js:27-30`), including non-OK bodies. Before any agent API or wallet GET exists, add a method/URL allowlist and require `response.ok` (and a safe `Content-Type`) before write-through.
5. **Never widen intercept to auth or wallet.** Non-GET already bypasses (`pwa/sw.js:19-21`). Future `/auth`, `/wallet`, or credentialed HTML must also bypass — cache-first of those responses is a security bug, not a perf win.
6. **Change `sw.js` itself when strategy changes.** The worker script is versioned by byte identity. A comment-only edit is enough to retrigger `install`; a `SHELL`-only edit in a file the worker already cached is not enough without a `CACHE` bump.
7. **Verify on localhost, not `file://`.** Re-run `python3 -m http.server 4173` from `pwa/`. Confirm `#sw-status` shows registered (`pwa/index.html:224-226`). In DevTools → Application: Cache Storage contains the new name only; the old name is gone after `activate`.
8. **Do not add a second worker or a CDN “to fix” stale clients.** There is one `navigator.serviceWorker.register("./sw.js")` call (`pwa/index.html:225`). A second registrant or an invented edge purge is out of tree.

## Verify

```bash
# From the product checkout
cd pwa
python3 -m http.server 4173
# Browser: http://localhost:4173
# Expect: “Service worker: registered — installable offline shell.”
# Chrome/Edge: Install app appears after beforeinstallprompt.
# DevTools → Application → Cache Storage → aimee-codes-v1 holds SHELL.
```

There is no PWA unit test or Lighthouse gate in the workspace. Do not claim CI covers this shell.

## Related

- [PWA](pwa.md) — shell features (agents, drafts, wallet stub). Delivery stays on this page.
- [TUI](tui.md) — terminal surface; the CLI remains source of truth.
- [Wallet](../web3/wallet.md) — WEB3 spend stays HITL; do not cache it here.
- [Install and Nix](../ops/install.md) — `nix run` / `cargo install` for the `aimee` binary, not this static tree.
