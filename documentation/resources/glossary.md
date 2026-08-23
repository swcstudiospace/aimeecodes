# Glossary

Terms used across Aimee Codes documentation, each tied to where the concept lives in the tree.

| Term | Meaning | Where |
|---|---|---|
| **Aimee** | The implementer agent and the product's CLI binary; also the engineering orchestrator that dispatches specialists via `task` | `crates/aimee_main`, `crates/aimee_domain/src/agent.rs` |
| **Anda** | Hash-chained session pathway system: append-only conversation checkpoints enabling chat-only rollbacks | `crates/aimee_anda`, [Anda / KIP](../web3/anda.md) |
| **AndaEternalMode** | Config enum for durability backends: `local` (receipts), `ic_oss`, `canister`, `s3` | `crates/aimee_config/src/anda.rs` |
| **Composition root** | The single place dependencies are wired: `AimeeAPI::init` builds Infra → Repo → Services → API | `crates/aimee_api`, [Composition root](../architecture/api.md) |
| **Context engine** | The services-side knowledge graph (nodes + relations) backing semantic search and workspace sync | `aimee.proto`, [gRPC contract](../reference/proto.md) |
| **Flock** | The three built-in agents — Sage, Muse, Aimee — plus the specialist roster | [The flock](../flock.md) |
| **HITL** | Human-in-the-loop: stop-and-ask gates, e.g. restricted-mode grants, `followup`, `/goal` probes, wallet spend | [Security](../security.md), [Loop autonomy](../architecture/domain.md) |
| **ICP** | Internet Computer — optional eternal durability backend for Anda pathways | `crates/aimee_anda_icp` |
| **KIP** | Knowledge-graph memory layer hooked by Anda (Cognitive Nexus) | [Anda / KIP](../web3/anda.md) |
| **Loop autonomy** | The five-probe `/goal` HITL contract (`GoalProbeSet`): done, verify, boundaries, owner, tracking | `crates/aimee_domain/src/loop_autonomy.rs` |
| **Muse** | The planning agent; writes checkbox plans under `plans/` and nothing else | [The flock](../flock.md) |
| **Pod** | Isolated container workspace provisioned by `aimee pod` (up/list/stop/delete/ssh/exec/ui) | [Pods and sandboxes](../ops/pod.md) |
| **Restricted mode** | Config switch requiring explicit permission grants for file/shell/fetch tools | [Security](../security.md) |
| **Sage** | The research agent; read-only analysis and reviews | [The flock](../flock.md) |
| **Sandboxes** | `--sandbox <name>`: isolated git worktree + branch (not a container) | [Pods and sandboxes](../ops/pod.md) |
| **Services URL** | Backend endpoint for indexing/search gRPC; default `https://api.aimeecodes.dev/` | [Cloud and services](../ops/cloud.md) |
| **Skill** | A `SKILL.md` package of domain knowledge/workflows the `skill` tool can load | [Skills and commands](../skills.md) |
| **Snapshot** | Pre-operation file state captured by file tools; what `undo` restores | `crates/aimee_snaps`, [undo](../reference/tools/undo.md) |
| **Specialists** | Built-in subagent roster (`fe-ui`, `be-api`, `plat-k8s`, …) Aimee dispatches | [The flock](../flock.md) |
| **Tool catalog** | The 16 built-in tools (`ToolCatalog` enum) | [Tool catalog](../reference/tools/catalog.md) |
| **ToolRegistry** | The router sending calls to catalog, agent, or MCP executors | `crates/aimee_app/src/tool_registry.rs` |
| **Turn** | One user prompt → model → tools → response cycle; bounded by request/failure budgets | [Reliability](../reliability.md) |
| **Workspace sync** | `aimee workspace sync`: index the project into the context engine | [CLI reference](../cli.md) |

## Related

- [Architecture overview](../architecture/overview.md)
- [Tool catalog](../reference/tools/catalog.md)
- [Troubleshooting and FAQ](troubleshooting.md)
