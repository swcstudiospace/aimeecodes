# Wallet

What exists today: a **PWA header stub** next to real **LLM provider auth**. There is no wallet connect, no SIWE, no Internet Identity, and no canister login in this tree. Payments and spend stay HITL (`AIMEE.md:314`, `README.md:328`).

Session identity (who is logged into OpenAI / Anthropic / …) is not chain identity. AuthN is not AuthZ (`AGENTS.md:198`).

Hash-chained conversation checkpoints are **not** a wallet. They live on [Anda / KIP](anda.md).

## What the tree actually has

| Surface | Behavior | Not |
|---|---|---|
| PWA `#wallet` | Button labelled **Wallet soon**. Click → `alert` only (`pwa/index.html:129`, `pwa/index.html:207-209`) | No `window.ethereum`, no WalletConnect, no signature request |
| CLI `aimee provider login` | Authenticates an **API provider** (`crates/aimee_main/src/cli.rs:135-136`, `crates/aimee_main/src/cli.rs:979-987`) | Not a chain wallet |
| `.credentials.json` | Provider API keys / OAuth tokens under the config base (`crates/aimee_domain/src/env.rs:176-180`) | No seed phrases, no private keys, no principals |
| `aimee_anda_icp` | Eternal **pathway** receipts. Non-local modes return `IcpError::NotConfigured` (`crates/aimee_anda_icp/src/lib.rs:1-4`, `crates/aimee_anda_icp/src/store.rs:45-79`) | Not ICP identity / Internet Identity |
| `/goal` HITL probes | Five human answers before a loop is active (`crates/aimee_domain/src/loop_autonomy.rs:19-27`) | Not a payment rail |

Product copy that matches this page:

- `README.md:93` — “wallet-aware PWA shell (spend stays HITL)”
- `README.md:328` — “PWA wallet login sits beside provider auth. Payments and spend stay HITL.”
- `AIMEE.md:314` — same sentence

“Wallet-aware” here means the shell **shows** a wallet control. It does not mean a wired adapter.

## PWA stub

Header actions (`pwa/index.html:127-130`):

```127:130:pwa/index.html
    <div class="actions">
      <button id="install" type="button" hidden>Install app</button>
      <button id="wallet" type="button" title="Wallet login lands with WEB3 auth">Wallet soon</button>
    </div>
```

Click handler (`pwa/index.html:207-209`):

```207:209:pwa/index.html
    document.getElementById("wallet").addEventListener("click", () => {
      alert("Wallet login is the WEB3 slice — HITL spend stays off this shell.");
    });
```

That is the entire client wallet UI. There is:

- no connect / disconnect
- no chain-id or network check
- no typed-data or SIWE message
- no address display
- no RPC URL (and therefore nothing to leak in an error)

Compose placeholder text (`:aimee ship the wallet login…`, `pwa/index.html:148`) is copy, not an API. Specialist prompts name SIWE / ICP identity as **future lane work** (`crates/aimee_repo/src/agents/fe-web3.md:4`, `crates/aimee_repo/src/agents/aimee.md:50`). Those strings are not implementations.

House rules if a wallet UI is added later (`crates/aimee_repo/src/agents/fe-web3.md:65-69`, `crates/aimee_repo/src/agents/fe-ui.md:69`):

- Never handle seed phrases or raw private keys in the client
- Session vs chain identity stay distinct
- Fail closed on disconnect, wrong network, and signature rejection
- User-visible errors must not leak RPC URLs with secrets
- Prefer adapters already in the tree — today that set is **empty** for SIWE / wallet connect

Do not invent an SDK to fill the gap.

## Provider auth (beside the stub)

Real login is **provider** auth for model APIs. It is not WEB3.

```bash
aimee provider login            # interactive menu
aimee provider login openai     # named provider
aimee provider logout
aimee provider list
```

`ProviderCommand::Login` / `Logout` / `List` (`crates/aimee_main/src/cli.rs:979-1003`). First CLI run walks provider login when no credentials are stored (`AIMEE.md:431`).

Stored credential shapes (`crates/aimee_domain/src/auth/credentials.rs:10-58`, `crates/aimee_domain/src/auth/credentials.rs:87-100`):

| `AuthDetails` | Meaning |
|---|---|
| `ApiKey` | Static provider key |
| `OAuth` / `OAuthWithApiKey` | OAuth tokens (+ optional key) |
| `AwsProfile` | Named AWS profile (Bedrock) |
| `GoogleAdc` | Google application-default token |

Path: `{base_path}/.credentials.json` (`crates/aimee_domain/src/env.rs:176-180`). Default base is `~/.aimee`. Do not put that file in git (`AIMEE.md:229`, `AIMEE.md:412`). Strategies live in `crates/aimee_infra/src/auth/strategy.rs` (API key, device/OAuth HTTP). None of them verify an Ethereum signature or an IC principal.

