# Reference: ai-clipping SOUL layout

Repo: `/root/src/repos/ai-clipping` (Autonomous AI Clipping Agency).  
Runtime: Agno AgentOS on **:7766** + Anda/KIP local brain.  
Scaffold skill: `agno-agentos-apps` → `references/multi-agency-anda-scaffold.md`.

## Tree

```text
AGENTS.md
.hermes.md          # force-read AGENTS.md + SOUL index
SOUL/SOUL.md
SOUL/00-overview/SOUL.md
SOUL/01-architecture/SOUL.md
SOUL/02-anda-kip/SOUL.md
SOUL/03-agents/SOUL.md
SOUL/04-clipping-tools/SOUL.md
SOUL/05-teams-workflows/SOUL.md
SOUL/06-agentos-mcp/SOUL.md
SOUL/07-autonomy-hitl/SOUL.md
SOUL/08-development/SOUL.md
SOUL/17-invariants/SOUL.md
```

## Hard invariants captured

- L2 autonomy: draft packages only; human publish + rights clearance
- Anda/KIP: no secrets in graph attributes; recall/formation protocol
- `TeamMode.coordinate` enum only
- Default port **7766** (avoid clash with ai-agency :7777)
- Factory auto-attaches `anda_brain` + `hermes_bridge` unless env-disabled
- Never commit `.env` / OAuth tokens / `tmp/secrets/`

## Hermes wiring lesson

Same as anda-bot: `.hermes.md` **replaces** AGENTS.md auto-load. Open with mandatory tool-read of `AGENTS.md`, then SOUL index + invariant summary + verify commands.

## Verify slice

```bash
cd /root/src/repos/ai-clipping && source .venv/bin/activate
export PYTHONPATH=. XAI_API_KEY=missing-ci-placeholder
export AGENCY_DISABLE_HERMES_BRIDGE=1 AGENCY_DISABLE_ANDA_BRAIN=1 AGENCY_DISABLE_ANDA_KNOWLEDGE=1
test -f SOUL/SOUL.md && test -f .hermes.md && test -f AGENTS.md
python -m evals.run_agent_evals
pytest tests/ -q
```

Use this layout as the default SOUL shape for **new Agno agencies** (domain folders rename; keep overview/architecture/memory/agents/tools/teams/mcp/autonomy/dev/invariants).
