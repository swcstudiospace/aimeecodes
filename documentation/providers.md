# Providers

A **provider** is a model vendor or compatible endpoint Aimee can chat with (or a context engine for indexing). IDs, wire protocols, and auth methods are domain + config. HTTP happens in infra. Credentials are file-based — never commit them.

## Purpose

Show humans:

- Which built-in `ProviderId` values exist
- How login and session selection work
- How to add an inline provider in `.aimee.toml` without inventing a protocol

## When to use

| Goal | Command / config |
|---|---|
| Store credentials | `aimee provider login` (interactive if you omit the id) |
| Drop credentials | `aimee provider logout` |
| See configured + built-in | `aimee provider list` / `aimee list provider` |
| Filter by kind | `aimee provider list --type llm` or `--type context_engine` |
| Pick default model | `session` in `.aimee.toml` or `aimee config` |
| Compatible gateway | Inline `[[providers]]` with `openai_compatible` / `anthropic_compatible` |

Do not use this page for DevPod `aimee pod provider` (that is a different command group).

## File interactions

```text
aimee provider login
        │
API::init_provider_auth / complete_provider_auth
        │  crates/aimee_api/src/aimee_api.rs:326-347
        ▼
AimeeProviderAuthService          crates/aimee_services/src/provider_auth.rs
        │  StrategyFactory (infra)
        ▼
ProviderRepository::upsert_credential   (aimee_repo, .credentials.json)

chat turn
        │
AimeeApp → AgentProviderResolver → refresh_provider_credential
        │  crates/aimee_app/src/app.rs:82-100
        ▼
AimeeProviderService::chat / models
        │  URL template render
        ▼
ChatRepository + AimeeHttpInfra
```

Domain types (`crates/aimee_domain/src/provider.rs`):

- `ProviderType`: `Llm` (default), `ContextEngine` (`:17-23`)
- `ProviderId` constants + `built_in_providers()` (`:48-141`)
- `ProviderResponse`: `OpenAI`, `OpenAIResponses`, `Anthropic`, `Bedrock`, `Google`, `OpenCode` (`:259-267`)
- `Provider<T>` template vs resolved `Url` (`:278-320`)
- `AnyProvider` for listings (`:322-350`)

Config mirror (`crates/aimee_config/src/config.rs:16-115`): `ProviderResponseType`, `ProviderTypeEntry`, `ProviderAuthMethod` (`ApiKey`, `GoogleAdc`), `ProviderEntry` for inline `[[providers]]`.

OAuth device / authorization-code flows are **not** expressible as `ProviderAuthMethod` in TOML; those providers use the file-based `provider.json` override (`crates/aimee_config/src/config.rs:38-42`).

CLI (`crates/aimee_main/src/cli.rs:968-1004`):

- `aimee provider login [PROVIDER]`
- `aimee provider logout [PROVIDER]`
- `aimee provider list --type <llm|context_engine>`

`FromStr` aliases (`crates/aimee_domain/src/provider.rs:196-250`): `omega` → `aimee`, `aimee_services` / `omega_services`, SuperGrok OAuth names (`xai_oauth`, `supergrok`, …). Custom unknown strings become owned `ProviderId`s.

### Built-in IDs

From `ProviderId::built_in_providers()` (`crates/aimee_domain/src/provider.rs:97-141`). Do not invent others as "built-in":

`aimee`, `openai`, `open_router`, `requesty`, `zai`, `zai_coding`, `cerebras`, `xai`, `xai_oauth`, `anthropic`, `claude_code`, `vertex_ai`, `vertex_ai_anthropic`, `big_model`, `azure`, `github_copilot`, `openai_compatible`, `openai_responses_compatible`, `anthropic_compatible`, `aimee_services`, `io_intelligence`, `bedrock`, `minimax`, `codex`, `opencode_zen`, `opencode_go`, `fireworks-ai`, `fireworks-ai-firepass`, `novita`, `vivgrid`, `google_ai_studio`, `modal`, `adal`, `xiaomi_mimo`, `nvidia`, `ambient`, `neuralwatt`, `orca_router`, `meta`, `kimi_coding`, `moonshot`, `alibaba_token_plan`.

