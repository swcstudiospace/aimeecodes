# Quickstart

From zero to a working flock in about two minutes.

## 1. Install

```bash
# Nix (Linux and macOS, x86_64 and aarch64)
nix run github:swcstudiospace/omegaloops

# Or from a local checkout
cargo install --path crates/aimee_main --locked --bin aimee
```

More options and details: [Installing](install.md).

## 2. Log in to a provider

```bash
aimee provider login
```

This walks you through authenticating with a model provider interactively. Credentials are stored under `~/.aimee` as `.credentials.json` — outside git. To see every provider ID first:

```bash
aimee provider list
```

## 3. Launch

```bash
aimee          # interactive TUI
aimee -p "summarize this repository"   # one-shot, then exit
```

If `~/.omega` exists from an earlier Omega Loops install, Aimee picks it up automatically. Move everything to the new location when ready:

```bash
aimee config migrate
```

## 4. Optional: the ZSH prefix

```bash
aimee setup    # wires the plugin into .zshrc
exec zsh       # or open a new shell
```

Then drive the flock without leaving your prompt:

```zsh
: sage how does the retry logic handle transient HTTP failures?
: muse plan a fix for the flaky importer test
: aimee implement the plan in plans/2026-08-23-fix-importer.md
```

## What to read next

* [Your first flock session](first-session.md) — a full walkthrough of one session
* [The : prefix (ZSH)](../usage/zsh-prefix.md) — file tagging, completion, diagnostics
* [CLI reference](../reference/cli.md) — every command

## See also

* [Installing](install.md)
* [Providers and model access](../integrations/providers.md)
* [Troubleshooting](../help/troubleshooting.md)

<!-- sources: README.md, AIMEE.md §16, crates/aimee_main/src/cli.rs -->
