# Configuration

How Aimee Codes finds its home directory, which keys actually exist, and which CLI verbs read or write them. Policy for agents is `AGENTS.md`. This page is the human map.

Do not invent keys. The sources of truth are `aimee.schema.json`, the embedded defaults in `crates/aimee_config/.aimee.toml`, and `AimeeConfig` (`crates/aimee_config/src/config.rs:122-345`). If a flag is not in those three, it is not a config key.

## Purpose

| Goal | Use |
|---|---|
| See the resolved file | `aimee config path` |
| Dump the merged view | `aimee config list` |
| Change the session / commit / suggest model, or reasoning effort | `aimee config set …` |
| Store provider secrets | `aimee provider login` — **not** `.aimee.toml` |
| Point Aimee at another home | `AIMEE_CONFIG` |
| Keep an Omega / Forge install working | Leave the compat path in place. Do not delete it. |

## When to use

Edit `~/.aimee/.aimee.toml` (or the file `aimee config path` prints) for persistent product settings: timeouts, retry, HTTP, restricted mode, Anda, sampling.

Use `aimee config set` only for the four typed fields the CLI actually writes: session model, commit model, suggest model, reasoning effort (`crates/aimee_main/src/cli.rs:792-818`).

Use environment variables to override a key for one process, or to relocate the config base.

Do **not** put API keys, OAuth tokens, or MCP client secrets in `.aimee.toml`. Those belong in `.credentials.json` / `.mcp-credentials.json` via `aimee provider login` and `aimee mcp login`. See [Security](security.md).

## Base-path resolution

Every path under “the Aimee home” is relative to one directory. Resolution is cached for the process (`crates/aimee_config/src/reader.rs:33-86`).

Order (`crates/aimee_config/src/reader.rs:67-86`, `AIMEE.md:209-215`, `README.md:270-276`):

1. `AIMEE_CONFIG` if set — wins.
2. `OMEGA_CONFIG` if set — legacy alias. Still read. Do not delete it from existing installs.
3. First existing candidate under `$HOME`, in this order: `aimee`, `.aimee`, `omega`, `.omega`, `forge`, `.forge`.
4. Otherwise `$HOME/.aimee` for a new install.

`aimee config path` prints `{base}/.aimee.toml` (`crates/aimee_main/src/ui.rs:5188-5190`, `crates/aimee_config/src/reader.rs:49-53`).

### What lives under the base

Resolved from `Environment` (`crates/aimee_domain/src/env.rs:62-180`):

| Path | Role |
|---|---|
| `.aimee.toml` | Primary TOML config |
| `.omega.toml` | Leftover Omega TOML, still loaded |
| `.config.json` | Legacy JSON config |
| `.credentials.json` | Provider API keys and OAuth tokens |
| `.mcp-credentials.json` | MCP OAuth tokens (separate store) |
| `.mcp.json` | User-scope MCP servers |
| `.mcp_trust.json` | Accept/reject hashes for project-local MCP |
| `permissions.yaml` | Restricted-mode permission grants |
| `provider.json` | File-based provider override (OAuth-capable) |
| `.aimee.db` | SQLite conversations |
| `logs/` | Rolling `aimee.log` |
| `agents/`, `skills/`, `commands/`, `AGENTS.md` | Global agents, skills, commands, policy |
| `cache/` | MCP cache (cacache) |
| `snapshots/` | File snapshots |
| `pathways/` | Anda checkpoints (when `[anda]` is enabled) |

Project-local overlays (cwd, not the base): `.aimee/agents/`, `.aimee/skills/`, `.aimee/commands/`, `AGENTS.md`, `.mcp.json`. Project `.mcp.json` is preferred over the user file (`AIMEE.md:231`, `README.md:314`).

## Merge order

`AimeeConfig::read` stacks sources, later wins (`crates/aimee_config/src/config.rs:355-361`):

