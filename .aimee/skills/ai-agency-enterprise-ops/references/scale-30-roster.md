# Agency scale roster (30 / 12 / 10)

Canonical counts after ops expansion. Prefer this over older “18 agents / 7 teams / 5 workflows” text in MCP skills until those are adopted/updated.

## Agents (30)

**Core 18:** hermes-ops, product-scout, supplier-sourcer, pricing-strategist, brand-strategist, creative-director, listing-specialist, seo-content, store-builder, compliance-officer, growth-media-buyer, influencer-manager, email-crm, customer-success, fulfillment-ops, inventory-planner, analyst, finance-controller

**Ops 12:** qa-inspector, returns-specialist, chargeback-specialist, cx-escalations, logistics-coordinator, ads-creative-ops, catalog-ops, risk-fraud-analyst, partnerships-manager, tax-compliance, community-manager, experimentation-lead

## Teams (12)

agency-director-team, research-team, supply-chain-team, creative-team, store-ops-team, growth-team, retention-team, **cx-operations-team**, **logistics-ops-team**, **growth-ops-team**, **risk-finance-ops-team**, **merchandising-team**

## Workflows (10)

full-product-lifecycle, marketing-launch, supplier-onboarding, post-purchase-ops, weekly-performance-review, **incident-response-ops**, **returns-rma-pipeline**, **creative-production-ops**, **experimentation-cycle**, **logistics-exception-handling**

## MCP run examples (ops)

```text
run_team(team_id="cx-operations-team", message="…")
run_team(team_id="logistics-ops-team", message="…")
run_workflow(workflow_id="returns-rma-pipeline", message="…")
run_workflow(workflow_id="incident-response-ops", message="…")
run_agent(agent_id="qa-inspector", message="…")
run_agent(agent_id="experimentation-lead", message="…")
```

## Ports

| Service | Port |
|---------|------|
| AgentOS | 7777 |
| Drop | 7788 |
| Hermes bridge | 7790 |
| Anda nexus | 8091 |
