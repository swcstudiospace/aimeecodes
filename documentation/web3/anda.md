# Anda / KIP

Anda is Aimee Codes’ **session pathway** layer: append-only, hash-chained conversation checkpoints with optional KIP (Cognitive Nexus) side effects and optional eternal receipts. It is the WEB3 durability hook for **chat state**, not a second agent runtime.

Source of truth is the product tree:

- `crates/aimee_anda` — domain, hooks, KIP backends, pathway stores
- `crates/aimee_anda_icp` — local receipts and ICP/S3 factory stubs
- `crates/aimee_config/src/anda.rs` — `[anda]` schema
- `crates/aimee_app/src/anda_pathway.rs` — app wiring
- `crates/aimee_main/src/cli.rs` — `aimee conversation pathway …`
- `AIMEE.md` section 10

## What Anda is

`aimee_anda` adds four things and nothing else (`crates/aimee_anda/src/lib.rs:1-7`):

1. **Append-only session pathway checkpoints** for agent output
2. **Hash-chained conversation snapshots** so a chat can be rolled back
3. **Optional KIP / Cognitive Nexus** recording of each checkpoint
4. **Hooks for eternal durability** implemented in the companion crate `aimee_anda_icp`

A pathway is metadata plus a chain of checkpoints. Metadata tracks the head of the chain (`crates/aimee_anda/src/domain/pathway.rs:27-43`). Each checkpoint stores a full `Conversation` snapshot, a 1-based sequence number, a parent hash (`"genesis"` for the first), and a SHA-256 content hash (`crates/aimee_anda/src/domain/checkpoint.rs:49-73`, `crates/aimee_anda/src/domain/checkpoint.rs:76-77`).

Checkpoint kinds that exist (`crates/aimee_anda/src/domain/checkpoint.rs:30-47`). JSON uses serde `snake_case`; CLI `kind` is `Display` of the variant name (`AgentResponse`, not `agent_response`).

| Variant / CLI `kind` | JSON | When it is written |
|---|---|---|
| `UserTurn` | `user_turn` | User message boundary (defined; not auto-logged by the hook) |
| `AgentResponse` | `agent_response` | LLM assistant response (`on_response`) |
| `ToolEnd` | `tool_end` | Tool call finished (defined; not auto-logged by the hook) |
| `TurnEnd` | `turn_end` | Full agent turn completed (`on_end`) |
| `Rollback` | `rollback` | Marker appended after a chat rollback |
| `Manual` | `manual` | Manual / external checkpoint |

The live hook only emits `AgentResponse` and `TurnEnd` (`crates/aimee_anda/src/hook.rs:87-115`). Rollback writes a `Rollback` marker after truncating later checkpoints (`crates/aimee_anda/src/services/pathway_service.rs:164-224`).

## What Anda is not

