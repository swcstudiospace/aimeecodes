# Best practices

Contributor contract for the Aimee Codes tree. Policy lives in `AGENTS.md`. This page is the human digest: how to change the product, how to use Sage / Muse / Aimee, and the anti-patterns the tree actually rejects.

Do not copy `AGENTS.md` wholesale. When the two disagree, the product file wins.

## Operating contract

Nine rules, in order (`AGENTS.md:18-47`):

1. **Scope the diff.** Change the requested subsystem. No drive-by refactors, dependency upgrades, or toolchain bumps unless asked.
2. **Match the tree.** Formatter, linter, test runner, package manager, and existing types win over generic advice.
3. **Smallest correct change.** Prefer editing an existing file to creating a new one. Do not add docs (`README`, `CHANGELOG`, architecture notes) unless the user asked for that document.
4. **Verify before claiming done.** Run the stack's verification commands on the crates/packages you touched. Quote failures accurately. Do not claim tests passed unless you ran them.
5. **Do not invent APIs.** Search the tree for the type, crate, package, or flag. If it is not there, it does not exist.
6. **No secrets.** Never commit tokens, private keys, connection strings, `.env` values, user data, or generated dumps. Use placeholders in examples.
7. **Do not revert unrelated work.** The working tree may contain other changes. Touch only what the task requires.
8. **Spawn a language specialist** for non-trivial work in that language (`rust-engineer`, `dotnet-engineer`, `typescript-engineer`, `python-engineer`, `php-engineer`). Stay in the parent to integrate.
9. **Git and GitHub CLI are installed.** Commits and GitHub comments must include:

   ```
   Co-Authored-By: AimeeCodes <noreply@aimeecodes.dev>
   ```

When a failing test is the task, **ask** whether to fix the implementation or update the test before changing assertions.

Aimee's own prompt restates the same six engineering rules (`crates/aimee_repo/src/agents/aimee.md:78-85`). Specialists inherit them.

## How humans should use Sage / Muse / Aimee

Three built-in agents. One loop (`AIMEE.md:56-66`).

| Agent | ID | Alias | Writes? | Use when |
|---|---|---|---|---|
| Sage | `sage` | `:ask` | No | Architecture, multi-file traces, reviews. Read-only. Does not plan or edit (`crates/aimee_repo/src/agents/sage.md:1-27`). |
| Muse | `muse` | `:plan` | Plans only | Large or ambiguous work. Writes a checkbox plan under `plans/` via the `plan` tool. Does not implement (`crates/aimee_repo/src/agents/muse.md:1-37`). |
| Aimee | `aimee` | `:act` | Yes | Implement, verify, report evidence. May dispatch Frontend / Backend / Platform specialists via `task`. Does not re-plan (`crates/aimee_repo/src/agents/aimee.md:1-39`). |

Suggested human loop:

1. **Ask Sage** (`:sage` / `:ask`) when you need the tree explained, a review, or a multi-file trace. Sage cites `path:line` and hands off: Muse if design is open, Aimee if the change is already obvious (`crates/aimee_repo/src/agents/sage.md:34-39`).
2. **Ask Muse** (`:muse` / `:plan`) before a large or ambiguous implementation. Muse writes `{YYYY-MM-DD}-{plan_name}-{version}.md` under `plans/` and never overwrites an existing plan — it bumps `version` (`crates/aimee_repo/src/agents/muse.md:34-37`). If you ask Muse to implement, it refuses and hands off to Aimee.
3. **Ask Aimee** (`:aimee` / `:act`) to apply the plan. Give Aimee the plan path, the verify command, and the boundaries. Aimee verifies on the tree before claiming done.
4. **Do not nest orchestrators.** Aimee dispatches specialists (`fe-*`, `be-*`, `plat-*`). Specialists stay in lane. Humans should not ask Sage to implement or Aimee to rewrite the plan unless the plan is wrong.

Surfaces (`AIMEE.md:13-16`):

- Interactive TUI: `aimee`
- One-shot: `aimee -p "…"`
- ZSH prefix: `: sage …`, `:muse …`, `:aimee …`

Project policy files Aimee reads: `AGENTS.md` (or `~/.aimee/AGENTS.md`) (`AIMEE.md:231`), plus `SOUL.md` when present (`crates/aimee_repo/src/agents/aimee.md:37`).

## Match the architecture

