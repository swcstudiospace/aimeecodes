---
name: agno-agentos-apps
description: "Build Agno AgentOS apps with Grok and Parallel."
version: 1.4.0
author: Hermes Agent
license: MIT
platforms: [linux, macos, windows]
metadata:
  hermes:
    tags: [agno, agentos, grok, xai, parallel, multi-agent, workflows, skills, localskills, anda, kip, multi-agency]
    related_skills: [openhands-llm-auth, hermes-agent-skill-authoring, ai-dropshipping-agency-mcp, ai-agency-enterprise-ops, anda-ecosystem, hermes-project-soul]
---

# Agno AgentOS Apps (Grok + Parallel)

Scaffold and run multi-agent apps on **Agno AgentOS** using **xAI Grok** (OpenAI-compatible), **Parallel Web Systems** research tools, and **Agno LocalSkills** packs.

**Canonical examples on this VPS:**

| Repo | Domain | Port | Notes |
|------|--------|------|-------|
| `/root/src/repos/ai-agency` | Dropshipping | **7777** | Full enterprise + Drop `:7788` |
| `/root/src/repos/ai-clipping` | AI clipping | **7766** | Lean Anda/KIP base (10a/5t/4wf) |

When the user wants **Anda / iclabs / ldclabs as the base agent framework**, follow the clipping pattern: local `kip_memory/` + factory auto-attach `anda_brain` + optional `ANDA_NEXUS_URL`. Detail: `references/multi-agency-anda-scaffold.md`.

## When to Use

- New Agno `Agent` / `Team` / `Workflow` + `AgentOS` project
- **Second/third agency** on the same host (port isolation, shared SuperGrok, forked foundation)
- Expand an agency with domain agents (ops, retention, compliance, finance, clipping)
- Wire Grok via SuperGrok OAuth or `XAI_API_KEY`
- Attach Parallel Search/Extract/Task/FindAll/Monitor as agent tools
- Author Agno `skills/<pack>/<name>/SKILL.md` (not Hermes skills)
- SOTA agent design: markdown personas, scoped skills, toolbelts, output schemas, evals
- Enable AgentOS as MCP server for Hermes (or other MCP clients)
- Hybrid FastMCP + ACP gateway on one port (see Drop pattern)
- Fix paste-pack scaffolds that break on current Agno or Parallel APIs

Don't use for: day-to-day *operating* the live dropshipping agency from Hermes (use `ai-agency-enterprise-ops` + `ai-dropshipping-agency-mcp`); raw Parallel CLI only (`parallel-web` / `parallel-*`); OpenHands dual-auth UI (`openhands-llm-auth`); Hermes skill authoring (`hermes-agent-skill-authoring`).

## Prerequisites

- Project venv (never pollute system/Hermes site-packages)
- `PARALLEL_API_KEY` — env, or `/root/.config/parallel/api.env`, or `~/.hermes/.env`
- xAI: `XAI_API_KEY` **or** device-code OAuth (see below)
- Offline Agno docs: skill `docs-agno` + FTS DB (external corpus; do not patch that skill)
- `PyYAML` comes with Agno for LocalSkills frontmatter

```bash
python3 -m venv .venv && source .venv/bin/activate
pip install -U "agno[os,mcp]" openai httpx python-dotenv uvicorn parallel-web \
  opentelemetry-api opentelemetry-sdk openinference-instrumentation-agno \
  fal-client agent-client-protocol
# fastmcp comes via agno[mcp] — required for AgentOS(mcp_server=...)
```

## Canonical layout (SOTA)

