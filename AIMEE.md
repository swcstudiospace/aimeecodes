# AIMEE.md — Aimee Codes application discovery

This file is the full product map of **Aimee Codes** (`aimee`), a Spectrum Web Co LLC CLI agent flock. It is grounded in the current tree. It is not a second set of house rules — `AGENTS.md` remains policy. `plans/` is historical unless a task cites a specific plan.

**Formerly Omega Loops.** Crates, binaries, config, templates, the ZSH plugin, and product URLs now use the Aimee Codes brand. Compat aliases remain so existing installs keep working.

---

## 1. Product

Aimee Codes is a **WEB3-aware, terminal-native coding agent**. Sage researches. Muse plans. Aimee implements and verifies. Users run it as:

- Interactive **ratatui TUI** (`aimee`)
- One-shot **CLI** (`aimee -p "…"`)
- ZSH **`:` prefix** (`: sage …`, `:muse …`, `:aimee …`)
- Installable **PWA** (`pwa/`) — drafts stay on-device until an agent API is wired

Workspace version is `0.1.0` (`Cargo.toml`). Edition is `2024`. MSRV is `rust-version = "1.94"`; the pin is `rust-toolchain.toml` (`1.97`). License is Apache-2.0. Copyright holder is **Spectrum Web Co LLC** (brand `@swcstudio`).

The CLI binary is **`aimee`** from `crates/aimee_main`. About string: `Aimee Codes — WEB3-native AI coding agent` (`crates/aimee_main/src/cli.rs:13-18`).

---

## 2. Branding and compat

| Surface | Current | Notes |
|---|---|---|
| Product name | Aimee Codes | README, PWA, LICENSE, CLI about |
| Command | `aimee` | `crates/aimee_main` `[[bin]]` |
| Config dir | `~/.aimee` | default for new installs |
| Config file | `~/.aimee/.aimee.toml` | schema `aimee.schema.json` |
| Config env | `AIMEE_CONFIG` | wins over legacy |
| Services API | `https://api.aimeecodes.dev/` | default `services_url` |
| Billing | `https://app.aimeecodes.dev/app/billing` | upgradeable plan banner |
| Docs / install | `https://aimeecodes.dev` | CLI curl, schema, docs |
| NPM release matrix | `swcstudiospace/npm-aimee-codes` | `crates/aimee_ci` |
| Homebrew tap | `antinomyhq/homebrew-aimee-codes` | already rebranded |
| GitHub origin | `https://github.com/swcstudiospace/omegaloops` | **live remote** — keep until the repo is renamed |
| Nix flake homepage | same GitHub origin | `flake.nix` |
| Eval clone URLs | same GitHub origin | `benchmarks/evals/*/task.yml` |
| Trademarks | Aimee Codes **and** Omega Loops | LICENSE §6 |

### Intentional Omega / Forge compat (do not delete)

These stay so existing users are not stranded:

- Env `OMEGA_CONFIG` is still read after `AIMEE_CONFIG` (`crates/aimee_config/src/reader.rs:67-73`)
- Base-path candidates: `aimee`, `.aimee`, `omega`, `.omega`, `forge`, `.forge` (`crates/aimee_config/src/reader.rs:76`)
- `AIMEE_*` env vars map onto `.aimee.toml`; legacy `OMEGA_*` vars are still read
- `aimee config migrate` moves `~/aimee`, `~/.omega`, or `~/omega` → `~/.aimee`
- ZSH `:omega` remains a documented alias where the plugin still maps it
- LICENSE still lists "Omega Loops" as a Spectrum Web Co trademark

---

## 3. Product loop

Three built-in agents. One loop. Definitions live in `crates/aimee_repo/src/agents/`. Custom agents live in `.aimee/agents/` (project) or `~/.aimee/agents/` (global).

| Agent | ID | Alias | Writes? | Role |
|---|---|---|---|---|
| Sage | `sage` | `:ask` | No | Research, architecture, reviews |
| Muse | `muse` | `:plan` | Plans only | Checkbox plans under `plans/` |
| Aimee | `aimee` | `:act` | Yes | Implement, verify, report evidence |

Aimee is also the **engineering orchestrator**. When a change is clearly in one specialty it dispatches a subagent via the `task` tool. It does not nest orchestrators.

### Specialist roster (built-in)

**Frontend:** `fe-ui`, `fe-web3`, `fe-realtime`, `fe-edge`, `fe-qa`

**Backend:** `be-api`, `be-web3`, `be-data`, `be-security`, `be-reliability`

