# Agno LocalSkills (validated on ai-agency, 2026-08)

Agno agents/teams accept `skills=Skills(loaders=[LocalSkills(...)])`. This is **not** the Hermes skill system.

## Layout

```text
skills/
  agency/                 # category folder (not a skill itself)
    product-scoring/      # folder name MUST equal frontmatter name
      SKILL.md
      references/optional.md
      scripts/optional.py
  marketing/
    ugc-hooks/SKILL.md
  ops/
    fulfillment-playbook/SKILL.md
```

`LocalSkills(path)` accepts either a single skill folder (contains `SKILL.md`) or a **directory of skill folders**.

## Frontmatter (Agent Skills spec subset)

Allowed fields: `name`, `description`, `license`, `allowed-tools`, `metadata`, `compatibility`.

```yaml
---
name: product-scoring
description: "Score dropshipping opportunities on margin and risk."
metadata:
  category: agency
---
```

### Validation gotchas

| Rule | Failure mode |
|------|----------------|
| `name` lowercase, hyphens only, ≤64 chars | SkillValidationError |
| Folder basename == `name` | SkillValidationError |
| `description` non-empty, ≤1024 | SkillValidationError |
| Unquoted `description` with `:` | YAML parse error (`mapping values are not allowed`) |
| Quote descriptions always if they may contain colons | Safe default |

## Loader singleton pattern

```python
from functools import lru_cache
from agno.skills import Skills, LocalSkills

@lru_cache(maxsize=1)
def get_agency_skills() -> Skills:
    return Skills(loaders=[
        LocalSkills("skills/agency", validate=True),
        LocalSkills("skills/marketing", validate=True),
        LocalSkills("skills/ops", validate=True),
    ])
```

Pass the same object to many agents: `Agent(..., skills=get_agency_skills())`.

## Runtime tools (progressive disclosure)

After attach, the agent gets tools roughly:

1. `get_skill_instructions(skill_name)` — full SKILL.md body  
2. `get_skill_reference(skill_name, reference_path)` — files under `references/`  
3. `get_skill_script(skill_name, script_path, execute=False)` — read/run `scripts/`

Skill **names are not callable functions**. Instructions should say: “Load skill X via get_skill_instructions before deciding.”

## Hermes vs Agno

| | Hermes skills | Agno LocalSkills |
|--|---------------|------------------|
| Where | `~/.hermes/skills/`, agent-skills | Project `skills/` |
| Author skill | `hermes-agent-skill-authoring` | this doc |
| Loaded by | Hermes runtime | `Skills` / `LocalSkills` on Agent/Team |

Do not expect a Hermes SKILL.md drop-in to validate under Agno without checking name/directory/description rules.