```text
app/main.py                 # AgentOS + optional mcp_server=MCPServerConfig
agents/_factory.py          # persona md + scoped skills + toolbelts + schemas
agents/profiles.py          # declarative per-agent config
agents/prompt_loader.py     # SOUL/SYSTEM/OUTPUT/EXAMPLES
agents/schemas.py           # Pydantic handoff contracts
agents/__init__.py          # LIGHT only — no eager roster imports
agents/*.py                 # thin wiring only
prompts/<agent>/{SOUL,SYSTEM,OUTPUT,EXAMPLES}.md
prompts/teams/<team>/SYSTEM.md
teams/_factory.py + teams/*.py
workflows/*.py              # Step(name=..., team=...)
tools/xai_oauth_pkce.py | xai_model.py | parallel_tools.py
tools/skills_loader.py      # skills_for(*names) scoped LocalSkills
tools/toolbelts.py          # role-shaped tool sets
tools/guardrails.py         # L2 autonomy tool_hooks
tools/mcp_custom.py         # optional custom /mcp tools
tools/anda_knowledge.py     # Anda brain tools (when Anda-base)
kip_memory/                 # local KIP + Brain (when Anda-base)
knowledge/anda/             # offline ldclabs docs slice
skills/{agency,<domain>,agents}/<name>/SKILL.md
evals/run_agent_evals.py    # structural evals
tests/test_*.py             # pytest multi-gate (required for new agencies)
scripts/start_agentos.sh
SOUL/ + AGENTS.md + .hermes.md
```

## Procedure

1. **Model factory (import-safe)**  
   Resolve token inside `get_grok_model()`; on missing creds use a placeholder key and log — do **not** raise at import so AgentOS can still load. Runtime calls fail clearly until login.

2. **xAI credentials (order)**  
   1. `XAI_API_KEY`  
   2. Project store `~/.config/<agency-slug>/xai_oauth.json` (mode 600) — e.g. `ai-agency` or `ai-clipping`  
   3. Fallback: sibling agency store and/or Hermes `~/.hermes/auth.json` provider `xai-oauth`  
   CLI: `python -m tools.xai_oauth_pkce login|status|logout`  
   Flow is **device code** (RFC 8628), not browser PKCE — filename is historical.  
   When forking OAuth helper code, retarget `DEFAULT_TOKEN_PATH` and allow reading the other agency’s store so SuperGrok login is shared.

3. **SOTA agent brain (prefer over inline instruction lists)**  
   - **Always-on:** `prompts/<agent>/SOUL.md` + `SYSTEM.md` → `instructions`  
   - **Contract:** `OUTPUT.md` → `expected_output` + Pydantic `output_schema`  
   - **Few-shot:** `EXAMPLES.md` → `additional_input`  
   - **On-demand playbooks:** Agno LocalSkills via `skills_for("a","b")` — **not** the full library on every agent  
   - **Tools:** named toolbelts (`parallel_research`, `economics`, …) — not “everyone gets Parallel”  
   See `references/persona-packs-and-toolbelts.md`.

4. **Agent factory**  
   ```python
   build_agent(
     name=..., role=..., persona="product_scout",
     toolbelts=("parallel_research","economics","linear"),
     skill_names=("product-scoring","unit-economics"),
     output_schema=ProductCandidateBatch,
     add_history_to_context=False,  # True for Ops/CS/Growth/Finance
     use_json_mode=True,
   )
   ```
   Keep `agents/*.py` thin; iterate prompts in markdown.  
   Anda-base factories also auto-append `anda_brain` + `hermes_bridge` unless `AGENCY_DISABLE_*`, and inject a short recall→formation protocol (no secrets in KIP).

5. **Agents / teams**  
   **Teams must use the enum**, not a bare string:
   ```python
   from agno.team.mode import TeamMode
   Team(mode=TeamMode.coordinate, members=[...], skills=skills_for(...), ...)
   ```
   Do **not** pass `show_tool_calls` (removed on current Agno Agent).  
   Scope team skills to the union of member domains — avoid global skill dump.

6. **Workflows**  
   Use `from agno.workflow import Step, Workflow` and `Step(name=..., team=...)`.  
   Dict steps `{"name": ..., "team": ...}` are outdated and break.

7. **Agno LocalSkills**  
   `skills_for(*names)` loads individual skill folders (exact name match). Empty `skills_for()` = all packs (legacy).  
   Each skill folder name == frontmatter `name` (lowercase, hyphens). Progressive tools: `get_skill_instructions` / `get_skill_reference` — skill names are **not** callable.  
   See `references/agno-localskills.md`.

