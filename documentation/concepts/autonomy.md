# Autonomy levels and guardrails

Aimee's autonomy is bounded by explicit, inspectable mechanisms — not by vibes. Five of them matter in practice.

## 1. Role-based write permissions

The flock split is the first guardrail: Sage cannot write, Muse writes only plan files, only Aimee mutates your tree. Even Aimee's writes flow through permission-checked tools. See [Three modes](../getting-started/modes.md).

## 2. Restricted mode

```toml
# .aimee.toml
restricted = true
```

With `restricted = true`, tool execution requires explicit permission grants — the agent proposes, you approve. This applies to shell commands, file mutations, and network access alike. Use it for sensitive repos, CI contexts, or while building trust.

## 3. The /goal probes (HITL)

`crates/aimee_domain/src/loop_autonomy.rs` defines exactly five human-in-the-loop probes a standing goal must answer (`GoalProbeSet`):

1. **What does done look like?** — observable outcome
2. **How will we verify?** — tests, commands, evidence
3. **What must not change?** — boundaries
4. **Who is the human owner** — and when should we stop and ask?
5. **What should we log against?** — Linear issue / GitHub PR / related work

A goal with unanswered probes isn't autonomous; it's ambiguous. The probe set forces the conversation before fan-out work begins.

## 4. Failure budgets and loop breakers

* `max_tool_failure_per_turn` caps consecutive tool failures per turn (default: unlimited).
* The doom-loop reminder interrupts repetitive failure patterns.
* Tool retries follow the retry template rather than hammering.

## 5. Orchestrator discipline

Aimee dispatches specialists via `task`, but orchestrators never nest: specialists cannot spawn further orchestrators, and aimee/muse/sage are invalid task targets. Fan-out width stays under the orchestrator's control. See [Swarm runs](../usage/swarm.md).

## Prompt depth

The autonomy module also defines `PromptDepth` and uplift machinery (`PromptUplift`, `PromptUpgrade`): incoming requests can be automatically sharpened before execution — restating intent, surfacing unknowns. This is how vague prompts become answerable ones without losing the user's meaning.

## Choosing your level

| Context | Suggested posture |
|---|---|
| Personal project, low stakes | Default autonomy + failure budget |
| Team repo | Restricted mode on risky surfaces; goals with full probe sets |
| Untrusted code | Pods + restricted mode; see [Pods](../surfaces/pods.md) |
| Production systems | Full probe discipline, `/master` pipeline, review gates |

## See also

* [How Aimee thinks: the loop](loop.md)
* [Security model](../operations/security.md)
* [Plans and todos](../usage/plans-and-todos.md)

<!-- sources: crates/aimee_domain/src/loop_autonomy.rs, templates/aimee-partial-swarm-policy.md, AIMEE.md §11,§15 -->
