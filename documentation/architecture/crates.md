# Crate map

Every workspace member (`crates/*`), grouped by concern. Shared dependency versions live in root `Cargo.toml` `[workspace.dependencies]`.

## Core loop

| Crate | Role |
|---|---|
| `aimee_domain` | Domain types, errors, tool catalog, policies, providers, loop autonomy |
| `aimee_config` | `.aimee.toml` schema, reader/writer, Anda config |
| `aimee_app` | `AimeeApp`, orchestrator, tool registry, git, Anda pathway hooks, DTOs |
| `aimee_services` | Application services generic over infra (`AimeeServices<F>`) |
| `aimee_infra` | FS, HTTP, auth, MCP, gRPC, env implementations |
| `aimee_repo` | Diesel/SQLite, proto, agent definitions, providers, skills, snapshots |
| `aimee_api` | Composition root + public `API` trait |
| `aimee_main` | CLI, TUI, ZSH, sandbox, pod, update — the `aimee` binary |

## WEB3 / sessions

| Crate | Role |
|---|---|
| `aimee_anda` | Hash-chained session pathways, KIP / Cognitive Nexus hooks |
| `aimee_anda_icp` | Eternal durability backends (local receipts; ICP/IC-OSS/S3 modes) |

## Presentation helpers

| Crate | Role |
|---|---|
| `aimee_display` | Syntax highlight, diff, grep, markdown formatters |
| `aimee_markdown_stream` | Streaming markdown renderer for LLM output |
| `aimee_spinner` | Terminal spinner / progress |
| `aimee_select` | Fuzzy picker widgets (nucleo) |
| `aimee_tracker` | Telemetry + `VERSION` |
| `aimee_stream` | `MpscStream` |

## Files, templates, tools

| Crate | Role |
|---|---|
| `aimee_fs` | Tokio FS with consistent anyhow context |
| `aimee_walker` | Directory walker used by discovery |
| `aimee_embed` | `include_dir` + Handlebars registration |
| `aimee_template` | Template `Element` |
| `aimee_tool_macros` | `ToolDescription` derive + `tool_description_file` |
| `aimee_json_repair` | Repair / coerce model JSON |
| `aimee_snaps` | Snapshot service |

## Streaming / HTTP

| Crate | Role |
|---|---|
| `aimee_eventsource` | SSE client over reqwest |
| `aimee_eventsource_stream` | Byte-stream SSE parser |

## CI / tests

| Crate | Role |
|---|---|
| `aimee_ci` | GitHub workflow generation (`gh-workflow`) |
| `aimee_test_kit` | Shared fixture loaders (`fixture!` / `json_fixture!`) |

Workspace facts: version 0.1.0, edition 2024, MSRV 1.94, toolchain pin 1.97. Verify with:

```bash
cargo fmt
cargo check -p <crate>
cargo clippy -p <crate> --all-targets -- -D warnings   # warnings are errors in CI
```

## See also

* [Architecture overview](overview.md)
* [Domain](domain.md)
* [Tool macros](tool-macros.md)

<!-- sources: AIMEE.md §5, Cargo.toml -->
