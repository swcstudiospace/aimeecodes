# sem_search

`sem_search` is AI-powered semantic code search: natural-language queries find code by *behavior and intent*, not keyword matching, over the current workspace (cwd and subdirectories). Input type: `SemanticSearch` (`crates/aimee_domain/src/tools/catalog.rs:438-448`). Description source: `descriptions/semantic_search.md`. Requires workspace indexing ([`aimee workspace sync`](../../cli.md)) and the services endpoint (`services_url`, default `https://api.aimeecodes.dev/`).

## Parameters

`queries` — array of `SearchQuery` pairs (`catalog.rs:324-429`), each with two required strings:

| Field | Type | Required | Role |
|---|---|---|---|
| `queries[].query` | string | yes | Embedding query: **what** the code does (technical terms, algorithms, structure names) |
| `queries[].use_case` | string | yes | Reranking query: **why** you need it — intent + codebase construct keywords |

## Example

```json
{
  "name": "sem_search",
  "arguments": {
    "queries": [
      {
        "query": "exponential backoff retry mechanism with configurable delays",
        "use_case": "I need the struct definition and trait implementation for retry policy to modify the backoff strategy, not setup docs"
      },
      {
        "query": "streaming LLM responses with SSE chunked transfer encoding",
        "use_case": "Find the fn implementation that parses SSE chunks so I can add a new event type"
      }
    ]
  }
}
```

## Query craft

The tool contract is unusually opinionated because reranking quality depends on it:

- Use **2–3 varied queries** in parallel — each captures a different aspect ("user login verification", "token generation", "OAuth flow").
- `query` should be specific but not over-broad; `"retry"` alone matches everything and risks timeouts.
- `use_case` **must** include construct keywords for code hunts — `struct`, `trait`, `impl`, `function`, `fn`, `definition`, `type`. The reranker weights these heavily; omitting them returns documentation instead of code. It must also differ from `query` and state the goal ("to fix a bug", "to add a feature", "not tests").
- Intent-to-content matching: documentation intent → doc keywords; implementation intent → implementation terms.

## Behavior

- Returns the top-K most relevant `file:line` locations with code context; each query is ranked independently, then reranked against the `use_case`.
- Only indexes **inside `{{env.cwd}}`**. For paths outside the workspace, use [fs_search](fs_search.md) with an explicit `path`.
- Prefer `sem_search` when you'd struggle to write the regex; prefer `fs_search` for exact strings, identifiers, TODOs, or "all occurrences of X".
- No permission gate: it is a read-only query against the index (`catalog.rs:1008`).

## Errors

Un-synced workspace (run `aimee workspace sync`), unreachable `services_url`, or overly broad query sets (timeouts) are the common failure modes.

## Related

- [Tool catalog](catalog.md)
- [fs_search](fs_search.md) — exact-match alternative
- [Cloud and services](../../ops/cloud.md) — the workspace indexing service
- [CLI reference](../../cli.md) — `aimee workspace sync` / `query`
