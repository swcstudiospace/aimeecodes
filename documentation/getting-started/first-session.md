# Your first flock session

A realistic walkthrough of one working session, from question to verified change. Commands shown are literal; the repository content in examples is illustrative.

## The setup

You have Aimee installed, a provider logged in (`aimee provider login`), and you're inside a Rust project with a flaky importer test.

## Step 1: Research before touching anything (Sage)

```zsh
: sage why does the importer test fail intermittently? check the timeout handling and the fixture loading
```

Sage is read-only. It searches your code, reads files, and answers — it cannot edit anything. Expect an answer that cites specific files and lines rather than a patch.

## Step 2: Turn findings into a plan (Muse)

```zsh
: muse plan a fix for the importer timeout handling
```

Muse writes a checkbox plan under `plans/`, for example `plans/2026-08-23-importer-timeout.md`:

```markdown
# Importer timeout fix

- [ ] Replace fixed 5s timeout with per-attempt budget
- [ ] Add retry with backoff on transient HTTP failures
- [ ] Cover both paths in tests
```

Open the file, edit checkboxes, reorder work. The plan is yours; Muse just drafts it.

## Step 3: Implement and verify (Aimee)

```zsh
: aimee implement the plan in plans/2026-08-23-importer-timeout.md
```

Aimee works through the checkboxes using its tools — reading files, applying patches, running shell commands:

```text
aimee · Implementing plans/2026-08-23-importer-timeout.md
  read    crates/importer/src/lib.rs
  patch   crates/importer/src/lib.rs        (+18 -6)
  shell   cargo test -p importer --lib       ✓ 42 passed
  todo    2 of 3 complete
```

When Aimee claims done, it reports evidence: which tests ran, which commands succeeded. If something failed, you'll see the failure, not a summary that hides it.

## Step 4: One-shot alternative

The same flow without the interactive TUI, for scripting or CI:

```bash
aimee -p "run cargo clippy -p importer and summarize any warnings"
```

## What you saw

| Role | Alias | Wrote anything? |
|---|---|---|
| Sage | `:ask` | No — research only |
| Muse | `:plan` | Plan files under `plans/` only |
| Aimee | `:act` | Code, patches, command execution |

Behind the scenes Aimee can dispatch specialists (frontend, backend, platform) through its `task` tool when a job needs one specialty.

## See also

* [The flock: Sage, Muse, Aimee](the-flock.md)
* [Three modes](modes.md)
* [Everyday workflows](../usage/workflows.md)

<!-- sources: AIMEE.md §3, README.md, commands/*.md -->
