# AGENTS.md

House rules for coding agents in the Aimee Codes tree. This file applies to
the entire repository unless a nested `AGENTS.md` exists; the nested file
wins on conflict.

This workspace is a **Rust 2024 Cargo workspace** (`crates/*`) with a
TypeScript eval harness (`benchmarks/`, `.github/scripts/`). Other language
playbooks apply when you touch an existing island in that stack, add one the
task requires, or write fixtures/examples in that language. Do not introduce
a stack that is not already in the tree unless the task requires it.

**Decision order:** nested `AGENTS.md` → this file → code in the files you
are editing → the language playbook below. Never invent a second pattern.

---

## Operating contract

1. **Scope the diff.** Change the requested subsystem. No drive-by refactors,
   dependency upgrades, or toolchain bumps unless asked.
2. **Match the tree.** Formatter, linter, test runner, package manager, and
   existing types win over generic advice.
3. **Smallest correct change.** Prefer editing an existing file to creating a
   new one. Do not add docs (`README`, `CHANGELOG`, architecture notes) unless
   the user asked for that document.
4. **Verify before claiming done.** Run the stack's verification commands on
   the crates/packages you touched. Quote failures accurately. Do not claim
   tests passed unless you ran them.
5. **Do not invent APIs.** Search the tree for the type, crate, package, or
   flag. If it is not there, it does not exist.
6. **No secrets.** Never commit tokens, private keys, connection strings,
   `.env` values, user data, or generated dumps. Use placeholders in examples.
7. **Do not revert unrelated work.** The working tree may contain other
   changes. Touch only what the task requires.
8. **Spawn a language specialist** for non-trivial work in that language
   (`rust-engineer`, `dotnet-engineer`, `typescript-engineer`,
   `python-engineer`, `php-engineer`). Stay in the parent to integrate.
9. **Git and GitHub CLI are installed.** Commits and GitHub comments must
   include:

   ```
   Co-Authored-By: AimeeCodes <noreply@aimeecodes.dev>
   ```

When a failing test is the task, **ask** whether to fix the implementation or
update the test before changing assertions.

---

## Repository map

| Path | Role |
|---|---|
| `crates/aimee_domain` | Domain types, errors, tool catalog, policies |
| `crates/aimee_app` | Application orchestration, DTOs, tool registry |
| `crates/aimee_services` | Application services (generic over infra) |
| `crates/aimee_infra` | Infrastructure trait impls (fs, http, auth, mcp) |
| `crates/aimee_repo` | Persistence (Diesel, SQLite, proto, agent defs) |
| `crates/aimee_main` | CLI, TUI, zsh integration |
| `crates/aimee_config` | `.aimee.toml` schema and IO |
| `crates/aimee_ci` | GitHub workflow generation |
| `crates/aimee_test_kit` | Shared test fixture loaders |
| `templates/` | Agent prompt templates (edit only when the task is prompts) |
| `benchmarks/` | TypeScript eval harness (`tsx`, Node test) |
| `shell-plugin/` | Zsh plugin |
| `plans/` | Historical design notes — not current spec unless cited |
| `docs/tool-guidelines.md` | Tool-description constraints |

Workspace MSRV is `rust-version = "1.94"` in `Cargo.toml`; the pin is
`rust-toolchain.toml` (currently `1.97`). Edition is `2024`. Put shared
dependency versions in the root `Cargo.toml` `[workspace.dependencies]` and
reference them from crate manifests. Do not add a crate that `std` or an
existing workspace crate already covers.

Diesel schema is generated to `crates/aimee_repo/src/database/schema.rs`.
Migrations live in `crates/aimee_repo/src/database/migrations`. Never edit a
shipped migration; add a new one.

CI sets `RUSTFLAGS=-D warnings`. Warnings are errors.

---

## Libraries policy

One package per concern. The tree wins.

1. If the project already depends on a library for a job, **use that
   library**. Do not add a second HTTP client, ORM, test runner, schema
   parser, or logger.
2. If you must add a dependency, pick from the **preferred** column in the
   playbook below and add it the way the island already manages deps
   (workspace `Cargo.toml`, `Directory.Packages.props`, lockfile, Composer).
3. Prefer the standard library / framework (Tokio, ASP.NET, FastAPI,
   Laravel, React) over a package that duplicates it.
4. Do not add MediatR, AutoMapper, Redux, Celery, Spatie permission packs,
   or similar "enterprise kits" unless the island already has them or the
   task names them.
5. New versions stay at the island's existing major. Do not jump majors to
   get a feature.

---

## Architecture (all stacks)

Clean architecture. Same invariants in every language:

- **No service-to-service calls.** A service depends on domain types and
  infrastructure abstractions (repositories, clocks, HTTP, file IO). If two
  use cases must collaborate, compose them at the composition root, not
  inside a service.
- **Infrastructure is injected.** One generic/port for infra per service
  when the language allows it. Compose multiple capabilities with
  intersection types / trait bounds / interfaces on **methods**, not on the
  constructor.
