# Persona packs, scoped skills, toolbelts, schemas

SOTA layering used in `/root/src/repos/ai-agency` (2026-08). Prefer this over long inline `instructions=[...]` lists.

## Layers

| Layer | Path | Load timing | Purpose |
|-------|------|-------------|---------|
| SOUL | `prompts/<agent>/SOUL.md` | Always → instructions | Identity, non-negotiables, autonomy |
| SYSTEM | `prompts/<agent>/SYSTEM.md` | Always → instructions | Procedures, tools, collaboration |
| OUTPUT | `prompts/<agent>/OUTPUT.md` | expected_output | Contract + self-check |
| EXAMPLES | `prompts/<agent>/EXAMPLES.md` | additional_input | Few-shot + anti-patterns |
| Skills | `skills/*/<name>/SKILL.md` | On demand via get_skill_* | Shared playbooks |
| Schemas | `agents/schemas.py` | output_schema | Typed handoffs |
| Toolbelts | `tools/toolbelts.py` | Agent tools= | Role-shaped actions |

## Factory pattern

```python
build_agent(
  persona="product_scout",
  toolbelts=("parallel_research", "economics", "linear"),
  skill_names=("product-scoring", "unit-economics", "linear-ops"),
  output_schema=ProductCandidateBatch,
  use_json_mode=True,
  add_history_to_context=False,  # True for Ops/CS/Growth/Finance
)
```

Profiles live in `agents/profiles.py` so markdown stays the editable brain.

## Skill scoping

```python
skills_for("product-scoring", "unit-economics")  # exact folders only
# NOT skills_for() on every agent — dumps all packs into context tools
```

Creative must not load supplier-vetting; Scout must not load ugc-hooks.

## Toolbelts (examples)

- `parallel_research` — search, extract, task, task_result, entity, monitor
- `parallel_core` / `parallel_light` — smaller sets
- `economics`, `supplier`, `shopify`, `linear`

## Guardrails

`tools/guardrails.py` autonomy hook: L2 blocks active publish and spend-like tool names unless `AGENCY_AUTONOMY=L3`.

## Evals

```bash
PYTHONPATH=. python -m evals.run_agent_evals
# optional LLM: --live-scout
```

Checks persona depth, skill scope exactness, schemas, AgentOS 18/7/5, history flags.
