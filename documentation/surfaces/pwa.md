# Web and mobile PWA

`pwa/` ships an installable app shell for Aimee Codes — the brand surface for browser and mobile. Install it from a served copy of the directory; no app store required.

## What it is today

The PWA is an installable client shell with offline support:

* **Manifest:** name "Aimee Codes", short name "Aimee", theme color `#FF5AC8`, icons at 192 and 512 px.
* **Service worker** (`sw.js`, cache `aimee-codes-v1`): cache-first strategy with a precached shell — `index.html`, the manifest, and both icons. Installs with `skipWaiting`, so updates activate promptly.
* **Drafts stay on-device.** The agent API is not wired into the PWA yet, so drafts you write there remain local to your browser. Do not expect live flock sessions from the browser today.

## Installing it

Serve the directory over HTTP (required for service workers; `file://` will not register them):

```bash
cd pwa && python3 -m http.server 4173
```

Then open `http://localhost:4173`:

| Browser | Install |
|---|---|
| Chrome / Edge (desktop) | Install icon in the address bar, or menu → "Install Aimee Codes" |
| Safari (iOS) | Share → Add to Home Screen |
| Chrome (Android) | Menu → Install app |

## Offline behavior

Cache-first means the shell loads instantly and works without a network once visited. Because only the shell is precached, first visit should be online. The cache version (`aimee-codes-v1`) bumps when the shell changes, triggering re-fetch.

## Where it's going

The on-device draft model is deliberate: nothing leaves your device until agent wiring lands. When the agent API is connected, sessions will follow the same permission and credential rules as the CLI — see [Authentication and credentials](../integrations/auth.md).

Wallet login exists in the product beside provider auth; payments and spend stay human-approved. See [Wallet](../integrations/wallet.md).

## See also

* [Terminal UI](tui.md)
* [Pods and sandboxes](pods.md)
* [Providers and model access](../integrations/providers.md)

<!-- sources: pwa/manifest.webmanifest, pwa/sw.js, pwa/README.md, AIMEE.md §6 -->
