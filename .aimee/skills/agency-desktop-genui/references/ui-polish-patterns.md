# Cockpit UI polish patterns (session)

Ship these when extending the mock before CopilotKit:

| Pattern | Implementation |
|---------|----------------|
| Command palette | ⌘/Ctrl+K overlay; Enter runs first match |
| View modes | Mission · Swarm (large constellation) · HITL focus |
| Agent filter | search box + All/Live/HITL chips |
| Inspector | selected agent tools/status/load on right rail |
| Typing | brief Hermes typing indicator before GenUI cards |
| Toasts | HITL approve/reject + dispatch feedback |
| Workflow strip | mini progress bars under topbar |
| Platform badge | `detectShell()` → Web PWA / Desktop / Mobile |
| Safe areas | `viewport-fit=cover` + `env(safe-area-inset-*)` |

GenUI polish: score bars on product_rank, CM color, gradient spend amounts, amber pending HITL cards, clickable constellation nodes.
