# Anda / KIP Eternal Session Pathways Implementation Plan

> **For Hermes:** Implement task-by-task. Prefer TDD. Do not pull the full Anda Engine into Aimee — Aimee remains the agent runtime; Anda/KIP is the eternal memory + pathway layer.

**Goal:** Introduce LDC Labs Anda + KIP into Aimee Codes so every agent output appends an immutable **session pathway checkpoint**, enabling eternal chat rollbacks (conversation state only, not workspace files), with optional ICP durability.

**Architecture:** Clean-architecture crates mirror Aimee conventions. Domain types and store traits live in `aimee_anda`. Services log checkpoints and perform rollbacks. Backends start with local file + in-memory stores and an HTTP Cognitive Nexus client; ICP export is a separate crate (`aimee_anda_icp`) with a local-receipt mode first and real ic-oss later. Integration into the orchestrator is a lifecycle `Hook` on `on_response` / `on_end`.

**Tech Stack:** Rust workspace crates, `anda_kip` (optional feature), HTTP JSON-RPC to Cognitive Nexus (`execute_kip`), SHA-256 content hashing, serde conversation snapshots, ICP export receipts (local → ic-oss).

**Upstream (LDC Labs, not iclabs):** https://github.com/ldclabs/KIP · anda-db · anda-brain · anda · ic-oss · https://anda.ai/

---

## Design summary

### What we are NOT doing
- Replacing Aimee’s orchestrator with `anda_engine`
- Logging workspace file state to ICP (file undo stays `aimee_snaps`)
- Requiring a live ICP canister for local development

### What we ARE doing
- Append-only **session pathway** per `ConversationId`
- On each agent output (response / turn end): snapshot conversation context → checkpoint → hash-chain → optional KIP UPSERT + eternal receipt
- Rollback restores conversation from checkpoint N and truncates the live head (history retained as pathway events)
- Dual durability: local pathway store always; ICP/ic-oss when configured

### Crate map

| Crate | Role |
|-------|------|
| `aimee_anda` | Domain types, store traits, pathway log + rollback services, local backends, Nexus HTTP client, orchestrator hook |
| `aimee_anda_icp` | Eternal durability backends: `local` receipts, future `ic_oss` / canister |
| (later) config + app wiring | `aimee_config` Anda section; `aimee_app` hook registration |

### Domain model

```text
SessionPathway
  pathway_id, conversation_id, agent_id?, head_seq, head_hash, created_at, updated_at

PathwayCheckpoint
  checkpoint_id, pathway_id, conversation_id, seq, parent_hash, content_hash
  kind: UserTurn | AgentResponse | ToolEnd | TurnEnd | Rollback | Manual
  agent_id?, message_count, conversation_snapshot (JSON), created_at
  kip_receipt?, eternal_receipt?

EternalReceipt
  mode (Local|IcOss|Canister|S3), label, content_hash, location, created_at, ok

KipReceipt
  command_digest, ok, response_summary?, executed_at
```

Hash chain: `content_hash = sha256(parent_hash || canonical_snapshot_bytes || kind || seq)`.

### Integration points
1. `Hook::on_response` → log `AgentResponse` checkpoint
2. `Hook::on_end` → log `TurnEnd` checkpoint + optional KIP formation UPSERT + eternal export
3. Future CLI: `aimee conversation pathway list|show|rollback <seq>`

### Config (phase 2)
```yaml
anda:
  enabled: true
  pathway_dir: ~/.aimee/pathways   # or workspace .aimee/pathways
  nexus_url: http://127.0.0.1:8091 # optional
  kip_enabled: true
  eternal:
    mode: local   # local | ic_oss | canister | s3
    label_prefix: aimee
```

---

## Phase 0 — Plan & workspace (this change)

- [x] Research Anda/KIP stack and Aimee layering
- [x] Write this plan under `plans/`
- [x] Add workspace members + dependency pins
- [x] Scaffold `aimee_anda` and `aimee_anda_icp` crates with tests green