**Platform:** `plat-k8s`, `plat-cloud`, `plat-compliance`, `plat-sre`

Agent IDs are first-class: `AgentId::AIMEE`, `AgentId::MUSE`, `AgentId::SAGE` (`crates/aimee_domain/src/agent.rs:37-39`).

---

## 4. Architecture

Clean architecture. Same invariants in every language (`AGENTS.md`).

```
CLI / TUI / ZSH / PWA
        │
   aimee_main          presentation (clap, ratatui, rustyline)
        │
   aimee_api           composition root  AimeeAPI::init
        │
   aimee_app           orchestration     AimeeApp + Orchestrator + ToolRegistry
        │
   aimee_services      application services  AimeeServices<F>
        │
   aimee_repo          persistence       AimeeRepo<F>
        │
   aimee_infra         infrastructure    AimeeInfra
        │
   aimee_domain        types, tools, policies
   aimee_config        .aimee.toml schema + IO
```

Composition at startup (`crates/aimee_api/src/aimee_api.rs:44-56`):

1. `AimeeInfra::new(cwd, config)`
2. `AimeeRepo::new(infra)`
3. `AimeeServices::new(repo)`
4. `AimeeAPI::new(services, repo)`

Rules that the tree actually enforces:

- **No service-to-service calls.** Compose at the composition root.
- Services take **at most one** generic (`F` / `R`), store infra as `Arc<T>`, put trait bounds on methods not `new()`.
- Domain errors are `thiserror`. Services/CLI use `anyhow`. Do not implement `From` that collapses distinct failures.
- Invalid states are unrepresentable (newtypes, enums, branded IDs).
- Diesel schema is generated. Never edit a shipped migration.

---

## 5. Crate map

Workspace members are `crates/*`. Shared versions live in root `Cargo.toml` `[workspace.dependencies]`.

### Core loop

| Crate | Role |
|---|---|
| `aimee_domain` | Domain types, errors, tool catalog, policies, providers, loop autonomy |
| `aimee_config` | `.aimee.toml` schema, reader/writer, Anda config |
| `aimee_app` | `AimeeApp`, orchestrator, tool registry, git, Anda pathway hooks, DTOs |
| `aimee_services` | Application services generic over infra (`AimeeServices<F>`) |
| `aimee_infra` | FS, HTTP, auth, MCP, gRPC, env, walker implementations |
| `aimee_repo` | Diesel/SQLite, proto, agent defs, providers, skills, snapshots |
| `aimee_api` | Composition root + public `API` trait |
| `aimee_main` | CLI, TUI, ZSH, sandbox, pod, update, `aimee` binary |

### WEB3 / sessions

| Crate | Role |
|---|---|
| `aimee_anda` | Hash-chained session pathways, KIP / Cognitive Nexus hooks |
| `aimee_anda_icp` | Eternal durability backends (local receipts; ICP/IC-OSS/S3 modes) |

### Presentation helpers

| Crate | Role |
|---|---|
| `aimee_display` | Syntax highlight, diff, grep, markdown formatters |
| `aimee_markdown_stream` | Streaming markdown renderer for LLM output |
| `aimee_spinner` | Terminal spinner / progress |
| `aimee_select` | Fuzzy picker widgets (`nucleo`) |
| `aimee_tracker` | Telemetry + `VERSION` |
| `aimee_stream` | `MpscStream` |

### Files, templates, tools

| Crate | Role |
|---|---|
| `aimee_fs` | Tokio FS with consistent anyhow context |
| `aimee_walker` | Directory walker used by discovery |
| `aimee_embed` | `include_dir` + Handlebars registration |
| `aimee_template` | Template `Element` |
| `aimee_tool_macros` | `ToolDescription` derive + `tool_description_file` |
| `aimee_json_repair` | Repair / coerce model JSON |
| `aimee_snaps` | Snapshot service |

### Streaming / HTTP

| Crate | Role |
|---|---|
| `aimee_eventsource` | SSE client over reqwest |
| `aimee_eventsource_stream` | Byte-stream SSE parser |

### CI / tests

| Crate | Role |
|---|---|
| `aimee_ci` | GitHub workflow generation (`gh-workflow`) |
| `aimee_test_kit` | Shared fixture loaders (`fixture!` / `json_fixture!`) |

---

## 6. Runtime surfaces

### CLI (`aimee`)

Top-level flags (`crates/aimee_main/src/cli.rs`): `-p/--prompt`, `-e/--event`, `--conversation`, `--conversation-id` (`--cid`), `--agent` (`--aid`), `-C/--directory`, `--sandbox`, `--pod`, `--verbose`.

