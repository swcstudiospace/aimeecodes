# plan

`plan` writes a Muse plan file under `plans/`. Input type: `PlanCreate` (`crates/aimee_domain/src/tools/catalog.rs:672-684`). Description source: `descriptions/plan_create.md`.

## Parameters

| Parameter | Type | Required | Notes |
|---|---|---|---|
| `plan_name` | string | yes | Used in the filename |
| `version` | string | yes | e.g. `"v1"`, `"v2"`, `"1.0"` |
| `content` | string | yes | Complete markdown plan body |

## Example

```json
{
  "name": "plan",
  "arguments": {
    "plan_name": "2026-08-23-retry-budget",
    "version": "v1",
    "content": "# Retry budget\n\n- [ ] Add `[retry]` defaults to embedded config\n- [ ] Snapshot test the merged table\n"
  }
}
```

## Behavior

- Plans land in the project's `plans/` directory as checkbox files — this is how [Muse](../../flock.md) delivers work that [Aimee](../../flock.md) later implements ("implement the plan in plans/…").
- Muse's write surface is limited to this tool: it cannot edit source, only create plans.
- `plans/` is treated as historical unless a task explicitly cites a plan — plans are inputs to implementation, not standing policy.

## Errors

Invalid name/version or filesystem failures surface as tool errors.

## Permissions

No permission gate (`catalog.rs:1011`) — plan files are additive documentation artifacts.

## Related

- [Tool catalog](catalog.md)
- [The flock](../../flock.md) — Sage / Muse / Aimee division of labor
- [How to use](../../howto.md) — research → plan → implement workflow
