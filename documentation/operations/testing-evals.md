# Testing and evals

The testing contract for the Rust workspace and the TypeScript eval harness — what "verified" means in this repo.

## The Rust contract

Every test has three named steps — **fixture → actual → expected** — asserting on the whole value:

```rust,ignore
use pretty_assertions::assert_eq;

#[cfg(test)]
mod tests {
    fn test_parses_config() {
        let fixture = Config::test();               // generic, reusable constructors
        let actual = fixture.resolve_base_dir();
        let expected = Some(PathBuf::from("/home/u/.aimee"));
        assert_eq!(actual, expected);
    }
}
```

Rules: tests live in the same file as source (`#[cfg(test)]`); `unwrap` freely inside test functions; `anyhow::Result` for fixtures; prefer full-value equality over field-by-field asserts; `expect("…")` with a real message beats bare `panic!`.

## Snapshots

Snapshot tests use **insta** (`insta.yaml` auto-accepts; runner is nextest):

```bash
cargo insta test --accept -p <crate>
```

Shared fixtures load through `aimee_test_kit` (`fixture!` / `json_fixture!`) when a crate depends on it.

## Verify loop (what agents run)

```bash
cargo fmt
cargo check -p <crate>
cargo clippy -p <crate> --all-targets -- -D warnings   # CI treats warnings as errors
cargo insta test --accept -p <crate>
```

Never `cargo build --release` for verification — it's slow and proves nothing about correctness.

## TypeScript evals

The eval/bounty island is ESM Node (`tsx`, node:test) rooted at `package.json`:

```bash
npx tsc --noEmit          # typecheck first
npm run eval              # benchmark/eval suite (benchmarks/)
npm run test:bounty       # bounty tests (.github/scripts/bounty/tests/)
```

Eval tasks declare their setup under `benchmarks/evals/*/task.yml`. Use the existing helpers in those trees; don't introduce a second test runner.

## When a test fails

If the task is "fix a failing test", decide first whether to fix the implementation or update the test — that's an explicit question to ask, not an assumption. Then follow root causes, not symptoms.

## See also

* [CI/CD of Aimee itself](cicd.md)
* [Crate map](../architecture/crates.md)
* [Reliability](reliability.md)

<!-- sources: AGENTS.md testing contract + Rust playbook, package.json, AIMEE.md §14 -->
