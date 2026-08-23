# HITL spend vault → Meta / TikTok LIVE

## Flow

```text
attach_funding_source(kind=bank|crypto, …)
        ↓
request_spend_approval(amount_usd, channel, purpose, campaign_draft_id, …)
        ↓  returns approval_id + human_confirm_code (once)
        ↓  Linear [Spend HITL] issue via agency_track
HUMAN: confirm_spend_approval(approval_id, code, human_ack)
        ↓  human_ack must include phrase "I authorize"
        ↓  returns spend_token (24h)
meta_launch_campaign / tiktok_launch_campaign(
    draft_id, approval_id, spend_token)
```

## Funding metadata only

- **bank:** `institution` + `last4` + optional `daily_cap_usd`
- **crypto:** `chain` + public `address` (reject seeds / private keys)

Vault file default: `tmp/spend_vault.json` (mode 600). Override `AGENCY_SPEND_VAULT`.

## Guardrails (`tools/guardrails.py`)

- Block agent calls to `confirm_spend_approval` at L2
- Block `meta_launch_campaign` / `tiktok_launch_campaign` unless both `approval_id` and `spend_token` present
- Vault still re-verifies token, channel, amount ≤ max_total, expiry

## Drafts without platform creds

- Local draft JSON under `tmp/ad_drafts/`
- After HITL, launch may return `APPROVED_STUB_LIVE` until META_/TIKTOK_ tokens set — gate is still correct

## MCP helpers

- `attach_agency_funding_source`
- `request_ad_spend_approval` (returns code to surface to human — do not auto-confirm)
- Never expose HITL codes in group chats if avoidable; prefer mode-600 sidecar files from lifecycle runs

## Not in scope

Supplier payments, card charges, wires, bulk POs — separate human process; do not extend this vault to pay factories without explicit product decision.