Top-level commands (`TopLevelCommand`): `agent`, `zsh` (`extension`), `list`, `banner`, `info`, `config`, `conversation` (`session`), `commit`, `mcp`, `suggest`, `provider`, `cmd`, `workspace`, `data`, `vscode`, `update`, `setup`, `doctor`, `logs`, `pod` (`codespace`/`devpod`), `select`.

`aimee pod` is a rebranded DevPod wrapper (`Up`/`List`/`Stop`/`Delete`/`Ssh`/`Exec` plus an Aimee-native `ui`). `--sandbox` is a git worktree, not a container.

### ZSH plugin (`shell-plugin/`)

Install: `aimee setup`. Lines starting with `:` are rewritten to `aimee`. File tagging is `@` + Tab (`aimee select file`). Diagnostics: `aimee doctor`. Keyboard: `aimee zsh keyboard`.

### TUI

`crates/aimee_main/src/ui.rs` is the ratatui interactive shell. Theme tokens match the PWA (`#ff5a7a` rose, `#00e5ff` cyan, void `#080612`).

### PWA (`pwa/`)

Installable app shell. `manifest.webmanifest` name **Aimee Codes**, theme `#ff5a7a`. `sw.js` is cache-first. Drafts stay on-device. Local serve: `cd pwa && python3 -m http.server 4173`.

### Config

Primary file: `~/.aimee/.aimee.toml`. Defaults are embedded from `crates/aimee_config/.aimee.toml`. JSON schema: `aimee.schema.json`.

Base-path resolution (`AIMEE_CONFIG` wins, then `OMEGA_CONFIG`):

1. `AIMEE_CONFIG` if set
2. `OMEGA_CONFIG` if set
3. Existing `~/aimee`, `~/.aimee`, `~/omega`, or `~/.omega`
4. Existing Forge-legacy `~/forge` or `~/.forge`
5. Otherwise `~/.aimee`

Notable keys: `services_url` (default `https://api.aimeecodes.dev/`), `restricted`, `tool_timeout_secs`, `subagents`, `research_subagent`, `use_aimee_committer`, `[reasoning]`, `[anda]`, `[retry]`, `[http]`, `[compact]`, `[updates]`.

`AIMEE_`-prefixed variables map onto `.aimee.toml` (`AIMEE_` prefix, `__` nested separator). Legacy `OMEGA_` variables are still read.

| Variable | Role |
|---|---|
| `AIMEE_CONFIG` | Config base directory |
| `AIMEE_SERVICES_URL` | Workspace / indexing API |
| `AIMEE_BIN` | Binary name used by the ZSH plugin (default `aimee`) |
| `AIMEE_LOG` | `tracing` filter (e.g. `aimee=info`) |
| `AIMEE_EDITOR` | Editor for `:edit` / `:config-edit` |

Credentials live under the config base as `.credentials.json`. Do not put API keys in git.

Project policy: `AGENTS.md` (or `~/.aimee/AGENTS.md`). Skills: `.aimee/skills/<name>/SKILL.md`. Commands: `.aimee/commands/`. Agents: `.aimee/agents/`. Project `.mcp.json` wins over `~/.aimee/.mcp.json`.

---

## 7. Tools

`ToolCatalog` (`crates/aimee_domain/src/tools/catalog.rs:41-61`) is the registered set:

| Variant | Tool | Description file |
|---|---|---|
| `Read` | `read` | `fs_read.md` |
| `Write` | `write` | `fs_write.md` |
| `FsSearch` | `fs_search` | `fs_search.md` |
| `SemSearch` | `sem_search` | `semantic_search.md` |
| `Remove` | `remove` | `fs_remove.md` |
| `Patch` | `patch` | `fs_patch.md` |
| `MultiPatch` | `multi_patch` | `fs_multi_patch.md` |
| `Undo` | `undo` | `fs_undo.md` |
| `Shell` | `shell` | `shell.md` |
| `Fetch` | `fetch` | `net_fetch.md` |
| `Followup` | `followup` | `followup.md` |
| `Plan` | `plan` | `plan_create.md` |
| `Skill` | `skill` | `skill_fetch.md` |
| `TodoWrite` | `todo_write` | `todo_write.md` |
| `TodoRead` | `todo_read` | `todo_read.md` |
| `Task` | `task` | `task.md` |

