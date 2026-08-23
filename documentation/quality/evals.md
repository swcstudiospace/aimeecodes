# Evals and benchmarks

`benchmarks/` is a TypeScript evaluation harness (`aimee-codes-evals` in the root `package.json`) that measures how well the `aimee` CLI — driven by real models — behaves on concrete coding tasks. It is separate from the Rust test suite ([Testing](testing.md)): Rust tests verify code correctness; evals verify **agent behavior**.

## Layout

```
benchmarks/
├── cli.ts                    # Entry point (npm run eval)
├── command-generator.ts      # Renders command templates
├── task-executor.ts          # Executes tasks with timeout support
├── model.ts                  # Task types
├── parse.ts                  # Argument parsing
├── utils.ts · verification.ts
└── evals/                    # One directory per evaluation
    ├── echo/ · commit_no_markdown/ · create_skill/
    ├── multi_file_patch/ · parallel_tool_calls/
    ├── patch_exact_match/ · read_over_cat/
    ├── redundant_cd_with_cwd/ · refactoring_uses_patch/
    └── search_over_find/ …
.github/scripts/bounty/       # Bounty automation (separate npm scripts)
```

## Running an evaluation

Prerequisite: an `aimee` symlink reachable from any directory, because tasks execute in temporary directories:

```bash
ln -sf $(pwd)/target/debug/aimee ~/bin/aimee
```

```bash
# Run one evaluation
npm run eval ./evals/create_skill/task.yml

# Debug logging
LOG_LEVEL=debug npm run eval ./evals/create_skill/task.yml
```

## Anatomy of a task

Each eval is a `task.yml`. The essential shape (abridged from [`evals/patch_exact_match/task.yml`](https://github.com/swcstudiospace/omegaloops/tree/main/benchmarks/evals)):

```yaml
run:                       # shell steps that build the fixture
  - mkdir -p test_files
  - |
    cat > test_files/plan.md << 'EOF'
    ...fixture content...
    EOF
  - |
    AIMEE_DEBUG_REQUESTS='{{dir}}/context.json' aimee --provider open_router \
      --model {{model}} -p '{{task}}'

parallelism: 2             # concurrent task slots
timeout: 180               # seconds per task
early_exit: true           # stop on first failure

validations:               # pass/fail checks over the captured context
  - name: "Should use patch tool"
    type: shell
    command: "jq -e '[.messages[]?.tool_calls[]? | select(.function.name == \"patch\")] | length > 0' {{dir}}/context.json"

sources:                   # the data matrix to run against
  - value:
      - model: "anthropic/claude-sonnet-4.5"
      - model: "z-ai/glm-4.6:exacto"
      - model: "minimax/minimax-m2.1"
  - csv: patch_exact_match_tasks.csv   # per-row {{task}} inputs
```

Key mechanics:

- **Template variables**: `{{dir}}` (task temp dir), `{{model}}`, `{{task}}` are rendered into commands via Handlebars.
- **Captured context**: setting `AIMEE_DEBUG_REQUESTS={{dir}}/context.json` makes the CLI dump its full message/tool-call transcript, which validations then query with `jq`.
- **Sources** form a matrix: inline model lists and CSV row sets are cross-producted.
- **Validations** are shell commands (exit 0 = pass) — regex assertions over output, or structural assertions over `context.json` ("did the agent call `patch`, did it avoid `cat`", etc.).
- Timestamped debug artifacts land under each eval's `debug/` directory.

The eval suite encodes tool-contract expectations as behavior tests: `read_over_cat` and `search_over_find` check that agents prefer dedicated tools; `redundant_cd_with_cwd` enforces the [shell](../reference/tools/shell.md) `cwd` contract; `parallel_tool_calls` verifies batching; the patch family validates exact-match editing discipline.

## Bounty automation

Root `package.json` also carries GitHub bounty sync scripts (used by `.github/workflows/bounty.yml`):

```bash
npm run test:bounty              # node:test suite for bounty scripts
npm run bounty:sync-issue        # issue → bounty state
npm run bounty:sync-pr           # PR → bounty state
npm run bounty:sync-all-issues
```

## Conventions

- The harness stays **ESM** (`"type": "module"`, run with `tsx`). Reuse existing helpers before adding dependencies.
- Typecheck after changes: `npx tsc --noEmit -p benchmarks/tsconfig.json`.
- New eval = one directory with `task.yml` (+ optional CSV) under `benchmarks/evals/`.

## Related

- [Testing](testing.md) — the Rust-side contract (fixture → actual → expected)
- [Tool catalog](../reference/tools/catalog.md) — what the behavioral evals assert about
- [CI/CD](../ops/cicd.md) — where these run automatically
