# Ops agent SOTA archetype (AI Agency)

User requirement: every new agent must match the original 18 — extensive persona packs, custom skills, and role tools ready for real business ops. Thin stubs are a regression.

## Mandatory package per agent

1. **Persona** — `prompts/<key>/{SOUL,SYSTEM,OUTPUT,EXAMPLES}.md`
   - Target total **≥ ~5–6KB** for specialists (Scout/Hermes Ops larger)
   - Include: identity, non-negotiables, L1/L2/L3 autonomy, accountability, success metrics, procedure order, anti-patterns, quality bar, collaboration contracts, tool-failure handling, security/privacy, KIP+Linear dual-write, handoffs, schema name, worked examples + HITL boundary example
2. **Thin module** — `agents/<key>.py` via `build_agent` + `profile_by_key`
3. **Profile** — `agents/profiles.py` (toolbelts, skills, schema, temp, history, json_mode)
4. **Schema** — Pydantic model in `agents/schemas.py`
5. **Domain skill** — `skills/ops/<domain>-playbook/SKILL.md` (or agency/marketing)
6. **Agent playbook** — `skills/agents/<key-with-dashes>-playbook/SKILL.md`
7. **Dedicated toolbelt** when the role owns external ops (see table)
8. **Register** — `app/main.py`, `agents/__init__.py`, Drop `drop_roster`, teams/workflows as needed

## Factory defaults (do not strip)

Unless env-disabled, `_factory.build_agent` appends:

- `hermes_bridge` — browser/skills/memory/KIP/CUA jobs
- `anda_brain` — formation/recall/sleep/docs
- `analytics` — SKU/ads metrics (not the brain)
- `FileSystemKnowledge(knowledge/anda)` with `search_knowledge=True`

## Ops toolbelts

| Belt | Module | Typical agent |
|------|--------|----------------|
| `qa_ops` | `tools/qa_ops_tools.py` | QA Inspector |
| `returns_ops` | `tools/returns_ops_tools.py` | Returns Specialist |
| `chargeback_ops` | `tools/chargeback_ops_tools.py` | Chargeback Specialist |
| `cx_ops` | `tools/cx_ops_tools.py` | CX Escalations |
| `logistics_ops` | `tools/logistics_ops_tools.py` | Logistics Coordinator |
| `creative_ops` | `tools/creative_ops_tools.py` | Ads Creative Ops |
| `catalog_ops` | `tools/catalog_ops_tools.py` | Catalog Ops |
| `fraud_ops` | `tools/fraud_ops_tools.py` | Risk Fraud Analyst |
| `partnership_ops` | `tools/partnership_ops_tools.py` | Partnerships Manager |
| `tax_ops` | `tools/tax_ops_tools.py` | Tax Compliance |
| `community_ops` | `tools/community_ops_tools.py` | Community Manager |
| `experiment_ops` | `tools/experiment_ops_tools.py` | Experimentation Lead |

Wire belts in `tools/toolbelts.py` **before** profiles reference them.

## Skill name collisions

`tools/skills_loader.py` indexes by **folder name** across `skills/{agency,marketing,ops,agents}`. Later roots overwrite earlier. Keep unique names — e.g. domain `catalog-ops-playbook` vs agent `catalog-ops-agent-playbook`. Prefer real skill id `compliance-ads-claims` over alias `claims-compliance`.

## Scale snapshot (2026-08)

- **30** agents · **12** teams · **10** workflows
- Docs: `docs/AGENCY_EXPANSION_30.md`, `docs/OPS_AGENTS_SOTA.md`, `docs/BRAIN_IMPROVEMENTS.md`
- Related skill body (control planes, dual-write, HITL): `ai-agency-enterprise-ops` SKILL.md
- Memory deep-dive: `anda-ecosystem` (if patch blocked: `hermes curator adopt anda-ecosystem`)
- MCP routing skill `ai-dropshipping-agency-mcp` may still say 18 agents — prefer this file + enterprise-ops references until adopted/updated

## User preference (this session)

When expanding the agency, **match the original agent archetype** (extensive SOUL/SYSTEM/OUTPUT/EXAMPLES + custom skills + external ops tools). Do not leave thin generated stubs as the final state.

## Ad-hoc verify new agents

```bash
cd /root/src/repos/ai-agency && source .venv/bin/activate
PYTHONPATH=. python - <<'PY'
from pathlib import Path
from agents.profiles import PROFILES, profile_by_key
from tools.skills_loader import list_skill_names, skills_for
from tools.toolbelts import TOOLBELTS
import importlib
known=set(list_skill_names())
for p in PROFILES:
    assert all(s in known for s in p.skills), (p.key, p.skills)
    for b in p.toolbelts:
        assert b in TOOLBELTS or b in {"hermes_bridge","anda_brain","analytics"}, b
# size floor
for k in ["qa_inspector","experimentation_lead"]:
    d=Path("prompts")/k
    tot=sum((d/f).stat().st_size for f in ["SOUL.md","SYSTEM.md","OUTPUT.md","EXAMPLES.md"])
    assert tot>=5000, (k,tot)
    ag=getattr(importlib.import_module(f"agents.{k}"), k)
    assert ag.output_schema and len(ag.tools or [])>=30
print("ok", len(PROFILES))
PY
```
