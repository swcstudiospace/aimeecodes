# Parallel Task ultra + product-rank pipeline

## API

```python
parallel_task(
  objective=...,
  processor="ultra",  # lite|base|core|pro|ultra
  output_schema={...},  # task_spec.output_schema (json schema or text)
  wait=True,
  timeout_s=3600,
)
# result: { task_id, status, output, basis? }
parallel_task_result(run_id, timeout_s=3600)
```

SDK: `client.task_run.create` then `client.task_run.result(run_id, api_timeout=...)`.  
REST: `POST /v1/tasks/runs` then long-poll `GET /v1/tasks/runs/{id}/result`.

## Agency autonomous ranker

```bash
cd /root/src/repos/ai-agency && source .venv/bin/activate
PYTHONPATH=. python -m scripts.autonomous_product_rank \
  --niche "..." --processor ultra --timeout 3600
# outputs: tmp/runs/product_rank_<ts>.{json,md}
```

Pipeline: Parallel Search (advanced) → Task ultra structured candidates → local `contribution_margin` composite score → optional Research Team synthesis.

Via Hermes MCP: `mcp_ai_agency_run_product_rank` then `read_product_rank_report(name="latest")`.

## Pitfalls

- Ultra can take many minutes; do not use 120s MCP timeouts.
- Prefer structured `output_schema` type json for rankers; parse fences if model returns markdown.
- Always re-score CM locally — do not trust model-only arithmetic.
- Planning COGS ⇒ prefer TEST over GO until landed quotes.
