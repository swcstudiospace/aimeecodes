# Everyday workflows

Proven chains of flock commands for common engineering situations. Adapt the examples to your repo.

## Fix a bug across files

```zsh
: sage why does checkout fail when the cart has a deleted item? trace the loader path
: muse plan handling deleted items in the cart loader
# review/edit the generated plans/*.md file
: aimee implement plans/2026-08-23-cart-loader.md
```

Research first means the plan is grounded; the plan means execution is checkable. Aimee finishes with evidence — test output, files changed.

## Review before opening a PR

```zsh
: aimee /review
```

The `review` command performs a principal-engineer pass over your working tree: risk assessment, test coverage, security findings. It won't rewrite unrelated code. Follow with:

```bash
aimee commit        # AI-generated conventional commit message from your diff
```

## Research spike without touching the tree

```bash
aimee --agent sage "map every place we assume UTC and list the DST risk"
```

Sage is read-only, so this is safe on any branch, any state.

## Ship checklist

```zsh
: aimee /ship v0.5.0
```

Gates: tests green, migrations safe, feature flags accounted. Output: changelog, risks, rollout and rollback plan. Pair with `/github-pr-description` when the PR body needs writing.

## Operations incidents

| Situation | Command |
|---|---|
| Alert firing at 3am | `/oncall` |
| Something is down | `/incident` |
| It's down and customers noticed | `/incident` then `/postmortem` after recovery |

Both are structured runbooks: triage → mitigate → communicate, then a blameless postmortem draft.

## Design-heavy work

```zsh
: aimee /rfc event-sourcing for order history
: aimee /adr adopt outbox pattern for order events
```

`rfc` produces the design doc; `adr` records the decision once made. For schema changes use `/migrate` (expand-contract planning) before implementation.

## Quality passes

| Command | Use when |
|---|---|
| `/perf` | A hot path needs investigation and an optimization plan |
| `/test-plan` | Coverage strategy needs unit/integration/e2e/chaos structure |
| `/threat-model` | A feature surface needs STRIDE analysis |
| `/harden` | SOC2/FedRAMP-minded hardening before audit |
| `/slo` | SLIs/SLOs/error budgets need defining |
| `/tpl-observability` | Logs/metrics/traces/alerts need wiring on a path |

## The full pipeline

When you want everything in one motion — understand, design, decompose, implement, verify, harden, review, ship — run:

```zsh
: aimee /master add team workspaces to the API
```

Each stage invokes its sub-command in the right position. See [Slash commands](commands.md) for stage details.

## See also

* [The : prefix (ZSH)](zsh-prefix.md)
* [Swarm runs](swarm.md)
* [Plans and todos](plans-and-todos.md)

<!-- sources: commands/*.md descriptions, crates/aimee_main/src/cli.rs (commit) -->
