# omega_anda / omega_anda_icp crate map

## Modules

### omega_anda

```text
src/
  domain/     pathway, checkpoint, receipt, error
  infra/      PathwayStore, KipBackend, EternalStore (+ Noop*)
  backends/   any_kip, memory_store, file_store, nexus_http
  services/   pathway_service (SessionPathwayService)
  hook.rs     PathwayLogHook (Clone; hard_fail; soft-fail default)
```

### omega_anda_icp

```text
src/
  local_receipt.rs  LocalReceiptEternalStore + PathwayCapsule
  store.rs          EternalStoreConfig + build_eternal_store
  error.rs          IcpError
```

### omega_config / omega_app (wiring)

```text
omega_config/src/anda.rs     AndaConfig, AndaEternalMode
omega_app/src/anda_pathway.rs  factory + chain helpers
omega_app/src/app.rs           chat() hook registration
```

## Checkpoint kinds

`UserTurn | AgentResponse | ToolEnd | TurnEnd | Rollback | Manual`

## Hash chain

- Seq 1-based; first `parent_hash = "genesis"`
- `content_hash = hex(sha256(parent|seq|kind|message_count|serde_json(conversation)))`
- Append validates parent link; rollback verifies chain through target then `truncate_after` + Rollback marker

## File pathway layout

```text
{root}/{conversation_id}/pathway.json
{root}/{conversation_id}/checkpoints/{seq:020}.json
```

## Local eternal layout

```text
{root}/{conversation_id}/{seq:020}-{hash12}.capsule.json
{root}/{conversation_id}/{seq:020}-{hash12}.receipt.json
```

Capsule `schema`: `omega.anda.pathway_capsule.v1`

## Workspace wiring

Root `Cargo.toml` workspace deps:

```toml
omega_anda = { path = "crates/omega_anda" }
omega_anda_icp = { path = "crates/omega_anda_icp" }
```

Members via `crates/*` — do not leave temporary `exclude` for these packages.

`omega_app` also depends on both path crates.

## Verify

```bash
cargo test -p omega_anda -p omega_anda_icp -p omega_config -p omega_app --lib
# ~11 + 3 + 30 + 724 as of 2026-08-19 (app includes 2 pathway factory tests)
cargo test -p omega_config --test schema   # refresh omega.schema.json
```

## Crates.io pins used

- Optional: `anda_kip = "0.11"` behind feature `kip`
- Nexus on host: `anda_cognitive_nexus_server` 0.11.x at `:8091`
