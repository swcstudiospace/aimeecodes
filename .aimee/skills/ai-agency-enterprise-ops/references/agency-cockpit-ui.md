# Agency Cockpit UI (React + Tauri v2 GenUI mock)

Path: `/root/src/repos/ai-agency/agency-cockpit/`

## Purpose

Desktop/web **mission control** for the 30-agent agency: swarm rail, generative chat cards (CopilotKit-*style*), HITL queue. High-fidelity **mock** with simulated swarm — wire CopilotKit runtime → AgentOS MCP next.

## Stack

| Layer | Choice |
|--------|--------|
| UI | React 19 + Vite 6 + TypeScript |
| Desktop | Tauri v2 (`src-tauri/`) |
| GenUI | Custom card renderers (not full CopilotKit yet) |
| Icons | lucide-react |

## Run

```bash
cd /root/src/repos/ai-agency/agency-cockpit
npm install --include=dev
# If esbuild postinstall blocked by npm allow-scripts:
node node_modules/esbuild/install.js
npm run dev          # http://127.0.0.1:1420
npm run build        # tsc -b && vite build → dist/
# npm run tauri:dev  # needs system WebView deps
```

## Layout

- **Left:** 30 agents / 12 teams + constellation SVG (Hermes center)
- **Center:** Generative mission chat
- **Right:** HITL approve/reject + live tool feed
- **Top:** 30/12/10 + AgentOS/Bridge/HITL pills

## GenUI card kinds

`product_rank` · `hitl_spend` · `qa_gate` · `swarm_status` · `workflow_progress` · `linear_issue` · `experiment` · `kpi_strip` · `fraud_hold`

Data: `src/data/agents.ts`, `src/data/demo.ts`. Renderer: `src/components/GenUI.tsx`.

## Production path (next)

1. `@copilotkit/react-core` + runtime proxy to AgentOS `:7777` / Drop `:7788`
2. Stream Agno run events → GenUI actions; HITL via `renderAndWaitForResponse`
3. Tauri commands for secrets (never in renderer)
4. Deep-link Linear + KIP recall panels

## Pitfalls

- Prefer **mock GenUI first** when AgentOS surface is stable — don’t block agent work on full CopilotKit
- Some runners treat bare `vite`/`npm run dev` as long-lived servers; for CI-ish verify use `npm run build` with log redirect
- `npm install` may need `--include=dev` if NODE_ENV=production omits vite