Display names (UI) are special-cased for acronyms (`crates/aimee_domain/src/provider.rs:151-186`): `xai_oauth` → SuperGrok, `openai` → OpenAI, and the other match arms in that function.

## How to use

```bash
aimee provider login
aimee provider login openai
aimee provider list --type llm
aimee list model
aimee info
```

Session selection (placeholders only — never paste a real key):

```toml
# ~/.aimee/.aimee.toml
session = { provider = "openai", model = "gpt-4o" }
commit  = { provider = "openai", model = "gpt-4o-mini" }
suggest = { provider = "openai", model = "gpt-4o-mini" }

# Inline provider: same id overrides a built-in field-by-field; new id appends
[[providers]]
id = "my_gateway"
api_key_var = "MY_GATEWAY_API_KEY"
url = "https://gateway.example.invalid/v1/chat/completions"
response_type = "OpenAI"
provider_type = "llm"
auth_methods = ["api_key"]
```

`ProviderEntry` fields: `id`, `api_key_var`, `url`, `models`, `response_type`, `url_param_vars`, `custom_headers`, `provider_type`, `auth_methods` (`crates/aimee_config/src/config.rs:78-115`). URL templates may contain `{{VAR}}` placeholders filled from credential URL params (`crates/aimee_services/src/provider_service.rs:26-61`).

Environment credentials can be migrated to the file store via `API::migrate_env_credentials` (`crates/aimee_api/src/aimee_api.rs:399-401`).

`merge_system_messages` exists for providers that reject non-leading system messages (vLLM, NVIDIA NIM) (`crates/aimee_config/src/config.rs:332-337`).

## Best practices

- Login through `aimee provider login`. Do not write `.credentials.json` by hand.
- Use placeholders (`$OPENAI_API_KEY`, `sk-...REDACTED`) in docs and tickets.
- Prefer a built-in ID. Add `[[providers]]` only for a real gateway you operate.
- Set `response_type` to one of the six wire protocols. Do not invent `Groq` / `Cohere` variants.
- Refresh is automatic on chat (`refresh_provider_credential`, 5 minute expiry buffer — `crates/aimee_app/src/services.rs:532-540`).
- `get_all_provider_models` fails closed: first provider error, not an empty list (`crates/aimee_api/src/api.rs:26-31`).

## Anti-patterns

| Don't | Do |
|---|---|
| Commit `.credentials.json` or `.env` | Config base, mode 600 / secret store |
| Invent `ProviderId::GROQ` in docs | Search `provider.rs` constants |
| Put OAuth device flow in `aimee.toml` `auth_methods` | File-based `provider.json` override |
| Log `AuthDetails::ApiKey` | Branch on presence only (see `user_info`) |
| Empty model list when every provider 401s | Surface the first error |
| `aimee pod provider` for LLM login | That is DevPod, not this page |
| Custom `response_type = "REST"` | One of `OpenAI` / `OpenAIResponses` / `Anthropic` / `Bedrock` / `Google` / `OpenCode` |

## Verify

```bash
cargo fmt
cargo check -p aimee_domain -p aimee_config -p aimee_services -p aimee_api
cargo clippy -p aimee_domain -p aimee_config -p aimee_services --all-targets -- -D warnings
cargo insta test --accept -p aimee_domain -p aimee_config
```

Never `cargo build --release`. Do not print credential files while verifying.

## Related

- [Domain](architecture/domain.md) — `ProviderId` / `ProviderResponse`
- [Composition root](architecture/api.md) — login methods on `API`
- [Infrastructure](architecture/infra.md) — HTTP / TLS
- [Configuration](configuration.md)
- `AIMEE.md` §2 (compat aliases `omega` / `OMEGA_*`)
