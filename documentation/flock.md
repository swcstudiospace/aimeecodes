# The flock

Three built-in agents. One product loop. Aimee may dispatch Frontend / Backend / Platform specialists. Aimee does **not** nest orchestrators.

Definitions live in `crates/aimee_repo/src/agents/`. First-class IDs are `AgentId::AIMEE`, `AgentId::MUSE`, `AgentId::SAGE` (`crates/aimee_domain/src/agent.rs:37-39`). Default agent is Aimee (`crates/aimee_domain/src/agent.rs:42-46`).

## Sage → Muse → Aimee

```
Sage (research, read-only)
        │
        ▼
Muse (checkbox plan under plans/)
        │
        ▼
Aimee (implement + verify; may dispatch specialists)
```

| Agent | ID | Writes? | Role | Definition |
|---|---|---|---|---|
| Sage | `sage` | No | Research, architecture, reviews | `crates/aimee_repo/src/agents/sage.md` |
| Muse | `muse` | Plans only | Checkbox plans via the `plan` tool | `crates/aimee_repo/src/agents/muse.md` |
| Aimee | `aimee` | Yes | Implement, verify, report evidence | `crates/aimee_repo/src/agents/aimee.md` |

Aimee is also the **engineering orchestrator**. When a change is clearly in one specialty it dispatches a subagent via the `task` tool and stays accountable for verification (`crates/aimee_repo/src/agents/aimee.md:35-39`).

## Aliases: `:ask` `:plan` `:act`

Product copy lists `:ask` / `:plan` / `:act` (`aimeecodes/README.md:73-77`, `AIMEE.md:60-64`). The **surfaces disagree**. Prefer the code.

### ZSH plugin (`shell-plugin/lib/dispatcher.zsh:125-132`)

| You type | Becomes |
|---|---|
| `:ask …` | agent `sage` |
| `:plan …` | agent `muse` |
| `:sage …` / `:muse …` / `:aimee …` | that agent |
| `:act …` | **not remapped** — treated as a command named `act` |

`:act` is **not** in the dispatcher `case`. If `act` is not a listed command, the plugin errors with `Command 'act' not found` (`shell-plugin/lib/dispatcher.zsh:21-24`). In zsh, implement with `:aimee`.

Bare `: <prompt>` uses the active agent (`_AIMEE_ACTIVE_AGENT`, default `aimee` on `:new` — `shell-plugin/lib/actions/core.zsh:11`).

### TUI / rustyline (`crates/aimee_main/src/model.rs:634-651`)

| You type | Switches to |
|---|---|
| `:act` / `:aimee` / `:omega` | Aimee |
| `:plan` / `:muse` | Muse |
| `:sage` | Sage |

`:ask` is **not** a TUI alias. There is no `alias = "ask"` on `AppCommand::Sage` (`crates/aimee_main/src/model.rs:646-651`). In the TUI, research with `:sage`.

`/` is accepted as a compat sentinel for the same commands (`crates/aimee_main/src/model.rs:316-325`). Canonical prefix is `:`.

## Specialist roster (on disk)

Aimee dispatches by `agent_id`. Do not impersonate them. Do not nest orchestrators (`crates/aimee_repo/src/agents/aimee.md:41-43`). The splash lists 17 chips: 3 loop + 14 specialists (`crates/aimee_main/src/banner.rs:18-40`).

Files: `crates/aimee_repo/src/agents/<id>.md`.

### Frontend

| ID | Use when |
|---|---|
| `fe-ui` | Components, layout, a11y, design tokens |
| `fe-web3` | Wallet / dapp UX, SIWE, ICP identity |
| `fe-realtime` | Streaming, SSE, live cluster clients |
| `fe-edge` | PWA, CDN, edge, performance budgets |
| `fe-qa` | E2E, visual regression, a11y evidence |

### Backend

| ID | Use when |
|---|---|
| `be-api` | Services, APIs, clean architecture |
| `be-web3` | Canisters, contracts, chain adapters, KIP |
| `be-data` | Persistence, migrations, durability |
| `be-security` | AuthN/Z, secrets, tenancy, controls |
| `be-reliability` | Timeouts, retries, tracing, SLOs |

### Platform

