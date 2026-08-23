# Wallet

WEB3 identity for Aimee Codes lives beside provider authentication: a wallet login path in the PWA, with payments and spend kept human-in-the-loop by design.

## What exists today

* **Wallet login** is offered in the PWA alongside provider auth — connect a wallet to identify yourself to the product surface.
* **Payments and spend are HITL.** Nothing in the flock moves money or commits you to spend without an explicit human approval step. This is a design invariant, not a toggle.

## How it relates to the rest

| Concern | Mechanism |
|---|---|
| Model access | Provider credentials (see [Auth](auth.md)) |
| Product identity | Wallet login (this page) |
| Session durability | Anda pathways (see [Anda / KIP](anda-kip.md)) |
| Billing | Hosted plan at app.aimeecodes.dev/app/billing |

Wallet login does not replace provider credentials — the CLI/TUI still authenticate to model providers through `.credentials.json`. The wallet is the WEB3-facing identity layer of the product.

## Deliberate boundaries

The wallet integration is intentionally narrow today. There is no autonomous transaction execution from agent loops, no on-chain writes issued by tools, and no spend authority delegated to specialists. If a workflow needs payment (hosted plan upgrades, service credits), it routes through the human.

## See also

* [Anda / KIP pathways](anda-kip.md)
* [Authentication and credentials](auth.md)
* [Cost awareness](../operations/cost.md)

<!-- sources: AIMEE.md §10 -->