Descriptions must stay under **1024 characters** (`docs/tool-guidelines.md`). New tools join `ToolCatalog` and the existing executor/registry path. Do not leave an unregistered variant.

`ToolRegistry` (`crates/aimee_app/src/tool_registry.rs`) routes catalog tools, agent tools, and MCP tools. Timeouts come from `tool_timeout_secs`. Restricted mode requires explicit permission grants.

When `subagents = true` (default in embedded config), Aimee gets `task` and Sage-as-a-tool is removed. When false, `task` is disabled and `sage` is available instead.

---

## 8. Providers

42 built-in provider IDs (`ProviderId::built_in_providers()` in `crates/aimee_domain/src/provider.rs:97-141`). `aimee provider list` is the source of truth.

`aimee`, `openai`, `open_router`, `requesty`, `zai`, `zai_coding`, `cerebras`, `xai`, `xai_oauth` (SuperGrok), `anthropic`, `claude_code`, `vertex_ai`, `vertex_ai_anthropic`, `big_model`, `azure`, `github_copilot`, `openai_compatible`, `openai_responses_compatible`, `anthropic_compatible`, `aimee_services`, `io_intelligence`, `bedrock`, `minimax`, `codex`, `opencode_zen`, `opencode_go`, `fireworks-ai`, `fireworks-ai-firepass`, `novita`, `vivgrid`, `google_ai_studio`, `modal`, `adal`, `xiaomi_mimo`, `nvidia`, `ambient`, `neuralwatt`, `orca_router`, `meta`, `kimi_coding`, `moonshot`, `alibaba_token_plan`.

Wire protocols: OpenAI, OpenAI Responses, Anthropic, Bedrock, Google, OpenCode (`crates/aimee_config/src/config.rs:18-25`). Inline `[[providers]]` in `.aimee.toml` merge with built-ins (same `id` overrides field-by-field).

---

## 9. Persistence

Diesel + SQLite. Generated schema: `crates/aimee_repo/src/database/schema.rs`.

Current table: `conversations` (`conversation_id`, `title`, `workspace_id`, `context`, `created_at`, `updated_at`, `metrics`).

Shipped migrations (`crates/aimee_repo/src/database/migrations/`):

- `2025-09-12-065405_create_conversations_table`
- `2025-09-12-065740_add_conversations_indexes`
- `2025-10-16-000000_add_metrics_to_conversations`
- `2025-11-13-054241_create_workspace_table`
- `2025-11-15-000000_create_indexing_auth_table`
- `2025-11-22-061212-0000_drop_indexing_auth_table`
- `2026-02-16-130933-0000_drop_workspace_table`

Never edit a shipped migration; add a new one.

gRPC contract: `crates/aimee_repo/proto/aimee.proto` (`package aimee.v1`, service `AimeeService`). RPCs cover search, upload/delete/list/chunk files, health, workspaces, API keys, validate, skill select, fuzzy search, and text-patch build. Default client target is `config.services_url`.

File snapshots: `aimee_repo` + `aimee_snaps`. MCP cache: cacache under the env cache dir.

---

## 10. WEB3 / Anda / KIP

`aimee_anda` adds append-only session pathway checkpoints and hash-chained conversation snapshots for **chat-only** rollbacks. It does not replace the agent runtime.

Enable in `.aimee.toml`:

```toml
[anda]
enabled = true
kip_enabled = true
```

`aimee conversation pathway <id> list` lists checkpoints. Eternal backends (`AndaEternalMode`): `local` (default receipts), `ic_oss`, `canister`, `s3` (`crates/aimee_config/src/anda.rs:11-21`). ICP modes live in `aimee_anda_icp` and return clear errors until configured.

Wallet login in the PWA sits beside provider auth. Payments and spend stay HITL.

---

## 11. Loop autonomy

`crates/aimee_domain/src/loop_autonomy.rs` defines HITL probes for `/goal`:

1. What does done look like (observable outcome)?
2. How will we verify (tests, commands, evidence)?
3. What must not change (boundaries)?
4. Who is the human owner, and when should we stop and ask?
5. What Linear issue / GitHub PR / related work should we log against?

Exactly five answered probes (`GoalProbeSet`). Default tool-failure budget is unlimited (`usize::MAX`) unless `max_tool_failure_per_turn` is set.

---

## 12. Skills, commands, templates

### Built-in skills (`.aimee/skills/`)

`create-agent`, `create-command`, `create-github-issue`, `create-plan`, `debug-cli`, `github-pr-comments`, `post-aimee-feature`, `resolve-conflicts`, `resolve-fixme`, `test-reasoning`, `write-release-notes`.

