# Skills, commands, and templates

Three extension mechanisms, three different jobs. Knowing which to reach for keeps customization clean.

## The distinction

| | Skill | Command | Template |
|---|---|---|---|
| Lives in | `.aimee/skills/<name>/SKILL.md` | `.aimee/commands/<name>.md` | `templates/*.md` (repo) |
| Loaded by | The `skill` tool, on demand | Slash invocation (`/name`) | Prompt assembly |
| Purpose | Teach *how* to do a task type | Package a *workflow* as one verb | Shape loop behavior |
| Author | You, per project or globally | You, per project | Aimee developers |

## Skills

A skill is a folder with a `SKILL.md`: instructions, conventions, and pitfalls for a recurring task type. The agent loads it when relevant. Built-ins shipped to `.aimee/skills/`:

`create-agent`, `create-command`, `create-github-issue`, `create-plan`, `debug-cli`, `github-pr-comments`, `post-aimee-feature`, `resolve-conflicts`, `resolve-fixme`, `test-reasoning`, `write-release-notes`

Plus catalog extras that appear when present: `execute-plan`, `github-pr-description`, `greploop`.

Create your own:

```bash
aimee skill new <name>     # if wired in your build; otherwise mkdir + SKILL.md
```

Structure:

```text
.aimee/skills/deploy-checklist/
└── SKILL.md
```

```markdown
---
name: deploy-checklist
description: Pre-deploy verification checklist for the API service
---

1. Run migrations against staging first.
2. Confirm feature flags default off.
...
```

Global skills live in `~/.aimee/skills/`; project skills win by name collision.

## Commands

Commands are slash-invocable workflow packages — markdown prompts with frontmatter. The full built-in catalog is documented at [Slash commands](../usage/commands.md). Project commands live beside skills:

```text
.aimee/commands/check.md      # shipped example
.aimee/commands/fixme.md      # shipped example
```

The ZSH dispatcher validates command names before execution; unknown names are rejected cleanly.

## Templates

Templates are the prompt partials the loop itself assembles — system frames, tool-use examples, reminders. They are production prompt surfaces: edit only when the task is changing agent/prompt behavior (house rule). Notable ones and their effects are listed in [the loop](loop.md).

## Which one for your problem

* Your agent keeps getting a task-type wrong → **skill**.
* You repeat a five-step workflow weekly → **command**.
* You're changing how every session behaves → **template** (and know why).

## See also

* [Slash commands](../usage/commands.md)
* [Tools: how agents touch your system](tools-overview.md)
* [How Aimee thinks: the loop](loop.md)

<!-- sources: AIMEE.md §12, templates/, .aimee/skills listing, commands/check.md -->
