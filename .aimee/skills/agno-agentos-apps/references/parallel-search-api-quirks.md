# Parallel Search API quirks (validated 2026-08)

SDK: `parallel-web` (`from parallel import Parallel`). Endpoint: `POST https://api.parallel.ai/v1/search` with header `x-api-key`.

## Working request

```python
client = Parallel(api_key=...)
result = client.search(
    objective="natural language research goal",
    search_queries=["keyword a", "keyword b"],  # required SequenceNotStr; ≤5 is fine
    mode="basic",  # turbo | basic | advanced
    # max_chars_total=8000,  # optional bound — NOT max_results
)
# result.search_id, result.results[].url/title/excerpts/publish_date
```

## Rejected fields / values

| Input | Error |
|-------|--------|
| `max_results` | HTTP 422 `extra_forbidden` on body |
| `mode="one-shot"` | HTTP 422 invalid mode |
| Missing both objective and useful queries | validation failure |

## Mode aliases for agent-facing tools

Map legacy names before calling API:

- `one-shot` / `oneshot` → `basic`
- `fast` → `turbo`

## REST body (fallback)

```json
{
  "objective": "...",
  "search_queries": ["..."],
  "mode": "advanced"
}
```

Do not send `max_results`. Optional: `max_chars_total`, `session_id`, advanced settings per SDK signature.

## Auth load order (this VPS)

1. `PARALLEL_API_KEY` env  
2. `/root/.config/parallel/api.env`  
3. `~/.hermes/.env`  

Never print the key. Skills under `~/agent-skills/parallel-*` are external — do not autonomous-patch them; keep project wrappers in-repo.