1. **Legacy JSON** — `{base}/.config.json`, converted to TOML (`crates/aimee_config/src/reader.rs:156-165`).
2. **Embedded defaults** — `crates/aimee_config/.aimee.toml`, compiled in (`crates/aimee_config/src/reader.rs:98-103`).
3. **User files** — `{base}/.omega.toml` then `{base}/.aimee.toml`. Missing files are skipped (`crates/aimee_config/src/reader.rs:142-154`).
4. **Environment** — `OMEGA_*` then `AIMEE_*`. `AIMEE_` overwrites `OMEGA_` when both are set (`crates/aimee_config/src/reader.rs:105-125`).

On `build()`, the reader also walks from the cwd to `/` and loads every `.env` it finds. Closer directories take priority (`crates/aimee_config/src/reader.rs:10-31`, `crates/aimee_config/src/reader.rs:127-131`).

When the CLI writes config it serializes the current `AimeeConfig` and prefixes a schema hint (`crates/aimee_config/src/writer.rs:27-36`):

```toml
"$schema" = "https://aimeecodes.dev/schema.json"
```

That `$schema` key is for editors. It is not an `AimeeConfig` field.

## `.aimee.toml` keys

Schema title is `AimeeConfig` (`aimee.schema.json:3-6`). JSON Schema `default` for many integers is `0`; **operational** defaults are the embedded file. Prefer the embedded values below.

### Top-level

| Key | Embedded default | What it does |
|---|---|---|
| `auto_open_dump` | `false` | Open HTML dumps in a browser after creation (`aimee.schema.json:34-37`). |
| `auto_dump` | absent | `json` or `html` session dump on task completion; disabled when absent (`crates/aimee_config/src/auto_dump.rs:7-11`). |
| `auto_install_vscode_extension` | `true` | Install the VS Code extension when running inside VS Code (`aimee.schema.json:29-32`, `crates/aimee_config/.aimee.toml:34`). |
| `currency_symbol` | `"$"` | Rprompt cost symbol (`aimee.schema.json:66-69`). |
| `currency_conversion_rate` | `1.0` | Multiply raw USD cost for display (`aimee.schema.json:61-64`). |
| `custom_history_path` | absent | Override `{base}/.aimee_history` (`aimee.schema.json:71-76`). |
| `debug_requests` | absent | Directory for raw request bodies. Disabled when absent. Do not point this at a git worktree. |
| `max_commit_count` | `20` | Recent commits fed to `aimee commit`. |
| `max_conversations` | `100` | Conversation list length. |
| `max_extensions` | `15` | File extensions listed in the system prompt. |
| `max_fetch_chars` | `50000` | Cap on `fetch` tool text. |
| `max_file_read_batch_size` | `50` | Files per batch read. |
| `max_file_size_bytes` | `104857600` | Read-tool file size cap (100 MiB). |
| `max_image_size_bytes` | `262144` | Read-tool image size cap. |
| `max_line_chars` | `2000` | Characters per line when reading a file. |
| `max_parallel_file_reads` | `64` | Concurrent file reads. |
| `max_read_lines` | `2000` | Lines per file read. |
| `max_requests_per_turn` | `100` | Requests allowed in one turn. |
| `max_search_lines` | `1000` | Lines returned by one `fs_search`. |
| `max_search_result_bytes` | `10240` | Bytes returned by one `fs_search`. |
| `max_sem_search_results` | `100` | Candidate hits from the vector query. |
| `max_stdout_line_chars` | `500` | Characters per captured shell line. |
| `max_stdout_prefix_lines` | `100` | Leading shell-output lines kept. |
| `max_stdout_suffix_lines` | `100` | Trailing shell-output lines kept. |
| `max_tokens` | `20480` | Generation cap for all agents (schema range 1–100000). |
| `max_tool_failure_per_turn` | absent | Force-complete after N tool failures. Unlimited (`usize::MAX`) when unset (`AIMEE.md:328`). |
| `merge_system_messages` | `false` | Collapse system messages for providers that reject mid-conversation system turns (vLLM, NIM) (`aimee.schema.json:235-238`). |
| `model_cache_ttl_secs` | `604800` | Cached model-list TTL (7 days). |
| `restricted` | `false` | Require permission grants before catalog tools run (`aimee.schema.json:270-273`). See [Security](security.md). |
| `research_subagent` | `false` | Add Sage to the agent list and enable `:sage` (`aimee.schema.json:265-268`). |
| `sem_search_top_k` | `10` | Hits kept after re-rank. |
| `services_url` | `https://api.aimeecodes.dev/` | Workspace / indexing API (`crates/aimee_config/.aimee.toml:23`). |
| `subagents` | `true` | Give Aimee the `task` tool and remove Sage-as-a-tool. When `false`, `task` is off and `sage` is available (`aimee.schema.json:309-312`). |
| `temperature` | absent | Sampling temperature 0.0–2.0. |
| `tool_supported` | `true` | Master switch for tool use (`aimee.schema.json:336-339`). |
| `tool_timeout_secs` | `300` | Per-tool timeout. Agent/`task` calls do not use this timeout (`crates/aimee_app/src/tool_registry.rs:54-60`, `crates/aimee_app/src/tool_registry.rs:119`). |
| `top_k` | `30` | Vocabulary cutoff 1–1000. |
| `top_p` | `0.8` | Nucleus sampling 0.0–1.0. |
| `use_aimee_committer` | `true` | `aimee commit` sets `GIT_COMMITTER_NAME` / `GIT_COMMITTER_EMAIL` to the Aimee identity (`aimee.schema.json:379-382`). |
| `use_text_patch_fallback` | `false` | Use the text-patch gRPC API instead of the legacy fuzzy range lookup (`aimee.schema.json:384-387`). |
| `verify_todos` | `true` | Remind the model about incomplete todos at end of task. |

