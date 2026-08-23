---
name: enterprise-engineering
description: "Use when doing structured eng work with Linear+Kanban/Scrum."
version: 0.2.0
author: Hermes Agent
license: MIT
metadata:
  hermes:
    tags: [engineering, linear, kanban, scrum, autopilot, process]
    related_skills: [hermes-agent, plan, test-driven-development]
---

# Enterprise Engineering

## Overview

Structured engineering on **Linear** (SPE issues/cycles) + **Hermes Kanban** board `eng`
(execution). Autopilot may already have filed a triage card + Linear issue for the session
(`idempotency_key sess:<session_id>`). Do not invent ticket IDs; confirm via connector or board.

## When to Use

- Multi-step implementation, bugs, features, PRs
- Worker sessions spawned from eng board cards
- Sprint/standup language from the user

## Autopilot graph etiquette

1. **Parent cards** may be orchestrator-owned after decompose — implement children, not the parent.
2. **Idempotency** — never create a second card for the same `session_id`; reuse `sess:…` keys.
3. **First action on a worker:** `kanban_show` (or tools) for title/body/links/comments.
4. **Progress** — comment at milestones (design choice, PR URL, test results). Prefer connector
   mirror / stamped comments when available.
5. **Linear** — SPE identifiers may appear in body/comments; update status via board completion
   so the connector can project Done (do not claim Linear Done without evidence).

## Lifecycle

1. Clarify goal (if not already specified on the card).
2. Implement in worktree when coding (`workspace worktree` tasks).
3. Tests before complete.
4. Complete your card with a short result summary; block with a clear reason if stuck.
5. Reviewer cards: review only — request changes via comment + block, or complete.

## Definition of Done (worker)

Workers **cannot** complete without DoD metadata when gates are enabled
(`autopilot.gates_enabled: true`).

### Implementer
Pass on complete:
```json
{"tests_pass": true, "summary": "what changed + test evidence", "role": "implementer"}
```
Example tool args: `metadata` JSON string or fields `tests_pass` + `result`/`summary`.

### Reviewer
```json
{"review_pass": true, "summary": "LGTM / requested changes resolved", "role": "reviewer"}
```

### Linear Done
Connector only projects **Done** when `dod_pass` is set (via complete metadata or
`POST /v1/mark-dod`). Completing without DoD leaves Linear open and posts a sync comment.

- [ ] Change implemented or explicitly blocked
- [ ] Tests run (`tests_pass: true`) or justified block
- [ ] Non-empty summary on the card
- [ ] No secrets in card body/comments

## Status

Autopilot v0.2 — intake/specify/decompose pipeline in eng-board package.
