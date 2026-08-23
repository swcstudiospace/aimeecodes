# Security

How Aimee Codes treats untrusted input, where secrets live, and which controls the tree actually implements. Policy for agents is `AGENTS.md`. This page is the human map.

AuthN is not AuthZ. Logging in a provider does not grant a tool the right to write your disk. Restricted mode and `permissions.yaml` are the authorization layer. They are off by default.

Do not invent controls. Every claim below cites the tree. SOC2 / FedRAMP mapping is a separate page (`compliance.md`).

## Purpose

| Goal | Use |
|---|---|
| Store a provider key | `aimee provider login` → `{base}/.credentials.json` |
| Store an MCP OAuth token | `aimee mcp login <name>` → `{base}/.mcp-credentials.json` |
| Gate catalog tools | `restricted = true` plus `{base}/permissions.yaml` |
| Trust a project-local MCP config | Interactive accept / reject → `{base}/.mcp_trust.json` |
| Rotate a key | `aimee provider login <id>` again (always re-configures) |
| Drop a key | `aimee provider logout <id>` / `aimee mcp logout <name\|all>` |

See [Configuration](configuration.md) for base-path resolution, keys, and CLI verbs.

## When to use

Turn `restricted` on for any machine that can reach production, customer data, or a wallet. Leave it off only on a throwaway workstation where you accept that catalog tools run without a grant.

Never put API keys, OAuth tokens, MCP client secrets, or `.env` values in `.aimee.toml`, in this GitBook, or in a commit. Use placeholders (`MY_GATEWAY_API_KEY`, `sk-…`).

## Untrusted inputs

Treat every input as hostile. The product has four inbound surfaces; none of them are pre-authorized.

| Surface | What arrives | Why it is untrusted |
|---|---|---|
| CLI / TUI / ZSH | `-p` prompts, `: sage …`, file tags, config edits | The human (or a pasted snippet) can inject tool-shaped text. |
| MCP | Server tools, resources, OAuth redirects, project `.mcp.json` | A project-local MCP config can spawn a process or call a URL. |
| Tool results | `shell` stdout/stderr, `fetch` HTML, `read` file bytes, MCP tool text | Models will follow what they just read. |
| Files | Workspace trees, `AGENTS.md`, skills, commands, `.env` walked from cwd | A cloned repo can contain instructions and secrets. |

Consequences that the tree actually implements:

- **Tool arguments are logged.** `ToolRegistry::call_inner` writes `tool_name` and the raw argument string at `info` (`crates/aimee_app/src/tool_registry.rs:101`). Do not pass a live key as a tool argument.
- **Shell command strings are logged.** `AimeeCommandExecutorService::prepare_command` logs the full command at `info` (`crates/aimee_infra/src/executor.rs:65`).
- **Fetch is a GET with a robots.txt check**, not an authenticated browser (`crates/aimee_services/src/tool_services/fetch.rs:31-72`). It cannot reach private resources that require cookies. It can still hit internal URLs the host can route to.
- **`.env` files from cwd to `/` are loaded into the process** on config build (`crates/aimee_config/src/reader.rs:10-31`). A malicious repo `.env` can override `AIMEE_*` for that process. Do not run Aimee in an untrusted tree without inspecting dotenv files first.

## AuthN is not AuthZ

### Authentication (who)

`crates/aimee_infra/src/auth/` is the AuthN crate. Strategies live in `strategy.rs`: API key, OAuth device / authorization-code, Google ADC, AWS profile (`crates/aimee_infra/src/auth/strategy.rs:19-58`, `crates/aimee_domain/src/auth/credentials.rs:85-105`).

`aimee provider login` writes an `AuthCredential` to `{base}/.credentials.json`. `aimee mcp login` writes an `McpCredentialEntry` (tokens + optional dynamic client registration) to `{base}/.mcp-credentials.json` (`crates/aimee_infra/src/auth/mcp_credentials.rs:31-46`). That proves the process can call that provider or MCP server. It does **not** decide whether `write`, `shell`, or `fetch` may run.

Credential kinds (`crates/aimee_domain/src/auth/credentials.rs:85-105`):

