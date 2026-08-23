# PWA

The Aimee Codes **installable app shell**. Same brand as the `aimee` TUI. Drafts stay on-device. The CLI remains the source of truth until `aimee_*` is wired as an API (`pwa/README.md:3`, `AIMEE.md:16`, `AIMEE.md:201-203`).

This page is the **shell**: what it is, how to open it locally, files, agent chips, theme, drafts, and current limits. How the worker caches and how **Install app** works lives on [PWA delivery](pwa-delivery.md). The header **Wallet soon** control is a stub — see [Wallet](../web3/wallet.md).

There is no agent HTTP API in this tree. Do not invent one. Send writes a canned local reply (`pwa/index.html:183-188`).

## What the shell is

A static Web App Manifest + service worker surface under `pwa/` (`AIMEE.md:201-203`, `README.md:318-325`). It is one of four runtime surfaces (`AIMEE.md:13-16`):

| Surface | Role |
|---|---|
| Interactive TUI | `aimee` (`crates/aimee_main`) |
| One-shot CLI | `aimee -p "…"` |
| ZSH `:` prefix | `: sage …`, `:muse …`, `:aimee …` |
| PWA | Browser / mobile installable shell |

The page lede states the contract in the UI itself (`pwa/index.html:139-141`):

> Install this PWA on your phone or desktop. The **aimee** CLI remains the source of truth; this shell is the browser/mobile surface. Drafts stay on-device until the agent API is wired.

It is not a second runtime. It does not call `aimee`, gRPC (`aimee.v1`), or `services_url`. The flock still runs in the terminal.

## Serve locally

From a product checkout (`pwa/README.md:7-12`, also `README.md:323-325` and `AIMEE.md:203`):

```bash
cd pwa
python3 -m http.server 4173
```

