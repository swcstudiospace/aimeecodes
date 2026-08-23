# Services — aimee_services

Application services: the use-case layer. One struct, `AimeeServices<F>`, generic over its infrastructure — exactly one generic parameter, `Arc<T>` fields, bounds on methods rather than the constructor.

## The shape

```rust,ignore
pub struct AimeeServices<F> {
    // infra held as Arc<F>, composed at AimeeAPI::init
}

impl<F> AimeeServices<F> {
    pub fn new(...) -> Self { /* no trait bounds here */ }
}

impl<F: SomePort> AimeeServices<F> {
    pub fn use_case(&self, ...) -> anyhow::Result<...> { /* bounds only where needed */ }
}
```

This is the canonical service pattern for the whole workspace (house policy in AGENTS.md): services never call each other; when two use cases collaborate they compose at the composition root. No `Box<dyn Trait>` in service fields.

## Error convention

Services return `anyhow::Result`. Distinct failure modes are preserved by converting explicitly with context at call sites — there are no blanket `From` impls collapsing different errors into one variant.

## What lives here

Use cases around conversations, providers/auth flows, workspaces, commands, and data generation surface through this crate and are wired into `AimeeAPI`. The concrete inventory shifts with features; the composition root (`aimee_api`) is the truthful index of what exists — see [API composition root](api.md).

## See also

* [Infrastructure](infra.md)
* [Domain](domain.md)
* [Architecture overview](overview.md)

<!-- sources: AGENTS.md Rust service shape, crates/aimee_services, crates/aimee_api/src/aimee_api.rs -->
