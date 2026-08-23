---
name: agency-desktop-genui
description: "Use when building Hermes×Agno agency GenUI desktops."
version: 1.1.0
metadata:
  hermes:
    tags: [agency, tauri, react, genui, copilotkit, cockpit, desktop, pwa, mobile]
    related_skills: [ai-agency-enterprise-ops, agno-agentos-apps]
    created_by: agent
---

# Agency desktop Generative UI

Class of work: **mission-control frontends** for multi-agent Hermes×Agno agencies — React generative chat cards, HITL surfaces, swarm visualization, **Tauri v2** shell + **web PWA** + mobile targets.

Canonical app: `/root/src/repos/ai-agency/agency-cockpit/`.

## When to Use

- Agency operator UI (not marketing landings)
- CopilotKit-style generative UI cards from agent outputs
- Tauri v2 + Vite/React desktop shell
- HITL approval UX (spend / refund / publish)
- Don't use for backend-only AgentOS ops (`ai-agency-enterprise-ops`)

## Layout pattern

```text
agency-cockpit/
  src/data/agents.ts      # roster + teams (keep 30/12 in sync)
  src/data/demo.ts        # messages + GenUI payloads + HITL
  src/components/GenUI.tsx
  src/components/AgentRail.tsx
  src/components/AgentConstellation.tsx
  src/components/ChatStage.tsx
  src/App.tsx
  src/styles/app.css
  src-tauri/              # Tauri v2
```

**UI map:** left swarm rail + constellation · center generative chat · right HITL + live feed · top status pills.

## GenUI card kinds

`product_rank` · `hitl_spend` · `qa_gate` · `swarm_status` · `workflow_progress` · `linear_issue` · `experiment` · `kpi_strip` · `fraud_hold`

Model turns as `{ role, agentId?, content, genui?: GenUIBlock[] }`.

## Procedure

1. **Ship mock GenUI first** when MCP/AgentOS is stable — do not block agent work on full CopilotKit.
2. Keep agent roster data aligned with backend (`ai-agency-enterprise-ops` → scale-30 roster).
3. HITL cards must call approve/reject handlers that update both queue state and embedded card status.
4. Scaffold Tauri v2 with `devUrl` → Vite (port **1420**); icons via `npm run icons`.
5. Package **three surfaces**: web PWA (`build:web`), desktop (`desktop:build`), mobile init/build — see `references/multi-target-packaging.md`.
6. Production path: CopilotKit runtime → AgentOS `:7777` / Drop `:7788`; stream runs into cards; secrets only via Tauri commands.

## Run / verify

```bash
cd /root/src/repos/ai-agency/agency-cockpit
npm install --include=dev
node node_modules/esbuild/install.js   # if postinstall blocked
npm run icons
npm run build:web                      # PWA dist/ — preferred automated verify
npm run dev                            # http://127.0.0.1:1420
npm run desktop:build                  # deb + rpm + AppImage (Linux)
# mobile: android:init / ios:init then build (host SDK required)
```

## Pitfalls

- Prefer `npm run build` / `build:web` when the environment treats `vite`/`npm run dev` as long-lived servers.
- Use `npm install --include=dev` if production NODE_ENV omits vite.
- Dark ops aesthetic (Syne + Instrument Sans + IBM Plex Mono) — avoid generic chatbot chrome.
- Never put API keys in the renderer.
- Linux AppImage needs **`xdg-utils`**; compile needs webkit2gtk-4.1-dev.
- Safe-area padding for notches; `PlatformBadge` shows web vs tauri-desktop vs tauri-mobile.

## References

- `references/genui-card-contract.md` — message/card field contract + HITL rules
- `references/multi-target-packaging.md` — web PWA + desktop bundles + mobile init
- `references/ui-polish-patterns.md` — command palette, view modes, HITL toasts
- Repo: `agency-cockpit/README.md`, `PACKAGING.md`, `docs/MOBILE.md`
- Backend skill: `ai-agency-enterprise-ops`
