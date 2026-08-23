# Omega app wiring for Anda pathways

## Config surface (`omega_config`)

- Module: `crates/omega_config/src/anda.rs`
- Types: `AndaConfig`, `AndaEternalMode::{Local,IcOss,Canister,S3}`
- On `OmegaConfig`: `pub anda: Option<AndaConfig>` (`skip_serializing_if = None`)
- Defaults documented (commented) in `crates/omega_config/.omega.toml`
- Schema: `cargo test -p omega_config --test schema` → updates `omega.schema.json`

Key fields: `enabled`, `pathway_dir`, `nexus_url`, `kip_enabled`, `eternal_enabled`, `eternal_mode`, `eternal_dir`, `eternal_label_prefix`, `log_responses`, `log_turn_end`, `hard_fail`.

## App factory (`omega_app::anda_pathway`)

File: `crates/omega_app/src/anda_pathway.rs`

| Fn | Role |
|----|------|
| `resolve_pathway_dirs(anda, omega_home)` | default `{home}/pathways` + `{pathways}/eternal` |
| `maybe_pathway_hooks(anda, omega_home, agent_id)` | `None` if disabled; else builds service + hooks |
| `chain_on_response(base, pathway)` | `EventHandleExt::and` when hooks present |
| `chain_on_end(base, pathway)` | same for turn end |

Internals:
- Store: `FilePathwayStore`
- KIP: `AnyKipBackend::from_nexus_url` (only remote when `kip_enabled` and URL set)
- Eternal: private `AppEternalStore::{Noop,Local}` implementing `EternalStore`; non-local modes warn and fall back to local
- Service options map from `AndaConfig` into `PathwayLogOptions`

## Orchestrator hook-up (`omega_app::app`)

In `OmegaApp::chat` after title/tracing handlers:

1. `pathway_hooks = omega_config.anda.as_ref().and_then(|a| maybe_pathway_hooks(a, ConfigReader::base_path(), agent.id.to_string()))`
2. Build normal `on_end` / response handlers
3. `on_end_hook = chain_on_end(on_end_hook, pathway_hooks.as_ref())`
4. `on_response_hook = chain_on_response(tracing.and(compaction), pathway_hooks.as_ref())`
5. Attach to `Hook::default().on_response(...).on_end(...)`

Deps: `omega_app` → `omega_anda` + `omega_anda_icp` workspace paths.

## Runtime layout

```text
~/.omega/pathways/{conversation_id}/pathway.json
~/.omega/pathways/{conversation_id}/checkpoints/{seq:020}.json
~/.omega/pathways/eternal/{conversation_id}/{seq}-{hash12}.capsule.json
~/.omega/pathways/eternal/{conversation_id}/{seq}-{hash12}.receipt.json
```

(`OMEGA_CONFIG` / existing `omega`|`.omega`|`forge` home resolution via `ConfigReader::base_path`.)

## Still open

- CLI: `omega conversation pathway list|show|rollback <seq>`
- Wire rollback into conversation repository restore
- Real ICP/ic-oss clients behind `eternal_mode`