- **Composition root owns lifetimes.** `AimeeServices` / `Program.cs` DI /
  FastAPI `lifespan` / Laravel service provider / React query client. Not
  the domain.
- **Domain errors are typed.** Map to HTTP/CLI/UI at the edge. Do not leak
  driver/ORM exceptions across the boundary.
- **Invalid states are unrepresentable.** Newtypes, enums, records, and
  branded IDs over booleans and string modes.
- **Migrations and public API changes are reviewed artifacts.** Additive
  schema first. Compatibility defaults for new persisted fields.

### Rust service shape (canonical for this repo)

Services take **at most one** generic parameter, store infra as `Arc<T>`,
implement `new()` **without** trait bounds, and apply bounds only on methods
that need them. Prefer tuple structs for a single dependency. No
`Box<dyn ...>` in service fields.

```rust,ignore
pub struct UserValidationService;

impl UserValidationService {
    pub fn new() -> Self { Self }
    pub fn validate_email(&self, email: &str) -> Result<()> { /* ... */ }
}

pub struct UserService<R> {
    repository: Arc<R>,
}

impl<R> UserService<R> {
    pub fn new(repository: Arc<R>) -> Self { /* ... */ }
}

impl<R: UserRepository> UserService<R> {
    pub fn create_user(&self, email: &str, name: &str) -> Result<User> { /* ... */ }
}

pub struct FileService<F>(Arc<F>);

impl<F: FileReader + Environment> FileService<F> {
    pub async fn read_with_validation(&self, path: &Path) -> Result<String> { /* ... */ }
}
```

The same topology in other stacks: one injected port (or a composed
interface), constructor does not demand the full bound set, composition root
wires implementations.

---

## Testing contract (all stacks)

Every test has three named steps: **fixture → actual → expected**. Assert
on the whole value, not field-by-field.

- Name locals `fixture`, `actual`, `expected`.
- Fixtures are generic and reusable. Prefer `Default` / builders / test
  constructors over struct literals with every field filled.
- Keep boilerplate low. Unwrap in tests unless the error value is the
  assertion.
- Colocate unit tests with the source (same file or `__tests__` next to the
  module — match the island). Integration tests stay in the project's
  existing `tests/` / `*.Tests` / `tests/` tree.
- Prefer equality on full objects.

**Good:** `assert_eq(actual, expected)` / `Assert.Equal(expected, actual)` /
`assert actual == expected`.

**Bad:** asserting `actual.a`, then `actual.b`, then `actual.c`.

When error context matters, `expect("List should not be empty")` (or the
language equivalent) beats `panic!` / `fail()` after an `if let`.

---

## Security and data

- Treat all external input as untrusted (HTTP, CLI args, MCP tool args,
  uploaded files, LLM tool results).
- Parameterized queries only. No f-string / interpolated SQL, shell, or
  HTML.
- AuthN is not AuthZ. Authorize every mutation; hiding a button is not a
  control.
- Never log secrets, tokens, raw credentials, or full PAN/PII. Redact.
- `unserialize` / `eval` / `pickle.loads` / `BinaryFormatter` on user data
  are forbidden.
- File uploads: validate MIME **and** extension; store outside webroot or
  with randomized names; no executable extensions.
- CSRF on cookie-authenticated state changes. CORS and CSP match the
  existing app — do not widen them to "fix" a call.
- Open redirects: allowlist.
- New endpoints inherit the existing auth scheme. Do not add an anonymous
  write path.
- Observability matches the tree (`tracing` in Rust, `ILogger<T>` in .NET,
  `logging`/`structlog` in Python). No `println!` / `console.log` /
  `dd()` in production paths. Do not log secrets.

---

## Anti-patterns (all stacks)

These apply everywhere. Language playbooks add stack-specific ones.

| Don't | Do |
|---|---|
| Service calling another service | Compose use cases at the composition root |
| Two libraries for one concern (HTTP, ORM, JSON, test runner, logger) | Use the one already in the tree |
| Stringly IDs, modes, and statuses | Newtypes / enums / branded types |
| Catch-all `Exception` / `any` / `mixed` swallowed to `null` / `false` / `200` | Typed errors at the boundary; fail closed |
| Field-by-field test assertions | `assert_eq(actual, expected)` on the whole value |
| Editing a shipped migration | Add a new migration |
| Business rules in controllers, routers, or views | Application service + domain types |
| God types (500-line services, "Utils", "Helpers", "Common") | Split by use case; keep helpers local |
| Secrets, tokens, or connection strings in source | Env / secret store / placeholders in examples |
| Inventing a package, crate, or API that is not in the tree | Search first; then the preferred table |
| Drive-by refactors, formatter wars, dependency upgrades | Smallest correct change |
| `eval`, `pickle.loads`, `unserialize($_*)`, `BinaryFormatter` | JSON + a schema |
| Logging PII or secrets | Structured logs with redaction |
| Timeouts that ignore cancellation | Honor `CancellationToken` / `AbortSignal` / context |
| Premature cache / queue / mediator layers | Direct call until the tree already has that layer |