Open [http://localhost:4173](http://localhost:4173). Chrome / Edge / Safari can then **Install app** / Add to Home Screen.

The working directory **must** be `pwa/` so relative URLs (`./index.html`, `./sw.js`, `./manifest.webmanifest`, `./icons/…`) resolve. Serving the repo root 404s the shell and fails service-worker registration (`pwa/index.html:224-228`).

Service workers need a secure context. `http://localhost` qualifies. Opening `index.html` as `file://` will not register the worker.

There is no production host in this tree. Delivery details (manifest, cache-first worker, install prompt) are on [PWA delivery](pwa-delivery.md).

## Files

| Path | Role |
|---|---|
| `pwa/index.html` | Branded shell, agent chips, compose, install + wallet buttons (`pwa/README.md:16`) |
| `pwa/manifest.webmanifest` | Standalone display, theme `#ff5a7a` (`pwa/README.md:17`) |
| `pwa/sw.js` | Cache-first app shell (`pwa/README.md:18`) |
| `pwa/icons/icon-192.png` | 192×192 PNG mark |
| `pwa/icons/icon-512.png` | 512×512 PNG mark |
| `pwa/README.md` | Local serve command |

No bundler, no `package.json` under `pwa/`, no TypeScript, no framework. CSS and JS are inline in `index.html`.

## Agent chips

Three built-in agents, same IDs as the flock (`AIMEE.md:60-64`). The rail is labelled `Agents` (`pwa/index.html:133-137`):

| Chip | `data-agent` | Label in the UI | Color token |
|---|---|---|---|
| `:aimee implement` | `aimee` | default `active` | `--rose` (`pwa/index.html:80`) |
| `:muse plan` | `muse` | plan | `--violet` (`pwa/index.html:81`) |
| `:sage research` | `sage` | research | `--gold` (`pwa/index.html:82`) |

Clicking a chip (`pwa/index.html:195-202`):

1. Removes `.active` from every chip and adds it to the clicked one.
2. Sets `agent` to `btn.dataset.agent`.
3. Updates the compose placeholder to `:{agent} …`.

The selected agent is **client-only**. It is stored on each draft message (`item.agent`) and shown as `ROLE · :agent` (`pwa/index.html:171-173`). It does not switch a live `AgentId` in `aimee_domain`. There is no `task` dispatch, no Sage research, and no Muse `plans/` write from this shell.

On viewports `max-width: 640px` the rail becomes a horizontal row and chips drop vertical writing-mode (`pwa/index.html:108-115`).

## Theme

PWA tokens are CSS custom properties on `:root` (`pwa/index.html:15-24`):

| Token | Hex | Used for |
|---|---|---|
| `--void` | `#080612` | Page background; manifest `background_color` |
| `--ink` | `#f0f4ff` | Body text |
| `--cyan` | `#00e5ff` | Install button, lede `<strong>`, scan accents |
| `--rose` | `#ff5a7a` | Wordmark, `:aimee` chip, theme-color |
| `--magenta` | `#ff2d95` | Wallet outline, user message rail |
| `--violet` | `#a855f7` | `:muse` chip, nav border |
| `--lime` | `#a3ff12` | `WEB3 · PWA` tag, Send |
| `--gold` | `#ffd200` | `:sage` chip, message `.who` |
| `--rail` | `#120c22` | Agent nav background |

Document + manifest theme is rose (`pwa/index.html:6`, `pwa/manifest.webmanifest:10`). Brand wordmark is `æ` + `AIMEE CODES` + tag `WEB3 · PWA` (`pwa/index.html:122-126`). Face stack is `"IBM Plex Mono"` then system monospace (`pwa/index.html:29`); there is no `@font-face` load.

`AIMEE.md:199` still says TUI tokens match this PWA palette. The shipped TUI palette in `crates/aimee_main/src/theme.rs` is Warp dark (`#01A4FF` / `#0B0D12`). Treat the table above as the **PWA** source of truth.

## Drafts stay on-device

Thread state is `localStorage` key `aimee-codes-pwa-v1` (`pwa/index.html:152-163`). Parse failures become `[]`. Nothing is posted.

**Send** (`pwa/index.html:178-193`):

1. Trim `#draft`. Empty text is a no-op.
2. Append `{ role: "user", agent, text, ts }`.
3. Append a canned `{ role: "aimee", agent, text, ts }` whose body is `PWA shell captured that under :{agent}. Wire aimee_* crates next for a live run.`
4. Persist, clear the textarea, re-render, scroll the last article into view.

Compose is a fixed footer (`pwa/index.html:147-150`). `#draft` is `maxlength="4000"`. Send is the lime button or `Ctrl`/`⌘`+Enter (`pwa/index.html:203-206`). User bubbles get a magenta left border; canned replies get rose (`pwa/index.html:92-93`).

This is not a conversation in `aimee_repo` (`conversations` table). Clearing site data drops the thread. There is no sync, no pathway checkpoint, and no `aimee conversation` resume.

## Install and wallet (header)

Two header actions (`pwa/index.html:127-130`):

| Control | What it does |
|---|---|
| `#install` **Install app** | Hidden until `beforeinstallprompt`. Clicking prompts the stored event (`pwa/index.html:211-222`). |
| `#wallet` **Wallet soon** | `alert("Wallet login is the WEB3 slice — HITL spend stays off this shell.")` (`pwa/index.html:207-209`). Title: `Wallet login lands with WEB3 auth`. |

Install does **not** ship the `aimee` binary. Wallet does **not** connect a chain account. Full install/cache behavior: [PWA delivery](pwa-delivery.md). Wallet identity: [Wallet](../web3/wallet.md).

`#sw-status` reports worker registration (`pwa/index.html:224-232`): registered, failed (serve over `http://localhost`), or not supported.

## Current limitations

Documented in the shell and README. Do not treat the PWA as a live agent.

| Limitation | Evidence |
|---|---|
| CLI is source of truth | `pwa/README.md:3`, `pwa/index.html:139-141`, `AIMEE.md:201-203` |
| No wired agent API | Canned reply only (`pwa/index.html:183-188`). No `fetch` to `aimee_*`, no gRPC, no `services_url`. |
| Drafts are device-local | `localStorage` `aimee-codes-pwa-v1` (`pwa/index.html:152-163`) |
| Agent chips are cosmetic | They set a string on local messages (`pwa/index.html:195-202`). They do not call Sage / Muse / Aimee. |
| Wallet is a stub | `#wallet` alerts only (`pwa/index.html:207-209`). No SIWE, no ICP identity, no connect. |
| Static shell only | One `index.html` with inline CSS/JS. No bundler. |
| Offline is the cached shell | `pwa/sw.js` cache-first. Changing HTML without bumping `CACHE` leaves stale clients — see [PWA delivery](pwa-delivery.md). |
| Not in Nix | `flake.nix` builds the `aimee` CLI, not `pwa/`. |

Until an API exists in the tree, this page will not describe one.

## Best practices

- Serve from `pwa/` on port **4173** so docs, README, and local muscle memory match (`pwa/README.md:8-9`).
- Keep drafts on-device. Do not add a write to a remote host from this shell until a real agent API exists.
- Do not put seed phrases, private keys, or provider tokens in `#draft` or `localStorage`.
- Change shell UX in `pwa/index.html`. Change cache/install in `pwa/sw.js` + the manifest, and document that on [PWA delivery](pwa-delivery.md).
- Bump `CACHE` in `pwa/sw.js:1` when shell bytes change, or users keep the old HTML.

## Anti-patterns

- Claiming the PWA runs Sage / Muse / Aimee.
- Inventing wallet connect, SIWE, or a canister call from this page.
- Opening `file://` and treating a failed worker as a product bug.
- Serving the repo root and wondering why `./sw.js` 404s.
- Putting API keys in the static tree or in the canned reply.

## Verify

```bash
# From the product checkout
cd pwa
python3 -m http.server 4173
# Browser: http://localhost:4173
# Expect: lede + “Service worker: registered — installable offline shell.”
# Click :muse / :sage — placeholder becomes :muse … / :sage …
# Send a line — two bubbles appear; canned reply mentions the selected agent.
# Reload — the thread is still there (localStorage aimee-codes-pwa-v1).
# Wallet soon — alert only; no popup provider, no signature request.
```

There is no PWA unit test in the workspace. Do not claim CI covers this shell.

## Related

- [PWA delivery](pwa-delivery.md) — manifest, cache-first worker, install prompt.
- [Wallet](../web3/wallet.md) — stub beside provider auth; spend stays HITL.
- [Anda / KIP](../web3/anda.md) — hash-chained **CLI** session pathways, not this shell.
- [TUI](tui.md) — terminal surface; the CLI remains source of truth.
