# Session learnings — scale, dual-write, cockpit (2026-08)

Complement to SKILL.md body (update body when curator can load SKILL.md without dedup).

## Scale

- Live AgentOS: **30 agents / 12 teams / 10 workflows**
- Roster: `references/scale-30-roster.md`
- Docs: `docs/AGENCY_EXPANSION_30.md`, `docs/OPS_AGENTS_SOTA.md`

## User preference — agent archetype

When adding agents, match original 18 package:

- `prompts/<key>/{SOUL,SYSTEM,OUTPUT,EXAMPLES}.md` total **≥ ~5–6KB**
- Domain skill + agent playbook + dedicated ops toolbelt + schema + main registry
- Thin stubs are **not done** — see `references/ops-agent-sota-archetype.md`

## Dual-write brain

- `kip_memory/dual_write.py` + `ANDA_NEXUS_URL=http://127.0.0.1:8091`
- Formation expect `remote_ok_ratio ≈ 1.0`
- Search: `SEARCH CONCEPT "q" LIMIT n`
- Props need `$PropositionType` via `ensure_agency_schema_remote()`
- Timer: **`daily-brain-maintenance.timer`** → `python -m scripts.daily_brain_maintenance`
- Analytics belt ≠ KIP brain — `references/kip-dual-write-and-analytics.md`

## Cockpit UI

- Path: `agency-cockpit/` React+Vite+Tauri v2 GenUI mock
- Dev `:1420` · verify `npm run build`
- Detail: `references/agency-cockpit-ui.md`
- Class skill (if present): `agency-desktop-genui`

## Implementation pitfalls discovered

- Workflow import: `from teams.agency_director import agency_director_team` (module stem)
- Skill loader last-wins on folder name — unique names across ops/agents
- Prefer skill id `compliance-ads-claims` (no `claims-compliance` alias)
- Playwright in MCP: ThreadPoolExecutor (hermes_bridge)
- MCP tool wrapper first arg must not be named `name`
- Vite/esbuild: `--include=dev`; `node node_modules/esbuild/install.js` if scripts blocked
- Automated runners may treat `vite` as long-lived — prefer build+log file

## Protected / adopt

- `anda-ecosystem` may be user-owned — `hermes curator adopt anda-ecosystem` before curator patches
- `ai-dropshipping-agency-mcp` may still say 18 agents until adopted/updated — trust this skill’s roster
