# Search and web tools

How agents find things: regex search across your tree, semantic search across your indexed workspace, and fetch for the outside web.

## `fs_search`

Regex search built on ripgrep. Full regex syntax, file filtering by glob (`*.js`, `**/*.tsx`) or language type (`rust`, `py`). The description explicitly forbids shelling out to `grep`/`rg` — this tool exists so permissions and access rules are enforced uniformly.

## `sem_search`

AI-powered semantic code search — the default discovery tool inside the workspace. Natural-language queries about behavior and concepts ("where do we validate webhook signatures"), not just keyword matching. Backed by workspace indexing (`aimee workspace` commands manage sync; see [Session model switching actions] in the ZSH dispatcher for sync status).

Use it when exploring unfamiliar code, finding feature implementations, or understanding cross-file patterns; drop to `fs_search` when you need exact-string precision.

## `fetch`

Retrieves URL content as markdown or raw text:

* Handles HTTP/HTTPS; converts HTML to readable markdown by default.
* Text-only — rejects binary downloads with an error (the description directs binary needs to `shell` + `curl -fLo`).
* Respects robots.txt; anti-scraping measures may block.
* Large pages return the first 40,000 characters with the complete content stored to a temporary file for follow-up reads.

Cannot access private/auth-gated resources — anything needing credentials is out of scope by design.

## Choosing between them

| Need | Tool |
|---|---|
| Exact string/regex in the tree | `fs_search` |
| "Where is X implemented?" | `sem_search` |
| External documentation/API reference | `fetch` |
| Downloading binaries | `shell` (curl), not `fetch` |

## See also

* [Filesystem tools](filesystem-tools.md)
* [Workflow tools](workflow-tools.md)
* [Tool catalog](catalog.md)

<!-- sources: crates/aimee_domain/src/tools/descriptions/{fs_search,semantic_search,net_fetch}.md -->
