# Anda / KIP pathways (WEB3)

Anda adds **append-only, hash-chained session pathways** to Aimee conversations: every agent output can be checkpointed so chats roll back independently of your files. KIP connects those checkpoints to a Cognitive Nexus knowledge graph. It does not replace the agent runtime — it wraps conversation state with verifiable history.

## Enabling

```toml
[anda]
enabled = true
kip_enabled = true
```

That's the minimum. Full options (all live in `crates/aimee_config/src/anda.rs`, schema-published):

| Key | Default | Meaning |
|---|---|---|
| `enabled` | `false` | Master switch for pathway logging |
| `pathway_dir` | `{aimee_home}/pathways` | Checkpoint + metadata storage |
| `nexus_url` | — | Cognitive Nexus base URL (e.g. `http://127.0.0.1:8091`) |
| `kip_enabled` | `false` | KIP UPSERT side effects per checkpoint |
| `eternal_enabled` | `true` | Export checkpoints to eternal storage |
| `eternal_mode` | `local` | Backend: `local` \| `ic_oss` \| `canister` \| `s3` |
| `eternal_dir` | `{aimee_home}/pathways/eternal` | Root for local capsules/receipts |
| `eternal_label_prefix` | (preset) | Label prefix in eternal receipts |
| `log_responses` | `true` | Checkpoint after each LLM response |

## How checkpoints work

With `enabled = true`, each agent response appends a hash-chained checkpoint to the conversation's pathway. The chain makes tampering evident: each checkpoint commits to its predecessor.

With `kip_enabled = true` and a `nexus_url` set, checkpoints are additionally recorded into the KIP graph via `execute_kip` — turning session history into queryable shared memory rather than a private log file.

## Eternal durability

Checkpoints export to a durability backend (`AndaEternalMode`):

* **`local`** (default) — content-addressed receipts on disk, offline-capable. Receipts land under `eternal_dir`.
* **`ic_oss`** — object storage on the Internet Computer.
* **`canister`** — a dedicated KIP/pathway canister.
* **`s3`** — any S3-compatible store.

The ICP modes live in the `aimee_anda_icp` crate and return clear configuration errors until wired up — they fail loudly, never silently.

## Rolling back a chat

Pathways are inspected and rolled back through the conversation group:

```bash
aimee conversation pathway <conversation-id> list    # show checkpoints
```

Rollback is **chat-only**: rewinding the conversation does not revert files on disk. Your tree stays as it is; the dialogue returns to an earlier checkpoint. This is deliberate — code changes have git; conversation state has pathways.

## Pods and the Anda nexus

`aimee pod connect` attaches the TUI to an existing container workspace by probing its Anda nexus endpoint first (see [Pods and sandboxes](../surfaces/pods.md)). The same nexus URL convention powers both features.

## What this is for

Typical reasons to enable: auditability of what the agent said and did across long engagements; durable session memory shared with a KIP Cognitive Nexus; verifiable history when multiple agents or humans touch the same conversation lineage.

## See also

* [Sessions](../usage/sessions.md)
* [Wallet](wallet.md)
* [Persistence and sessions](../concepts/persistence.md)

<!-- sources: crates/aimee_config/src/anda.rs, crates/aimee_anda_icp/src/, crates/aimee_main/src/cli.rs (pathway), crates/aimee_main/src/pod.rs -->