8. **AgentOS + optional MCP server**  
   ```python
   from agno.os.config import MCPServerConfig
   agent_os = AgentOS(
     ...,
     tracing=True,
     mcp_server=MCPServerConfig(
       enable_builtin_tools=True,
       tools=get_mcp_custom_tools(),  # optional
       allowed_hosts=["localhost","127.0.0.1"],
       result_mode="trimmed",
     ),
   )
   ```
   Requires `agno[mcp]` / `fastmcp`. Endpoint: **`/mcp`** (Streamable HTTP).  
   Built-ins: `get_agentos_config`, `run_agent`, `run_team`, `run_workflow`, `continue_run`, `cancel_run`, `get_sessions`, `get_session_runs`.  
   Hermes registration: skill `ai-dropshipping-agency-mcp` + `references/agentos-mcp-server.md`.

9. **Hybrid FastMCP + ACP gateway (optional second port)**  
   When external clients need Linear + reasoning + lifecycle on one URL (ai-agency Drop pattern):
   ```python
   mcp = FastMCP("drop-…")
   mcp_app = mcp.streamable_http_app()  # already routes /mcp

   @asynccontextmanager
   async def lifespan(app):
       async with mcp.session_manager.run():  # REQUIRED
           yield

   Starlette(routes=[Route("/health", …), Mount("/", app=mcp_app)], lifespan=lifespan)
   ```
   ACP stdio: `agent-client-protocol` + `python -m drop_server.acp_agent`.  
   Ops details: skill `ai-agency-enterprise-ops` → `references/drop-mcp-acp-gateway.md`.

10. **Parallel tools**  
    Prefer `parallel` SDK; REST fallback with `x-api-key`. Task: `parallel_task(..., processor="ultra", wait=True, timeout_s=3600)` for deep research. Search quirks below.

11. **Autonomy hooks**  
    Default L2: `tool_hooks` block irreversible live actions and agent self-confirm. Dropshipping: active Shopify publish / spend confirm; Meta/TikTok launch needs HITL tokens. Clipping: live publish + rights auto-clear blocked; `request_publish_approval` surfaces human code once.

12. **End-to-end agency shapes**  
    - Dropshipping: Research → Supply → Creative → Store Ops + Compliance → Growth → Retention → Director/Finance.  
    - Clipping: Source → Transcript → Viral moments → Edit → Captions → Package → Rights → HITL publish → KIP learnings.

13. **Scaffold a new agency from an existing one (multi-agency)**  
    Prefer **copy foundation modules**, not the whole roster. Full checklist: `references/multi-agency-anda-scaffold.md`.
    1. Empty repo + project `.venv`.
    2. Copy reusable tools + optional entire `kip_memory/` + Anda docs slice.
    3. **Rewrite** `agents/__init__.py` light (never copy ai-agency eager imports).
    4. Retarget identity: OAuth path, Parallel UA, KIP `$self`, `formation(space=)`, skill roots, AgentOS id/port.
    5. Unique port (7777 taken by dropshipping → clipping uses **7766**).
    6. Domain tools + thin agents + personas + profiles + teams + workflows.
    7. Hermes SOUL wiring (`hermes-project-soul`).
    8. Gates: structural evals **and** pytest; offline env disables bridge/brain/knowledge.
    9. Reset `kip_memory/data/*.db` after genesis identity retarget if already bootstrapped.

## Pitfalls

