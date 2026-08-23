# skill

`skill` loads one skill by name, injecting its instructions and workflow into the conversation. Input type: `SkillFetch` (`crates/aimee_domain/src/tools/catalog.rs:686-691`). Description source: `descriptions/skill_fetch.md`.

## Parameters

| Parameter | Type | Required | Notes |
|---|---|---|---|
| `name` | string | yes | Skill name, e.g. `"pdf"`, `"code_review"` |

## Example

```json
{
  "name": "skill",
  "arguments": { "name": "resolve-conflicts" }
}
```

## Behavior

- Skills provide domain-specific knowledge, workflows, and best practices — the agent should load one when a task matches it rather than improvising.
- Only skills **listed in the available-skills section** of the current context may be invoked; the contract forbids invoking an already-active skill twice.
- Skill sources resolve in order: project `.aimee/skills/<name>/SKILL.md`, then global `<config-base>/skills/<name>/SKILL.md`. See [Skills and commands](../../skills.md) for the built-in list and authoring rules.

## Errors

Unknown skill name (not in the available list) is the expected failure.

## Permissions

No permission gate (`catalog.rs:1012`) — loading instructions has no side effects.

## Related

- [Tool catalog](catalog.md)
- [Skills and commands](../../skills.md)
- [task](task.md) — delegate whole jobs, not just load knowledge