| `AuthDetails` | Refresh? |
|---|---|
| `ApiKey` | No |
| `AwsProfile` | No — AWS SDK owns the session |
| `GoogleAdc` | Always checked (short-lived) |
| `OAuth` / `OAuthWithApiKey` | When `expires_at` is inside the buffer |

Google ADC for Vertex is refreshed on every provider load when the stored marker is `google_adc_marker` (`crates/aimee_repo/src/provider/provider_repo.rs:463-480`).

### Authorization (what)

Authorization is `restricted` + `PolicyEngine`. It is **not** implied by a successful login.

`restricted` defaults to `false` (`aimee.schema.json`, `crates/aimee_config/.aimee.toml`). When `true`, `ToolRegistry` checks catalog tools **before** the tool timeout so a hung prompt cannot skip the grant (`crates/aimee_app/src/tool_registry.rs:140-153`).

A denied call returns `permission_denied` text to the model. It does not throw.

`Task` (subagent dispatch) returns **before** the restricted check (`crates/aimee_app/src/tool_registry.rs:108-133`). Nested catalog tools inside the subagent still hit the gate. MCP tools and agent-as-tool calls skip this catalog gate entirely (`crates/aimee_app/src/tool_registry.rs:167-207`). Trust the MCP server and the agent definition, not this policy file, for those paths.

## Restricted mode and permission grants

Present in the tree. Documented because it exists.

### Enable

```toml
# {base}/.aimee.toml
restricted = true
```

or `export AIMEE_RESTRICTED=true`. There is no `aimee config set restricted` verb — `config set` only writes session / commit / suggest / reasoning-effort ([Configuration](configuration.md)).

### What is checked

`PermissionOperation` (`crates/aimee_domain/src/policies/operation.rs:5-26`):

| Operation | Typical catalog tools |
|---|---|
| `Read` | `read` |
| `Write` | `write`, `patch`, `multi_patch`, `remove` |
| `Execute` | `shell` |
| `Fetch` | `fetch` |

A tool that cannot map to one of those four is not gated.

### Decision order

`PolicyEngine::evaluate_policies` (`crates/aimee_domain/src/policies/engine.rs:30-58`):

1. Empty policy list → `Confirm`.
2. Walk policies in file order.
3. First `Deny` or `Confirm` wins immediately.
4. Last matching `Allow` is kept; if none, default is `Confirm`.

`AimeePolicyService::check_operation_permission` (`crates/aimee_services/src/policy.rs:161-206`) then:

| Engine result | Runtime |
|---|---|
| `Allow` | Run the tool. |
| `Deny` | Return `allowed: false`. |
| `Confirm` | Prompt: Accept / Reject / Accept and Remember (`crates/aimee_services/src/policy.rs:16-28`). Reject or dismiss → deny. Accept and Remember appends a new allow rule (extension glob for files, `{host}*` for fetch). |

### Where grants live

`{base}/permissions.yaml` (`crates/aimee_domain/src/env.rs:97-99`). If the file is missing the first restricted call writes the embedded default (`crates/aimee_services/src/policy.rs:111-131`).

Embedded default (`crates/aimee_services/src/permissions.default.yaml:1-13`):

```yaml
policies:
  - permission: allow
    rule:
      read: "**/*"
  - permission: allow
    rule:
      write: "**/*"
  - permission: allow
    rule:
      command: "*"
  - permission: allow
    rule:
      url: "*"
```

That default **allows everything**. Restricted mode with the stock file is a confirm-on-unknown-only setup, not a sandbox. Tighten the globs before you rely on it.

Rule shapes (`crates/aimee_domain/src/policies/rule.rs:10-54`): `read`, `write`, `command`, `url`, each with an optional `dir` glob against cwd.

## Credential storage

### Locations

Resolved from `Environment` (`crates/aimee_domain/src/env.rs:81-180`, `crates/aimee_infra/src/auth/mcp_credentials.rs:93-98`):

| File | Contents | Written by |
|---|---|---|
| `{base}/.credentials.json` | Provider `AuthCredential` list (API keys, OAuth tokens, ADC marker, AWS profile name) | `aimee provider login`, one-shot env → file migration |
| `{base}/.mcp-credentials.json` | MCP OAuth tokens + optional client registration, keyed by server URL | `aimee mcp login` |
| `{base}/.mcp_trust.json` | Accept / reject hashes for project-local `.mcp.json` | MCP trust gate |
| `{base}/provider.json` | File-based provider override (OAuth-capable) | Operator / login flows that need more than TOML |
| `{cwd}/.mcp.json` | Project MCP servers (preferred over the user file) | Operator |
| `{base}/.mcp.json` | User-scope MCP servers | `aimee mcp` |