Clean architecture. Same invariants in every language (`AGENTS.md:103-124`, `AIMEE.md:80-116`).

- **No service-to-service calls.** A service depends on domain types and infrastructure abstractions. If two use cases must collaborate, compose them at the composition root (`AimeeAPI::init` in `crates/aimee_api/src/aimee_api.rs:44-56`), not inside a service.
- **Infrastructure is injected.** One generic/port per service when the language allows it. Trait bounds live on **methods**, not on `new()`.
- **Composition root owns lifetimes.** `AimeeServices` wires implementations. The domain does not.
- **Domain errors are typed** (`thiserror`). Services/CLI use `anyhow`. Do not implement `From` that collapses distinct failures (`AGENTS.md:263-269`).
- **Invalid states are unrepresentable.** Newtypes, enums, branded IDs.
- **Migrations are reviewed artifacts.** Diesel schema is generated to `crates/aimee_repo/src/database/schema.rs`. Migrations live in `crates/aimee_repo/src/database/migrations`. **Never edit a shipped migration; add a new one** (`AGENTS.md:76-78`, `AIMEE.md:292`).

### Rust service shape

Services take **at most one** generic, store infra as `Arc<T>`, implement `new()` **without** trait bounds, and apply bounds only on methods that need them. Prefer tuple structs for a single dependency. No `Box<dyn …>` in service fields (`AGENTS.md:126-157`).

```rust
pub struct UserService<R> {
    repository: Arc<R>,
}

impl<R> UserService<R> {
    pub fn new(repository: Arc<R>) -> Self { /* ... */ }
}

impl<R: UserRepository> UserService<R> {
    pub fn create_user(&self, email: &str, name: &str) -> Result<User> { /* ... */ }
}
```

Construct values with `new`, `Default`, and `derive_setters` — not raw struct literals, and not ad-hoc `with_*` helpers (`AGENTS.md:274-289`).

## Libraries policy

One package per concern. The tree wins (`AGENTS.md:84-100`).

1. If the project already depends on a library for a job, **use that library**. Do not add a second HTTP client, ORM, test runner, schema parser, or logger.
2. New versions stay at the island's existing major. Do not jump majors to get a feature.
3. Put shared Rust versions in the root `Cargo.toml` `[workspace.dependencies]` and reference them from crate manifests. Do not add a crate that `std` or an existing workspace crate already covers (`AGENTS.md:72-74`).

Canonical Rust choices (`AGENTS.md:336-358`): `thiserror` + `anyhow`, `tokio`, `reqwest`, `diesel` (in `aimee_repo` only), `tracing`, `clap`, `ratatui`, `handlebars`, `gix`, `rmcp`, `pretty_assertions` + `insta` + `aimee_test_kit`. Do not add `eyre`, `sqlx`, `async-std`, `log` + `env_logger`, `structopt`, or a second YAML/HTTP/test crate.

TypeScript evals already have `tsx`, `zod`, `yaml`, `yargs`, `pino`, `ai`, `handlebars` (`package.json:17-35`). Use those before adding anything.

## Tool descriptions (1024 characters)

Tool descriptions stay under **1024 characters**. This is enforced for LLM API compatibility (`docs/tool-guidelines.md:22`, `AGENTS.md:327`, `AIMEE.md:258`).

New tools must:

1. Join `ToolCatalog` in `crates/aimee_domain/src/tools/catalog.rs:41-61`.
2. Carry a `#[tool_description_file = "crates/aimee_domain/src/tools/descriptions/<name>.md"]` (see `SkillFetch` at `crates/aimee_domain/src/tools/catalog.rs:686-691`).
3. Route through `ToolRegistry` (`crates/aimee_app/src/tool_registry.rs`) and the existing executor path (`crates/aimee_app/src/tool_executor.rs`). Do not leave an unregistered variant (`docs/tool-guidelines.md:29-31`, `AGENTS.md:328-329`).

Write the description to explain what the tool does, when to use it, when **not** to use it, what each parameter means, and what it will not return. Prioritize that explanation over examples. Aim for 3–4 sentences; never exceed 1024 characters (`docs/tool-guidelines.md:8-27`).

## Security and data

Treat every input as untrusted: HTTP, CLI args, MCP tool args, uploaded files, LLM tool results (`AGENTS.md:191-211`, `AIMEE.md:406-413`).

