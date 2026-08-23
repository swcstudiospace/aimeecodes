# The flock: Sage, Muse, Aimee

Three built-in agents, one product loop. Definitions live in `crates/aimee_repo/src/agents/`.

| | Agent | ID | Alias | Writes? | Role |
|---|---|---|---|---|---|
| 🔍 | **Sage** | `sage` | `:ask` | No | Research, architecture, reviews |
| 📋 | **Muse** | `muse` | `:plan` | Plans only | Checkbox plans under `plans/` |
| ⚡ | **Aimee** | `aimee` | `:act` | Yes | Implement, verify, report evidence |

## Sage — research and review

Sage answers questions about your codebase without modifying it. Reach for it when you want to understand code before changing it: architecture explanations, impact analysis ("what breaks if I change X"), code review opinions. Because Sage has no write access, it's safe to point at anything.

```zsh
: sage what owns the retry policy for provider calls?
```

## Muse — planning

Muse turns intent into a structured plan: a checkbox file saved under `plans/`. It doesn't edit source. You can hand-edit the plan before execution — reorder tasks, strike items, add constraints. Plans are the contract Aimee executes against.

```zsh
: muse plan migrating the config loader to the new schema
```

## Aimee — implement and verify

Aimee implements changes and verifies them with real commands (tests, builds, linters), reporting evidence for each claim. It is also the engineering orchestrator: when a change clearly belongs to one specialty, Aimee dispatches a specialist subagent through the `task` tool instead of doing everything inline. Orchestrators never nest — specialists don't spawn further orchestrators.

```zsh
: aimee implement plans/2026-08-23-retry-policy.md
```

## The specialist roster

When a task needs a specialty, Aimee dispatches to these built-in agents:

| Domain | Agents |
|---|---|
| Frontend | `fe-ui`, `fe-web3`, `fe-realtime`, `fe-edge`, `fe-qa` |
| Backend | `be-api`, `be-web3`, `be-data`, `be-security`, `be-reliability` |
| Platform | `plat-k8s`, `plat-cloud`, `plat-compliance`, `plat-sre` |

Each specialist's prompt definition ships in `crates/aimee_repo/src/agents/` (`fe-ui.md`, `be-security.md`, …).

## Custom agents

Define your own in `.aimee/agents/<name>/` (project) or `~/.aimee/agents/<name>/` (global). List what's visible with:

```bash
aimee agent list        # built-ins
aimee agent list --custom
```

Agent IDs are first-class values in the domain model (`AgentId::AIMEE`, `AgentId::MUSE`, `AgentId::SAGE`).

## See also

* [Three modes](modes.md)
* [Swarm runs](../usage/swarm.md)
* [How Aimee thinks: the loop](../concepts/loop.md)

<!-- sources: AIMEE.md §3, crates/aimee_repo/src/agents/, crates/aimee_domain/src/agent.rs -->