- **Not a replacement for the agent runtime.** Sage, Muse, and Aimee still run through `aimee_app` / `aimee_services`. Anda only snapshots conversation state after those turns (`crates/aimee_anda/src/lib.rs:3-4`, `AIMEE.md:300-302`).
- **Not a workspace backup.** Rollback restores **chat context only**. Workspace files are not reverted (`crates/aimee_main/src/cli.rs:959-961`).
- **Not an on-chain wallet, payment rail, or canister client.** No canister IDs, wallet login, or spend APIs live in these crates. Payments and spend stay HITL (`AIMEE.md:314`).
- **Not a KIP parser by default.** HTTP nexus works without the optional `kip` Cargo feature (`crates/aimee_anda/Cargo.toml:8-11`).
- **Not a live ICP/S3 exporter in the app path.** Non-local eternal modes exist on the enum and return clear factory errors; the running agent currently falls back to local receipts (see [ICP and non-local modes](#icp-and-non-local-modes)).

## Enablement

Anda is **off** until you set `[anda].enabled = true`. `AndaConfig::default()` has `enabled = false` and `kip_enabled = false` (`crates/aimee_config/src/anda.rs:88-103`). The embedded defaults comment the section out (`crates/aimee_config/.aimee.toml:76-87`).

Put the table in `~/.aimee/.aimee.toml` (or whichever config base `ConfigReader::base_path()` resolved — `AIMEE_CONFIG` wins; see `crates/aimee_config/src/reader.rs:67-86`). `AimeeConfig.anda` is `Option<AndaConfig>` (`crates/aimee_config/src/config.rs:339-344`).

Only these keys exist (`crates/aimee_config/src/anda.rs:23-78`):

```toml
[anda]
enabled = true
# pathway_dir = "/absolute/path/to/pathways"   # default: {aimee_home}/pathways
# nexus_url = "http://127.0.0.1:8091"
kip_enabled = false
eternal_enabled = true
eternal_mode = "local"                         # local | ic_oss | canister | s3
# eternal_dir = "/absolute/path/to/eternal"    # default: {pathway_dir}/eternal
eternal_label_prefix = "aimee"
log_responses = true
log_turn_end = true
hard_fail = false
```

| Key | Type | Default | Role |
|---|---|---|---|
| `enabled` | bool | `false` | Master switch. Hooks are not built when false (`crates/aimee_app/src/anda_pathway.rs:68-70`). |
| `pathway_dir` | optional path | `{aimee_home}/pathways` | File store root (`crates/aimee_app/src/anda_pathway.rs:45-49`). |
| `nexus_url` | optional string | unset | Cognitive Nexus base URL. Used only when `kip_enabled` is true (`crates/aimee_app/src/anda_pathway.rs:76-81`). |
| `kip_enabled` | bool | `false` | Execute a KIP `UPSERT` per checkpoint. App wiring also requires a remote backend (`crates/aimee_app/src/anda_pathway.rs:100-101`). |
| `eternal_enabled` | bool | `true` | Export each checkpoint to the eternal store. |
| `eternal_mode` | enum | `local` | See [Eternal modes](#eternal-modes). |
| `eternal_dir` | optional path | `{pathway_dir}/eternal` | Local capsule/receipt root (`crates/aimee_app/src/anda_pathway.rs:50-53`). |
| `eternal_label_prefix` | string | `"aimee"` | Prefix for receipt labels `{prefix}-{conversation_id}-{seq}` (`crates/aimee_anda/src/services/pathway_service.rs:130-135`). |
| `log_responses` | bool | `true` | Checkpoint after each LLM response. |
| `log_turn_end` | bool | `true` | Checkpoint when a turn ends. |
| `hard_fail` | bool | `false` | If true, pathway failures fail the agent turn; otherwise warn and continue (`crates/aimee_anda/src/hook.rs:9-11`, `crates/aimee_anda/src/hook.rs:64-76`). |

There is no wallet key, no canister id, no IC-OSS endpoint, and no S3 bucket on `[anda]`. Those fields exist only on the unused factory config `EternalStoreConfig` in `aimee_anda_icp` (`crates/aimee_anda_icp/src/store.rs:8-20`).

Minimal enablement matching `AIMEE.md` section 10 (`AIMEE.md:304-310`):

```toml
[anda]
enabled = true
kip_enabled = true
```

`kip_enabled = true` without `nexus_url` selects `AnyKipBackend::Noop` (`crates/aimee_anda/src/backends/any_kip.rs:13-19`). The app then sets `kip_enabled` on the service to **false** because `kip.is_remote()` is false (`crates/aimee_app/src/anda_pathway.rs:100-101`). KIP side effects therefore require **both** `kip_enabled = true` **and** a non-empty `nexus_url`.

## Eternal modes

`AndaEternalMode` and `aimee_anda::EternalMode` are the same four snake_case values (`crates/aimee_config/src/anda.rs:8-21`, `crates/aimee_anda/src/domain/receipt.rs:6-19`):

| TOML value | Enum | What exists today |
|---|---|---|
| `local` (default) | `Local` | Content-addressed local capsules + receipts. Offline-capable. |
| `ic_oss` | `IcOss` | Represented. Factory returns `NotConfigured`. App falls back to local receipts. |
| `canister` | `Canister` | Represented as “dedicated KIP/pathway canister”. Same stub behavior. |
| `s3` | `S3` | Represented as S3-compatible object storage. Same stub behavior. |

`aimee_anda_icp` documents this split in its crate docs (`crates/aimee_anda_icp/src/lib.rs:1-4`): default mode writes local receipts; ICP / IC-OSS modes are in the API and return clear errors until configured.

The `ic-oss` Cargo feature on `aimee_anda_icp` is empty — “Future: real ic-oss client wiring” (`crates/aimee_anda_icp/Cargo.toml:8-11`).

### Local receipts (the working backend)

`LocalReceiptEternalStore` writes (`crates/aimee_anda_icp/src/local_receipt.rs:53-59`):

```text
{eternal_dir}/{conversation_id}/{seq:020}-{content_hash_prefix}.capsule.json
{eternal_dir}/{conversation_id}/{seq:020}-{content_hash_prefix}.receipt.json
```

Capsule schema is `aimee.anda.pathway_capsule.v1` (`crates/aimee_anda_icp/src/local_receipt.rs:26-27`). The receipt is an `EternalReceipt` with `mode = local`, `ok = true`, and `location` pointing at the capsule path (`crates/aimee_anda_icp/src/local_receipt.rs:114-129`).

When `eternal_enabled = false`, the app uses `NoopEternalStore`, which returns an in-memory success at `noop://{checkpoint_id}` and writes nothing (`crates/aimee_anda/src/infra/eternal_store.rs:14-31`, `crates/aimee_app/src/anda_pathway.rs:94-98`).

### File pathway store

Checkpoints themselves live in `FilePathwayStore` (`crates/aimee_anda/src/backends/file_store.rs:8-14`):

```text
{pathway_dir}/{conversation_id}/pathway.json
{pathway_dir}/{conversation_id}/checkpoints/{seq:020}.json
```

Append is idempotent-hostile: rewriting an existing seq is an error (`crates/aimee_anda/src/backends/file_store.rs:93-97`). The store also rejects a broken parent-hash link (`crates/aimee_anda/src/backends/file_store.rs:99-111`).

`MemoryPathwayStore` exists for tests (`crates/aimee_anda/src/backends/memory_store.rs:9-12`).

## Commands

`conversation` is aliased as `session` (`crates/aimee_main/src/cli.rs:118-120`). Pathway is a subcommand of that group (`crates/aimee_main/src/cli.rs:933-940`).

```bash
aimee conversation pathway <CONVERSATION_ID> list
aimee conversation pathway <CONVERSATION_ID> list --porcelain
aimee conversation pathway <CONVERSATION_ID> show <SEQ>
aimee conversation pathway <CONVERSATION_ID> rollback <SEQ>

# equivalent
aimee session pathway <CONVERSATION_ID> list
```

`SEQ` is the 1-based checkpoint sequence (`crates/aimee_main/src/cli.rs:953-965`).

| Subcommand | Behavior |
|---|---|
| `list` | Reads the file store and prints checkpoints (or TSV with `--porcelain`). Empty path: `No session pathway checkpoints for {id}` (`crates/aimee_main/src/ui.rs:1031-1066`). Works even when `[anda]` is disabled (`crates/aimee_app/src/anda_pathway.rs:160-166`). |
| `show <seq>` | One checkpoint: kind, message count, content/parent hashes, timestamps, optional agent / KIP / eternal flags (`crates/aimee_main/src/ui.rs:1068-1093`). Missing seq: `No pathway checkpoint seq {seq} for conversation {id}`. |
| `rollback <seq>` | Restores **conversation chat state** to that snapshot, upserts it, and prints `Rolled conversation {id} back to pathway seq {seq} ({n} messages)` (`crates/aimee_main/src/ui.rs:1095-1101`). |

The conversation must already exist in the SQLite conversation store or the command fails with `Conversation '{id}' not found` (`crates/aimee_main/src/ui.rs:1025`, `crates/aimee_main/src/ui.rs:1107-1113`).

Rollback internals (`crates/aimee_anda/src/services/pathway_service.rs:164-224`):

1. Load pathway; missing → `AndaError::PathwayNotFound`
2. Load checkpoint `seq`; missing → `AndaError::SeqNotFound`
3. Verify content hashes from genesis through `seq`; mismatch → `HashChainBroken` or `ContentHashMismatch`
4. `truncate_after(seq)` — delete checkpoints with `seq > target`
5. Restore the snapshot
6. Append a `Rollback` marker so history stays auditable

CLI rollback builds a service with `kip_enabled: false` and `hard_fail: true` (`crates/aimee_app/src/anda_pathway.rs:216-226`). A broken chain therefore fails the command instead of warning.

Porcelain `list` columns, tab-separated (`crates/aimee_main/src/ui.rs:1034-1042`):

```text
seq  kind  content_hash  message_count  created_at(RFC3339)
```

## File interactions

```text
.aimee.toml [anda]
        │
        ▼
aimee_config::AndaConfig / AndaEternalMode
        │
        ▼
aimee_app::anda_pathway          (composition)
   maybe_pathway_hooks
   list / show / rollback
        │
        ├─ aimee_anda::FilePathwayStore      checkpoints on disk
        ├─ aimee_anda::AnyKipBackend         Noop | Nexus HTTP
        ├─ aimee_anda::SessionPathwayService log + rollback
        ├─ aimee_anda::PathwayLogHook        on_response / on_end
        │
        └─ aimee_anda_icp::LocalReceiptEternalStore
                 ▲
                 │  factory (not used by the app today)
                 │
           aimee_anda_icp::build_eternal_store
                 └─ IcpError::NotConfigured for ic_oss / canister / s3
```

| Crate / file | Role |
|---|---|
| `crates/aimee_config/src/anda.rs` | Schema and defaults. Re-exported from `aimee_config` (`crates/aimee_config/src/lib.rs:1`, `crates/aimee_config/src/lib.rs:16`). |
| `crates/aimee_config/src/config.rs:339-344` | Optional `anda` field on `AimeeConfig`. |
| `crates/aimee_anda/src/domain/` | `PathwayId`, `SessionPathway`, `PathwayCheckpoint`, `CheckpointKind`, `EternalMode`, `EternalReceipt`, `KipReceipt`, `AndaError`. |
| `crates/aimee_anda/src/infra/` | Traits: `PathwayStore`, `KipBackend`, `EternalStore`. |
| `crates/aimee_anda/src/backends/` | `FilePathwayStore`, `MemoryPathwayStore`, `NexusHttpBackend`, `AnyKipBackend`, `pathway_event_upsert`. |
| `crates/aimee_anda/src/services/pathway_service.rs` | `SessionPathwayService`: ensure, log, list, rollback. |
| `crates/aimee_anda/src/hook.rs` | Orchestrator `EventHandle` for response and turn-end. |
| `crates/aimee_anda_icp/src/local_receipt.rs` | Working eternal backend. |
| `crates/aimee_anda_icp/src/store.rs` | `EternalStoreConfig` + `build_eternal_store` (clear errors until configured). |
| `crates/aimee_anda_icp/src/error.rs` | `IcpError::NotConfigured { mode, detail }`. |
| `crates/aimee_app/src/anda_pathway.rs` | Resolves dirs, builds hooks, exposes CLI helpers. |
| `crates/aimee_app/src/app.rs:153-179` | Chains pathway hooks onto the orchestrator when `config.anda` is present and enabled. |
| `crates/aimee_app/src/lib.rs:45-48` | Public: `maybe_pathway_hooks`, `list_session_pathway`, `show_session_pathway`, `rollback_session_pathway`. |
| `crates/aimee_main/src/cli.rs:933-965` | Clap surface. |
| `crates/aimee_main/src/ui.rs:1013-1104` | Command execution. |

### How a checkpoint is logged

`SessionPathwayService::log_checkpoint` (`crates/aimee_anda/src/services/pathway_service.rs:90-162`):

1. `ensure_pathway` — create empty pathway at genesis if needed
2. Build checkpoint at `head_seq + 1` with `parent_hash = head_hash`
3. Verify content hash
4. If KIP enabled, `execute_kip(pathway_event_upsert(…))`
5. If eternal enabled, `export_checkpoint` with label `{prefix}-{conversation_id}-{seq}`
6. `append_checkpoint` then update pathway head

KIP and eternal failures warn and continue unless `hard_fail` is true (`crates/aimee_anda/src/services/pathway_service.rs:115-141`).

### KIP / Cognitive Nexus

`NexusHttpBackend` POSTs JSON-RPC 2.0 to `{base_url}/kip` (`crates/aimee_anda/src/backends/nexus_http.rs:6-21`):

```json
{"jsonrpc":"2.0","id":1,"method":"execute_kip","params":{"command":"…"}}
```

HTTP or JSON-RPC errors become a `KipReceipt` with `ok = false`, not a panic (`crates/aimee_anda/src/backends/nexus_http.rs:55-65`). Transport failures map to `AndaError::Kip` (`crates/aimee_anda/src/backends/nexus_http.rs:41-47`).

The command is a KML `UPSERT` of an `Event` concept named `aimee-pathway-{conversation_id}-seq-{seq}` with attributes `conversation_id`, `seq`, `content_hash`, `kind`, `source = "aimee_anda"` (`crates/aimee_anda/src/backends/nexus_http.rs:76-99`).

`NoopKipBackend` always returns `ok = true` with summary `"noop"` (`crates/aimee_anda/src/infra/kip_backend.rs:16-24`).

## ICP and non-local modes

Two behaviors exist. Document both; do not collapse them.

### Factory: clear `NotConfigured` errors

`build_eternal_store` (`crates/aimee_anda_icp/src/store.rs:45-79`) is the API AIMEE.md refers to when it says ICP modes “return clear errors until configured” (`AIMEE.md:312`).

| Mode | Error (`IcpError::NotConfigured`) |
|---|---|
| `local` without `local_root` | `eternal mode local is not configured: local_root is required` |
| `ic_oss` without endpoint | `eternal mode ic_oss is not configured: set ic_oss_endpoint and enable feature ic-oss` |
| `ic_oss` with endpoint | `eternal mode ic_oss is not configured: endpoint={e} set but client not enabled; enable feature ic-oss` |
| `canister` without id | `eternal mode canister is not configured: set canister_id when canister client is available` |
| `canister` with id | `eternal mode canister is not configured: canister_id={id} set but client not enabled` |
| `s3` without bucket | `eternal mode s3 is not configured: set s3_bucket when S3 client is available` |
| `s3` with bucket | `eternal mode s3 is not configured: bucket={b} set but client not enabled` |

Display format is `eternal mode {mode} is not configured: {detail}` (`crates/aimee_anda_icp/src/error.rs:6-8`).

`[anda]` has **no** keys for `ic_oss_endpoint`, `canister_id`, or `s3_bucket`. Those exist only on `EternalStoreConfig` (`crates/aimee_anda_icp/src/store.rs:14-19`). Do not invent canister IDs or put them in `.aimee.toml`.

### App path: warn and use local receipts

`maybe_pathway_hooks` does **not** call `build_eternal_store`. If `eternal_enabled` and `eternal_mode` is `ic_oss`, `canister`, or `s3`, it logs (`crates/aimee_app/src/anda_pathway.rs:83-98`):

```text
anda eternal mode is not fully wired yet; using local receipts
```

and still constructs `LocalReceiptEternalStore` under `eternal_dir`. Setting `eternal_mode = "canister"` in config therefore does **not** fail the agent turn and does **not** talk to a canister. Receipts stay on disk.

CLI rollback uses the same local/noop split (`crates/aimee_app/src/anda_pathway.rs:211-215`).

Until a client is wired, treat non-local modes as **declared but not exported**. Local receipts remain the only durable eternal backend.

## Best practices

- **Enable explicitly.** Leave `[anda]` commented or `enabled = false` until you want chat checkpoints. Defaults keep the feature off (`crates/aimee_config/src/anda.rs:91`).
- **Do not treat the pathway as a backup of the workspace.** Rollback restores conversation context only (`crates/aimee_main/src/cli.rs:959-961`). Use git (or `aimee` file snapshots) for files. A successful pathway rollback can still leave edited files in place.
- **Keep spend HITL.** Anda does not authorize wallet transfers or canister calls. Payments and spend stay human-in-the-loop (`AIMEE.md:314`).
- **Prefer `hard_fail = false` in interactive work.** Pathway durability must not block the agent turn (`crates/aimee_anda/src/hook.rs:9-11`). Turn `hard_fail` on only when you are debugging the pathway itself.
- **KIP needs a real nexus.** `kip_enabled` without `nexus_url` is a no-op in the app (`crates/aimee_app/src/anda_pathway.rs:76-81`, `crates/aimee_app/src/anda_pathway.rs:100-101`). Point `nexus_url` at a local Cognitive Nexus (the schema example is `http://127.0.0.1:8091`) if you want UPSERTs.
- **Leave `eternal_mode = "local"`** unless you are developing the ICP factory. Non-local values only change a warning line in the running app.
- **Inspect before rollback.** `list` then `show <seq>`, then `rollback <seq>`. Rollback truncates later checkpoints and appends a marker; it is not an undo of the undo (`crates/aimee_anda/src/services/pathway_service.rs:206-222`).
- **Do not store secrets in chat if Anda is on.** Checkpoints serialize the full conversation, including tool results, into JSON under `pathway_dir` (`crates/aimee_anda/src/domain/checkpoint.rs:68-69`). Treat that directory like conversation history, not a secret store.
- **On-chain logging is evidence, not a secret store.** When ICP export is eventually wired, receipts prove a hash existed; they must not carry tokens, controller keys, or wallet seeds.

## How to verify

From the Aimee Codes workspace (`aimeecodes/`):

```bash
cargo check -p aimee_anda -p aimee_anda_icp
```

Optional, same crates:

```bash
cargo clippy -p aimee_anda -p aimee_anda_icp --all-targets -- -D warnings
cargo insta test --accept -p aimee_anda -p aimee_anda_icp
```

CLI parse coverage for the pathway surface lives in `crates/aimee_main/src/cli.rs:2198-2238` (`list` and `rollback`). App hook wiring tests: `crates/aimee_app/src/anda_pathway.rs:273-307`.

Do not `cargo build --release` for this check.

Wallet login and PWA spend sit beside provider auth and are documented on the Wallet page, not here.
