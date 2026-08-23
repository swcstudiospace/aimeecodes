# Skills and commands

Skills, slash-commands, and custom agents are how Aimee loads specialized workflows without stuffing every procedure into the system prompt. This page maps **where files live**, **which ones exist on disk**, and **how the `skill` tool loads them**. It does not dump prompt bodies.

Source of truth: `AIMEE.md` §12, `crates/aimee_repo/src/skill.rs`, `crates/aimee_repo/src/agent.rs`, `crates/aimee_services/src/command.rs`, `crates/aimee_domain/src/env.rs`.

## Project vs global paths

`Environment` resolves every location (`crates/aimee_domain/src/env.rs:90-157`). `base_path` is the config root (typically `~/.aimee` for new installs; see `AIMEE.md:206-216`). `cwd` is the workspace.

| Kind | Project (cwd) | Global (`base_path`) | Other |
|---|---|---|---|
| Skills | `.aimee/skills/<name>/SKILL.md` | `<base_path>/skills/<name>/SKILL.md` | `~/.agents/skills/<name>/SKILL.md` |
| Commands | `.aimee/commands/<name>.md` | `<base_path>/commands/<name>.md` | Built-in from `commands/*.md` (embedded) |
| Agents | `.aimee/agents/<id>.md` | `<base_path>/agents/<id>.md` | Built-in from `crates/aimee_repo/src/agents/*.md` |
| Policy | `AGENTS.md` | `<base_path>/AGENTS.md` | Nested `AGENTS.md` wins on conflict |
| MCP | `.mcp.json` | `<base_path>/.mcp.json` | Project wins (`AIMEE.md:231`) |

Path helpers:

- Skills: `local_skills_path` → `cwd/.aimee/skills` (`crates/aimee_domain/src/env.rs:134-137`); `global_skills_path` → `base_path/skills` (`crates/aimee_domain/src/env.rs:122-125`); `agents_skills_path` → `$HOME/.agents/skills` or `None` (`crates/aimee_domain/src/env.rs:127-132`).
- Commands: `command_path_local` → `cwd/.aimee/commands` (`crates/aimee_domain/src/env.rs:144-147`); `command_path` → `base_path/commands` (`crates/aimee_domain/src/env.rs:139-142`).
- Agents: `agent_cwd_path` → `cwd/.aimee/agents` (`crates/aimee_domain/src/env.rs:93-95`); `agent_path` → `base_path/agents` (`crates/aimee_domain/src/env.rs:90-92`).

Missing directories are skipped. They do not fail the load (`crates/aimee_repo/src/skill.rs:35-36`, `crates/aimee_repo/src/agent.rs:32-33`).

## Precedence

Later sources replace earlier ones with the same name.

**Skills** — CWD > `~/.agents/skills` > global custom > built-in (`crates/aimee_repo/src/skill.rs:20-26`, `crates/aimee_repo/src/skill.rs:102-104`).

**Commands** — CWD > global custom > built-in (`crates/aimee_services/src/command.rs:148-165`).

**Agents** — CWD > global custom > built-in (`crates/aimee_repo/src/agent.rs:19-24`, `crates/aimee_repo/src/agent.rs:63-70`).

A project-local `.aimee/skills/create-plan/SKILL.md` therefore shadows the same name from `~/.aimee/skills` or a built-in.

## How the `skill` tool works

`skill` is a first-class catalog tool (`ToolCatalog::Skill`, `crates/aimee_domain/src/tools/catalog.rs:56`).