```rust,ignore
// BAD: service depending on another service
pub struct BadUserService<R, E> {
    repository: R,
    email_service: E,
}

// BAD: trait objects in service fields
pub struct BadUserService {
    repository: Box<dyn UserRepository>,
}

// BAD: many type parameters + bounds on new()
pub struct BadUserService<R, C, L> { /* ... */ }
impl<R: UserRepository, C: Cache, L: Logger> BadUserService<R, C, L> {
    pub fn new(repository: R, cache: C, logger: L) -> Self { /* ... */ }
}
```

---

## Language playbooks

### Rust (this workspace)

**Errors**

- Services and repositories: `anyhow::Result`.
- Domain errors: `thiserror`.
- **Do not implement `From` to convert domain errors.** Convert manually so
  the site and the value remain visible. Skipping `From` on `thiserror`
  variants (`#[from(skip)]`) is the intended pattern when a crate already
  derives `From` for a subset of variants.
- Library/domain code does not `unwrap` / `expect` on user input or
  recoverable IO. Panic only for broken internal invariants.

**Types**

- `derive_setters` on structs that need builders. Use `strip_option` and
  `into`.
- Construct values with `new`, `Default`, and setters — not raw struct
  literals, and not ad-hoc `with_*` helpers.

  ```rust,ignore
  // Good
  User::default().age(12).is_happy(true).name("John")
  User::new("Job").age(12).is_happy()
  User::test()

  // Bad
  User { name: "John".to_string(), is_happy: true, age: 12 }
  User::with_name("Job")
  ```

- Make illegal states unrepresentable. No `clone()` / `Arc` as a
  borrow-checker escape hatch unless the sharing is real.
- `unsafe` requires a `SAFETY` comment and a safe wrapper. `clippy.toml`
  forbids `String::from_utf8_lossy`; use `bstr::ByteSlice::to_str_lossy`.
- Do not hold `std::sync::Mutex` across `.await` on Tokio. Do not block the
  runtime.

**Tests**

```rust,ignore
use pretty_assertions::assert_eq; // always

fn test_foo() {
    let fixture = /* ... */;
    let actual = /* execute */;
    let expected = /* handwritten */;
    assert_eq!(actual, expected);
}
```

- Tests live in the same file as the source (`#[cfg(test)]`).
- `unwrap` in test functions; `anyhow::Result` in fixtures.
- Load files via `aimee_test_kit::fixture!` / `json_fixture!` when the crate
  already depends on it.
- Snapshot tests use **insta** (`insta.yaml` auto-accepts; runner is
  nextest). Prefer `cargo insta test --accept` over ad-hoc snapshot files.

**Docs**

- `///` on all public methods, functions, structs, enums, and traits.
- `# Arguments` and `# Errors` when they apply.
- **No code examples in Rust docs** — docs are for agents. Describe
  behavior, not tutorials.

**Tools (agent runtime)**

- Tool descriptions stay under **1024 characters**.
- New tools join `aimee_domain` `ToolCatalog` and the existing executor/
  registry path. Do not leave an unregistered variant.

**Libraries (workspace-canonical)**

Take versions from root `Cargo.toml` `[workspace.dependencies]`. Do not add
a crate that is already represented here.

| Concern | Prefer | Do not add |
|---|---|---|
| Domain errors | `thiserror` | `eyre`, `snafu`, `color-eyre` |
| Service/CLI errors | `anyhow` | wrapping everything in `thiserror` at the edge |
| Async runtime | `tokio`, `futures`, `async-trait`, `tokio-stream`, `tokio-util` | `async-std`, `smol` |
| HTTP client | `reqwest` (workspace TLS features) | `ureq`, `isahc`, a second `reqwest` with native-tls |
| gRPC | `tonic` | `grpcio` |
| JSON / schema | `serde`, `serde_json`, `schemars`, `eserde` | `simd-json` unless measured and isolated |
| YAML / TOML | `serde_yml`, `toml_edit` | a second YAML crate |
| Config merge | `merge` | hand-rolled deep merge |
| ORM / SQL | Diesel (`aimee_repo`) | `sqlx`, `sea-orm`, `diesel` async rewrite |
| Logging | `tracing`, `tracing-subscriber`, `tracing-appender` | `log` + `env_logger`, `slog` |
| CLI | `clap` | `structopt`, `argh` |
| TUI | `ratatui` | `tui` 0.19, `cursive` |
| Prompts | `handlebars` | `tera`, `askama` as a second engine |
| Git | `gix` | shelling out to `git` for reads the crate already covers |
| MCP | `rmcp` | a hand-rolled MCP client |
| Retry | `backon` | `retry`, custom sleep loops |
| Bytes / lossy UTF-8 | `bytes`, `bstr` | `String::from_utf8_lossy` |
| IDs / time | `uuid`, `chrono` | a second UUID or clock crate |
| Enums / derives | `strum`, `derive_more`, `derive_setters` | `enum_dispatch` unless needed |
| Tests | `pretty_assertions`, `insta`, `aimee_test_kit`, `tempfile`, `mockito` | `assert2`, `expect-test`, `claim` |
| Fuzzy select | `nucleo`, `nucleo-picker` | `fuzzy-matcher` as a second engine |
| Auth providers | workspace AWS / Google crates already listed | extra SDK copies |

