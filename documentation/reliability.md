# Reliability

How Aimee Codes bounds a turn: tool timeouts, HTTP retries, request caps, context compaction, HITL `/goal` probes, and logs. This page is the operator map. Policy for agents is `AGENTS.md`. Streaming internals live on [Streaming](architecture/streaming.md). CI lives on [CI/CD](ops/cicd.md).

There are **no published SLOs, error budgets, or Grafana dashboards in the `aimeecodes` tree**. Do not invent them. Reliability here means the knobs and interrupt paths that actually exist.

## Purpose

| Goal | Use |
|---|---|
| Cap a hung tool | `tool_timeout_secs` (default `300`) |
| Cap provider retries | `[retry]` in `.aimee.toml` |
| Cap model calls in one turn | `max_requests_per_turn` (default `100`) |
| Cap repeated tool errors | `max_tool_failure_per_turn` (optional; default unlimited) |
| Shrink a long conversation | `[compact]` or `:compact` |
| Force a human definition of done | `/goal` five-probe HITL |
| Read what just happened | `aimee logs` / `AIMEE_LOG` |

See [Configuration](configuration.md) for how those keys merge.

## When to use

Set timeouts and request caps on any machine that talks to a paid provider. Leave `max_tool_failure_per_turn` unset only when you accept that a bad tool loop can run until `max_requests_per_turn` or the human stops it.

Use `/goal` when the work has an observable outcome and you need the loop to stop and ask. Do not treat compaction `retention_window` as a legal retention policy — that knob is prompt-window only ([Security](security.md)).

## Timeouts

### Tool calls

Every catalog tool future is wrapped in `tokio::time::timeout` using `tool_timeout_secs` (`crates/aimee_app/src/tool_registry.rs:45-60`). Default is `300` seconds (`crates/aimee_config/.aimee.toml:26`, `crates/aimee_config/src/config.rs:164-167`).

On expiry the registry returns `Error::CallTimeout`. The timeout value reported in that error is **minutes** (`as_secs() / 60`), not seconds (`crates/aimee_app/src/tool_registry.rs:57-59`).

`aimee info` prints the resolved tool timeout (`crates/aimee_main/src/info.rs:389`).

Restricted-mode permission checks run **before** the timeout so a hung grant prompt cannot skip the gate (`crates/aimee_app/src/tool_registry.rs:140-153`). See [Security](security.md).

### HTTP client

`[http]` defaults (`crates/aimee_config/.aimee.toml:46-58`, `crates/aimee_config/src/http.rs:28-55`):

| Key | Default | Role |
|---|---|---|
| `connect_timeout_secs` | `30` | TCP/TLS connect |
| `read_timeout_secs` | `900` | Response body |
| `pool_idle_timeout_secs` | `90` | Idle connection reuse |
| `pool_max_idle_per_host` | `5` | Per-host idle pool |
| `max_redirects` | `10` | Redirect cap |
| `keep_alive_interval_secs` | `60` | HTTP keep-alive ping |
| `keep_alive_timeout_secs` | `10` | Keep-alive idle |
| `accept_invalid_certs` | `false` | Do not flip this on to "make a call work" |

`read_timeout_secs = 900` is the long-poll budget for streaming completions. It is not a tool timeout.

## Retries

Provider chat turns retry through `retry_with_config` (`crates/aimee_app/src/orch.rs:282-310`, `crates/aimee_app/src/retry.rs:7-39`). The strategy is `backon::ExponentialBuilder` with jitter.

Only `aimee_domain::Error::Retryable` triggers another attempt (`crates/aimee_app/src/retry.rs:31-38`). Other errors fail immediately.

`RetryConfig` fields (`crates/aimee_config/src/retry.rs:11-26`) and embedded defaults (`crates/aimee_config/.aimee.toml:38-44`):

| Key | Default | Role |
|---|---|---|
| `initial_backoff_ms` | `200` | First delay (struct field; builder uses `min_delay_ms`) |
| `min_delay_ms` | `1000` | Minimum delay passed to `ExponentialBuilder` |
| `backoff_factor` | `2` | Multiplier |
| `max_attempts` | `8` | `with_max_times` |
| `status_codes` | `429, 500, 502, 503, 504, 408, 522, 524, 520, 529` | Which HTTP statuses the provider layer marks retryable |
| `max_delay_secs` | unset | Optional ceiling |
| `suppress_errors` | `false` | Suppress retry log/events |