Additional skills exist in the product skill catalog (`execute-plan`, `github-pr-description`, `greploop`) when present under `.aimee/skills/`.

### Project commands (`.aimee/commands/`)

`check.md`, `fixme.md`.

### Prompt templates (`templates/`)

`aimee-command-generator-prompt.md`, `aimee-commit-message-prompt.md`, `aimee-custom-agent-template.md`, `aimee-doom-loop-reminder.md`, `aimee-partial-skill-instructions.md`, `aimee-partial-summary-frame.md`, `aimee-partial-system-info.md`, `aimee-partial-tool-error-reflection.md`, `aimee-partial-tool-use-example.md`, `aimee-pending-todos-reminder.md`, `aimee-system-prompt-title-generation.md`, `aimee-tool-retry-message.md`.

Edit templates only when the task is agent/prompt behavior.

---

## 13. Other islands

| Path | Role |
|---|---|
| `AGENTS.md` | House rules for coding agents |
| `docs/tool-guidelines.md` | Tool-description constraints |
| `benchmarks/` | TypeScript eval harness (`tsx`, Node, `npm run eval`) |
| `scripts/` | `tdd-gate.sh`, `greptile-pre-push.sh`, `dev-aimee.sh`, … |
| `plans/` | Muse plans — historical unless cited |
| `assets/brand/` | Wordmark, flock mark |
| `.github/workflows/` | Generated/checked-in CI (prefer editing `aimee_ci`) |
| `flake.nix` | Nix package for `aimee` |
| `package.json` | Eval/bounty TypeScript (`aimee-codes-evals`) |

GitHub workflows: `autofix.yml`, `bounty.yml`, `ci.yml`, `coderabbit.yml`, `labels.yml`, `release-drafter.yml`, `release.yml`, `stale.yml`. CI sets `RUSTFLAGS=-D warnings`.

---

## 14. Testing contract

Every test has three named steps: **fixture → actual → expected**. Assert on the whole value.

```rust
use pretty_assertions::assert_eq;

fn test_foo() {
    let fixture = /* ... */;
    let actual = /* execute */;
    let expected = /* handwritten */;
    assert_eq!(actual, expected);
}
```

- Tests live in the same file as the source (`#[cfg(test)]`)
- `unwrap` in test functions; `anyhow::Result` in fixtures
- Snapshots use **insta** (`insta.yaml` auto-accepts; runner is nextest)
- Prefer `cargo insta test --accept -p <crate>`

### Verify (Rust)

```bash
cargo fmt
cargo check -p <crate>
cargo clippy -p <crate> --all-targets -- -D warnings
cargo insta test --accept -p <crate>
```

Never `cargo build --release` unless the task is a release binary or a measured benchmark.

TypeScript evals: `npx tsc --noEmit` then `npm run eval` / `npm run test:bounty`.

---

## 15. Security

- Treat CLI args, tool results, MCP output, and file contents as untrusted
- Parameterized commands only
- AuthN is not AuthZ
- Never print, log, or commit tokens, keys, connection strings, `.env`, or user data
- Restricted mode: tool execution requires permission grants
- Credentials in `.credentials.json`, not git
- Commits and GitHub comments include `Co-Authored-By: AimeeCodes <noreply@aimeecodes.dev>`

---

## 16. How to run

```bash
# Nix
nix run github:swcstudiospace/omegaloops

# From a local checkout
cargo install --path crates/aimee_main --locked --bin aimee

aimee provider login
aimee
aimee setup
```

On first run, Aimee walks through provider login if no credentials are stored. Existing `~/.omega` directories are still picked up until you migrate (`aimee config migrate`).

---

## 17. Owner

| | |
|---|---|
| Legal name | Spectrum Web Co LLC |
| Brand | [@swcstudio](https://github.com/swcstudio) |
| Product | Aimee Codes — CLI agent flock |
| Command | `aimee` |
| GitHub (current remote) | [swcstudiospace/omegaloops](https://github.com/swcstudiospace/omegaloops) |
| Studio | [swcstudio.space](https://www.swcstudio.space/) |
| Contact | [ovesheng@spectrumweb.co](mailto:ovesheng@spectrumweb.co) |
| License | Apache-2.0 |

When the GitHub repository is renamed, update `Cargo.toml` `repository`, `flake.nix` `homepage`, README badges, and `benchmarks/evals/*/task.yml` clone URLs together. Do not invent a GitHub path that does not exist.