**Verify (Rust)**

```bash
cargo fmt
cargo check -p <crate>
cargo clippy -p <crate> --all-targets -- -D warnings
cargo insta test --accept -p <crate>
```

- **Never** `cargo build --release` unless the task is a release binary or a
  measured benchmark. Release is slow and unused for correctness.
- Prefer `cargo check` > `cargo insta test` > `cargo build` (debug).
- Do not pass `--all-features` if a crate documents a broken matrix; then
  test the documented matrix.

**Anti-patterns**

- `From` impls that collapse distinct failures (`serde_json::Error` → one
  domain variant) so the call site disappears.
- `Box<dyn Trait>` in service fields; extra generic type parameters;
  trait bounds on `new()`.
- `unwrap` / `expect` on user input, IO, or lock `Result` in library code.
- `clone()` / `Arc::clone` to silence the borrow checker.
- `std::sync::Mutex` held across `.await`; `std::thread::sleep` or
  `block_on` inside async.
- `unsafe` without a `SAFETY` comment and a safe API around it.
- Raw struct literals instead of `Default` + `derive_setters`.
- `with_*` constructors; `String::from_utf8_lossy`.
- Adding `eyre` / `sqlx` / `async-std` / `log` next to the workspace
  choices.
- `cargo build --release` for verification; committing `target/`.
- Tests in a separate file when neighbors use `#[cfg(test)]` in-module.
- Field-by-field `assert_eq!`; `panic!` instead of `expect`.
- Code samples in `///` docs; unregistered `ToolCatalog` variants;
  tool descriptions over 1024 characters.

---

### .NET / C#

Match `global.json`, `TargetFramework`, nullable context, and analyzers.
C# is the default; stay in F# when the project is F#. Do not bump TFM or
language version unless asked.

**Types and nullability**

- Nullable reference types stay on. Public APIs express nullability. Do not
  silence with `!` or `#pragma warning disable` without a one-line reason.
- `record` for values, `class` for identity. File-scoped namespaces, primary
  constructors, `required` / `init` — match the file's C# version.
- `IReadOnlyList<T>` / `IReadOnlyDictionary<K,V>` on inputs. Return concrete
  types when callers need them.
- Domain errors: Result-shaped types if the project has them; otherwise
  typed exceptions at the application edge, Problem Details (`RFC 9457`) at
  HTTP.

**Async, DI, IO**

- Async all the way. No `.Result`, `.Wait()`, or `GetAwaiter().GetResult()`
  on request paths. No `async void` except existing event-handler shapes.
- `CancellationToken` on every IO entry point; honor it.
- `ConfigureAwait(false)` in **libraries**. In ASP.NET Core, follow
  neighbors (usually omit).
- DI lifetimes are correctness: a singleton must not capture a scoped
  service (`DbContext`, `HttpContext`). Background work uses
  `IServiceScopeFactory`.
- `IHttpClientFactory` only — never `new HttpClient()` per call.
- `IOptions<T>`, `TimeProvider`, `Channel<T>`, `ILogger<T>`. No
  `Console.WriteLine` in app code. No `DateTime.Now` in domain logic when
  `TimeProvider` is available.

**ASP.NET**

- Minimal APIs vs controllers: match the app. Do not dual-stack.
- Bind and validate at the edge. No `dynamic` JSON.
- `[Authorize]` / policies on mutations. Endpoint filters and auth stay
  consistent with existing endpoints.

**EF Core**

- Filter then project. No `ToList()` before `Where`. Watch N+1 (`Include` /
  `AsSplitQuery` as the project does).
- `AsNoTracking()` on read paths. Do not share `DbContext` across threads.
- Do not edit a shipped migration.

**Tests**

```csharp
[Fact]
public async Task CreateUser_ReturnsUser()
{
    var fixture = User.Default().Age(12).IsHappy(true).Name("John");
    var actual = await service.CreateAsync(fixture, CancellationToken.None);
    var expected = /* handwritten */;
    Assert.Equal(expected, actual);
}
```

Use xUnit / NUnit / MSTest — whichever the solution already has.
FluentAssertions / Shouldly only if already referenced. `WebApplicationFactory`
for HTTP tests when the project uses it.

**Libraries**

BCL and ASP.NET first. Add a package only when the framework does not
cover it.