`auto_dump`, `debug_requests`, `custom_history_path`, `temperature`, `max_tool_failure_per_turn`, and the nested tables below are omitted from the file when unset (`skip_serializing_if`).

The embedded defaults file also contains `terminal_context = false` (`crates/aimee_config/.aimee.toml:24`). That key is **not** on `AimeeConfig` and **not** in `aimee.schema.json`. Serde ignores it. Do not set it expecting a behavior change. Terminal context is injected by the ZSH plugin via `_AIMEE_TERM_*` environment variables (`crates/aimee_app/src/terminal_context.rs:7-17`), not via TOML.

### `[session]`, `[commit]`, `[suggest]`

Each is a `ModelConfig` (`crates/aimee_config/src/model.rs:14-18`, `aimee.schema.json:298-323`):

```toml
[session]
provider_id = "openai"
model_id = "gpt-4.1"

[commit]
provider_id = "openai"
model_id = "gpt-4.1-mini"

[suggest]
provider_id = "openai"
model_id = "gpt-4.1-mini"
```

| Table | Role |
|---|---|
| `session` | Default provider + model when an agent does not override. |
| `commit` | Model used by `aimee commit`. `None` falls back to the session pair. |
| `suggest` | Model used by `aimee suggest`. |

### `[reasoning]`

`ReasoningConfig` (`crates/aimee_config/src/reasoning.rs:13-33`). Embedded default (`crates/aimee_config/.aimee.toml:72-74`):

```toml
[reasoning]
enabled = true
effort = "medium"
```

| Key | Role |
|---|---|
| `effort` | `none`, `minimal`, `low`, `medium`, `high`, `xhigh`, `max` (`crates/aimee_config/src/reasoning.rs:42-56`). |
| `max_tokens` | Thinking-token budget. Must be &gt; 1024 and less than `max_tokens`. |
| `exclude` | Think but hide the reasoning from the caller. |
| `enabled` | Enable reasoning at medium effort with no exclusions. |

Supported surface varies by provider (OpenRouter, Anthropic, Aimee). `aimee config set reasoning-effort <lvl>` writes `effort` through `ConfigOperation::SetReasoningEffort` (`crates/aimee_domain/src/env.rs:30-31`).

### `[retry]`

`RetryConfig` (`crates/aimee_config/src/retry.rs:11-26`). Embedded default (`crates/aimee_config/.aimee.toml:38-44`):

