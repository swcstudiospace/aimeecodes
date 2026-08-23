# Tool macros — aimee_tool_macros

The proc-macro crate that keeps tool descriptions honest: one derive, one file convention.

## What it provides

* **`ToolDescription` derive** — generates the description plumbing for tool types so each catalog variant carries its model-facing text without hand-written glue.
* **`tool_description_file`** — binds a tool to its markdown description file (`fs_read.md`, `semantic_search.md`, …) so text lives in editable files rather than string literals in code.

## Why it matters

Two policies depend on this machinery:

1. **1024-character cap** on tool descriptions (`docs/tool-guidelines.md`) — enforced at review time against the files this macro loads.
2. **No unregistered variants** — a tool type without a description file fails the pattern visibly instead of silently shipping an under-described tool.

Because descriptions are files, they're diffable, reviewable, and editable without recompiling logic — and the same text is what you see mirrored in the [Tool catalog](../reference/tools/catalog.md).

## Related

`aimee_json_repair` handles the other half of model-tool reliability: coercing slightly-broken model JSON into valid arguments before execution.

## See also

* [Tool catalog](../reference/tools/catalog.md)
* [Domain](domain.md)
* [Crate map](crates.md)

<!-- sources: AIMEE.md §5,§7, docs/tool-guidelines.md -->
