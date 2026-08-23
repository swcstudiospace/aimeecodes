# Swarm runs

Swarm is the flock's parallel mode: one goal, decomposed into independent workstreams, executed concurrently by specialist subagents.

## When to swarm

Use it when work splits cleanly — "migrate these five modules", "add tests to each of these crates", "fix lint categories across the tree". Don't use it when steps depend on each other; sequential Aimee is better there.

## Running a swarm

```zsh
: aimee /swarm add unit tests to every crate in crates/
```

Or through the CLI in any session. The `swarm` command definition (`commands/swarm.md`) drives the behavior:

1. **Decompose** the goal into independent workstreams, each with bounded files and its own verify command.
2. **Persist as a standing `/goal` loop** with the same text, so continuation uses the goal loop rather than re-prompting.
3. **Fan out** specialist subagents via the `task` tool, preferring concurrent calls for independent streams.
4. **Judge completion** against the goal before declaring done.

## The policy rules

From the swarm policy shipped in `templates/aimee-partial-swarm-policy.md` and the command itself:

* Prefer concurrent task calls for independent workstreams.
* Never nest orchestrators: aimee/muse/sage are not valid task targets.
* Each specialist gets explicit constraints — files it may touch, what must not change, how to verify.
* Specialists report evidence; the orchestrator integrates.

## Specialists available

Frontend: `fe-ui`, `fe-web3`, `fe-realtime`, `fe-edge`, `fe-qa` · Backend: `be-api`, `be-web3`, `be-data`, `be-security`, `be-reliability` · Platform: `plat-k8s`, `plat-cloud`, `plat-compliance`, `plat-sre`. See [The flock](../getting-started/the-flock.md) for roles.

## The /goal loop

A standing goal keeps multi-turn work coherent across sessions: the loop knows what "done" means because the goal definition answered the HITL probes (observable outcome, verification method, boundaries, owner). Details in [Autonomy levels and guardrails](../concepts/autonomy.md).

## See also

* [Slash commands](commands.md)
* [The flock: Sage, Muse, Aimee](../getting-started/the-flock.md)
* [Autonomy levels and guardrails](../concepts/autonomy.md)

<!-- sources: commands/swarm.md, templates/aimee-partial-swarm-policy.md, AIMEE.md §3,§11 -->