```toml
[retry]
backoff_factor = 2
initial_backoff_ms = 200
max_attempts = 8
min_delay_ms = 1000
status_codes = [429, 500, 502, 503, 504, 408, 522, 524, 520, 529]
suppress_errors = false
```

`max_delay_secs` exists on the struct and is optional. It is not set in the embedded file.

### `[http]`

`HttpConfig` (`crates/aimee_config/src/http.rs:31-55`). Embedded default (`crates/aimee_config/.aimee.toml:46-58`):

```toml
[http]
accept_invalid_certs = false
adaptive_window = true
connect_timeout_secs = 30
hickory = false
keep_alive_interval_secs = 60
keep_alive_timeout_secs = 10
keep_alive_while_idle = true
max_redirects = 10
pool_idle_timeout_secs = 90
pool_max_idle_per_host = 5
read_timeout_secs = 900
tls_backend = "default"
```

Also on the struct, not in the embedded file:

| Key | Role |
|---|---|
| `min_tls_version` / `max_tls_version` | `1.0`, `1.1`, `1.2`, `1.3` (`crates/aimee_config/src/http.rs:7-15`). |
| `root_cert_paths` | Extra PEM/DER roots. Comma-separated when set via env. |

`tls_backend` is `default` or `rustls`. `accept_invalid_certs = true` calls `danger_accept_invalid_certs` (`crates/aimee_infra/src/http.rs:97-99`). Leave it `false`. See [Security](security.md).

### `[compact]`

`Compact` (`crates/aimee_config/src/compact.rs:47-96`). Embedded default (`crates/aimee_config/.aimee.toml:60-66`):

```toml
[compact]
eviction_window = 0.2
max_tokens = 2000
message_threshold = 200
on_turn_end = false
retention_window = 6
token_threshold = 100000
```

Also on the struct: `token_threshold_percentage` (0.0–1.0), `turn_threshold`, `model`. `eviction_window` and `token_threshold_percentage` reject values outside `[0.0, 1.0]` (`crates/aimee_config/src/compact.rs:209-238`).

### `[updates]`

`Update` (`crates/aimee_config/src/compact.rs:37-42`). Embedded default (`crates/aimee_config/.aimee.toml:68-70`):

```toml
[updates]
auto_update = true
frequency = "daily"
```

`frequency` is `daily`, `weekly`, `never`, or `always` (`crates/aimee_config/src/compact.rs:13-18`).

### `[anda]`

`AndaConfig` (`crates/aimee_config/src/anda.rs:30-78`). **Commented out** in the embedded file — pathways stay off until you uncomment (`crates/aimee_config/.aimee.toml:76-86`):

```toml
[anda]
enabled = true
kip_enabled = true
nexus_url = "http://127.0.0.1:8091"
eternal_enabled = true
eternal_mode = "local"
eternal_label_prefix = "aimee"
log_responses = true
log_turn_end = true
hard_fail = false
```

| Key | Default when the table is present | Role |
|---|---|---|
| `enabled` | `false` | Master switch for hash-chained checkpoints. |
| `pathway_dir` | `{base}/pathways` | Checkpoint directory. |
| `nexus_url` | absent | Cognitive Nexus base URL. |
| `kip_enabled` | `false` | KIP `execute_kip` side effects. |
| `eternal_enabled` | `true` | Export to eternal storage. |
| `eternal_mode` | `local` | `local`, `ic_oss`, `canister`, `s3` (`crates/aimee_config/src/anda.rs:11-21`). |
| `eternal_dir` | `{base}/pathways/eternal` | Local receipt root. |
| `eternal_label_prefix` | `"aimee"` | Receipt label prefix. |
| `log_responses` / `log_turn_end` | `true` | When to append a checkpoint. |
| `hard_fail` | `false` | Fail the turn if pathway logging fails. |

Checkpoints are chat-only. They do not revert workspace files (`crates/aimee_main/src/cli.rs:959-961`). Inspect with `aimee conversation pathway <id> list`.

### `[[providers]]`

Inline provider entries merge with the built-in list. Same `id` overrides field-by-field; a new `id` is appended (`crates/aimee_config/src/config.rs:78-115`, `aimee.schema.json:247-252`).