The PWA cannot read this file. Browser drafts and CLI credentials do not share a session.

## ICP in this repo is durability, not identity

`aimee_anda_icp` exports hash-chained **conversation** checkpoints (`crates/aimee_anda_icp/src/lib.rs:1-4`). Default mode writes content-addressed **local receipts**. Modes `ic_oss`, `canister`, and `s3` exist on the enum and return `IcpError::NotConfigured` until a client is wired (`crates/aimee_anda_icp/src/store.rs:45-79`).

The app hook already falls back to local receipts and warns when those modes are selected (`crates/aimee_app/src/anda_pathway.rs:83-91`). That is not a wallet, not Internet Identity, and not a spend path.

Inspect pathways from the CLI (`crates/aimee_main/src/cli.rs:933-964`):

```bash
aimee conversation pathway <id> list
aimee conversation pathway <id> show <seq>
aimee conversation pathway <id> rollback <seq>
```

Rollback restores **chat context only**, not workspace files and not a chain account. Full config and checkpoint model: [Anda / KIP](anda.md).

## Payments and spend stay HITL

No payment, invoice, token transfer, allowance, or on-chain spend API exists in `aimeecodes`. Billing URL `https://app.aimeecodes.dev/app/billing` is product copy for plan upgrades (`AIMEE.md:34`) — it is not called from the PWA.

Two different words named “spend” in the tree:

| Meaning | Where | WEB3? |
|---|---|---|
| Policy: do not auto-pay / auto-sign | PWA alert (`pwa/index.html:208`); README / `AIMEE.md:314` | Intent only — no rail to disable |
| LLM token **cost** in the TUI | Accumulated session cost on the prompt (`crates/aimee_main/src/ui.rs:300-303`) | No. Provider usage, not crypto |
| Reasoning token budget | `spend thinking` in model config (`crates/aimee_config/src/reasoning.rs:19`) | No |

Human-in-the-loop for agent **goals** is real and separate. `/goal` requires exactly five answered probes (`crates/aimee_domain/src/loop_autonomy.rs:10-27`, `crates/aimee_domain/src/goal.rs:327-337`):

1. What does done look like (observable outcome)?
2. How will we verify (tests, commands, evidence)?
3. What must not change (boundaries)?
4. Who is the human owner, and when should we stop and ask?
5. What Linear issue / GitHub PR / related work should we log against?

Fixture text uses “stop on spend” as a **probe answer**, not as a wallet hook (`crates/aimee_domain/src/loop_autonomy.rs:369`). Goal continuation tests mention “HITL spend stays off” as a subgoal string (`crates/aimee_domain/src/goal.rs:494-499`). Those tests do not move funds.

Until a spend API exists, a human must approve any payment **outside** this product. The PWA will not grow a one-click pay button from this page.

## What is not in the tree

A search of `aimeecodes` finds **no** SIWE message, no `siwe` crate, no WalletConnect / MetaMask / `window.ethereum` adapter, no Internet Identity login, and no IC `Principal` session type. Do not document those as shipped.

If you need a wallet later:

1. Search again. If it is not there, it does not exist (`AGENTS.md:31`).
2. Keep provider auth and chain identity on separate types.
3. Fail closed. Do not weaken CORS / CSP to make a connect call work.
4. Keep spend HITL. Do not auto-sign.

## Best practices

- Treat **Wallet soon** as a placeholder. Do not demo it as a connect flow.
- Log into models with `aimee provider login`. That is the only login that works.
- Keep `.credentials.json` off git and out of the PWA origin.
- Put pathway / ICP durability questions on [Anda / KIP](anda.md), not here.
- Never paste a seed phrase into `#draft` or into provider login.

## Anti-patterns

- Writing “connect wallet” docs against this tree.
- Equating `aimee provider login` with SIWE or Internet Identity.
- Equating `aimee_anda_icp` with a user wallet.
- Caching a future `/wallet` GET in `pwa/sw.js` (cache-first on every GET — see [PWA delivery](../surfaces/pwa-delivery.md)).
- Logging RPC URLs, API keys, or raw signatures.

## Verify

```bash
# From the product checkout
cd pwa && python3 -m http.server 4173
# Browser: http://localhost:4173 → Wallet soon → alert only.

# Provider auth is CLI-only
aimee provider list
# Expect: built-in model providers. No wallet / SIWE / ICP identity row.
```

There is no wallet unit test. Goal HITL tests live next to `GoalProbeSet` (`crates/aimee_domain/src/loop_autonomy.rs:351-376`).

## Related

- [PWA](../surfaces/pwa.md) — installable shell; drafts on-device.
- [PWA delivery](../surfaces/pwa-delivery.md) — do not cache a future wallet route.
- [Anda / KIP](anda.md) — session pathways and ICP durability backends.
- [Providers](../providers.md) — `aimee provider login` and credential shapes.
