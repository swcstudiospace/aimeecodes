# GenUI card contract (agency cockpit)

## Message shape

```ts
type ChatMessage = {
  id: string;
  role: "user" | "system" | "agent" | "swarm";
  agentId?: string;  // matches agents.ts id
  content: string;
  ts: number;
  genui?: GenUIBlock[];
};

type GenUIBlock = {
  kind: GenUIKind;
  title?: string;
  data: Record<string, unknown>;
};
```

## Kind → data fields

| kind | Required data fields |
|------|----------------------|
| `kpi_strip` | agents, teams, workflows, hitlOpen |
| `product_rank` | niche, winners[{name,score,price,cm,decision}] |
| `hitl_spend` | hitlId, channel, dailyUsd, durationDays, objective, status |
| `qa_gate` | sku, verdict, defects[], shipHold |
| `swarm_status` | active[], workflow |
| `workflow_progress` | items[{name,pct}] |
| `linear_issue` | key, title, state |
| `experiment` | hypothesis, metric, days, ice |
| `fraud_hold` | orderId, score, action |

## HITL

- Pending cards expose Approve/Reject → update `status` on the block **and** the HITL queue entry with the same `hitlId`.
- Never auto-approve from the UI mock without user click.

## Backend mapping (future)

| Card | AgentOS / tools |
|------|-----------------|
| product_rank | Product Scout / autonomous_product_rank |
| hitl_spend | spend_vault + Growth Media Buyer |
| qa_gate | qa_ops tools |
| linear_issue | linear_tools agency_track |
| experiment | experiment_ops |
