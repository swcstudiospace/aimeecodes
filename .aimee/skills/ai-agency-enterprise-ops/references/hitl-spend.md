# HITL spend → live ads

## Sequence

1. `attach_funding_source(kind="bank"|"crypto", …)` — metadata only
2. `request_spend_approval(amount_usd, channel, purpose, campaign_draft_id=…)`
3. Surface `approval_id` + `human_confirm_code` to operator (or read mode-600 `lifecycle_*_HITL_CODES.json`)
4. Human: `confirm_spend_approval(id, code, human_ack)` — ack must include **"I authorize"**
5. `meta_launch_campaign` / `tiktok_launch_campaign(draft_id, approval_id, spend_token)`

## Rules

- Agents blocked from step 4 (`guardrails.py`)
- Launch blocked without approval_id + spend_token
- Bank: institution + last4; crypto: chain + public address — never seeds/keys
- Supplier PO/wire is **out of scope** for this vault

## Module

`/root/src/repos/ai-agency/tools/spend_vault.py`