## Phase 1 — `aimee_anda` domain + local pathway store

### Task 1.1: Domain types
- [x] Create `crates/aimee_anda` with pathway / checkpoint / receipt types
- [x] Use `derive_setters`, serde, uuid, chrono, thiserror
- [x] Unit tests for hash chain and constructors

### Task 1.2: Store traits
- [x] `PathwayStore` (append, get pathway, list checkpoints, truncate_after)
- [x] `KipBackend` (execute_kip command)
- [x] `EternalStore` (export capsule/checkpoint receipt)

### Task 1.3: In-memory + file backends
- [x] `MemoryPathwayStore` for tests
- [x] `FilePathwayStore` under a configurable root (`{conversation_id}/pathway.json` + `checkpoints/{seq}.json`)

### Task 1.4: Services
- [x] `SessionPathwayService::log_checkpoint(...)`
- [x] `SessionPathwayService::rollback_to(conversation_id, seq) -> Conversation`
- [x] Fail closed on hash-chain breaks

### Task 1.5: Nexus HTTP backend
- [x] POST `{nexus_url}/kip` JSON-RPC `execute_kip` with `params: { command }`
- [x] Map DESCRIBE/SEARCH/UPSERT helpers for pathway Event concepts

### Task 1.6: Lifecycle hook
- [x] `PathwayLogHook` implementing `EventHandle` for Response + End
- [x] Depends on `Arc<SessionPathwayService<_>>`
- [ ] Wire into `aimee_app` (phase 3)

## Phase 2 — `aimee_anda_icp` eternal durability

### Task 2.1: Local receipt eternal store
- [x] Write sha256 receipt JSON under pathway dir / eternal receipts
- [x] Always available offline

### Task 2.2: ICP mode enum + factory
- [x] `EternalMode::{Local, IcOss, Canister, S3}`
- [x] Stub IcOss/Canister with clear `not yet configured` errors
- [ ] Feature-gate real clients later

### Task 2.3: Capsule export shape
- [x] Serialize pathway slice as portable JSON capsule (KIP-compatible metadata fields)
- [x] Receipt includes content_hash + location URI

## Phase 3 — Wire into Aimee

### Task 3.1: `aimee_config` Anda section
- [x] `AndaConfig` + `AndaEternalMode`
- [x] `AimeeConfig.anda: Option<AndaConfig>`
- [x] Defaults documented in `.aimee.toml` (disabled)
- [x] Schema regenerated (`aimee.schema.json`)

### Task 3.2: Register hook in `aimee_app::app` when enabled
- [x] `anda_pathway::maybe_pathway_hooks` factory
- [x] Chain onto `on_response` + `on_end` when `anda.enabled = true`
- [x] File pathway store under `{aimee_home}/pathways`
- [x] Local eternal receipts under `{pathway_dir}/eternal`

### Task 3.3: CLI pathway list / rollback
- [x] `aimee conversation pathway <id> list [--porcelain]`
- [x] `aimee conversation pathway <id> show <seq>`
- [x] `aimee conversation pathway <id> rollback <seq>` (chat only; upserts restored conversation)

### Task 3.4: Optional embedded nexus feature (`anda_cognitive_nexus` + `anda_db`)
- [ ] Later; HTTP nexus client already supported via `nexus_url`

## Phase 4 — Verification

```bash
cargo check -p aimee_anda -p aimee_anda_icp
cargo test -p aimee_anda -p aimee_anda_icp
cargo check   # workspace still builds
```

**Verified 2026-08-19:** `cargo test -p aimee_anda -p aimee_anda_icp` → **14 passed**.

## Success criteria
- [x] Crates compile and tests pass in isolation
- [x] Logging N agent outputs yields N hash-linked checkpoints
- [x] Rollback to seq K restores that conversation snapshot
- [x] Eternal local receipt written with matching content_hash
- [ ] Optional Nexus `execute_kip` call succeeds when `anda-nexus` is up (HTTP client ready; live E2E pending)
- [x] No change to file snapshot behavior
