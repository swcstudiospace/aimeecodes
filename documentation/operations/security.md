# Security model

Aimee acts on your system, so its security posture is explicit: untrusted-input discipline, permission-gated tools, secrets outside git, and human control over anything irreversible or expensive.

## Threat stance

Everything entering the agent is treated as untrusted until validated:

* CLI arguments, file contents, tool results, fetched web pages
* MCP server output — third-party tool responses get the same validation as any external content

House rules inherited by the whole stack: parameterized commands only (no interpolated shell/SQL), no `eval`-family execution on user data, and no leaking secrets into logs.

## Permission gates

| Mechanism | Effect |
|---|---|
| Role permissions | Sage can't write; Muse writes plans only; only Aimee mutates |
| `restricted = true` | Every tool execution needs an explicit grant |
| `tool_timeout_secs` | No tool call runs unbounded |
| `max_tool_failure_per_turn` | Failure budget stops runaway loops |
| Orchestrator discipline | Specialists are bounded; orchestrators never nest |
| Pods | Machine-level isolation for untrusted code |

Defense in depth: any one mechanism failing still leaves the others. See [Autonomy levels and guardrails](../concepts/autonomy.md).

## Secrets handling

* Credentials live in `~/.aimee/.credentials.json` — never committed, never logged.
* API keys reach providers through the auth flow, not through prompts or config files in git.
* Output paths redact tokens; agents are instructed never to print or commit secrets.
* If a key leaks: `aimee provider logout`, rotate at the provider, log back in.

## Isolation options

| Risk | Tool |
|---|---|
| Parallel local work | `--sandbox` (git worktree) |
| Untrusted PRs / experiments | `aimee pod up` (container workspace) |
| Full dev environment | Dev Container with pinned tooling |

## What Aimee won't do

No autonomous payments or spend (HITL by design — see [Wallet](../integrations/wallet.md)); no anonymous write paths onto hosted services; no silent expansion of its own permissions at runtime.

## Reporting issues

Security issues in Aimee Codes itself go to the repository maintainers (Spectrum Web Co LLC — contact in the repo README). Include reproduction steps and affected versions.

## See also

* [Data privacy](privacy.md)
* [Pods and sandboxes](../surfaces/pods.md)
* [Authentication and credentials](../integrations/auth.md)

<!-- sources: AGENTS.md security section, AIMEE.md §15, templates/aimee-partial-security-baseline.md -->