```toml
[[providers]]
id = "my_gateway"
url = "https://llm.example.internal/v1/chat/completions"
api_key_var = "MY_GATEWAY_API_KEY"
response_type = "OpenAI"
auth_methods = ["api_key"]
models = "https://llm.example.internal/v1/models"
```

| Field | Role |
|---|---|
| `id` | Provider id used in model paths. |
| `url` | Chat URL. May contain `{{VAR}}` placeholders. |
| `api_key_var` | Env var holding the key (placeholder name only in docs). |
| `models` | URL string **or** a static `[[providers.models]]` list. |
| `response_type` | `OpenAI`, `OpenAIResponses`, `Anthropic`, `Bedrock`, `Google`, `OpenCode` (`crates/aimee_config/src/config.rs:18-25`). |
| `url_param_vars` | Template variables (`name`, optional `options`, `optional`). |
| `custom_headers` | Extra headers on every request. Do not put secrets here. |
| `provider_type` | `llm` (default) or `context_engine`. |
| `auth_methods` | `api_key` or `google_adc`. Defaults to `["api_key"]`. |

OAuth device / authorization-code flows are **not** expressible as `ProviderAuthMethod` in TOML. Those providers use `{base}/provider.json` (`crates/aimee_config/src/config.rs:38-42`). `aimee provider list` is the source of truth for built-in ids. See [Providers](providers.md).

## Environment variables

### Schema mapping

`AIMEE_`-prefixed variables map onto `.aimee.toml` (`crates/aimee_config/src/reader.rs:105-125`):

- Prefix: `AIMEE` (or legacy `OMEGA`)
- Prefix separator: `_`
- Nested separator: `__`
- Lists: comma-separated for `retry.status_codes` and `http.root_cert_paths`

Examples (placeholders only):

```bash
export AIMEE_RESTRICTED=true
export AIMEE_TOOL_TIMEOUT_SECS=120
export AIMEE_SERVICES_URL=https://api.aimeecodes.dev/
export AIMEE_HTTP__ACCEPT_INVALID_CERTS=false
export AIMEE_HTTP__CONNECT_TIMEOUT_SECS=15
export AIMEE_RETRY__MAX_ATTEMPTS=4
export AIMEE_RETRY__STATUS_CODES=429,500,502,503
export AIMEE_REASONING__EFFORT=medium
```

`AIMEE_*` overwrites `OMEGA_*` when both are set. Do not put secrets in these variables in shell history, CI logs, or committed dotenv files.

### Path and process variables (not schema keys)

These are **not** `AimeeConfig` fields (`AIMEE.md:221-227`, `README.md:295-301`):

| Variable | Role |
|---|---|
| `AIMEE_CONFIG` | Config base directory. Wins over every candidate. |
| `OMEGA_CONFIG` | Legacy base-directory alias. Still honored. |
| `AIMEE_SERVICES_URL` | Same as `services_url` (this one *is* a schema key). |
| `AIMEE_BIN` | Binary name the ZSH plugin invokes (default `aimee`). |
| `AIMEE_LOG` | `tracing` filter, e.g. `aimee=info` (`crates/aimee_tracker/src/log.rs:32`). |
| `AIMEE_EDITOR` | Editor for `:edit` / `:config-edit`. |