- Parameterized queries only. No interpolated SQL, shell, or HTML.
- AuthN is not AuthZ. Authorize every mutation; hiding a button is not a control.
- Never log secrets, tokens, raw credentials, or full PAN/PII. Redact.
- Credentials live under the config base as `.credentials.json`. Do not put API keys in git (`AIMEE.md:229-230`).
- Restricted mode: tool execution requires permission grants (`AIMEE.md:411`).
- New endpoints inherit the existing auth scheme. Do not add an anonymous write path.
- Observability matches the tree (`tracing` in Rust). No `println!` in production paths.
- `unserialize` / `eval` / `pickle.loads` / `BinaryFormatter` on user data are forbidden.

## Anti-patterns

These apply everywhere (`AGENTS.md:215-235`). Language playbooks add stack-specific ones.

| Don't | Do |
|---|---|
| Service calling another service | Compose use cases at the composition root |
| Two libraries for one concern | Use the one already in the tree |
| Stringly IDs, modes, and statuses | Newtypes / enums / branded types |
| Catch-all `Exception` / `any` swallowed to `null` / `false` / `200` | Typed errors at the boundary; fail closed |
| Field-by-field test assertions | `assert_eq(actual, expected)` on the whole value |
| Editing a shipped migration | Add a new migration |
| Business rules in controllers, routers, or views | Application service + domain types |
| God types (500-line services, "Utils", "Helpers") | Split by use case; keep helpers local |
| Secrets, tokens, or connection strings in source | Env / secret store / placeholders |
| Inventing a package, crate, or API that is not in the tree | Search first |
| Drive-by refactors, formatter wars, dependency upgrades | Smallest correct change |
| `eval`, `pickle.loads`, `unserialize($_*)` | JSON + a schema |
| Logging PII or secrets | Structured logs with redaction |
| Timeouts that ignore cancellation | Honor the language's cancel token |
| Premature cache / queue / mediator layers | Direct call until the tree already has that layer |

Rust-specific don'ts (`AGENTS.md:376-395`):

- `From` impls that collapse distinct failures so the call site disappears.
- `Box<dyn Trait>` in service fields; extra generic type parameters; trait bounds on `new()`.
- `unwrap` / `expect` on user input, IO, or lock `Result` in library code.
- `clone()` / `Arc::clone` to silence the borrow checker.
- `std::sync::Mutex` held across `.await`; `std::thread::sleep` or `block_on` inside async.
- `unsafe` without a `SAFETY` comment and a safe API around it.
- Raw struct literals instead of `Default` + `derive_setters`; `with_*` constructors; `String::from_utf8_lossy`.
- `cargo build --release` for verification; committing `target/`.
- Code samples in `///` docs (docs are for agents — describe behavior, not tutorials) (`AGENTS.md:318-323`).
- Unregistered `ToolCatalog` variants; tool descriptions over 1024 characters.

## Prompts, plans, and generated CI

- `templates/*.md` and `crates/aimee_repo/src/agents/*.md` are production prompt surfaces. Change them only when the task is agent/prompt behavior (`AGENTS.md:876-879`).
- `plans/` is historical. Implement against source + `AGENTS.md`, not against an old plan, unless the user pointed at that plan (`AGENTS.md:880-881`).
- GitHub workflows are generated from `crates/aimee_ci`. Edit the Rust generator, not the YAML, when both exist — unless the task is a one-off workflow the generator does not own (`AGENTS.md:882-884`).

## Git

- `git` and `gh` are available. Use `gh` for GitHub operations (`AGENTS.md:864-872`).
- Every commit and GitHub comment includes `Co-Authored-By: AimeeCodes <noreply@aimeecodes.dev>`.
- Do not commit `target/`, `node_modules/`, `vendor/`, `bin/`, `obj/`, `.env`, dumps, or snapshots that are not already the project's insta workflow.
- Do not force-push shared branches unless the user asked.
- Conventional, scoped messages that describe the change — not the agent.

`scripts/greptile-pre-push.sh` runs `greptile review` if the CLI is on `PATH`; otherwise it skips with a warning (`scripts/greptile-pre-push.sh:1-9`). It is not a substitute for the verify matrix.

## Related

- [Testing](quality/testing.md) — fixture → actual → expected, verify commands
- [Skills and commands](skills.md) — project vs global skills, the `skill` tool
- Product policy: `AGENTS.md`, `AIMEE.md` §4 / §15, `docs/tool-guidelines.md`
