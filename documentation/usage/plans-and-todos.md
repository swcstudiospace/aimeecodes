# Plans and todos

Two mechanisms keep long work on rails: **plans** (reviewable checkbox files under `plans/`) and **todos** (in-session task tracking the agent maintains while it works).

## Plans: Muse's artifacts

Muse writes plans as markdown files with checkboxes:

```zsh
: muse plan adding rate limiting to the public endpoints
```

produces something like `plans/2026-08-23-rate-limiting.md`:

```markdown
# Rate limiting for public endpoints

- [ ] Add limiter middleware with per-IP buckets
- [ ] Emit 429 with Retry-After headers
- [ ] Cover burst behavior in tests
```

Plans are ordinary files — edit them, reorder items, strike what you disagree with. The plan is the contract: when you hand it to Aimee, it works through the checkboxes and can report progress against them.

Execute a plan:

```zsh
: aimee implement plans/2026-08-23-rate-limiting.md
```

The `plan` tool exists in the agent's catalog for creating plan files programmatically; Muse uses the same format. The `create-plan` built-in skill and the `tpl-design` command produce related planning artifacts.

## Todos: tracking inside a run

While Aimee executes, it tracks its own task list through the `todo_write` / `todo_read` tools: short-lived items like "patch importer", "run tests", "update changelog". You'll see them tick over in the session output.

A reminder prompt (`templates/aimee-pending-todos-reminder.md`) is injected into the loop when todos remain pending, so unfinished work resurfaces instead of being quietly dropped.

## How they fit together

| | Plan | Todo |
|---|---|---|
| Created by | Muse (or you) | Aimee during execution |
| Lives | `plans/*.md` — your repo, reviewable in git | Session state |
| Lifetime | Until you delete or archive it | The current run |
| Purpose | Agreement on *what* | Tracking *progress* |

Typical flow: research with Sage → plan with Muse → edit the plan file → execute with Aimee → watch todos complete → keep the plan as documentation of what shipped.

## See also

* [Slash commands](commands.md)
* [Everyday workflows](workflows.md)
* [Autonomy levels and guardrails](../concepts/autonomy.md)

<!-- sources: AIMEE.md §3,§7,§12, templates/aimee-pending-todos-reminder.md -->