`base` is whatever `ConfigReader::resolve_base_path` picked. Confirm with `aimee config path` before you copy or shred files. Compat candidates (`~/.omega`, `~/.forge`) can hold live secrets. Do not delete them as a “cleanup”. See [Configuration](configuration.md).

### File mode

On Unix, both credential writers set **`0o600`** (owner read/write only) after every write:

- Provider: `set_owner_only_permissions` (`crates/aimee_repo/src/provider/provider_repo.rs:617-623`). Tests assert a fresh file is `0o600` and that a loosened `0o644` is tightened on the next write (`crates/aimee_repo/src/provider/provider_repo.rs:1518-1592`).
- MCP: `McpCredentialStore::save` (`crates/aimee_infra/src/auth/mcp_credentials.rs:69-89`).

Windows has no equivalent mode in these functions. Protect the directory with OS ACLs.

### Env → file migration

If `.credentials.json` does not exist, `migrate_env_to_file` copies matching provider env vars into the file once (`crates/aimee_repo/src/provider/provider_repo.rs:334-389`). After that, the file wins. `AIMEE_API_KEY` still falls back to `OMEGA_API_KEY` during that pass only (`crates/aimee_repo/src/provider/provider_repo.rs:401-404`).

### Gitignore posture

Product repo `.gitignore` (`aimeecodes/.gitignore:21-47`) ignores:

- `.env`
- `*.db`, `*.db-*`, `*.sqlite*`
- `*.log*`
- `*-dump.json`, `*-dump.html`
- `.mcp.json` (project-local)
- `**/.aimee/request.body.json` and `**/.omega/request.body.json`

It does **not** list `.credentials.json` or `.mcp-credentials.json` because those belong under the config base (`~/.aimee/…`), not the git worktree. That is not a license to copy them into a repo. If you ever materialize them inside a project, add them to **that** project’s `.gitignore` and rotate the keys.

`debug_requests` dumps raw POST bodies (`crates/aimee_infra/src/http.rs:237-248`). Bodies can contain bearer tokens. The repo ignores `**/.aimee/request.body.json`; a custom `debug_requests` path is **not** automatically ignored.

## What must never be logged or committed

| Class | Examples | Why |
|---|---|---|
| Provider secrets | API keys, OAuth access / refresh tokens, `id_token` | Live AuthN material. |
| MCP secrets | `access_token`, `refresh_token`, `client_secret` | Same, separate store. |
| Dotenv | Any `.env` walked from cwd | Loaded into the process. |
| Request dumps | `debug_requests` files, `*-dump.json` / `*-dump.html` | May contain Authorization headers and prompts. |
| PAN / PII / customer data | Card numbers, government IDs, production dumps | `AGENTS.md:32-33`. |
| Conversation DB | `{base}/.aimee.db` | Chat history. |

Placeholders only in docs and tickets: `MY_GATEWAY_API_KEY`, `sk-…`, `Bearer [REDACTED]`.

`sanitize_headers` redacts `authorization`, `x-api-key`, `x-goog-api-key`, and `api-key` to `[REDACTED]` before request-header debug logs (`crates/aimee_infra/src/http.rs:214-235`, test at `crates/aimee_infra/src/http.rs:514-544`). OpenAI and OpenAI-Responses repos call the same helper. It does **not** redact:

- Cookie, `x-auth-token`, or custom header names.
- POST bodies (`write_debug_request` writes them verbatim).
- Tool arguments or shell command strings (logged at `info`).
- Google ADC token **prefixes** (first 20 characters at `debug`, `crates/aimee_repo/src/provider/provider_repo.rs:539-546`).

Do not raise `AIMEE_LOG` to `debug`/`trace` on a shared host. Logs land under `{base}/logs/` as JSON (`crates/aimee_tracker/src/log.rs:11-34`). When tracking is enabled they also go to the PostHog writer.

## Threat notes grounded in code

These are observations about the current tree, not a penetration-test report.

