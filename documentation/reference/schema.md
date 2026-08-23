# Config JSON schema

The machine-readable schema for `~/.aimee/.aimee.toml` is [`aimee.schema.json`](https://github.com/swcstudiospace/omegaloops/blob/main/aimee.schema.json) at the repository root — generated from the `Config` type in `crates/aimee_config` (schemars). Defaults are embedded from `crates/aimee_config/.aimee.toml`. Editors and tools can use the schema for validation and completions; `aimee config list` prints resolved values.

This page maps the schema's top-level keys to their meaning. Types below are indicative — the JSON file is authoritative.

## Top-level keys

### Session, model, sampling

| Key | Type | Meaning |
|---|---|---|
| `session` | object | Active provider + model selection |
| `providers` | array | Inline provider overrides merged with built-ins (same `id` wins field-by-field) |
| `model_cache_ttl_secs` | integer | Model-list cache lifetime |
| `temperature`, `top_p`, `top_k`, `max_tokens` | number / integer | Sampling controls |
| `tool_supported` | boolean | Force tool-calling capability flag |
| `reasoning` | object | `{ enabled, effort }` reasoning controls |
| `research_subagent` | — | Research subagent wiring |

### Loop budgets

| Key | Type | Default | Meaning |
|---|---|---|---|
| `max_requests_per_turn` | integer | `100` | Cap model calls per turn |
| `max_tool_failure_per_turn` | integer | unlimited | Cap repeated tool errors per turn |
| `tool_timeout_secs` | integer | `300` | Per-tool-call timeout ([Reliability](../reliability.md)) |
| `verify_todos` | boolean | — | Require todo verification before completion |

### Tool I/O limits

These back the truncation knobs referenced in tool contracts (`maxReadSize`, `maxLineLength`, stdout caps in [read](tools/read.md) and [shell](tools/shell.md)):

`max_read_lines`, `max_line_chars`, `max_file_size_bytes`, `max_file_read_batch_size`, `max_parallel_file_reads`, `max_search_lines`, `max_search_result_bytes`, `max_sem_search_results`, `sem_search_top_k`, `max_fetch_chars`, `max_stdout_prefix_lines`, `max_stdout_suffix_lines`, `max_stdout_line_chars`.

### Behavior switches

| Key | Meaning |
|---|---|
| `restricted` | Restricted mode: tool execution requires permission grants ([Security](../security.md)) |
| `subagents` | Enable `task` delegation (disables Sage-as-a-tool when true) |
| `use_aimee_committer` | Aimee writes commit messages for `aimee commit` |
| `use_text_patch_fallback` | Fall back to gRPC fuzzy patch building |
| `merge_system_messages` | Merge consecutive system messages |
| `debug_requests` | Log raw provider requests |
| `auto_install_vscode_extension` | Install the VS Code extension automatically |

### Sub-objects

| Key | Schema definitions involved | See |
|---|---|---|
| `anda` | `AndaConfig`, `AndaEternalMode` (`local` \| `ic_oss` \| `canister` \| `s3`) | [Anda / KIP](../web3/anda.md) |
| `compact` | `Compact` (context compaction window) | [Reliability](../reliability.md) |
| `retry` | `RetryConfig` (provider retry policy) | [Reliability](../reliability.md) |
| `http` | `HttpConfig`, `TlsBackend`, `TlsVersion` | [Reliability](../reliability.md) |
| `updates` | `Update`, `UpdateFrequency` | [Install and Nix](../ops/install.md) |
| `commit` | Commit-message behavior | [CLI reference](../cli.md) |
| `suggest` | Natural-language → command suggestions | [CLI reference](../cli.md) |

### Provider plumbing

Schema definitions `ProviderEntry`, `ProviderTypeEntry`, `ProviderAuthMethod`, `ProviderResponseType`, `ProviderUrlParam`, `Model`, `ModelConfig`, `ModelListConfig`, `InputModality` describe inline `[[providers]]` entries and model metadata. The 42 built-in IDs are documented in [Providers](../providers.md).

### Misc

`auto_dump` + `AutoDumpFormat` + `auto_open_dump` (conversation export), `custom_history_path`, `currency_symbol` + `currency_conversion_rate` (cost display), `max_conversations`, `max_commit_count`, `max_extensions`.

## Validation workflow

```bash
# Inspect resolved config
aimee config list
aimee config get model

# Edit safely
aimee config set model <provider> <model>
```

When changing the schema itself: edit the `Config` types in `crates/aimee_config`, regenerate `aimee.schema.json`, and keep new persisted fields compatibility-defaulted.

## Related

- [Configuration](../configuration.md) — file locations, merge order, env mapping
- [Environment variables](env.md)
- [Persistence](../architecture/persistence.md)
