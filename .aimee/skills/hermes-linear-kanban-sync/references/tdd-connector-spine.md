# TDD spine for connector + board packages

## Markers

- `live` — real Linear API (`LINEAR_API_KEY`)
- `hermes` — real `hermes kanban` CLI / board DB  
Default: `pytest -m "not live and not hermes"`.

## Vertical slices (order)

1. Package version + discovery fixture (no secrets)
2. Pure StatusMap against fixture (all SPE cases)
3. Idempotency keys + sync comment stamps
4. Config: team/board pins; key only from env
5. SyncStore: unique links, cursors, dead letters
6. LinearClient via `httpx.MockTransport`; optional live viewer
7. HermesKanban injectable runner + RO `task_events`
8. SyncEngine fakes: ensure once, status, skip stamped comments
9. Poller advances cursor; dead-letter on apply errors
10. Live e2e: eng card → SPE issue → Done → Canceled; title `[hermes-sync-test]`

## Board package

Classifier unit tests before plugin hooks: deny, explicit track, tools, keywords, too_short.

## Live hygiene

Cancel Linear test issues; archive kanban test cards in finalizers. Never commit keys.