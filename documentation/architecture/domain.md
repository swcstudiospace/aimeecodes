# Domain — aimee_domain

The innermost layer: types, rules, and catalogs with no knowledge of HTTP, databases, or terminals.

## What lives here

* **Agent model** — `AgentId` (`AIMEE`, `MUSE`, `SAGE` as first-class values), agent definitions.
* **Tool catalog** — the sixteen registered tools (`ToolCatalog` in `src/tools/catalog.rs`), each pairing a variant with its description file. Description text is capped at 1024 characters by policy.
* **Providers** — 42 built-in `ProviderId` constants plus wire-protocol knowledge; `built_in_providers()` is the registry.
* **Loop autonomy** — HITL goal probes (`GoalProbe`, `GoalProbeSet`), prompt depth/uplift types, failure-budget semantics (`src/loop_autonomy.rs`).
* **Policies** — what agents may do, expressed as data rather than scattered checks.

## Design rules

Errors are `thiserror`-derived domain errors. The crate deliberately does **not** implement blanket `From` conversions that would collapse distinct failures into one variant — call sites convert explicitly so context survives. No `unwrap`/`expect` on user input. Newtypes over strings for identifiers.

This crate depends on almost nothing outside serde-adjacent fundamentals, which is what lets every other layer depend on it safely.

## When you touch it

Adding a tool means adding a catalog variant **and** its description file **and** registering it in the executor/registry path — an unregistered variant is a defect. Adding a provider means a constant plus display name plus protocol mapping. Both flows have reference pages: [Tool catalog](../reference/tools/catalog.md), [Providers](../integrations/providers.md).

## See also

* [Application](app.md)
* [Tools: how agents touch your system](../concepts/tools-overview.md)

<!-- sources: AIMEE.md §4,§5,§7, crates/aimee_domain/src/{agent,provider,loop_autonomy}.rs -->
