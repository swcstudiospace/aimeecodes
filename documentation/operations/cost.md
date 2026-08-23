# Cost awareness

What drives spend when using Aimee Codes, and the levers that keep it sane.

## Where money goes

Model usage is the cost center. Every turn sends context to your chosen provider at that provider's rates. Costs scale with: conversation length (context grows each turn), tool-result size fed back into prompts, and model choice per job.

Aimee itself doesn't meter or bill model usage — your provider account does. The hosted side (`aimee_services` / workspace indexing) bills through your Aimee plan; plan management lives at `https://app.aimeecodes.dev/app/billing`.

## Levers

### 1. Right-size models per job

Dedicated cheap/fast models for high-frequency jobs:

```bash
aimee config set commit <provider_id> <model_id>     # commit messages
aimee config set suggest <provider_id> <model_id>    # shell suggestions
```

Session-scoped switching from the ZSH dispatcher handles one-off changes without editing config. Route heavy reasoning to capable models only where it pays.

### 2. Compact long conversations

Context length is the multiplier on every subsequent call:

```bash
aimee conversation compact --cid <id>
```

Tuning lives under `[compact]` in `.aimee.toml`. For swarm-heavy work, bounded workstreams keep each specialist's context small by construction.

### 3. Bound the loops

* `max_tool_failure_per_turn` stops thrash before it burns tokens.
* The doom-loop reminder interrupts repetitive patterns automatically.
* `/goal` probes force verification criteria up front, reducing exploratory drift.

### 4. Local-first features

Semantic search runs over your indexed workspace rather than paying a model to re-read files each session. Snapshots and undo are local. Anda pathways checkpoint locally in the default mode.

## Observability

Per-conversation usage metrics persist alongside history:

```bash
aimee conversation stats --cid <id>
aimee conversation info --cid <id>
```

Provider-side dashboards remain the authoritative billing view.

## See also

* [Providers](../integrations/providers.md)
* [Autonomy levels and guardrails](../concepts/autonomy.md)
* [Sessions](../usage/sessions.md)

<!-- sources: crates/aimee_main/src/cli.rs (conversation stats/info), AIMEE.md §2,§6, templates/aimee-partial-swarm-policy.md -->
