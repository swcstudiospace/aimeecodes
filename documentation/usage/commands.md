# Slash commands

Commands are structured prompt templates the flock executes in-session. Two kinds exist: **built-in engineering commands** shipped with the repo, and **custom commands** you define per project.

## Running commands

From your shell via the plugin dispatcher, or directly through the CLI:

```bash
aimee cmd list                       # what's available
aimee cmd execute review "the auth module"
aimee cmd execute --cid <id> ship v0.4.0
```

Inside a session, address the command by name. The ZSH dispatcher validates names against `aimee cmd list` before executing and creates a conversation ID automatically when needed.

## Built-in command catalog

All definitions live in `commands/*.md` in the repository. Names, purposes as declared by each file:

| Command | Purpose |
|---|---|
| `adr` | Architecture Decision Record |
| `api-contract` | Design or review an API contract (OpenAPI-minded) |
| `compliance` | SOC2/FedRAMP control gap analysis for the feature |
| `cost` | Cloud cost review and reduction opportunities |
| `data-privacy` | Privacy review — PII flows, retention, access |
| `github-pr-description` | Updates the description of the PR |
| `harden` | Security hardening pass (SOC2/FedRAMP-minded) |
| `incident` | Incident commander runbook — triage, mitigate, communicate |
| `k8s-review` | Kubernetes manifests / Helm / GitOps review |
| `master` | Full pipeline — plan, swarm, build, verify, ship |
| `migrate` | Safe schema/data migration plan with expand-contract |
| `oncall` | On-call triage of alerts, dashboards, failing checks |
| `perf` | Performance investigation and optimization plan |
| `postmortem` | Blameless postmortem draft |
| `review` | Enterprise code review: risk, tests, security findings |
| `rfc` | Engineering RFC / design doc |
| `runbook` | Operator runbook for a service or failure mode |
| `ship` | Release readiness — changelog, risks, rollout, rollback |
| `slo` | SLIs/SLOs/error budgets for a service |
| `swarm` | Fan-out work across specialists as a standing /goal loop |
| `test-plan` | Enterprise test plan (unit/integration/e2e/chaos) |
| `threat-model` | STRIDE-oriented threat model |

## Template commands

The `tpl-*` family is lighter-weight starting points:

`tpl-benchmark`, `tpl-debug`, `tpl-design`, `tpl-docs-inline`, `tpl-explain`, `tpl-handoff`, `tpl-implement`, `tpl-migrate-plan`, `tpl-observability`, `tpl-pr`, `tpl-refactor`, `tpl-release-notes`, `tpl-review-diff`, `tpl-security-pass`, `tpl-tdd`

Each states its job in its own description — for example `tpl-tdd` ("RED-GREEN-REFACTOR test-first"), `tpl-review-diff` ("Review working tree / PR diff for ship blockers").

## Deep dives

### `/master` — the full pipeline

`master` chains every stage in order: understand → design → decompose (swarm) → implement → verify → harden → review → ship. Each stage invokes the appropriate command (`explain`, `tpl-design`, `swarm`, …) in its correct position — planning before code, review before hardening, verification before release.

### `/swarm` — parallel specialists

`swarm` decomposes your goal into independent workstreams, persists it as a standing `/goal` loop, and executes with concurrent specialist subagents via the `task` tool. Policy rules from the command definition: prefer concurrent task calls for independent work; never nest orchestrators (no aimee/muse/sage as task targets); each workstream gets bounded files and a verify command.

### `/review` — principal-engineer pass

Reviews the current change set with risk, tests, and security findings. Constraints from the definition: don't rewrite unrelated code, prefer evidence from the tree over speculation, dispatch specialists when FE/BE/platform depth is needed.

### `/ship` — release readiness

Gates on tests green, migrations safe, feature flags; produces changelog, risks, rollout and rollback plan.

## Custom commands

Per-project commands live in `.aimee/commands/<name>.md`. The repo ships two examples (`check.md`, `fixme.md`). A custom command is markdown with a small frontmatter header:

```markdown
---
name: audit-deps
description: Audit dependency drift against the lockfile
---

<role>dependency_auditor</role>
<objective>Report outdated/vulnerable dependencies and propose pinned bumps.</objective>
```

List what's registered:

```bash
aimee cmd list --custom
```

## See also

* [The : prefix (ZSH)](zsh-prefix.md)
* [Swarm runs](swarm.md)
* [Skills, commands, and templates](../concepts/skills-commands-templates.md)

<!-- sources: commands/*.md, crates/aimee_main/src/cli.rs (cmd group), shell-plugin/lib/dispatcher.zsh -->