| Concern | Prefer | Do not add |
|---|---|---|
| Web host | ASP.NET Core (minimal APIs **or** MVC — match the app) | a second web stack (Nancy, Carter unless present) |
| DI / options / logging | `Microsoft.Extensions.*`, `ILogger<T>`, `IOptions<T>` | Autofac / DryIoc unless already the container |
| HTTP out | `IHttpClientFactory` | `new HttpClient()`, RestSharp, Flurl as a second client |
| JSON | `System.Text.Json` | `Newtonsoft.Json` unless the tree already uses it |
| ORM | EF Core + the project's provider | a second ORM; generic repository wrapping `DbContext` |
| Validation | DataAnnotations **or** FluentValidation — match the app | both |
| Mapping | hand-written projections / Mapperly if already used | AutoMapper profiles for two-field DTOs |
| Resilience | `Microsoft.Extensions.Http.Resilience` / Polly v8 if present | custom retry-with-sleep |
| Telemetry | OpenTelemetry + `ILogger<T>` | Serilog **and** NLog **and** OTel |
| Auth | ASP.NET Identity / JWT bearer as the app already does | rolling crypto, IdentityServer as a surprise |
| Tests | xUnit (default), `Microsoft.AspNetCore.Mvc.Testing`, NSubstitute **or** Moq (match), Testcontainers if already used | a second test framework; FakeItEasy next to Moq |
| Assertions | xUnit asserts, or FluentAssertions / Shouldly if referenced | both FluentAssertions **and** Shouldly |
| Time | `TimeProvider` | `DateTime.Now` in domain logic |

Do **not** introduce MediatR, MassTransit, AutoMapper, or Hangfire unless
the solution already has them or the task names them.

**Verify**

```bash
dotnet format
dotnet build --nologo
dotnet test --nologo
```

Treat analyzer warnings as errors when the project does. Do not widen
suppressions.

**Anti-patterns**

- `async void` on the request path; `.Result` / `.Wait()` /
  `GetAwaiter().GetResult()`.
- `new HttpClient()` per call; `HttpClient` as a static field without
  `IHttpClientFactory`.
- Singleton capturing scoped `DbContext` / `HttpContext`.
- `ToList()` then LINQ-to-objects; N+1 `Include` of entire graphs;
  tracking queries on read-only endpoints.
- Returning EF entities as the HTTP contract; `dynamic` JSON; `null!`
  to shut nullable up.
- Catching `Exception` and returning `null` / `false` / `200`.
- Generic repository on top of EF Core (`IRepository<T>` that hides
  `IQueryable` and adds nothing).
- MediatR for three endpoints; AutoMapper for two-field DTOs.
- `Newtonsoft.Json` next to `System.Text.Json`.
- `DateTime.Now` in domain logic when `TimeProvider` exists.
- `#pragma warning disable` for nullable without a comment.
- Checking in `bin/` / `obj/` / `.suo`; bumping TFM to land a change.

---

### React / TypeScript

Applies to UI islands **and** this repo's eval/bounty TypeScript
(`package.json` is `"type": "module"`, `tsx`, Node test). Match the local
`tsconfig`, package manager (`pnpm` / `npm` / `yarn`), and bundler. Do not
change `module` / `moduleResolution` / `target` unless asked.

**Type system**

- `strict` is the floor. No `any`, `as any`, or `@ts-ignore`. If a cast is
  unavoidable, `@ts-expect-error` with a one-line reason.
- Narrow; do not assert. Discriminated unions, `satisfies`, type guards,
  exhaustive `switch` (`const _exhaustive: never = x`).
- Types erase: runtime truth is a parser (zod / valibot / arktype — whatever
  the tree uses) plus a type. `unknown` at boundaries, parse before use.
- Named vs default exports: match the file. `import type` for type-only
  imports.

**React**

- Function components and hooks only. Do not introduce a second state
  library (Redux, Zustand, Jotai, …) if one already exists — or if none
  exists and local state / the existing data layer suffices.
- Server vs client components stay on the correct side of the boundary. Do
  not import server-only modules into `"use client"` graphs.
- Props are explicit. Do not type `children` as `any`.
- Data fetching: the library already in the tree (TanStack Query, SWR,
  Remix/React Router loaders, Next fetch). No ad-hoc `useEffect` +
  `fetch` for server state when a cache library is present.
- Accessibility is part of the change: label controls, keyboard paths,
  roles only when native HTML is insufficient. Do not add
  `dangerouslySetInnerHTML` with untrusted strings.
- Styling: match neighbors (CSS modules, Tailwind, existing design tokens).
  Do not add a CSS-in-JS runtime to a project that does not have one.
- After UI changes, verify in the browser (or the closest substitute:
  component tests + Playwright/Cypress if present). A screenshot is not
  verification. Exercise the flow, shared routes that read the same state,
  empty/error states, and desktop + mobile viewports when layout changed.

**Evals / Node TS in this repo**

- `benchmarks/` and `.github/scripts/bounty/` stay ESM. Prefer existing
  helpers in those trees. Run `npm run eval` / `npm run test:bounty` for
  work in those paths — not a new test runner.