| ID | Use when |
|---|---|
| `plat-k8s` | Kubernetes, GitOps, network policy |
| `plat-cloud` | IaC, cloud identity, multi-region |
| `plat-compliance` | SOC2 / FedRAMP gaps and evidence |
| `plat-sre` | CI/CD, supply chain, SLSA, incidents |

These IDs exist as files. There is no built-in `rust-engineer` / `planner` agent on disk. Custom agents go in `.aimee/agents/` (project) or `~/.aimee/agents/` (global).

List what the running binary actually loaded:

```bash
aimee agent list
aimee list agent
aimee list agent --custom
```

## When to dispatch vs stay in the parent

Stay in **Sage** when the job is research, a multi-file trace, or a review. Sage does not plan and does not edit (`crates/aimee_repo/src/agents/sage.md:24-25`). Handoff: Muse if design is open, Aimee if the change is already obvious (`sage.md:39`).

Stay in **Muse** when the work is large or ambiguous. Muse writes `{YYYY-MM-DD}-{plan_name}-{version}.md` under `plans/` and never overwrites — it bumps `version` (`muse.md:34-37`). If you ask Muse to implement, it refuses and hands off to Aimee.

Stay in **Aimee** (the parent) when:

- The change is already scoped and you want one implementer
- You need to integrate specialist output and verify on the tree
- The work spans lanes and you are the orchestrator

**Dispatch** a specialist when the change is clearly one specialty (`aimee.md:39`). Launch specialists concurrently when workstreams are independent. Give each a bounded prompt: files, constraints, verify command (`aimee.md:74`). After they return, Aimee verifies on the tree before claiming done.

Do **not**:

- Ask Sage to implement
- Ask Muse to edit product code
- Ask Aimee to rewrite the plan unless the plan is wrong
- Nest Sage / Muse / Aimee inside another Sage / Muse / Aimee
- Ask a specialist to impersonate Aimee or to re-plan the product

When `subagents = true` (default in embedded config), Aimee gets `task` and Sage-as-a-tool is removed. When false, `task` is disabled and `sage` is available as a tool instead (`AIMEE.md:262-263`).

## Commands

```bash
aimee --agent sage
aimee --agent muse
aimee --agent aimee
aimee --agent fe-ui
aimee list tool aimee
aimee list tool sage
```

ZSH:

```zsh
:sage how does the caching layer work?
:ask  how does the caching layer work?     # plugin remaps to sage
:muse design a deployment strategy
:plan design a deployment strategy         # plugin remaps to muse
:aimee implement the plan in plans/…
:agent fe-ui                               # switch active agent
```

TUI:

```
:sage
:plan
:act
:agent
```

## File interactions

| Path | Role |
|---|---|
| `crates/aimee_domain/src/agent.rs:37-39` | Built-in `AgentId` constants |
| `crates/aimee_repo/src/agents/*.md` | Prompt + tool lists |
| `crates/aimee_main/src/banner.rs:18-40` | Splash chips (loop + 14 specialists) |
| `crates/aimee_main/src/model.rs:634-651` | TUI switch commands |
| `shell-plugin/lib/dispatcher.zsh:125-132` | ZSH `:ask` / `:plan` remap |
| `.aimee/agents/` | Project custom agents |
| `~/.aimee/agents/` | Global custom agents |
| `plans/` | Muse output — historical unless cited |

## Best practices

- One loop step at a time: research, then plan, then implement.
- Give Aimee the plan path, the verify command, and the boundaries.
- Dispatch by ID. Do not ask Aimee to “be” `fe-ui`.
- Custom agents shadow built-ins by the same ID (CWD > global > built-in). See [Skills and commands](skills.md).

## Anti-patterns

- Treating README aliases as universal. `:act` is TUI-only; `:ask` is plugin-only.
- Nesting orchestrators (`sage` / `muse` / `aimee` as a `task` target of each other).
- Asking Muse to implement, or Sage to write `plans/`.
- Inventing specialist IDs that are not files under `crates/aimee_repo/src/agents/`.

## Related

- [How to use](howto.md)
- [Best practices](best-practices.md)
- [TUI](surfaces/tui.md)
- [ZSH plugin](zsh.md)
