---
name: rust-workspace-rebrand
description: "Use when rebranding a Rust workspace (crates, bins, types)."
version: 1.0.0
author: Hermes Agent
license: MIT
platforms: [linux, macos, windows]
metadata:
  hermes:
    tags: [rust, rebrand, rename, cargo, workspace, crates, branding]
    related_skills: [rust-cli-binary-rename, ratatui-agent-tui, hermes-project-soul]
---

# Rust workspace / product rebrand

## Overview

Rename **crates, folders, types, env vars, config paths, slash commands, and
the CLI binary** together when a Rust workspace is the base for a new product.
Broader than `rust-cli-binary-rename` (command only) and `ratatui-agent-tui`
(TUI strings only).

Grounded in omegaloops: Forge Code → **Omega Loops** (`forge` → `omega`,
`forge_*` crates → `omega_*`, `:forge` → `:omega`).

## When to Use

- User asks to change forge/old-brand → new brand **in files, folders, and code**
- Product rename that includes crate prefix + binary + home dir + env prefix
- Slash/colon agent commands named after the old product

Don't use for: command-only cut that keeps crate/home → `rust-cli-binary-rename`;
TUI banner/palette only → `ratatui-agent-tui`.

## Scope decisions

| Usually rename | Keep unless asked |
|----------------|-------------------|
| `[[bin]] name` + clap `about` | Historical `plans/` changelog archaeology |
| Crate dirs + workspace.dependencies | Third-party tokens (`forget`, `forged`) |
| `OldType` / `old_mod` identifiers | External npm/GitHub orgs you do not own |
| `OLD_*` env, `.old` / `~/old` paths | `target/` (regenerate) |
| Slash aliases (`:old` → `:new`) | |
| Templates, proto package, insta snap names | |

Keep a **legacy path fallback**: still read `~/old` and `~/.old` if they exist,
default new installs to `~/.new`.

## Procedure

### 1. Inventory (exclude `target/` and `.git/`)

List crate dirs, `[[bin]]`, env prefixes, config filenames, proto package,
agent ids, reserved slash commands, snapshot filenames.

Done when: you have OLD→NEW maps for crate prefix, binary, type prefix,
env prefix, hidden dir, agent id.

### 2. Two-pass content replace, then path rename

**Pass 1 — longest literals, then word-boundary tokens:**

```
Old Product → New Product
old-product.dev → new-product.dev
OLD_ → NEW_
old_ → new_
.old → .new
\bOLD\b → NEW
\bOld\b → New
\bold\b → new
```

`\bOld\b` does **not** match `OldConfig` / `OldServices` (next char is a
word character).

**Pass 2 — PascalCase type prefix (plain string replace):**

```
Old → New     # OldConfig → NewConfig, OldAPI → NewAPI
```

Safe because `forget`/`forged` are lowercase. Do **not** run a lowercase
ungarded `old` → `new` (breaks `forget`).

Then rename files/dirs **deepest first** (`old_foo.rs` then `old_foo/`).
Skip `target/`.

Done when: `rg '\bold\b'` over source (no target/plans) is empty except
intentional legacy fallbacks.

### 3. Workspace glob hygiene

If `Cargo.toml` has `members = ["crates/*"]`, every directory under `crates/`
must have a `Cargo.toml`. Empty stub dirs (`crates/new_foo/src/…` with no
manifest) fail the whole workspace load.

Done when: `ls crates/*/Cargo.toml` matches `ls -d crates/*`.

### 4. Snapshots, proto, clap

- Insta snap **filenames** embed the crate module path
  (`old_app__foo.snap` → `new_app__foo.snap`)
- Proto `package old.v1` + `include_proto!("new.v1")` + generated client
  (`OldServiceClient` → `NewServiceClient`)
- Clap `try_parse_from(["new", …])`, reserved-command lists, agent `id:`

### 5. TUI + docs

Banner/theme via `ratatui-agent-tui`. README product name, install command,
and config paths. Roadmap (PWA/WEB3) only if the user asked — do not build
the PWA in the same cut.

### 6. Verify (slice, then workspace)

```bash
cargo check --workspace
cargo build -p NEW_MAIN --bin NEWNAME
./target/debug/NEWNAME --help    # Usage: NEWNAME
./target/debug/NEWNAME banner    # if present
cargo test -p NEW_MAIN --lib -- banner theme highlighter
rg -n --glob '!target/**' --glob '!plans/**' -e 'name = "old"' -e 'Usage: old'
```

Done when: help Usage is NEWNAME; workspace check loads; leftover brand
scan is only documented legacy paths.

## Pitfalls

1. **Word-boundary misses PascalCase** — `\bForge\b` leaves `ForgeConfig`.
   Always run a second `Old` → `New` prefix pass.
2. **`forget` / `forged`** — never unguarded lowercase `old` → `new`.
3. **Empty `crates/*` dirs** — workspace glob requires a manifest in every
   member directory.
4. **ratatui 0.29 vs nucleo-picker** — 0.29 pins `unicode-width =0.2.0`;
   nucleo-picker wants `^0.2.2`. Use ratatui **0.30+**.
5. **Insta snap names** — crate rename without renaming `*_oldcrate__*.snap`
   files fails snapshot lookup.
6. **Dropping legacy homes** — existing `~/old` / `~/.old` users lose config
   unless resolve order keeps those candidates.
7. **Scanning `target/`** — stale `old.v1.rs` build output looks like a miss;
   ignore it, regenerate via cargo.
8. **Verification `head -8` on banner** — agent tips sit below the art;
   search the full `banner` / `--help` output.

## Verification checklist

- [ ] `[[bin]] name` + `--help` Usage == NEWNAME
- [ ] All `crates/*` have Cargo.toml
- [ ] `OldType` / `old_mod` / `OLD_` gone except legacy fallbacks
- [ ] Proto package + `include_proto!` + client type match
- [ ] Insta snap filenames use new crate module prefix
- [ ] Config resolve: NEW paths first, OLD homes if they exist
- [ ] `cargo check --workspace` + targeted banner/theme tests

## References

- `references/omegaloops-forge-cut.md` — Forge Code → Omega Loops path map
