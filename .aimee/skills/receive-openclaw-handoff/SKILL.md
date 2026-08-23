---
name: receive-openclaw-handoff
description: >
  Detects implementation handoffs published by OpenClaw (primarily via the
  Telegram channel using the standard prefix) and treats them as high-priority
  coding tasks. Also supports OGP and shared-file fallbacks.
version: 1.2.0
author: Hermes Agent
license: MIT
platforms: [linux, macos, windows]
metadata:
  hermes:
    tags: [Handoff, OpenClaw, Implementation, Telegram, OGP, GitHub]
    related_skills: [github-issues, github-auth, github-pr-workflow]
---

# Receive OpenClaw Handoff (Hermes side)

## When to use
Trigger when any of the following occur:

- A Telegram message arrives that begins with:
  `[HANDOFF TO HERMES – IMPLEMENTATION]`
- An OGP message arrives on the topic `implementation-handoff`
- A new `*-handoff.md` file appears in the watched shared directory
  (`/tmp/hermes-handoffs` on this VPS, or `$HANDOFF_SHARED_DIR`)

## Prerequisites (this VPS)

```bash
source ~/agent-scripts/load-handoff-env.sh
# Telegram: Hermes gateway should have TELEGRAM_BOT_TOKEN (+ allowed users/home channel)
# GitHub: gh auth status  OR  GITHUB_TOKEN in ~/.hermes/.env / handoff.env
```

## Workflow

### 1. Detect & extract
- Identify the handoff by the standard prefix or OGP topic.
- Extract from the message body:
  - Goal
  - Requirements / technical context
  - Success criteria
  - GitHub Issue URL (if present)
  - Labels and Milestone information

Shared-file fallback:

```bash
ls -1t "${HANDOFF_SHARED_DIR:-/tmp/hermes-handoffs}"/*-handoff.md 2>/dev/null | head -5
```

### 2. Acknowledge quickly
Reply **once** in the same Telegram group (or via OGP). Prefer the Hermes bot token for acks so OpenClaw’s publisher is not double-used.

```bash
source ~/agent-scripts/load-handoff-env.sh
export TELEGRAM_BOT_TOKEN="${TELEGRAM_HERMES_BOT_TOKEN:-$TELEGRAM_BOT_TOKEN}"
python3 ~/agent-scripts/publish_telegram.py - --style ack <<EOF
✅ <b>Handoff received</b>
Starting implementation.
Tracking: ${ISSUE_URL:-no issue created}
EOF
```

Do **not** re-send the full handoff brief. OpenClaw already published the package.

Or via OGP back to OpenClaw:

```bash
# OpenClaw peer id as seen from Hermes
OPENCLAW_PEER_ID="${OPENCLAW_PEER_ID:-302a300506032b65700321007d012ded}"
ogp --for hermes federation agent "$OPENCLAW_PEER_ID" implementation-handoff \
  "ACK: received handoff. Tracking: ${ISSUE_URL:-no issue}"
```

### 3. Execute the work
- Treat the handoff as high priority.
- Follow the requirements and technical context in the brief.
- Work in the repositories / paths mentioned.
- If a GitHub issue exists, post progress comments at meaningful milestones.

```bash
# Example progress comment
gh issue comment "$ISSUE_URL" --body "In progress: scaffolding module X"
```

### 4. Close the loop
When finished (or blocked):
- Update the GitHub issue (comment + state change if appropriate)
- Post a final status message back to the Telegram channel, e.g.:

```
[HANDOFF COMPLETE]
Issue: {{url}}
Status: done | blocked
Summary: {{one-sentence result}}
PR/Commits: {{links if any}}
```

```bash
export TELEGRAM_BOT_TOKEN="${TELEGRAM_HERMES_BOT_TOKEN:-$TELEGRAM_BOT_TOKEN}"
python3 ~/agent-scripts/publish_telegram.py - --style ack <<EOF
🏁 <b>HANDOFF COMPLETE</b>
Issue: ${ISSUE_URL:-none}
Status: done
Summary: …
PR/Commits: …
EOF
```

## Rules
- Prioritise structured handoffs that carry the standard prefix.
- If the brief is ambiguous, ask for clarification in the channel before doing large amounts of work.
- Keep the GitHub issue updated so both sides share a single source of truth.
- Do not require the user to re-explain context that is already present in the handoff package.
