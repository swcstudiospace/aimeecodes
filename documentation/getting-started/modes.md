# Three modes

Every interaction with the flock runs in one of three modes. The mode decides which agent answers and — critically — what that agent is allowed to write.

## The modes

| Mode | Agent | Alias | Write scope |
|---|---|---|---|
| Ask | Sage | `:ask` | Nothing — read-only research |
| Plan | Muse | `:plan` | Checkbox plan files under `plans/` |
| Act | Aimee | `:act` | Full toolset: edits, patches, shell |

## Choosing a mode

**Ask** when you want understanding without risk. Architecture questions, "why does this exist", review feedback on a diff. Nothing in your tree changes.

**Plan** when the work needs a shape before it needs hands. Muse produces a reviewable artifact; you approve or edit it before anyone touches code.

**Act** when the plan exists and execution should start. Aimee implements, verifies with commands, and reports evidence.

## Switching modes

In the ZSH plugin, address the agent directly:

```zsh
: sage explain the migration strategy
: muse plan applying the migration
: aimee apply plans/2026-08-23-migration.md
```

From the CLI, select the agent explicitly:

```bash
aimee --agent sage "review the auth module for injection risks"
aimee -p "summarize open risks" --agent muse
```

Inside an interactive session you can switch agents mid-conversation; each agent keeps its own write permissions no matter how the session started.

## Why the split matters

The mode system encodes a safety boundary into everyday use: reading is free, planning produces artifacts you can audit, and only one role can mutate your tree — and even that role works through permission-gated tools (see [Security model](../operations/security.md)).

## See also

* [The flock: Sage, Muse, Aimee](the-flock.md)
* [Autonomy levels and guardrails](../concepts/autonomy.md)
* [Your first flock session](first-session.md)

<!-- sources: AIMEE.md §3, README.md -->