### Parameterized process spawn

`AimeeCommandExecutorService` starts the configured shell as an argv program and passes the user string as a single `-c` / `/C` argument (`crates/aimee_infra/src/executor.rs:36-63`):

```text
Command::new(shell)  +  arg("-c")  +  arg(command_str)
```

The shell binary is not interpolated into a string. The **command string is still a shell script**. Quotes, `$()`, backticks, and `;` inside `command_str` run with the user’s privileges. Restricted mode is the control that decides whether that string may run at all.

`AimeeShell::validate_command` only rejects empty / whitespace commands (`crates/aimee_services/src/tool_services/shell.rs:33-38`). It does not block `rm`, absolute paths, or `cd`. The tool description mentions an unrestricted `-u` flag; **that flag is not on `cli.rs`**. Do not document `-u` as a product switch.

### Header sanitization

See above. Callers that log headers must go through `aimee_infra::sanitize_headers` (`crates/aimee_infra/src/lib.rs` re-export, used from `crates/aimee_infra/src/http.rs:209` and the OpenAI repos). Adding a new sensitive header name requires a code change and a test next to `test_sanitize_headers_redacts_sensitive_values`.

### OAuth HTTP client: no redirects

`build_http_client` sets `redirect(Policy::none())` with an explicit SSRF comment (`crates/aimee_infra/src/auth/util.rs:44-49`). Provider HTTP (`AimeeHttpInfra`) still follows up to `http.max_redirects` (default 10, `crates/aimee_infra/src/http.rs:63`). Those are different clients.

### TLS

Default `http.accept_invalid_certs = false`. Setting it `true` calls `danger_accept_invalid_certs(true)` (`crates/aimee_infra/src/http.rs:97-99`). Leave it false. Extra roots go in `http.root_cert_paths` (PEM or DER). Do not widen TLS to make a call work.

### Fetch / SSRF

`AimeeFetch` uses a plain `reqwest::Client`, checks `robots.txt` `Disallow` prefixes, then GET (`crates/aimee_services/src/tool_services/fetch.rs:31-72`). There is no allowlist of hosts. In restricted mode a `url` grant is the allowlist. Without restricted mode, the model can fetch any URL the host can route — including link-local and RFC1918.

### MCP trust gate

Project-local `.mcp.json` is hashed. `apply_trust_gate` skips the prompt only when that hash is already accepted or rejected (`crates/aimee_services/src/mcp/manager.rs:68-73`). Reject persists the hash and returns an empty config. A changed file is a new hash — the operator is prompted again. User-scope `{base}/.mcp.json` is not gated the same way; treat it as trusted operator config.

MCP stdio servers spawn `command` + `args` with optional `env` (`crates/aimee_domain/src/mcp.rs:69-79`). That is process execution outside `permissions.yaml`. Review project `.mcp.json` before Accept.

### `debug_requests`

When set, every provider POST / EventSource body is appended, unsanitized (`crates/aimee_infra/src/http.rs:237-248`). Point it at a directory outside any git worktree, mode `0700`, and delete the files after the session.

### Compat paths

`OMEGA_CONFIG`, `~/.omega`, `~/.forge`, and `OMEGA_*` env vars are still read (`AIMEE.md:43-52`). An old Omega home can still hold `.credentials.json`. `aimee config migrate` moves `~/aimee`, `~/.omega`, or `~/omega` → `~/.aimee` and **refuses** if `~/.aimee` already exists. It does not move Forge dirs. Document, do not delete.

## Operator best practices

- Run `aimee config path` and confirm the base before you copy, back up, or shred secrets.
- Prefer `aimee provider login` / `aimee mcp login` over exporting keys into the shell. File mode is `0o600`; shell history is not.
- Enable `restricted = true` and replace the default `permissions.yaml` with deny-by-default globs (`read: "src/**"`, `command: "cargo *"` — not `*`).
- Review project `.mcp.json` and `.env` before the first run in a new clone.
- Keep `http.accept_invalid_certs = false`.
- Do not set `debug_requests` unless you are reproducing a provider bug, and never inside the repo.
- After rotating a provider, `aimee provider logout <id>` then login. Logout does not shred old log lines.
- On a shared host, unset `AIMEE_LOG` / keep it at `info`, and restrict `{base}/logs/`.
- Leave Omega / Forge compat directories in place until you have migrated and verified `aimee config path`.

