---
name: omega-anda-pathways
description: "Use when wiring Anda/KIP chat pathways into Omega Loops."
version: 1.1.0
author: Hermes Agent
license: MIT
platforms: [linux, macos, windows]
metadata:
  hermes:
    tags: [omega, anda, kip, pathways, icp, rollback, rust, crates]
    related_skills: [anda-ecosystem, plan, framework-monorepo-streamline]
---

# Omega × Anda session pathways

Integrate **LDC Labs Anda/KIP** into Omega Loops as **eternal chat pathways** — not as a replacement agent runtime.

**Upstream:** https://github.com/ldclabs (people say “iclabs”; correct to **ldclabs**). Product: https://anda.ai/

## When to use

- Add Anda/KIP memory or ICP durability to Omega
- Session pathway logging, conversation rollback (chat only)
- New `omega_anda*` crates following Omega clean architecture
- Don't use for: file undo (`omega_snaps`), full `anda_engine` swap-in, generic RAG, **DevPod/codespace sandboxes** (`omega-loops-cli` / `omega pod`), or **dTEE agent isolation** (not in this Omega tree — do not invent an Anda dTEE API)

## Design rules

1. **Omega stays the orchestrator** — Anda is memory + pathway durability only.
2. **Chat ≠ files** — pathways snapshot `Conversation`; workspace undo stays snaps.
3. **Clean crates** — domain/traits/services in `omega_anda`; eternal backends in `omega_anda_icp`.
4. **Light default deps** — file store + HTTP nexus + local receipts; feature-gate `anda_kip` / embedded nexus / ic-oss.
5. **Plans** — write under repo `plans/YYYY-MM-DD-*-vN.md` (Omega convention) and optionally `.hermes/plans/`.

## Crate map (shipped)

| Crate | Role |
|-------|------|
| `omega_anda` | Domain pathway types, traits, memory+file stores, `AnyKipBackend`, `NexusHttpBackend`, `SessionPathwayService`, `PathwayLogHook` |
| `omega_anda_icp` | `LocalReceiptEternalStore`, capsule `omega.anda.pathway_capsule.v1`, `build_eternal_store`, IcOss/Canister/S3 stubs |
| `omega_config` | `AndaConfig` / `AndaEternalMode`; `OmegaConfig.anda: Option<AndaConfig>` (off by default) |
| `omega_app` | `anda_pathway` factory; chains hooks onto orchestrator `on_response` + `on_end` when enabled |

Repo: `/root/src/repos/omegaloops`  
Plan: `plans/2026-08-19-anda-kip-eternal-pathways-v1.md`  
Detail: `references/crate-map.md` · `references/app-wiring.md`

## Enable (user)

In `~/.omega/.omega.toml` (or project config):

```toml
[anda]
enabled = true
kip_enabled = true
nexus_url = "http://127.0.0.1:8091"
eternal_enabled = true
eternal_mode = "local"
eternal_label_prefix = "omega"
log_responses = true
log_turn_end = true
hard_fail = false
```

Defaults (when enabled without overrides):
- Pathways: `{ConfigReader::base_path()}/pathways` → usually `~/.omega/pathways`
- Eternal: `{pathway_dir}/eternal`
- Best-effort log failures (`hard_fail = false`) so durability never blocks a turn

## Workflow — new pathway feature

1. Read Omega layering: `omega_domain` types/traits, `omega_app` hooks, `omega_services` tuple-struct + `Arc<T>`, AGENTS.md tests.
2. Put Anda integration in **dedicated crates** (heavy deps stay out of `omega_domain`).
3. Domain first: checkpoint kinds, hash chain, receipts, `thiserror` domain errors (no `From` auto-convert between domain errors).
4. Traits: `PathwayStore`, `KipBackend`, `EternalStore` — services take `Arc` infra, constructor without bounds.
5. Backends: memory (tests), file (dev), HTTP nexus (`POST {url}/kip`, `params:{"command":...}`), `AnyKipBackend` for app wiring.
6. Hook: `PathwayLogHook` implements `EventHandle` for `Response` / `End`; put **trait bounds on methods that call the service** (not only on the EventHandle impl).
7. Wire via `omega_app::anda_pathway::maybe_pathway_hooks` + `chain_on_response` / `chain_on_end` in `app.rs` when `anda.enabled`.
8. Config: add nested section on `OmegaConfig`, export from `lib.rs`, regenerate `omega.schema.json` (`cargo test -p omega_config --test schema`).
9. Eternal: local sha256 capsule+receipt first; non-local modes fall back to local with a warning until clients exist.
10. Verify:
    ```bash
    cargo test -p omega_anda -p omega_anda_icp -p omega_config -p omega_app --lib
    ```

## Nexus JSON-RPC

```bash
curl -sX POST http://127.0.0.1:8091/kip \
  -H 'content-type: application/json' \
  -d '{"jsonrpc":"2.0","id":1,"method":"execute_kip","params":{"command":"DESCRIBE PRIMER"}}'
```

## Pitfalls

- Root `Cargo.toml` may `exclude` incomplete scaffold crates — **remove exclude** once real manifests exist or packages never join the workspace.
- `derive_setters` on a field named `ok` collides with a method `ok` — use `EternalReceipt::success`.
- `Conversation` is not `PartialEq` — don't derive `PartialEq` on checkpoint structs embedding it.
- Optional title setters need `Some("…".into())` when struct has `#[setters(into)]` without `strip_option`.
- Formatter may claim “Rust 2015 async” before the package is a workspace member; trust `cargo test`, not the partial lint.
- Do not embed full Cognitive Nexus by default — HTTP to `:8091` matches the VPS layout and keeps CI light.
- `PathwayLogHook::log` needs `where S: PathwayStore, K: KipBackend, E: EternalStore` or `log_checkpoint` is invisible through `Arc`.
- Prefer **two cloned hooks** (response-only + end-only) sharing one `Arc<SessionPathwayService>` — don't try to clone `Box<dyn EventHandle>`.
- Keep private backend enums private: expose hook fields as `pub(crate)` on `PathwayHooks` to avoid `private_interfaces` warnings.
- After config shape changes, always refresh `omega.schema.json` or CI schema test fails.
- **No dTEE in Omega** — grep of `omega_anda` / `omega_anda_icp` / anda-bot has no dTEE/TEE runtime. `/goal pod` uses DevPod Docker (`omega-loops-cli` → `references/devpod-wrap.md`). Do not claim agents run in an Anda dTEE until ldclabs ships an API and a crate wraps it.

## Status

- [x] Crates + domain + local eternal receipts
- [x] `omega_config` `anda:` section + schema
- [x] `omega_app` hook registration when enabled
- [ ] CLI pathway list / show / rollback
- [ ] Real ic-oss / canister feature clients
- [ ] Optional tool-boundary checkpoints
- [ ] dTEE / confidential agent runtime (not present; do not stub)

## Related

- Offline Anda docs: `ai-agency/knowledge/anda/`
- Hermes/Agno brain ops: skill `anda-ecosystem` (overlap note: ecosystem = runtime/VPS brain; this skill = Omega Rust integration)
- VPS nexus: `anda-nexus.service` → `127.0.0.1:8091`
- `references/no-dtee.md` — dTEE is not in-tree; sandboxes are DevPod (`omega-loops-cli`)
