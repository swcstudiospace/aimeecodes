# Kanban CLI: `--board` placement

## Correct

```bash
hermes kanban --board eng create "title" --triage \
  --idempotency-key "linear:SPE-8" --json --body "…"

hermes kanban --board eng specify <id> --json
hermes kanban --board eng list --json
```

`--board` is a **global** option on `hermes kanban`, before the subcommand.

## Wrong

```bash
hermes kanban create "title" --board eng …
# → unrecognized arguments: --board eng
```

## Agency dual-write

`/root/src/repos/ai-agency/tools/linear_tools.ensure_kanban_card` must use:

`hermes kanban --board eng create … --idempotency-key linear:<identifier>`

Config board slug: `~/.config/hermes-linear/config.yaml` → `hermes.board` (default `eng`).

## Port note

On some hosts `127.0.0.1:8788` is **grok-hermes-connector**, not Linear ensure-issue. Prefer GraphQL + this CLI when `/v1/health` is not the Linear connector.