1. **Schema.** Input is `SkillFetch { name }` — just the skill name (`crates/aimee_domain/src/tools/catalog.rs:686-691`). Description file: `crates/aimee_domain/src/tools/descriptions/skill_fetch.md:1`.
2. **Prompt instructions.** The system prompt tells the model to check `<available_skills>` before acting, call `skill` with `{"name": "…"}`, and follow the returned body. Only listed skills. Do not invoke a skill that is already active. Skills are not CLI commands (`templates/aimee-partial-skill-instructions.md:1-34`).
3. **Execute.** `ToolExecutor` matches `ToolCatalog::Skill` and calls `SkillFetchService::fetch_skill` (`crates/aimee_app/src/tool_executor.rs:321-323`).
4. **Service.** `AimeeSkillFetch` caches `repository.load_skills()` in a `OnceCell`, then finds the skill by name. Unknown names return `Skill '{name}' not found. Please check the available skills list.` (`crates/aimee_services/src/tool_services/skill.rs:26-38`).
5. **Repository.** `AimeeSkillRepository::load_skills` concatenates built-in (embedded), global, `~/.agents/skills`, and project-local, then resolves conflicts and renders `{{global_skills_path}}` / `{{agents_skills_path}}` / `{{local_skills_path}}` in the command body (`crates/aimee_repo/src/skill.rs:78-113`, `crates/aimee_repo/src/skill.rs:229-249`).
6. **On-disk shape.** Each skill is a directory containing `SKILL.md` (YAML front matter `name` + `description`, then the body). Sibling files become `resources` and are **not** included if they are named `SKILL.md` (`crates/aimee_repo/src/skill.rs:148-201`). Incomplete front matter falls back to the directory name (`crates/aimee_repo/src/skill.rs:192-200`).
7. **Policy.** `skill` does not require a permission grant (`crates/aimee_domain/src/tools/catalog.rs:1008-1015`).

Aimee is the agent that ships `skill` in its default tool list (`crates/aimee_repo/src/agents/aimee.md:19`). `fe-qa` also lists it (`crates/aimee_repo/src/agents/fe-qa.md:14`). Sage and Muse do not.

## Built-in skills on disk

Two layers. Do not invent names that are not in one of these lists.

### Embedded (always present)

Loaded from `crates/aimee_repo/src/skills/` via `include_str!` (`crates/aimee_repo/src/skill.rs:48-61`). Tests assert exactly three (`crates/aimee_repo/src/skill.rs:402-412`).

| Name | Path | Use when |
|---|---|---|
| `create-skill` | `crates/aimee_repo/src/skills/create-skill/SKILL.md` | Author or update a skill |
| `execute-plan` | `crates/aimee_repo/src/skills/execute-plan/SKILL.md` | Execute a `plans/{date}-{task}-{version}.md` file |
| `github-pr-description` | `crates/aimee_repo/src/skills/github-pr-description/SKILL.md` | Draft / create a PR with `gh` |

`AIMEE.md:338` mentions `greploop` as present **when it exists** under `.aimee/skills/`. It is **not** in this checkout's `.aimee/skills/` and is **not** embedded. Do not document it as installed.

### Project skills (this repo's `.aimee/skills/`)

On disk today (`AIMEE.md:334-336`):

| Name | Use when (front matter) |
|---|---|
| `create-agent` | Add or edit agents under `.aimee/agents/` |
| `create-command` | Add or edit commands under `.aimee/commands/` |
| `create-github-issue` | File a GitHub issue with `gh`, using the repo's templates |
| `create-plan` | Write a checkbox plan; no product-code edits |
| `debug-cli` | Debug or extend the `aimee` CLI; do not commit |
| `github-pr-comments` | Resolve inline review comments on a PR |
| `post-aimee-feature` | Draft a Twitter/X post for a feature (with video) |
| `resolve-conflicts` | Structured merge-conflict resolution |
| `resolve-fixme` | Find and implement every `FIXME` |
| `test-reasoning` | Check `ReasoningConfig` serialization to providers |
| `write-release-notes` | Generate release notes from a GitHub tag |

Each lives at `.aimee/skills/<name>/SKILL.md`. Invoke with the `skill` tool and that `name`. Do not paste the skill body into docs or into a new prompt.

## Commands

Commands are slash / `:` palette entries. They are **not** skills. A command is a single `.md` file with YAML front matter `name` + `description` and a markdown body (`crates/aimee_services/src/command.rs:226-238`).

### Project commands (`.aimee/commands/`)

Present in this checkout (`AIMEE.md:340-342`):

| File | Name | Role |
|---|---|---|
| `.aimee/commands/check.md` | `check` | Pre-commit: nightly fmt + clippy `--fix`, then `cargo insta test --accept --unreferenced=delete` |
| `.aimee/commands/fixme.md` | `fixme` | Find `FIXME` comments and attempt to fix them |

### Built-in commands (`commands/`, embedded)

`CommandLoaderService::init_default` embeds every file listed below (`crates/aimee_services/src/command.rs:27-127`). These show in the `:` / palette.

