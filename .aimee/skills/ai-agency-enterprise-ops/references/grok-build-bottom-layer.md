# Grok Build bottom layer (replaces Warp)

## Decision

User tried **Warp/Oz** as the agency bottom CLI and rejected it as not optimal (2026-08-12).  
**Do not reintroduce** `tools/warp_tools.py`, `configs/warp/`, `oz`, Warp toolbelts, or Warp showcase scripts.

Canonical stack:

```text
Hermes (top) → Agno AgentOS (middle) → Grok Build CLI `grok` (bottom · SuperGrok)
```

CodeRabbit remains the review gate (local tools + CI), not the bottom executor.

## Repo map

| Piece | Path |
|-------|------|
| Tools | `tools/grok_build_tools.py` |
| Belt | `grok_build` in `tools/toolbelts.py` |
| Factory | `agents/_factory.py` auto-attach + `GROK_BUILD_OFFLOAD_INSTRUCTIONS` |
| Teams | `teams/_factory.py` blurb + team tools `grok_build`/`coderabbit`/`linear` |
| Workflows | `workflows/_grok_build.with_grok_build_guidance()` |
| Project rules | root `AGENTS.md` (Grok discovers) |
| Custom agents | `configs/grok-build/agents/*.md` (YAML frontmatter required) |
| Config | `configs/grok-build/config.toml`, `agency-mcp.json` |
| Showcase | `scripts/showcase_grok_build_dropshipping_flow.py` |
| Tests | `tests/test_grok_build_coderabbit.py` |
| Docs | `docs/GROK_BUILD_AND_CODERABBIT.md` |
| Artifacts | `tmp/grok_build_runs/`, `tmp/runs/grok_build_dropshipping_showcase_*` |

## Agent tools

- `grok_build_status` — bin + SuperGrok auth snapshot (no secrets)
- `grok_build_offload_shell` — audited bash → `tmp/grok_build_runs/shell_*.json` (blocks `.env` dumps)
- `grok_build_run` — headless `grok -p … --always-approve --max-turns N`
- `grok_build_orchestrate_agency_task` — goal + agency-bottom profile
- `grok_build_inspect` — `grok inspect`
- `grok_build_agent_stdio` — probe only (`grok agent` is long-lived)

## SuperGrok auth + model ids

- CLI install: `curl -fsSL https://x.ai/cli/install.sh | bash` → `~/.grok/bin/grok`
- Auth: `~/.grok/auth.json` (SuperGrok login) and/or `XAI_API_KEY`
- Agno chat models and Grok Build share the SuperGrok family
- **Critical:** with `XAI_API_KEY`, `grok models` may list only **`grok-4.5`**
  - Default headless model to **`grok-4.5`**
  - Do **not** force `-m grok-build` unless `grok models` shows it — fails with `unknown model id`
- Custom agent frontmatter `model:` should match an available id (`grok-4.5`)

## Custom agent profile format

Grok rejects bare markdown agents:

```text
failed to parse agent definition: missing frontmatter delimiters
```

Required shape:

```markdown
---
name: agency-bottom
description: …
model: grok-4.5
---

Body instructions…
```

Shipped profiles: `agency-bottom`, `dropshipping-pipeline`, `agency-coder`.

## Showcase commands

```bash
export PATH="$HOME/.grok/bin:$PATH"
cd /root/src/repos/ai-agency && source .venv/bin/activate
export PYTHONPATH=.

# Offload path (pipeline scripts via grok_build_offload_shell)
python -m scripts.showcase_grok_build_dropshipping_flow --skip-locate

# Live headless Grok Build turn (needs SuperGrok auth)
python -m scripts.showcase_grok_build_dropshipping_flow --try-grok-agent

# Raw CLI smoke
grok -p "Reply exactly: GROK_BUILD_OK" --always-approve --max-turns 1
```

Dropshipping loop agents should prefer:

1. Rank / locate / post_locate scripts via `grok_build_offload_shell`
2. Multi-step coding/refactors via `grok_build_run` or `grok_build_orchestrate_agency_task`
3. Never invent shell orchestration loops that Grok Build already covers

## Living README / CI

Any bottom-layer change updates README + `scripts/check_readme_freshness.py` must_exist paths (`tools/grok_build_tools.py`, `docs/GROK_BUILD_AND_CODERABBIT.md`, `configs/grok-build/agents/agency-bottom.md`, showcase script).

## Rollback note

If someone asks to “add Warp again,” point to this decision and keep Grok Build unless the user explicitly reverses it.
