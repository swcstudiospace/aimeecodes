# Anda / KIP Eternal Session Pathways Implementation Plan

> **For Hermes:** Implement task-by-task. Prefer TDD. Do not pull the full Anda Engine into Omega — Omega remains the agent runtime; Anda/KIP is the eternal memory + pathway layer.

**Goal:** Introduce LDC Labs Anda + KIP into Omega Loops so every agent output appends an immutable **session pathway checkpoint**, enabling eternal chat rollbacks (conversation state only, not workspace files), with optional ICP durability.

**Architecture:** Clean-architecture crates mirror Omega conventions. Domain types and store traits live in `omega_anda`. Services log checkpoints and perform rollbacks. Backends start with local file + in-memory stores and an HTTP Cognitive Nexus client; ICP export is a separate crate (`omega_anda_icp`) with a local-receipt mode first and real ic-oss later. Integration into the orchestrator is a lifecycle `Hook` on `on_response` / `on_end`.

**Tech Stack:** Rust workspace crates, `anda_kip` (optional feature), HTTP JSON-RPC to Cognitive Nexus (`execute_kip`), SHA-256 content hashing, serde conversation snapshots, ICP export receipts (local → ic-oss).

**Upstream (LDC Labs, not iclabs):** https://github.com/ldclabs/KIP · anda-db · anda-brain · anda · ic-oss · https://anda.ai/

---

## Design summary

### What we are NOT doing
- Replacing Omega’s orchestrator with `anda_engine`
- Logging workspace file state to ICP (file undo stays `omega_snaps`)
- Requiring a live ICP canister for local development

### What we ARE doing
- Append-only **session pathway** per `ConversationId`
- On each agent output (response / turn end): snapshot conversation context → checkpoint → hash-chain → optional KIP UPSERT + eternal receipt
- Rollback restores conversation from checkpoint N and truncates the live head (history retained as pathway events)
- Dual durability: local pathway store always; ICP/ic-oss when configured

### Crate map

| Crate | Role |
|-------|------|
| `omega_anda` | Domain types, store traits, pathway log + rollback services, local backends, Nexus HTTP client, orchestrator hook |
| `omega_anda_icp` | Eternal durability backends: `local` receipts, future `ic_oss` / canister |
| (later) config + app wiring | `omega_config` Anda section; `omega_app` hook registration |

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
3. Future CLI: `omega conversation pathway list|show|rollback <seq>`

### Config (phase 2)
```yaml
anda:
  enabled: true
  pathway_dir: ~/.omega/pathways   # or workspace .omega/pathways
  nexus_url: http://127.0.0.1:8091 # optional
  kip_enabled: true
  eternal:
    mode: local   # local | ic_oss | canister | s3
    label_prefix: omega
```

---

## Phase 0 — Plan & workspace (this change)

- [x] Research Anda/KIP stack and Omega layering
- [x] Write this plan under `plans/`
- [ ] Add workspace members + dependency pins
- [ ] Scaffold `omega_anda` and `omega_anda_icp` crates with tests green

## Phase 1 — `omega_anda` domain + local pathway store

### Task 1.1: Domain types
- Create `crates/omega_anda` with pathway / checkpoint / receipt types
- Use `derive_setters`, serde, uuid, chrono, thiserror
- Unit tests for hash chain and constructors

### Task 1.2: Store traits
- `PathwayStore` (append, get pathway, list checkpoints, truncate_after)
- `KipBackend` (execute_kip command)
- `EternalStore` (export capsule/checkpoint receipt)

### Task 1.3: In-memory + file backends
- `MemoryPathwayStore` for tests
- `FilePathwayStore` under a configurable root (`{conversation_id}/pathway.json` + `checkpoints/{seq}.json`)

### Task 1.4: Services
- `SessionPathwayService::log_checkpoint(...)`
- `SessionPathwayService::rollback_to(conversation_id, seq) -> Conversation`
- Fail closed on hash-chain breaks

### Task 1.5: Nexus HTTP backend
- POST `{nexus_url}/kip` JSON-RPC `execute_kip` with `params: { command }`
- Map DESCRIBE/SEARCH/UPSERT helpers for pathway Event concepts

### Task 1.6: Lifecycle hook
- `PathwayLogHook` implementing `EventHandle` for Response + End
- Depends on `Arc<SessionPathwayService<_>>`
- Does not mutate conversation on log failure (warn + continue) unless config says hard-fail

## Phase 2 — `omega_anda_icp` eternal durability

### Task 2.1: Local receipt eternal store
- Write sha256 receipt JSON under pathway dir / eternal receipts
- Always available offline

### Task 2.2: ICP mode enum + factory
- `EternalMode::{Local, IcOss, Canister, S3}`
- Stub IcOss/Canister with clear `not yet configured` errors
- Feature-gate real clients later

### Task 2.3: Capsule export shape
- Serialize pathway slice as portable JSON capsule (KIP-compatible metadata fields)
- Receipt includes content_hash + location URI

## Phase 3 — Wire into Omega (follow-up PR)

### Task 3.1: `omega_config` Anda section
### Task 3.2: Register hook in `omega_app::app` when enabled
### Task 3.3: CLI pathway list / rollback
### Task 3.4: Optional embedded nexus feature (`anda_cognitive_nexus` + `anda_db`)

## Phase 4 — Verification

```bash
cargo check -p omega_anda -p omega_anda_icp
cargo insta test -p omega_anda -p omega_anda_icp --accept
cargo check   # workspace still builds
```

---

## Risks & mitigations

| Risk | Mitigation |
|------|------------|
| Heavy Anda deps slow CI | Feature-gate `kip` / `nexus`; default path is HTTP + file |
| Large conversation snapshots | Store compressed/truncated option later; hash still over full snapshot |
| ICP unavailable | Local eternal receipts always work; ICP is additive |
| Hash chain vs concurrent writers | Single-writer per conversation (orchestrator already serializes turns) |
| Rollback vs file snaps confusion | Docs + API names: pathway = chat only |

## Open questions
1. Default pathway root: global `~/.omega/pathways` vs per-workspace `.omega/pathways`?
2. Hard-fail agent turn if pathway log fails, or best-effort?
3. Should tool-call boundaries also checkpoint, or only LLM responses + turn end?

## Success criteria
- [ ] Crates compile and tests pass in isolation
- [ ] Logging N agent outputs yields N hash-linked checkpoints
- [ ] Rollback to seq K restores that conversation snapshot
- [ ] Eternal local receipt written with matching content_hash
- [ ] Optional Nexus `execute_kip` call succeeds when `anda-nexus` is up
- [ ] No change to file snapshot behavior
