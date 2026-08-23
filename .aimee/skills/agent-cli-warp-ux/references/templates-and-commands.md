# Aimee Codes: internal templates vs user commands

## Two layers (do not conflate)

| Layer | Path | Who uses it | Appears in `:` / `/` palette? |
|-------|------|-------------|-------------------------------|
| **Internal Handlebars** | `templates/aimee-*.md` | Agent system (commit msg, shell suggest, summary frames, custom agent body) | **No** |
| **User commands** | `commands/*.md` | Operator types `:name` or `/name` | **Yes** |

Internal templates are registered via `aimee_embed` / `TemplateService` as Handlebars partials (`{{> aimee-partial-….md }}`).

User commands are YAML frontmatter + Markdown/XML body with `{{parameters}}`, embedded in `CommandLoaderService::init_default` via `include_str!`.

## Palette UX

- Empty bol `:` or `/` → rustyline Conditional → `Cmd::Complete` → full menu
- Descriptions for user templates: prefix `Template · …` or `⚙ …`
- Filter: type `tpl-` to see prompt templates; enterprise packs are bare names (`review`, `ship`)

## Custom command parse (critical bugfix 2026-08)

**Wrong (old):** `Template::new(extract_command_value(cmd, rest))` preferred **user args** as the template string when rest was non-empty → `:tpl-debug login 500` rendered template `"login 500"` and dropped the XML body.

**Right:**

```rust
// prompt body = AimeeCommand.value (from Command.prompt)
// parameters = rest tokens joined into {{parameters}}
let template = Template::new(command.value.clone().unwrap_or_default());
UserCommand::new(name, template, parameters)
```

Render path (`user_prompt.rs`):

```rust
template_engine.render_template(
    command.template.clone(),
    &json!({"parameters": command.parameters.join(" ")}),
)?;
```

## User-facing packs on this tree

### Enterprise (ops)

`review`, `harden`, `incident`, `ship`, `oncall`, `rfc`, `adr`, `migrate`, `perf`, `slo`, `threat-model`, `compliance`, `runbook`, `postmortem`, `api-contract`, `k8s-review`, `cost`, `data-privacy`, `test-plan`, `swarm`, `github-pr-description`

### Prompt templates (`tpl-*`)

`tpl-explain`, `tpl-debug`, `tpl-implement`, `tpl-refactor`, `tpl-tdd`, `tpl-pr`, `tpl-review-diff`, `tpl-design`, `tpl-migrate-plan`, `tpl-observability`, `tpl-security-pass`, `tpl-docs-inline`, `tpl-benchmark`, `tpl-release-notes`, `tpl-handoff`

Bodies use XML tags (`<role>`, `<objective>`, `<process>`, …). **Do not** put `{{> partial }}` in command bodies unless the command render engine has those partials registered — prefer inlined policy snippets.

## Internal template hygiene

Useful partials (agent system):

- `aimee-partial-swarm-policy.md` — parallel specialists, no nested orchestrators
- `aimee-partial-verification.md` — evidence before done
- `aimee-partial-security-baseline.md` — secrets / least privilege

Include from `aimee-custom-agent-template.md` via Handlebars partials.

## Adding a new user template

1. `commands/tpl-<name>.md` with frontmatter `name` + `description`
2. XML body + `{{parameters}}`
3. `include_str!` entry in `aimee_services` `init_default`
4. Test: `cargo test -p aimee_services --lib -- test_init_default_contains_builtin_commands`
5. Rebuild binary; type `:` and filter `tpl-`

## Verification

```bash
cd /root/src/repos/aimeecodes
cargo test -p aimee_services --lib -- test_init_default_contains_builtin_commands
# Live: : opens menu; :tpl-debug foo keeps full prompt body
```
