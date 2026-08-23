# Testing

House rules for tests live in the product tree (`AGENTS.md` testing contract, `AIMEE.md` §14). This page is the human map: how a test is shaped, where it lives, and which commands actually exist.

Policy, not this GitBook, wins on conflict. Do not invent a second runner, a second snapshot library, or an npm script that is not in root `package.json`.

## Contract: fixture → actual → expected

Every test has three named steps. Assert on the **whole value**, not field-by-field (`AGENTS.md:166-184`, `AIMEE.md:370-383`).

```rust
use pretty_assertions::assert_eq;

fn test_foo() {
    let fixture = /* reusable input */;
    let actual = /* execute */;
    let expected = /* handwritten */;
    assert_eq!(actual, expected);
}
```

Rules that the tree actually enforces:

- Name the locals `fixture`, `actual`, `expected`.
- Fixtures are generic and reusable. Prefer `Default`, builders, or test constructors over struct literals with every field filled.
- Keep boilerplate low. Unwrap in test functions unless the error value is the assertion. Use `anyhow::Result` in fixtures.
- Prefer equality on the full object. Field-by-field `assert_eq!` on `actual.a`, then `actual.b`, then `actual.c` is the anti-pattern (`AGENTS.md:184-185`).
- When error context matters, `expect("List should not be empty")` beats `panic!` after an `if let`.
- Always import `pretty_assertions::assert_eq` in Rust tests (`AGENTS.md:300-308`).

A colocated example of the same shape is `AimeeSkillFetch` (`crates/aimee_services/src/tool_services/skill.rs:78-97`): fixture skills, `actual` from `fetch_skill`, `expected` as a handwritten `Skill`.

When a failing test **is** the task, ask whether to fix the implementation or update the test before changing assertions (`AGENTS.md:46-47`). Never delete a failing test to go green.

## Where tests live

| Kind | Location | Rule |
|---|---|---|
| Rust unit | Same file as the source, `#[cfg(test)]` | Neighbors already do this. Do not split a unit test into a sibling file (`AGENTS.md:311`, `AGENTS.md:392`). |
| Shared fixtures | `crates/aimee_test_kit` | Load files with `aimee_test_kit::fixture!` / `json_fixture!` when the crate already depends on it (`AGENTS.md:313-314`). |
| Snapshots | **insta**, nextest runner | `insta.yaml` auto-accepts. Prefer `cargo insta test --accept` over ad-hoc snapshot files (`AGENTS.md:315-316`). |
| TypeScript evals | `benchmarks/` and `.github/scripts/bounty/` | ESM. Use the existing helpers. Do not add a second test runner (`AGENTS.md:559-563`). |
| Integration | The project's existing `tests/` tree | Match the island. Do not invent a new layout. |

`aimee_test_kit` is the shared fixture crate (`AIMEE.md:179`). It exposes:

- `fixture(path)` / `fixture!("relative/to/crate")` — async UTF-8 load relative to `CARGO_MANIFEST_DIR` (`crates/aimee_test_kit/src/lib.rs:15-33`).
- `json_fixture` / `json_fixture!` — same path, parsed as JSON. Behind the `json` feature (`crates/aimee_test_kit/src/lib.rs:41-59`, `crates/aimee_test_kit/Cargo.toml:12-14`).

Callers already use it that way, for example `aimee_test_kit::fixture!("/src/fixtures/skills/with_name_and_description.md")` in `crates/aimee_repo/src/skill.rs:459`.

## Snapshots and nextest

`insta.yaml` at the repo root is the snapshot policy:

```yaml
test:
  auto_accept: true
  auto_accept_unseen: true
  runner: nextest
```

That is the whole file (`insta.yaml:1-4`). Auto-accept is intentional. The runner is **nextest**.

`.config/nextest.toml` sets the default profile: tests slower than `1s` are marked slow and terminated after 30 periods; both live status and the final summary show **fail** only (`.config/nextest.toml:1-10`).

Prefer:

```bash
cargo insta test --accept -p <crate>
```

over writing snapshot files by hand (`AGENTS.md:367`, `AIMEE.md:388`).

## Verify (Rust)

Run only what matches the crates you touched (`AGENTS.md:362-368`, `AIMEE.md:390-396`):

```bash
cargo fmt
cargo check -p <crate>
cargo clippy -p <crate> --all-targets -- -D warnings
cargo insta test --accept -p <crate>
```

Order of preference: `cargo check` > `cargo insta test` > `cargo build` (debug) (`AGENTS.md:372`).

Never:

- `cargo build --release` unless the task is a release binary or a **measured** benchmark (`AGENTS.md:370-371`, `AIMEE.md:399`).
- `--all-features` when a crate documents a broken matrix — then test the documented matrix (`AGENTS.md:373-374`).
- Commit `target/` (`AGENTS.md:391`).
- Claim tests passed unless you ran them (`AGENTS.md:28-29`).

CI sets `RUSTFLAGS=-D warnings`. Warnings are errors (`AGENTS.md:80`, `AIMEE.md:367`). Workspace MSRV is `rust-version = "1.94"`; the pin is `rust-toolchain.toml` (`1.97`). Edition is `2024` (`AGENTS.md:70-71`).

### TDD gate

`scripts/tdd-gate.sh` is the optional coverage gate, not a substitute for the verify matrix:

```bash
./scripts/tdd-gate.sh            # defaults to aimee_domain
./scripts/tdd-gate.sh aimee_app  # any workspace crate
```