Each retry logs at `error` with `agent_id`, root cause, and model, then emits `ChatResponse::RetryAttempt` to the TUI (`crates/aimee_app/src/orch.rs:295-307`).

## Request and tool-failure budgets

### `max_requests_per_turn`

Default `100` in the embedded TOML (`crates/aimee_config/.aimee.toml:12`). On `AimeeConfig` the field is `Option<usize>` (`crates/aimee_config/src/config.rs:258-260`). Agent definitions can override; otherwise the global value is copied on (`crates/aimee_app/src/agent.rs:125-128`).

The orchestrator increments a request counter each loop and yields when the cap is hit (`crates/aimee_app/src/orch.rs:261-264`, `crates/aimee_app/src/orch.rs:390`).

### `max_tool_failure_per_turn`

Optional. When omitted, the default budget is `usize::MAX` — the orchestrator never force-completes on tool errors (`crates/aimee_domain/src/loop_autonomy.rs:13-17`, `AIMEE.md:328`).

The README example sets `max_tool_failure_per_turn = 3` (`README.md:282`). That is illustrative. The embedded defaults **do not** set the key.

When set, `AimeeApp` builds a `ToolErrorTracker` (`crates/aimee_app/src/app.rs:111-195`). After each tool batch the orchestrator annotates failing results with `attempts_left` / `allowed_max_attempts` via `aimee-tool-retry-message.md` (`crates/aimee_app/src/orch.rs:344-359`). Hitting the limit sends `ChatResponse::Interrupt` with `InterruptionReason::MaxToolFailurePerTurnLimitReached` and yields (`crates/aimee_app/src/orch.rs:372-379`).

## Backpressure (what exists)

- **Task tools run in parallel; every other catalog tool runs sequentially** so the TUI can handshake `ToolCallStart` before stdout (`crates/aimee_app/src/orch.rs:72-115`).
- **Stream channel capacity is 1** (`MpscStream`). A slow renderer stalls the producer. Documented on [Streaming](architecture/streaming.md).
- **Stdout / fetch / read are truncated** by `max_stdout_*`, `max_fetch_chars`, `max_read_lines`, `max_file_size_bytes` (`crates/aimee_config/.aimee.toml:5-18`, `crates/aimee_app/src/app.rs:33-41`). These are payload caps, not SLOs.

There is no token-bucket, queue-depth metric, or cluster backpressure in this repo.

## Compaction and truncation

`[compact]` defaults (`crates/aimee_config/.aimee.toml:60-66`, `crates/aimee_config/src/compact.rs:44-79`):

| Key | Default | Role |
|---|---|---|
| `eviction_window` | `0.2` | Max fraction of context eligible to summarize |
| `max_tokens` | `2000` | Tokens to keep after compact |
| `message_threshold` | `200` | Turn-count trigger (when present) |
| `on_turn_end` | `false` | Compact after every turn |
| `retention_window` | `6` | Recent messages never summarized |
| `token_threshold` | `100000` | Absolute token trigger |

`retention_window` is **not** a data-retention policy. Conversations stay in local SQLite until you delete them ([Persistence](architecture/persistence.md), [Security](security.md)).

ZSH / TUI `:compact` is the manual path (`README.md:161`).

## HITL `/goal` probes

Five questions are mandatory before a `/goal` loop becomes active (`crates/aimee_domain/src/loop_autonomy.rs:1-27`):

1. What does done look like (observable outcome)?
2. How will we verify (tests, commands, evidence)?
3. What must not change (boundaries)?
4. Who is the human owner, and when should we stop and ask?
5. What Linear issue / GitHub PR / related work should we log against?

`GoalProbeSet` rejects the wrong count or a blank answer (`crates/aimee_domain/src/loop_autonomy.rs:39-73`). This is how the product forces a stop-and-ask contract. It is not a payment rail and not an on-call page.

## Logs

