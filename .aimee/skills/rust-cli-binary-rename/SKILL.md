---
name: rust-cli-binary-rename
description: "Use when renaming a Rust CLI binary across releases."
version: 1.0.0
author: Hermes Agent
license: MIT
platforms: [linux, macos, windows]
metadata:
  hermes:
    tags: [rust, cli, rename, cargo, release, homebrew, installer, branding]
    related_skills: [hermes-project-soul, ratatui-agent-tui, sveltekit-marketing-landing, github-pr-workflow]
---

# Rust CLI binary rename (command name)

## Overview

Rename the **user-facing CLI command** for a Rust package without necessarily renaming the crate/package, home directory, or launcher. Coordinated change across Cargo `[[bin]]`, updater asset names, install scripts, CI release artifacts, launcher discovery, clap help, and docs/i18n.

Grounded in anda-bot: `anda` → **`autonogrammer`** (package still `anda_bot`, home still `~/.anda`, launcher still `anda_launcher`).

## When to Use

- User asks to change the CLI command / binary name (e.g. `anda` → `autonogrammer`)
- Full or partial product rebrand that includes the installable command
- Release assets / Homebrew / installers still ship the old binary name

Don't use for: TUI-only string rebrand → `ratatui-agent-tui`; marketing site dual-brand only → `sveltekit-marketing-landing`; crate/folder/type rebrand → `rust-workspace-rebrand`.

## Scope decisions (ask or default)

| Keep stable (default unless asked) | Rename with the command |
|------------------------------------|-------------------------|
| Cargo **package** name (`anda_bot`) | `[[bin]] name` + `default-run` |
| Data home (`~/.anda`, `ANDA_HOME`) | Updater `BINARY_NAME` constant |
| Secondary bins (`anda_launcher`) | Release artifact prefix (`autonogrammer-$target`) |
| Skills zip / internal agent handles | Install scripts `BINARY_NAME` |
| Store IDs, plist labels (unless asked) | Launcher sibling/PATH discovery |
| | Clap `about` / examples / status strings |
| | README, docsite all locales, AGENTS/SOUL |

Optional soft migration: installers create symlink `oldname` → `newname` for one release. Only if user wants it.

## Procedure

### 1. Cargo binary

```toml
default-run = "NEWNAME"

[[bin]]
name = "NEWNAME"
path = "src/main.rs"
```

Package `name` stays unless full crate rename.

Done when: `cargo build -p PKG --bin NEWNAME` succeeds; `./target/debug/NEWNAME --help` shows `Usage: NEWNAME`.

### 2. In-tree BINARY_NAME / asset builders

Find constants like `BINARY_NAME`, `asset_name()`, release target helpers. They must emit:

`{NEWNAME}-{os}-{arch}{exe_ext}`

**Critical:** CI release copy and updater download **must use the same prefix**. If only Cargo bin changes, updates break in production.

Update unit tests that hardcode old asset strings (`"old-linux-x86_64"` → `"new-linux-x86_64"`).

### 3. Install & package scripts

| Script class | What to change |
|--------------|----------------|
| `install.sh` / `install.ps1` | `BINARY_NAME=newname` (asset + install name derive from it) |
| Homebrew publish | URL/sha asset names, `bin.install binary => "newname"`, caveats/`--version` test |
| Windows installer staging | `newname.exe` paths, Start-HiddenProcess target |
| GitHub `release.yml` | `cp .../release/NEWNAME$ext release/NEWNAME-${target}$ext` |

Leave `LAUNCHER_BINARY_NAME` alone unless renaming the launcher too.

### 4. Launcher / companion process discovery

Any code that looks beside itself for the main binary:

- Sibling: `parent.join("newname")` / `newname.exe`
- Fallbacks: `~/.local/bin/newname`, Homebrew prefixes
- Tests that write temp `join("old")` fixtures

### 5. Clap UX

- `about` / `after_help` examples use `NEWNAME`
- Subcommand docs: “Equal to running \`NEWNAME restart\`”
- Runtime status lines users see (`NEWNAME daemon is running…`)
- `Cli::try_parse_from(["NEWNAME", ...])` in tests

### 6. Docs & agent orientation

Bulk-replace **command invocations** carefully:

- Prefer replacing `` `old` `` and `` `old status` `` forms, not package path `anda_bot` or home `.anda`
- Negative lookarounds so `anda_launcher`, `anda-bot` repo URL, `anda_bot` crate stay intact
- All docsite i18n locales + README EN/CN + AGENTS/SOUL
- Website unstuck/copy that shells out `` `old status` ``

### 7. Verify

```bash
cargo build -p PKG --bin NEWNAME
./target/debug/NEWNAME --help   # Usage: NEWNAME
cargo test -p PKG -- release_target_matches asset_name status_report parse
make lint
make test   # full suite when rename touches daemon strings
```

Done when: help Usage line is NEWNAME; asset-name unit test expects new prefix; install script BINARY_NAME matches; docs search for old command forms is clean (excluding changelog history if desired).

## Pitfalls

1. **Cargo bin without release/updater** — users install NEWNAME but `update` downloads OLD assets (or vice versa).
2. **Greedy replace** — rewriting `anda_bot`, `anda-bot`, `anda_launcher`, `.anda` home. Always protect those tokens.
3. **Launcher still spawns `old`** — tray/menu bar starts wrong binary.
4. **Homebrew formula name vs bin** — formula can stay `anda` while `bin.install => "autonogrammer"` if desired; document clearly.
5. **Windows staging** — installer copies must match `NEWNAME.exe`.
6. **Leaving clap parse tests on old argv0** — update `try_parse_from(["old", ...])`.
7. **Changelog archaeology** — optional to leave historical entries; don’t “fix” past release notes unless asked.

## Verification checklist

- [ ] `default-run` and `[[bin]] name` == NEWNAME
- [ ] Updater BINARY_NAME == NEWNAME; asset unit tests green
- [ ] release.yml copies NEWNAME artifacts
- [ ] install.sh / install.ps1 / Homebrew / Windows installer aligned
- [ ] Launcher discovers NEWNAME
- [ ] `--help` Usage line is NEWNAME
- [ ] Docs/i18n command forms updated without breaking package paths
- [ ] `make lint` + targeted (or full) tests pass

## References

- `references/anda-bot-autonogrammer-rename.md` — concrete path list for the anda → autonogrammer cut