**Workflows:** `github-pr-description`, `review`, `harden`, `incident`, `ship`, `oncall`, `rfc`, `adr`, `migrate`, `perf`, `slo`, `threat-model`, `compliance`, `runbook`, `postmortem`, `api-contract`, `k8s-review`, `cost`, `data-privacy`, `test-plan`, `swarm`.

**Prompt templates** (prefixed `tpl-`, also in the palette): `tpl-explain`, `tpl-debug`, `tpl-implement`, `tpl-refactor`, `tpl-tdd`, `tpl-pr`, `tpl-review-diff`, `tpl-design`, `tpl-migrate-plan`, `tpl-observability`, `tpl-security-pass`, `tpl-docs-inline`, `tpl-benchmark`, `tpl-release-notes`, `tpl-handoff`.

Example: `:review` is an enterprise review prompt (`commands/review.md:1-29`). `:tpl-tdd` is the red/green/refactor template (`commands/tpl-tdd.md:1-13`). `:github-pr-description` delegates to the pull-request skill (`commands/github-pr-description.md:1-8`).

## Custom agents

Built-in agent definitions are embedded from `crates/aimee_repo/src/agents/` (`crates/aimee_repo/src/agent.rs:73-98`):

| ID | Role |
|---|---|
| `aimee` | Implement + verify; orchestrator |
| `muse` | Plan only |
| `sage` | Research / review, read-only |
| `fe-ui` `fe-web3` `fe-realtime` `fe-edge` `fe-qa` | Frontend specialists |
| `be-api` `be-web3` `be-data` `be-security` `be-reliability` | Backend specialists |
| `plat-k8s` `plat-cloud` `plat-compliance` `plat-sre` | Platform specialists |

Custom agents are extra `*.md` files in `.aimee/agents/` (project) or `<base_path>/agents/` (global) (`AIMEE.md:58`, `crates/aimee_repo/src/agent.rs:12-30`). This checkout has **no** `.aimee/agents/` directory — only the built-ins.

File shape (from `create-agent`, not a dump of the template): YAML front matter `id`, `title`, `description`, `reasoning`, `tools`, `user_prompt`; markdown body is the agent instructions. Filename is `{agent-id}.md`.

The production prompt wrapper for custom agents is `templates/aimee-custom-agent-template.md`. Edit templates **only** when the task is agent/prompt behavior (`AGENTS.md:876-879`, `AIMEE.md:348`). Do not copy the template into this GitBook.

When `subagents = true` (default in the embedded config), Aimee gets `task` and Sage-as-a-tool is removed. When false, `task` is disabled and `sage` is available instead (`AIMEE.md:262-263`, `crates/aimee_repo/src/agent.rs:165-180`).

## Prompt templates (`templates/`)

These are Handlebars partials the runtime injects. They are **not** skills and **not** commands. On disk (`AIMEE.md:344-346`, plus later additions):

- `aimee-command-generator-prompt.md`
- `aimee-commit-message-prompt.md`
- `aimee-custom-agent-template.md`
- `aimee-doom-loop-reminder.md`
- `aimee-partial-security-baseline.md`
- `aimee-partial-skill-instructions.md`
- `aimee-partial-summary-frame.md`
- `aimee-partial-swarm-policy.md`
- `aimee-partial-system-info.md`
- `aimee-partial-tool-error-reflection.md`
- `aimee-partial-tool-use-example.md`
- `aimee-partial-verification.md`
- `aimee-pending-todos-reminder.md`
- `aimee-system-prompt-title-generation.md`
- `aimee-tool-retry-message.md`

Do not dump these bodies here. Change them only when the task is prompt behavior.

## Authoring without inventing APIs

- New **skill**: directory `.aimee/skills/<name>/SKILL.md` with `name` + `description` front matter. Use the `create-skill` or `create-command` / `create-agent` skills rather than guessing the schema.
- New **command**: `.aimee/commands/<name>.md`. Optional `<lint>` / `<test>` tags (see `create-command`).
- New **agent**: `.aimee/agents/<id>.md`. Required front matter listed above.
- New **catalog tool**: not a skill. Join `ToolCatalog`, keep the description under 1024 characters, register the executor. See [Best practices](best-practices.md#tool-descriptions-1024-characters).

## Related

- [Best practices](best-practices.md) — operating contract, tool-description limit
- [Testing](quality/testing.md) — `:check`, `:tpl-tdd`, `test-reasoning`
- Product map: `AIMEE.md` §7 (tools) and §12 (skills / commands / templates)