- Already in root `package.json`: `tsx`, `typescript`, `zod`, `yaml`,
  `yargs`, `pino`, `ai`, `handlebars`, `csv-parse`. Use those before adding
  anything.

**Tests**

```ts
it("creates a user", () => {
  const fixture = userFixture({ age: 12, isHappy: true, name: "John" });
  const actual = createUser(fixture);
  const expected = { /* handwritten */ };
  expect(actual).toEqual(expected);
});
```

Vitest / Jest / `node:test` — match the island. Testing Library for DOM.
Prefer equality on full objects. Colocate tests as the island already does.

**Libraries**

| Concern | Prefer | Do not add |
|---|---|---|
| UI | `react`, `react-dom` (function components) | class components, Preact in a React app |
| Language | TypeScript `strict` | JSDoc types as a substitute in `.ts` trees |
| Bundler / app | Vite **or** Next.js — match the island | CRA, Webpack-from-scratch, dual App+Pages routers |
| Server state | TanStack Query **or** SWR **or** router loaders — match | Redux for server cache; `useEffect` + `fetch` |
| Client state | component state, then the store already in the tree | Redux + Zustand + Jotai in one app |
| Routing | React Router **or** TanStack Router **or** Next App Router | a second router |
| Forms | React Hook Form + the tree's schema resolver | Formik next to RHF; uncontrolled soup |
| Schema | `zod` (already in this repo's evals) | yup **and** zod; io-ts unless present |
| HTTP | `fetch` / `ky` / the app's API client | axios **and** fetch; `jquery` |
| CSS | Tailwind **or** CSS modules — match | styled-components in a Tailwind app |
| Dates | `Temporal` if used, else `date-fns` | `moment`, `dayjs` as a second clock |
| Icons | the set already imported (`lucide-react`, etc.) | a second icon pack |
| Tests | Vitest + Testing Library; Playwright for e2e | Jest **and** Vitest; Enzyme |
| API mocks | MSW if present | ad-hoc `jest.mock('node-fetch')` in UI tests |
| Node evals (this repo) | `tsx`, `zod`, `yaml`, `yargs`, `pino`, `ai` | `ts-node`, `winston` next to `pino` |

**Verify**

```bash
# Prefer package scripts. Fallbacks:
npx tsc --noEmit -p <tsconfig you touched>
# then the repo's lint + test (eslint/biome, vitest, playwright)
```

Typecheck the affected package in a project-references monorepo, not only
the root.

**Anti-patterns**

- `any`, `as any`, `@ts-ignore`, disabling `strict` / `noImplicitAny`.
- `useEffect(() => { fetch(...) }, [])` for server state when Query/SWR/
  loaders exist.
- Redux (or a new store) for data that is already cached server state.
- Derived values stored in `useState` and synced with `useEffect`.
- `key={index}` on dynamic lists; array-index keys for reorderable rows.
- `dangerouslySetInnerHTML` with untrusted strings; `eval` / `new Function`.
- Importing server-only modules into `"use client"` graphs.
- `moment`; axios next to `fetch`; styled-components next to Tailwind.
- Giant `index.ts` barrels that re-export the whole app and create cycles.
- Context for high-frequency server state; prop-drilling through six
  layers when a context or query already exists.
- Accessibility as an afterthought: clickable `div`s, unlabeled inputs,
  missing keyboard paths.
- Screenshot-only "verification" of UI changes.
- Adding a runtime dependency for a type-only problem.

---

### FastAPI / Python

Match Python version, installer (`uv` / `poetry` / `pip-tools` / `pdm`),
Pydantic major, and the existing app factory. Do not add a second web
framework.

**Types and layout**

- Public functions are annotated. `Protocol` / `TypedDict` / `NewType` /
  generics over `Any` and `dict[str, Any]`.
- Pydantic v2 vs v1: match the installed major. Settings via the project's
  existing settings object (`pydantic-settings` if present).
- Layering: `APIRouter` → application service → repository/port. Routers do
  not contain business rules or ORM calls. Services do not import
  `fastapi`.
- Validate at the edge (`response_model`, input models). Trust internals
  after parse.
- `Annotated[..., Depends()]` for DI. Lifespan (`lifespan=`) owns pools and
  clients — one HTTP/DB client per app, not per request.

**Async**

- A function is sync or async; do not mix casually. No `time.sleep` in async
  code. No nested `asyncio.run()`.
- Cancel-safe cleanup (`try/finally` or context managers). Do not swallow
  `CancelledError`.
- `httpx.AsyncClient` (or the project's client) for outbound HTTP, created
  in lifespan.

**Errors and logging**

- Domain exceptions mapped to HTTP in exception handlers. Do not raise
  `HTTPException` from repositories.
- `logging.getLogger(__name__)` or the project's structlog/loguru. Chain
  with `raise X from e`. No mutable default arguments. No `from module
  import *`.

**Data**

- SQLAlchemy 2.x / the project's ORM: parameterized queries only. No
  f-string SQL. Transactions around multi-write operations.
- Alembic (or the project's migrator): additive migrations; never rewrite
  shipped revisions.

**Tests**

```python
def test_create_user(client: TestClient) -> None:
    fixture = UserFactory(age=12, is_happy=True, name="John")
    actual = client.post("/users", json=fixture.model_dump()).json()
    expected = {"age": 12, "is_happy": True, "name": "John"}
    assert actual == expected
```

`pytest` + `httpx.AsyncClient` / `TestClient` as the app already does.
`pytest-asyncio` only if already configured. Fixture factories over
hand-built dicts. Prefer equality on full payloads.

**Libraries**

| Concern | Prefer | Do not add |
|---|---|---|
| Web | `fastapi` | Flask / Django / Starlette-only apps next to FastAPI |
| Server | `uvicorn` (or the process manager already in the tree) | gunicorn+sync workers for an async app |
| Models / settings | Pydantic v2, `pydantic-settings` | Pydantic v1 in a v2 app; `os.environ[...]` in business code |
| HTTP out | `httpx.AsyncClient` (lifespan-scoped) | `requests` inside `async def`; new client per request |
| ORM | SQLAlchemy 2.x async + `asyncpg` / `psycopg` | SQLAlchemy 1.4 session API; Tortoise next to SA |
| Migrations | Alembic | rewriting old revisions; a second migrator |
| Redis | `redis.asyncio` | `aioredis` (legacy) |
| Tasks | the queue already in the tree (`arq` / `taskiq` / Celery) | Celery in an app with no workers |
| Auth | the app's existing JWT/session stack (`pwdlib` / `bcrypt`, PyJWT) | `passlib` **and** `pwdlib`; rolling crypto |
| Logging | stdlib `logging` or `structlog` — match | `print`, `loguru` next to structlog |
| Lint / types | `ruff`, `mypy` or `pyright` | `flake8`+`black`+`isort` next to Ruff |
| Tests | `pytest`, `httpx` ASGI / `TestClient`, `pytest-asyncio` if present | `unittest` as a second runner |
| Retry | `tenacity` if present | ad-hoc `time.sleep` loops |
| Installer | `uv` / Poetry / pip-tools — match | a second lockfile format |

**Verify**

```bash
# Prefer project scripts (uv run, poetry run, make). Fallbacks:
ruff check <paths>
ruff format --check <paths>
mypy <paths>   # or pyright / ty — whatever the repo uses
pytest <paths>
```

**Anti-patterns**

- Business logic or ORM calls in routers; `HTTPException` raised from
  repositories.
- `def` route that does IO; `requests.get` / `time.sleep` inside
  `async def`; `asyncio.run()` from a running loop.
- Engine / `Session` / `httpx.Client` created per request; global
  mutable session.
- Pydantic v1 models in a v2 codebase; `dict[str, Any]` as the domain.
- `from module import *`; mutable default arguments (`def f(x=[])`).
- f-string SQL; `os.system` / `shell=True` with untrusted input.
- Catching `Exception` to return `None`; swallowing `CancelledError`.
- `typing.Any` (or `# type: ignore`) to make mypy pass.
- Flask or Django next to FastAPI; Celery "just in case".
- Committing `venv/`, `__pycache__/`, `.pyc`.

---

### Laravel / PHP

Match `composer.json` `php` and `laravel/framework` majors, Pint / php-cs-fixer,
and PHPUnit / Pest. `declare(strict_types=1);` on new files if neighbors use
it. Do not bump PHP or Laravel major unless asked.

**Types and HTTP**

- Property, parameter, and return types on all new code. Enums over magic
  strings. No `mixed` unless the boundary is mixed **and** you parse
  immediately.
- Thin controllers. FormRequest (or equivalent) + policy/gate on every
  mutation. **Never** `$request->all()` into `create()` / `update()`.
- API Resources / dedicated response DTOs at the edge. Do not leak Eloquent
  models as JSON unless that is already the project convention.
- Facades vs injected contracts: match the file. Do not mix a new style
  into an existing class.

**Eloquent and jobs**

- `$fillable` / `$guarded` are security boundaries. Casts are explicit.
  Eager-load to avoid N+1.
- Pass IDs into queued jobs when the project already does; do not serialize
  Eloquent models onto the queue unless that is the existing pattern.
- Jobs are idempotent. Retry/backoff match neighbors.
- Do not edit a shipped migration.

**Security**

- Blade auto-escape stays on. `{!! !!}` only for proven-safe HTML.
- CSRF on cookie-authenticated web routes. `unserialize` on user input is
  forbidden — use JSON.
- Passwords go through the framework hasher. File uploads: MIME **and**
  extension, randomized names, not webroot.

**Tests**

```php
public function test_it_creates_a_user(): void
{
    $fixture = User::factory()->make(['age' => 12, 'is_happy' => true, 'name' => 'John']);
    $actual = $this->postJson('/users', $fixture->toArray())->json();
    $expected = [ /* handwritten */ ];
    $this->assertEquals($expected, $actual);
}
```

Pest if the project uses Pest; otherwise PHPUnit. HTTP tests go through the
application kernel (`postJson`, acting-as). RefreshDatabase / database
transactions as the suite already does.

**Libraries**

Laravel already ships routing, Eloquent, queues, cache, mail, filesystem,
and validation. Prefer those. First-party and widely used extensions:

| Concern | Prefer | Do not add |
|---|---|---|
| Framework | `laravel/framework` (match major) | Symfony as a second app kernel |
| Auth (API) | Laravel Sanctum **or** Passport — match | JWT package next to Sanctum |
| Permissions | `spatie/laravel-permission` if already used | a second ACL pack |
| DTOs | `spatie/laravel-data` if already used | both Data and handwritten DTO stacks |
| Query APIs | `spatie/laravel-query-builder` if already used | rolling ad-hoc filter DSLs |
| SPA bridge | Inertia **or** Livewire — match the island | Inertia **and** Livewire for the same UI |
| Queues UI | Laravel Horizon if Redis queues exist | Horizon in a sync `sync` queue app |
| Tests | Pest **or** PHPUnit — match | both as primary runners |
| Static analysis | Larastan (`larastan/larastan`) + Pint | Psalm **and** PHPStan without a reason |
| HTTP tests | Laravel HTTP kernel (`postJson`, acting-as) | Guzzle hitting a live server in unit tests |
| Media / backup / activity | Spatie packages **if already in composer.json** | adding the whole Spatie catalog "for later" |
| Errors | `sentry/sentry-laravel` if the app already reports | a second APM next to Sentry |
| Redis | `phpredis` / `predis` as configured | both clients |

**Verify**

```bash
vendor/bin/pint --test
vendor/bin/phpstan analyse
vendor/bin/phpunit   # or: vendor/bin/pest
```

Honor `composer test` / `composer analyse` when defined.

**Anti-patterns**

- Fat controllers; queries in Blade; `$request->all()` into `create()` /
  `update()`.
- God Eloquent models; model observers that call other models (hidden
  service-to-service).
- N+1 (`User::all()` then `$user->posts` in a loop) without `with()`.
- `env()` outside config files; `DB::raw()` with request data.
- `{!! $untrusted !!}`; `unserialize($request->…)`; `eval`, `extract`,
  `@` error suppression, variable-variables.
- `shell_exec` / `exec` / backticks with untrusted input.
- Catching `\Throwable` and returning empty success; disabling TLS verify.
- Serializing Eloquent models onto the queue when the project passes IDs.
- Mixing facade style into a class that already uses constructor injection
  (or the reverse).
- Adding Doctrine, Cake, or a second router to a Laravel app.
- Committing `vendor/` unless the project already vendors; rewriting old
  migrations; bumping PHP or Laravel major to land a change.

---

## Verification matrix

Run **only** what matches the files you touched, then widen if the change
crosses a public contract, schema, or runtime boundary.

| Island | Format | Static | Test | Never |
|---|---|---|---|---|
| Rust crate | `cargo fmt` | `cargo check` + `clippy -D warnings` | `cargo insta test --accept -p <crate>` | `cargo build --release` |
| TypeScript evals | project formatter | `tsc --noEmit` | `npm run eval` / `npm run test:bounty` | a second test runner |
| React UI | project formatter | `tsc` + eslint/biome | unit + e2e the project has; **browser-exercise the flow** | screenshot-only "proof" |
| .NET | `dotnet format` | `dotnet build` | `dotnet test` | TFM bump, `bin/` commit |
| FastAPI | `ruff format` | `ruff check` + mypy/pyright | `pytest` | extra framework, unpinned dep dump |
| Laravel | `pint --test` | `phpstan analyse` | `pest` / `phpunit` | rewriting old migrations |

UI work that changes layout, routing, or client state is not done until the
changed flow is exercised the way a user would (browser tools if available;
otherwise tests + HTTP against the dev server, and say what you could not
click).

---

## Git and GitHub

- `git` and `gh` are available. Use `gh` for GitHub operations.
- Every commit and GitHub comment includes
  `Co-Authored-By: AimeeCodes <noreply@aimeecodes.dev>`.
- Do not commit `target/`, `node_modules/`, `vendor/`, `bin/`, `obj/`,
  `.env`, dumps, or snapshots that are not already the project's insta
  workflow.
- Do not force-push shared branches unless the user asked.
- Conventional, scoped messages that describe the change — not the agent.

---

## Prompts, plans, and generated CI

- `templates/*.md` and `crates/aimee_repo/src/agents/*.md` are production
  prompt surfaces. Change them only when the task is agent/prompt behavior.
- `plans/` is historical. Implement against source + this file, not against
  an old plan, unless the user pointed at that plan.
- GitHub workflows are generated from `crates/aimee_ci`. Edit the Rust
  generator, not the YAML, when both exist — unless the task is a one-off
  workflow the generator does not own.
