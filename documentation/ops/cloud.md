# Cloud and services

Aimee Codes talks to a hosted workspace/indexing API and can log in to **user** Vertex AI and Amazon Bedrock accounts. That is provider login, not Spectrum cloud tenancy. This page is grounded in the `aimeecodes` tree. It does not invent AWS accounts, regions, or IAM.

## Workspace / indexing API

Default base URL:

```text
https://api.aimeecodes.dev/
```

| Surface | Value |
|---|---|
| Embedded default | `services_url = "https://api.aimeecodes.dev/"` in `crates/aimee_config/.aimee.toml:23` |
| Config field | `AimeeConfig.services_url` — "Base URL of the Aimee services API used for semantic search and indexing" (`crates/aimee_config/src/config.rs:189-193`) |
| Default assertion | `test_default_services_url_uses_aimee_codes_domain` expects `https://api.aimeecodes.dev/` (`crates/aimee_config/src/config.rs:526-530`) |
| Branding | Services API `https://api.aimeecodes.dev/` as default `services_url` (`AIMEE.md:33`, `AIMEE.md:217`) |
| Billing (not the workspace API) | `https://app.aimeecodes.dev/app/billing` (`AIMEE.md:34`) |

Override:

1. Set `services_url` in `~/.aimee/.aimee.toml`.
2. Or set `AIMEE_SERVICES_URL`. `AIMEE_`-prefixed env vars map onto `.aimee.toml` (`crates/aimee_config/src/reader.rs:105-124`). Nested keys use `__`; a top-level key is `AIMEE_SERVICES_URL` (`AIMEE.md:221-224`, `README.md:185`, `README.md:293-298`).

The gRPC client target is `config.services_url` (`crates/aimee_infra/src/aimee_infra.rs:85`). HTTPS URLs enable TLS with webpki roots (`crates/aimee_infra/src/grpc.rs:41-47`). The contract is `package aimee.v1` / `AimeeService` (`crates/aimee_repo/proto/aimee.proto:5-53`): search, upload/delete/list/chunk files, health, workspaces, API keys, validate, skill select, fuzzy search, and text-patch build.

HTTP user/usage calls append `auth/user` and `auth/usage` to `services_url` with a Bearer token (`crates/aimee_services/src/auth.rs:7-8`, `crates/aimee_services/src/auth.rs:20-55`).

The built-in `aimee_services` provider is a **context engine** entry (`provider_type: context_engine`) whose URL param is `AIMEE_WORKSPACE_SERVER_URL` (`crates/aimee_repo/src/provider/provider.json:1685-1689`). That is the same workspace server, not a cloud-account login.

## Provider login: Vertex AI and Bedrock

Vertex and Bedrock are **LLM provider IDs**. Users run `aimee provider login` against their own Google Cloud or AWS credentials. They are not Spectrum org accounts and they are not the workspace API.

| Provider ID | Constant | Auth methods in tree | URL params |
|---|---|---|---|
| `vertex_ai` | `ProviderId::VERTEX_AI` (`crates/aimee_domain/src/provider.rs:60`, `crates/aimee_domain/src/provider.rs:110`) | `api_key`, `google_adc` (`crates/aimee_repo/src/provider/provider.json:1490-1494`, `crates/aimee_repo/src/provider/provider.json:1604`) | `PROJECT_ID`, `LOCATION` |
| `vertex_ai_anthropic` | `ProviderId::VERTEX_AI_ANTHROPIC` (`crates/aimee_domain/src/provider.rs:61`, `crates/aimee_domain/src/provider.rs:111`) | `google_adc` (`crates/aimee_repo/src/provider/provider.json:2679-2683`, `crates/aimee_repo/src/provider/provider.json:2866`) | `PROJECT_ID`, `LOCATION` |
| `bedrock` | `ProviderId::BEDROCK` (`crates/aimee_domain/src/provider.rs:71`, `crates/aimee_domain/src/provider.rs:120`) | `api_key`, `aws_profile` (`crates/aimee_repo/src/provider/provider.json:1701-1704`, `crates/aimee_repo/src/provider/provider.json:2676`) | `AWS_REGION` |

### Google ADC (Vertex)

`AuthMethod::GoogleAdc` (`crates/aimee_domain/src/auth/auth_method.rs:14-15`) is implemented by `GoogleAdcStrategy` (`crates/aimee_infra/src/auth/strategy.rs:424-524`). It uses workspace dep `google-cloud-auth = "1.8.0"` (`Cargo.toml:143`; wired in `crates/aimee_infra/Cargo.toml:50` and `crates/aimee_repo/Cargo.toml:53`).

ADC discovery order in code:

1. `GOOGLE_APPLICATION_CREDENTIALS` (service account)
2. `gcloud` application-default user credentials
3. GCP metadata server

The strategy requests scope `https://www.googleapis.com/auth/cloud-platform` and stores a `google_adc_marker` plus `PROJECT_ID` / `LOCATION` — not a long-lived access key (`crates/aimee_infra/src/auth/strategy.rs:440-492`). Tokens refresh on load (`crates/aimee_repo/src/provider/provider_repo.rs:465-530`). `vertex_ai` also accepts `VERTEX_AI_AUTH_TOKEN` as an API-key path (`crates/aimee_repo/src/provider/provider.json:1491`).

`vertex.json` at the Aimee repo root is a **static Vertex model catalog** (id, name, context length, tools, modalities). It is embedded by `OpenAIProvider::inner_vertex_models` (`crates/aimee_repo/src/provider/openai.rs:343-349`). It is not a service-account file and does not contain credentials.

### AWS profile / Bedrock

`AuthMethod::AwsProfile` (`crates/aimee_domain/src/auth/auth_method.rs:16-17`) is implemented by `AwsProfileStrategy` (`crates/aimee_infra/src/auth/strategy.rs:526-608`). It requires `AWS_PROFILE`, loads `aws_config::from_env().profile_name(...)`, and validates credentials via the AWS SDK chain (SSO, IAM, or other types already in `~/.aws/config`). The stored credential is the profile name, not a long-lived access key.

Runtime uses `aws-sdk-bedrockruntime` (`Cargo.toml:26-27`; `crates/aimee_repo/Cargo.toml:33-34`). `BedrockProvider` supports:

- Bearer token (API key) for a Bedrock Access Gateway
- Named AWS profile

`AWS_REGION` comes from URL params and **defaults to `us-east-1` only when the param is absent** (`crates/aimee_repo/src/provider/bedrock.rs:20-71`). That default is a client fallback, not a Spectrum-owned region or account.

## What is not in `aimeecodes`

- No Terraform, CloudFormation, or other IaC for an Aimee SaaS control plane lives in `aimeecodes`.
- No AWS account IDs, org units, or customer-managed key aliases for Aimee hosting are declared in that repo.
- Multi-AZ / multi-region **platform** topology is not defined here. Bedrock model IDs in `provider.json` include geo prefixes (`us.`, `eu.`, `global.`, …) as **model catalog entries**, not as Aimee deploy regions.

Spectrum Web Co cluster IaC is a **separate** tree: `spectrum/` (`aws_terraform/`, `charts/`). That repo documents AWS EKS (production) and kind (local) (`spectrum/README.md:1-7`). Do not treat `spectrum/` as Aimee SaaS Terraform.

## Verify

From this docs repo:

```bash
python3 scripts/verify-docs.py
```

From the product tree, the default URL lock is `test_default_services_url_uses_aimee_codes_domain` (`crates/aimee_config/src/config.rs:526-530`). Vertex config lock is `test_vertex_ai_config` (`crates/aimee_repo/src/provider/provider_repo.rs:725-750`).
