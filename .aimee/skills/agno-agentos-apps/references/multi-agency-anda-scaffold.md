# Multi-agency scaffold with Anda/KIP base

How to stand up a **second** Agno AgentOS agency on the same host, using **Anda (ldclabs/iclabs)** as the memory/agent base — distilled from `ai-clipping` (2026-08-19).

## Ports (this VPS)

| Service | Port |
|---------|------|
| ai-agency (dropshipping) AgentOS | **7777** |
| ai-clipping AgentOS | **7766** |
| Drop hybrid MCP | 7788 |
| Hermes reverse bridge | 7790 |
| Anda Cognitive Nexus | 8091 |

Never bind two agencies to the same port.

## Copy vs rewrite

**Copy (foundation):**

- `tools/xai_model.py`, `xai_oauth_pkce.py`, `envutil.py`, `parallel_tools.py`
- `tools/skills_loader.py`, `guardrails.py` (then domain-tune blocks)
- `tools/anda_knowledge.py`, `hermes_bridge_tools.py` (optional)
- Entire `kip_memory/` package + seed capsules under `capsules_seed/`
- Thin slice of `knowledge/anda/*.md` docs

**Do not copy wholesale:**

- `agents/__init__.py` from ai-agency (eager-imports every dropshipping agent → import bombs)
- Full agent roster / teams / workflows / domain toolbelts
- Dropshipping-only tools (Shopify, spend vault, etc.) unless the new domain needs them

## Identity retarget checklist

After copying foundation code:

1. OAuth store: `~/.config/<slug>/xai_oauth.json` (+ optional fallback to sibling agency path)
2. Parallel `User-Agent` string
3. `kip_memory/nexus.py` `$self` description (not “Dropshipping…”)
4. `kip_memory/brain.py` `formation(..., space="<domain>")` default
5. Domain genesis capsule e.g. `capsules_seed/<domain>_genesis.kip`
6. `skills_loader.SKILL_ROOTS` → `skills/{agency,<domain>,agents}`
7. `app/main.py` AgentOS `id` / `name` / description / port env
8. Delete `kip_memory/data/nexus.db` (+ brain_state) after identity edits so primer refreshes

## Factory contract (Anda-first agencies)

`build_agent` should:

- Auto-append toolbelts `hermes_bridge` and `anda_brain` unless `AGENCY_DISABLE_*`
- Inject a short always-on Anda protocol (recall → act → formation; no secrets in KIP)
- Optionally attach `FileSystemKnowledge(knowledge/anda)` with `search_knowledge=True`

## Hermes orientation (every new agency repo)

- `AGENTS.md` — coding policy + Project SOUL section
- `.hermes.md` — **must** force tool-read of `AGENTS.md` (Hermes loads this *instead of* AGENTS)
- Nested `SOUL/` — index + subsystems + invariants  
  Skill: `hermes-project-soul`

## Verification gates (user preference: multi-gate, not smoke-only)

```bash
export PYTHONPATH=. XAI_API_KEY=missing-ci-placeholder
export AGENCY_DISABLE_HERMES_BRIDGE=1 AGENCY_DISABLE_ANDA_BRAIN=1 AGENCY_DISABLE_ANDA_KNOWLEDGE=1
ruff check agents app tools teams workflows evals scripts kip_memory tests
pytest tests/ -q
python -m evals.run_agent_evals
```

Minimum pytest coverage: profile count, persona files, Agent type imports, skill name resolution, `TeamMode.coordinate`, AgentOS roster counts, domain tool smoke, KIP primer, SOUL/AGENTS/.hermes present, L2 guardrail blocks.

## Reference tree (ai-clipping)

```text
ai-clipping/
  app/main.py                 # :7766 + /mcp
  agents/{_factory,profiles,schemas,prompt_loader,*.py}
  prompts/<agent>/{SOUL,SYSTEM,OUTPUT,EXAMPLES}.md
  teams/  workflows/
  tools/{clipping_tools,toolbelts,guardrails,anda_knowledge,...}
  kip_memory/  knowledge/anda/
  skills/{agency,clipping,agents}/
  SOUL/  AGENTS.md  .hermes.md
  evals/run_agent_evals.py  tests/test_clipping_base.py
```

Pipeline shape: source → transcript → viral moments → edit → captions → package → rights → HITL publish → KIP learnings.