JSON logs go under `{config-base}/logs/` (`crates/aimee_tracker/src/log.rs:11-34`). Filter default is `AIMEE_LOG` if set; otherwise the subscriber's built-in level. Only targets starting with `aimee_` are written.

```bash
aimee logs                 # tail the newest file (follow)
aimee logs -n 100          # last 100 lines
aimee logs --no-follow     # print and exit
aimee logs --list          # newest-first paths
aimee logs -f /path/to.log # specific file
```

`LogsArgs`: `-n/--lines` (default 20), `--no-follow`, `-l/--list`, `-f/--file` (`crates/aimee_main/src/cli.rs:1090-1104`). Implementation shells out to `tail` (`crates/aimee_main/src/logs.rs:11-20`, `crates/aimee_main/src/logs.rs:70-79`).

```bash
AIMEE_LOG=aimee=info aimee
aimee --verbose
aimee info                 # resolved tool timeout and environment
aimee doctor               # shell plugin diagnostics
```

There is **no redaction layer** in the file logger (`crates/aimee_tracker/src/log.rs:21-29`). Tool arguments and shell command strings are logged at `info` ([Security](security.md)). Do not put live keys in prompts or tool args.

## SLOs

Not defined in this repository. No Prometheus scrape config, no error-budget policy, no latency target for `api.aimeecodes.dev` lives in `aimeecodes`. Spectrum cluster monitoring is a separate tree (`spectrum/`). Do not paste invented percentages here.

## File interactions

```
.aimee.toml [retry]/[http]/[compact]
        │
   aimee_config::RetryConfig / HttpConfig / Compact
        │
   aimee_app::Orchestrator::run
        ├─ retry_with_config → execute_chat_turn (provider)
        ├─ ToolRegistry::call_with_timeout (tools)
        ├─ ToolErrorTracker (optional failure cap)
        └─ max_requests_per_turn yield
        │
   aimee_main::logs / aimee_tracker::init_tracing
```

| Path | Role |
|---|---|
| `crates/aimee_config/.aimee.toml` | Embedded defaults |
| `crates/aimee_config/src/retry.rs` | `RetryConfig` |
| `crates/aimee_config/src/http.rs` | `HttpConfig` |
| `crates/aimee_config/src/compact.rs` | `Compact` + update frequency |
| `crates/aimee_app/src/retry.rs` | `backon` wrapper |
| `crates/aimee_app/src/orch.rs` | Turn loop, budgets, interrupts |
| `crates/aimee_app/src/tool_registry.rs` | Tool timeout |
| `crates/aimee_domain/src/loop_autonomy.rs` | HITL probes, unlimited default |
| `crates/aimee_main/src/logs.rs` | `aimee logs` |
| `crates/aimee_tracker/src/log.rs` | JSON file logger |

## Best practices

- Set `max_tool_failure_per_turn` on long unattended runs. Unlimited is the compiled default.
- Keep `accept_invalid_certs = false`.
- Use `AIMEE_LOG=aimee=info` (or `debug` for one crate) instead of pasting full JSON logs into tickets.
- Answer all five `/goal` probes with observable verify commands. "LGTM" is not a probe answer.
- Prefer `:compact` or a new conversation over raising `token_threshold` until the context is unbounded.
- Isolate untrusted work in `--sandbox` or `aimee pod` ([Pods and sandboxes](ops/pod.md)).

## Anti-patterns

- Treating README's `max_tool_failure_per_turn = 3` as the shipped default. It is not in `.aimee.toml`.
- Claiming five-nines or a P99 for the TUI or `api.aimeecodes.dev` — no SLO file exists.
- Using `[compact].retention_window` as GDPR retention.
- Setting `read_timeout_secs` below streaming completion time and calling it a "hang".
- Deleting log files to hide a secret. Rotate the key instead.

## Verify

Documentation links (this space):

```bash
python3 documentation/scripts/verify-docs.py
```

From the product tree (no release build):

```bash
cargo check -p aimee_app -p aimee_config -p aimee_main
cargo clippy -p aimee_app --all-targets -- -D warnings
```

`aimee info` should print the resolved tool timeout after a config change.