## Contributor best practices

- Never commit tokens, private keys, connection strings, `.env` values, user data, or generated dumps (`AGENTS.md:32-33`).
- New header names that carry secrets must be added to `sanitize_headers` **and** `test_sanitize_headers_redacts_sensitive_values` (`crates/aimee_infra/src/http.rs:216-235`, `crates/aimee_infra/src/http.rs:514-544`).
- Do not log `AuthCredential`, `OAuthTokens`, `McpOAuthTokens`, or raw `HeaderMap` values. Log provider id + “redacted”.
- Do not add a second HTTP client. Use `AimeeHttpInfra`. Do not disable TLS verify to make a test pass.
- Parameterized argv only. Do not `format!("{} {}", shell, cmd)` into `std::process::Command`.
- Restricted-mode checks stay **before** timeouts (`crates/aimee_app/src/tool_registry.rs:140-141`). Do not invert that.
- MCP and `Task` currently skip the catalog permission gate. If you add a new executor path, decide explicitly whether it is gated, and document the hole if it is not.
- Invalid states unrepresentable: newtypes for `ApiKey`, `AccessToken`, `RefreshToken`, `ProviderId`. Do not pass raw `String` secrets across crate boundaries.
- Tests may exercise defensive mechanisms. Do not check in exploit payloads or live credentials.

## File interactions

```text
untrusted prompt / MCP / file / tool result
        │
ToolRegistry::call_inner                 crates/aimee_app/src/tool_registry.rs:93
        │
        ├── Task ──────────────────────► AgentExecutor   (no catalog grant)
        ├── catalog tool
        │       │
        │       restricted?              crates/aimee_app/src/tool_registry.rs:142
        │           │
        │           └── PolicyEngine     crates/aimee_domain/src/policies/engine.rs:24
        │                   │
        │                   └── permissions.yaml
        │
        ├── agent-as-tool ─────────────► AgentExecutor   (no catalog grant)
        └── MCP tool ──────────────────► McpExecutor     (no catalog grant)
                                              │
                                              └── trust gate on project .mcp.json
                                                  crates/aimee_services/src/mcp/manager.rs:68

aimee provider login
        │
AuthStrategy::complete                   crates/aimee_infra/src/auth/strategy.rs:41
        │
{base}/.credentials.json  mode 0o600     crates/aimee_repo/src/provider/provider_repo.rs:617

aimee mcp login
        │
McpCredentialStore::save  mode 0o600     crates/aimee_infra/src/auth/mcp_credentials.rs:73
        │
{base}/.mcp-credentials.json

HTTP request headers
        │
sanitize_headers → [REDACTED]            crates/aimee_infra/src/http.rs:216
```

## Anti-patterns

- Treating `aimee provider login` as permission to write the workspace.
- Shipping the embedded `permissions.yaml` and calling the machine “restricted”.
- Putting secrets in `.aimee.toml`, `[[providers]].custom_headers`, or `AIMEE_*` in a committed dotenv.
- Setting `http.accept_invalid_certs = true` “just for this gateway”.
- Deleting `~/.omega` / `~/.forge` because the brand is now Aimee. Compat is intentional (`AIMEE.md:43-45`).
- Pointing `debug_requests` at the repo.
- Logging a `HeaderMap` or an `AuthCredential` in a new `tracing::debug!`.
- Assuming MCP tools or `task` honor `permissions.yaml`. They do not.
- Documenting a `-u` unrestricted CLI flag. It is not on `cli.rs`.

## Verify

```bash
aimee config path
ls -la "$(dirname "$(aimee config path)")/.credentials.json" \
       "$(dirname "$(aimee config path)")/.mcp-credentials.json" 2>/dev/null
# expect -rw------- (0o600) on Unix when the files exist

cargo test -p aimee_infra sanitize_headers
cargo test -p aimee_repo test_credentials_file
```

Do not print the credential files.

## See also

- [Configuration](configuration.md) — base path, keys, `aimee config` / `aimee provider`.
- [Providers](providers.md) — built-in ids and wire protocols.
- [Best practices](best-practices.md) — contributor contract.
- `AGENTS.md` — house rules, including “no secrets”.