- **`Team(mode="coordinate")` breaks AgentOS `/teams`** — schema does `team.mode.value`. Always `TeamMode.coordinate`.
- **LocalSkills YAML** — quote `description:` when it contains `:`.
- **Directory name == skill `name`** — lowercase hyphens only.
- **Do not load all skills on every agent** — scopes tokens and attention; Creative should not load `supplier-vetting`.
- **Inline instruction lists do not scale** — move brains to `prompts/<agent>/*.md`; keep Python thin.
- **Parallel Search has no `max_results`** — 422 `extra_forbidden`. Use `max_chars_total` if needed.
- **Search modes** — only `turbo` | `basic` | `advanced` (alias `fast`→`turbo`, `one-shot`→`basic`).
- **Task ultra is slow** — `wait=True` + timeout ≥ 3600; Hermes MCP server timeout must match.
- **OAuth HTTP 403 on refresh** — tier gate; use `XAI_API_KEY`; do not loop device-code.
- **MCP needs `agno[mcp]`/fastmcp** — without it `mcp_server=True` fails at get_app/serve.
- **MCP tools inject at Hermes process start** — restart TUI/gateway after `hermes mcp add`.
- **`hermes mcp add` is interactive** — non-interactive: `printf 'n\nY\n' | hermes mcp add ...` (n = no auth for local open MCP).
- **Custom FastMCP + Starlette** — lifespan `session_manager.run()` required; mount at `/` not nested `/mcp`.
- **Autonomy L2** — never let agents self-confirm spend/publish; launch/publish needs HITL tokens.
- **Tracing deps** — opentelemetry + openinference-instrumentation-agno.
- **Install into project `.venv`** — never global hermes site-packages.
- **Hermes skills ≠ Agno skills** — different packages and frontmatter.
- **DIM shipping** — `billable_g = max(weight_g, L*W*H/5)` grams; no extra ×1000.
- **Copying `agents/__init__.py` from ai-agency** — eager-imports every dropshipping agent → `ModuleNotFoundError`. Keep `__init__` empty/light.
- **Port clash** — second AgentOS must not bind **7777** if dropshipping is live; clipping uses **7766**.
- **Forked `kip_memory` still says Dropshipping** — retarget nexus `$self` + `formation(space=...)` + domain genesis capsule; delete local `nexus.db` after fix.
- **skills_loader roots** — new agencies need their own pack dirs (e.g. `skills/clipping/`), not dead `marketing/`/`ops/` copies.
- **Structural evals alone** — also ship `tests/` pytest; prefer multi-gate verification over “smoke only”.

## Verification

```bash
source .venv/bin/activate && export PYTHONPATH=.
export XAI_API_KEY=missing-ci-placeholder
export AGENCY_DISABLE_HERMES_BRIDGE=1 AGENCY_DISABLE_ANDA_BRAIN=1 AGENCY_DISABLE_ANDA_KNOWLEDGE=1
ruff check agents app tools teams workflows evals scripts kip_memory tests
pytest tests/ -q
python -m evals.run_agent_evals
python -m tools.xai_oauth_pkce status
# Ports: ai-agency :7777 · ai-clipping :7766 · drop :7788 · hermes-bridge :7790
curl -s localhost:7777/health            # dropshipping
curl -s localhost:7766/health            # clipping
curl -s localhost:7777/info | jq .mcp
curl -s localhost:7777/teams             # TeamMode enum
hermes mcp test ai-agency
hermes mcp test ai-clipping              # after mcp add :7766/mcp
hermes mcp test drop                     # if Drop registered
```

Live Parallel smoke: `parallel_search(..., mode="basic")` → `search_id` + results.

Ad-hoc verify pattern: `/tmp/hermes-verify-*.py` with project venv, delete after (not CI).

## References

- `references/multi-agency-anda-scaffold.md` — fork ai-agency → new domain (clipping) with Anda/KIP base
- `references/persona-packs-and-toolbelts.md` — SOTA agent layering (md + skills + schemas)
- `references/agentos-mcp-server.md` — MCPServerConfig, built-ins, Hermes register
- `references/parallel-search-api-quirks.md` — Search request shape
- `references/parallel-task-ultra.md` — Task wait/ultra product-rank pattern
- `references/xai-device-code-standalone.md` — OAuth module contract
- `references/agno-localskills.md` — LocalSkills validation
- `references/agentos-team-mode.md` — TeamMode + `/teams`
- `references/enterprise-lifecycle-tools.md` — lifecycle tool map
- `references/hitl-spend-and-ads.md` — spend vault + Meta/TikTok
- Live dropshipping ops: skill **`ai-agency-enterprise-ops`** + user-owned **`ai-dropshipping-agency-mcp`**
- Anda/KIP ops: skill **`anda-ecosystem`** · SOUL wiring: **`hermes-project-soul`**
- Drop hybrid gateway ops: `ai-agency-enterprise-ops` → `references/drop-mcp-acp-gateway.md`
- Offline Agno: `docs-agno` · Parallel umbrella: `parallel-web` (external)
- SuperGrok protocol: `openhands-llm-auth` → `references/xai-supergrok-oauth.md`