It runs `cargo test -p "$pkg" --offline --lib`. If `cargo-llvm-cov` is installed, it then runs `cargo llvm-cov -p "$pkg" --lib --fail-under-lines 95`. If llvm-cov is missing, tests still must pass and coverage is skipped (`scripts/tdd-gate.sh:1-11`).

The `:tpl-tdd` command is the human prompt for the same cycle: red (failing test), green (minimal implementation), refactor with tests still green (`commands/tpl-tdd.md:1-13`).

The project-local `:check` command runs a broader pre-commit pair (`.aimee/commands/check.md:6-8`):

```text
cargo +nightly fmt --all
cargo +nightly clippy --fix --allow-staged --allow-dirty --workspace
cargo insta test --accept --unreferenced=delete
```

That is a **command** for agents in this repo, not the default verify matrix. Day-to-day crate work still uses `cargo fmt` / `check` / `clippy -D warnings` / `cargo insta test --accept -p <crate>`.

## TypeScript evals

Root `package.json` name is `aimee-codes-evals`. `"type": "module"`. The scripts that exist (`package.json:9-14`):

| Script | Command | Use when |
|---|---|---|
| `npm run eval` | `tsx benchmarks/cli.ts` | Running a `benchmarks/evals/*/task.yml` |
| `npm run test:bounty` | `tsx --test .github/scripts/bounty/tests/*.test.ts` | Bounty-harness unit tests |
| `npm run bounty:sync-issue` | `tsx .github/scripts/bounty/src/sync-issue.ts` | Issue sync (ops, not correctness) |
| `npm run bounty:sync-pr` | `tsx .github/scripts/bounty/src/sync-pr.ts` | PR sync (ops) |
| `npm run bounty:sync-all-issues` | `tsx .github/scripts/bounty/src/sync-all-issues.ts` | Bulk issue sync (ops) |

There is **no** `tsc` script in `package.json`. Typecheck is the documented fallback (`AGENTS.md:603-608`, `AIMEE.md:401`):

```bash
npx tsc --noEmit -p benchmarks/tsconfig.json
npm run eval ./evals/<name>/task.yml
# or, for bounty tests:
npm run test:bounty
```

`benchmarks/tsconfig.json` is `strict`, `nodenext`, `esnext` (`benchmarks/tsconfig.json:10-36`). Evals already depend on `tsx`, `typescript`, `zod`, `yaml`, `yargs`, `pino`, `ai`, `handlebars`, `csv-parse` (`package.json:17-35`, `AGENTS.md:564-566`). Use those before adding anything.

Eval shape (TypeScript island, `AGENTS.md:570-576`):

```ts
it("creates a user", () => {
  const fixture = userFixture({ age: 12, isHappy: true, name: "John" });
  const actual = createUser(fixture);
  const expected = { /* handwritten */ };
  expect(actual).toEqual(expected);
});
```

How to run an eval from `benchmarks/` (`benchmarks/README.md:23-28`):

```bash
npm run eval ./evals/create_skill/task.yml
LOG_LEVEL=debug npm run eval ./evals/create_skill/task.yml
```

Do not introduce Jest, Vitest, or a second TS runner next to `tsx` / `node:test` (`AGENTS.md:563`).

`scripts/benchmark.sh` is a **timing** harness around `target/debug/aimee` (10 iterations, optional `--threshold`). It is not the correctness suite. It runs `cargo build` (debug), not `--release` (`scripts/benchmark.sh:22-59`).

## Verification matrix

Run only what matches the files you touched, then widen if the change crosses a public contract, schema, or runtime boundary (`AGENTS.md:842-855`).

| Island | Format | Static | Test | Never |
|---|---|---|---|---|
| Rust crate | `cargo fmt` | `cargo check` + `clippy -D warnings` | `cargo insta test --accept -p <crate>` | `cargo build --release` |
| TypeScript evals | project formatter | `npx tsc --noEmit -p benchmarks/tsconfig.json` | `npm run eval` / `npm run test:bounty` | a second test runner |
| React UI (if present) | project formatter | `tsc` + eslint/biome | unit + e2e the project has; **browser-exercise the flow** | screenshot-only "proof" |

UI work that changes layout, routing, or client state is not done until the changed flow is exercised the way a user would (`AGENTS.md:856-859`).

## What not to do

| Don't | Do |
|---|---|
| Delete or skip a failing test to go green | Fix the implementation, or ask before changing the assertion |
| Field-by-field asserts | `assert_eq!(actual, expected)` on the whole value |
| `panic!` after `if let` | `expect("…")` with the reason |
| Tests in a separate file when neighbors use `#[cfg(test)]` | Colocate |
| Hand-rolled snapshot files | `cargo insta test --accept -p <crate>` |
| `cargo build --release` for correctness | `cargo check` / `cargo insta test` |
| Invent `npm run test` / `npm run lint` | Use `eval` and `test:bounty`, which exist |
| A second HTTP mock, assertion, or test crate | `pretty_assertions`, `insta`, `aimee_test_kit`, `tempfile`, `mockito` (`AGENTS.md:357`) |
| Claim green without quoting command output | Run the command; paste the failure |

## Related

- [Best practices](../best-practices.md) — operating contract and anti-patterns
- [Skills and commands](../skills.md) — `:check`, `:tpl-tdd`, `test-reasoning`
- Product policy: `AGENTS.md` (testing contract + verify matrix), `AIMEE.md` §14
