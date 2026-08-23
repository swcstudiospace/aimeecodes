# Providers and model access

Aimee speaks to 42 built-in providers and any compatible endpoint you define. `aimee provider list` is the live source of truth on your install.

## Managing providers

```bash
aimee provider list            # everything available
aimee provider login           # interactive authentication
aimee provider logout          # remove stored credentials
aimee provider list --porcelain   # machine-readable output
```

`login` walks you through the chosen provider's auth flow — API key entry or OAuth where supported. Credentials land in `.credentials.json` under your config base (`~/.aimee/` by default), never in git.

## Built-in provider IDs

`aimee`, `openai`, `open_router`, `requesty`, `zai`, `zai_coding`, `cerebras`, `xai`, `xai_oauth` (SuperGrok, OAuth device login — no API key), `anthropic`, `claude_code`, `vertex_ai`, `vertex_ai_anthropic`, `big_model`, `azure`, `github_copilot`, `openai_compatible`, `openai_responses_compatible`, `anthropic_compatible`, `aimee_services`, `io_intelligence`, `bedrock`, `minimax`, `codex`, `opencode_zen`, `opencode_go`, `fireworks-ai`, `fireworks-ai-firepass`, `novita`, `vivgrid`, `google_ai_studio`, `modal`, `adal`, `xiaomi_mimo`, `nvidia`, `ambient`, `neuralwatt`, `orca_router`, `meta`, `kimi_coding`, `moonshot`, `alibaba_token_plan`.

## Wire protocols

Six response protocols cover every built-in: **OpenAI**, **OpenAI Responses**, **Anthropic**, **Bedrock**, **Google**, and **OpenCode**. A provider entry declares which protocol its endpoint speaks; that's what makes "compatible" providers work — anything exposing an OpenAI-shaped API is reachable via `openai_compatible`.

## Custom providers

Declare inline providers in `.aimee.toml`; entries with the same `id` as a built-in override it field-by-field:

```toml
[[providers]]
id = "my_gateway"
response_type = "openai"        # one of the six wire protocols
base_url = "https://llm.internal.example.com/v1"
# api_key comes from provider login or your secret store — not this file
```

## Choosing models per job

The ZSH dispatcher offers per-session model switching (current session only) plus dedicated model picks for two jobs:

* **Commit messages** — `aimee config set commit <provider_id> <model_id>`
* **Command suggestions** — `aimee config set suggest <provider_id> <model_id>`

The SuperGrok path (`xai_oauth`) uses OAuth device login; run it from the plugin action or `aimee provider login` and follow the device code.

## Aimee services

The `aimee_services` provider talks to the hosted workspace/indexing API (default `https://api.aimeecodes.dev/`, configurable as `services_url`). Billing lives at `https://app.aimeecodes.dev/app/billing`. See [Configuration model](../concepts/configuration.md) for the key.

## See also

* [Authentication and credentials](auth.md)
* [Config reference](../reference/config.md)
* [Cost awareness](../operations/cost.md)

<!-- sources: crates/aimee_domain/src/provider.rs, crates/aimee_config/src/config.rs, AIMEE.md §2,§8 -->