Provider-specific key names (`OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, …) are read only during the one-shot env → file migration when `.credentials.json` does not yet exist (`crates/aimee_repo/src/provider/provider_repo.rs:334-389`). After that, login writes the file. `AIMEE_API_KEY` still falls back to `OMEGA_API_KEY` during that migration (`crates/aimee_repo/src/provider/provider_repo.rs:401-404`). Host aliases: `OLLAMA_HOST` ← `OLLAMA_URL`, same pattern for vLLM / LM Studio / llama.cpp / Jan, and `AIMEE_WORKSPACE_SERVER_URL` ← `OMEGA_WORKSPACE_SERVER_URL` (`crates/aimee_repo/src/provider/provider_repo.rs:106-116`).

## CLI: `aimee config`

`ConfigCommandGroup` (`crates/aimee_main/src/cli.rs:747-774`). `--porcelain` is global on the group.

```bash
aimee config path
aimee config list
aimee config list --porcelain

aimee config get model
aimee config get provider
aimee config get commit
aimee config get suggest
aimee config get reasoning-effort

aimee config set model <provider> <model>
aimee config set commit <provider> <model>
aimee config set suggest <provider> <model>
aimee config set reasoning-effort medium

aimee config migrate
```

| Verb | Behavior |
|---|---|
| `path` | Print `{base}/.aimee.toml`. |
| `list` | Show the merged configuration (`handle_config_command` → `on_show_config`, `crates/aimee_main/src/ui.rs:5185-5187`). |
| `get` | Typed fields only: `model`, `provider`, `commit`, `suggest`, `reasoning-effort` (`crates/aimee_main/src/cli.rs:823-833`). Prints `…: Not set` when absent. |
| `set` | Typed fields only. `model` activates provider + model atomically. `commit` / `suggest` validate that the model belongs to that provider. `reasoning-effort` accepts `none`, `minimal`, `low`, `medium`, `high`, `xhigh`, `max` (`crates/aimee_main/src/ui.rs:5243-5287`). |
| `migrate` | Rename `~/aimee`, `~/.omega`, or `~/omega` → `~/.aimee`. Errors if no legacy dir exists, if `~/.aimee` already exists, or if the rename fails (`crates/aimee_main/src/ui.rs:5199-5240`). Does **not** move `~/forge` or `~/.forge`. |

`set` / `get` do **not** accept arbitrary dotted keys. To change `restricted`, `tool_timeout_secs`, `[http]`, `[anda]`, or `[[providers]]`, edit the TOML (or set `AIMEE_*`) and restart.

ZSH persist helpers write the same file: `:config-model` / `:cm`, `:config-reload` / `:cr` (`README.md:176-178`). Session-only `:model` / `:reasoning-effort` do not persist.

## CLI: `aimee provider`

`ProviderCommandGroup` (`crates/aimee_main/src/cli.rs:968-1003`). This is **not** `aimee pod provider` (a DevPod verb).

```bash
aimee provider login
aimee provider login openai
aimee provider logout
aimee provider logout openai
aimee provider list
aimee provider list --type llm
aimee provider list --type context_engine
aimee provider list --porcelain
```

| Verb | Behavior |
|---|---|
| `login [provider]` | Interactive picker when the id is omitted. Always re-configures so you can rotate a key (`crates/aimee_main/src/ui.rs:1265-1272`). Writes `{base}/.credentials.json`. |
| `logout [provider]` | Drops that provider’s credential. Picker when omitted. |
| `list` | Built-in + configured providers. `--type` may be repeated (`llm`, `context_engine`). |

On first run with no stored credentials, the TUI walks through login (`README.md:65`). There are 42 built-in ids; `aimee provider list` is authoritative (`AIMEE.md:268`).

Related: `aimee mcp login <name>` / `aimee mcp logout <name|all>` store OAuth in `.mcp-credentials.json`, not `.credentials.json` (`crates/aimee_main/src/cli.rs:671-715`).

## Compat paths (do not delete)

Intentional Omega / Forge compatibility (`AIMEE.md:43-52`). Existing installs must keep working.

| Compat | Still honored | Do not |
|---|---|---|
| `OMEGA_CONFIG` | Read after `AIMEE_CONFIG` (`crates/aimee_config/src/reader.rs:67-73`) | Unset it on a machine that still uses `~/.omega` |
| Candidate dirs `omega`, `.omega`, `forge`, `.forge` | Used if they exist and no `AIMEE_CONFIG` is set (`crates/aimee_config/src/reader.rs:76`) | `rm -rf ~/.omega` or `~/.forge` “to clean up” |
| `{base}/.omega.toml` | Loaded before `.aimee.toml` (`crates/aimee_config/src/reader.rs:151`) | Delete it until settings have been saved as `.aimee.toml` |
| `OMEGA_*` env vars | Read, then overwritten by `AIMEE_*` | Assume they are ignored |
| `aimee config migrate` | Moves `~/aimee`, `~/.omega`, `~/omega` → `~/.aimee` | Run it if `~/.aimee` already exists — it will refuse |
| ZSH `:omega` | Documented alias where the plugin still maps it | Remove it from a user’s `.zshrc` as a “cleanup” |
| `AIMEE_API_KEY` ← `OMEGA_API_KEY` | Env → file migration only | Put a live key in this page |

`~/forge` and `~/.forge` are **detected** as a base path. They are **not** renamed by `aimee config migrate`. Leave them until the operator copies settings by hand.

## Secrets: placeholders only

```toml
# Wrong — never do this
# [[providers]]
# custom_headers = { Authorization = "Bearer sk-live-…" }

# Right — name the variable, store the value via login
[[providers]]
id = "my_gateway"
url = "https://llm.example.internal/v1/chat/completions"
api_key_var = "MY_GATEWAY_API_KEY"
```

```bash
# Right
aimee provider login my_gateway
# paste the key at the prompt; it lands in ~/.aimee/.credentials.json (mode 0o600)
```

Never commit `.credentials.json`, `.mcp-credentials.json`, `.env`, or a `debug_requests` dump. See [Security](security.md).

## File interactions

```text
process start
    │
ConfigReader::resolve_base_path     crates/aimee_config/src/reader.rs:67-86
    │  AIMEE_CONFIG → OMEGA_CONFIG → candidates → ~/.aimee
    ▼
AimeeConfig::read                   crates/aimee_config/src/config.rs:355-361
    │  legacy JSON → defaults → .omega.toml → .aimee.toml → OMEGA_* → AIMEE_*
    ▼
Environment.base_path               crates/aimee_domain/src/env.rs:58-59
    │
    ├── .aimee.toml                 config
    ├── .credentials.json           provider secrets
    ├── .mcp-credentials.json       MCP OAuth
    ├── permissions.yaml            restricted-mode grants
    └── provider.json               OAuth-capable provider override

aimee config set model …
    │
ConfigOperation::SetSessionConfig   crates/aimee_domain/src/env.rs:16-31
    ▼
AimeeConfig::write → ConfigWriter   crates/aimee_config/src/writer.rs:27-36

aimee provider login
    │
ProviderRepository::upsert_credential
    ▼
{base}/.credentials.json            mode 0o600 on Unix
```

## Best practices

- Start from `aimee config path` so you know which directory is live. Compat candidates can surprise you.
- Set only the keys you need. Embedded defaults already cover timeouts, retry, and HTTP.
- Prefer `aimee config set model <provider> <model>` over hand-editing `[session]` so the provider is validated.
- Override with `AIMEE_*` in a process supervisor or direnv — not in a committed `.env`.
- Keep `http.accept_invalid_certs = false`. Do not weaken TLS to make a call work.
- Enable `[anda]` only when you want hash-chained chat checkpoints. It is off by default.
- After an Omega install, run `aimee config migrate` **once**, or keep using `~/.omega` via the candidate list. Do not delete the old directory first.

## Anti-patterns

- Inventing keys (`log_level`, `api_key`, `terminal_context` as a TOML switch). They are not on `AimeeConfig`.
- Putting secrets in `.aimee.toml`, `[[providers]].custom_headers`, or this GitBook.
- Using `aimee config set restricted true` — that verb does not exist. Edit the file or export `AIMEE_RESTRICTED=true`.
- Deleting `~/.omega` / `~/.forge` because the brand is now Aimee. Compat is intentional (`AIMEE.md:43-45`).
- Pointing `debug_requests` at the repo. Request bodies can contain bearer tokens.
- Treating `aimee pod provider` as `aimee provider`. Different command group.

## Verify

```bash
aimee config path
aimee config list
aimee config get model
cargo test -p aimee_config
```

Crate tests cover env overlay, TOML round-trip, provider entry shapes, and default `services_url` (`crates/aimee_config/src/config.rs:376-531`).
