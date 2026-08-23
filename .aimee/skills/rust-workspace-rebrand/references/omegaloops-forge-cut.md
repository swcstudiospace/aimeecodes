# Omega Loops cut (Forge Code → Omega Loops)

Repo: `/root/src/repos/omegaloops`. Product **Omega Loops**, CLI **`omega`**.

## Maps

| Kind | Old | New |
|------|-----|-----|
| Product | Forge Code / ForgeCode | Omega Loops / OmegaLoops |
| Binary | `forge` | `omega` |
| Crate prefix | `forge_*` | `omega_*` |
| Types | `ForgeConfig`, `ForgeAPI` | `OmegaConfig`, `OmegaAPI` |
| Env | `FORGE_*` | `OMEGA_*` |
| Home | `~/forge`, `~/.forge` | `~/.omega` (legacy still read) |
| Config | `.forge.toml`, `forge.yaml` | `.omega.toml`, `omega.yaml` |
| Agent / slash | `id: forge`, `:forge` | `id: omega`, `:omega` (`:act` alias) |
| Proto | `package forge.v1` | `package omega.v1` |
| Site | forgecode.dev | omegaloops.dev |

25 crates under `crates/omega_*`. Binary crate `omega_main`, `[[bin]] name = "omega"`.

## Two-pass script shape

1. Literal + word-boundary replace on text files (skip `target/`, binaries).
2. Plain `Forge` → `Omega` for PascalCase types.
3. Rename paths deepest-first (`forge_foo.rs` then `crates/forge_foo`).

Do not unguarded-replace lowercase `forge` inside `forget`.

## Config resolve order (keep)

`OMEGA_CONFIG` → `~/omega` → `~/.omega` → `~/forge` → `~/.forge` → default `~/.omega`.

## TUI

- Banner: pyfiglet `smslant` “OMEGA LOOPS” (~58×4).
- Live splash is ratatui `Buffer` → truecolor ANSI (no alt-screen); chips `:omega` / `:muse` / `:sage`.
- Palette in `crates/omega_main/src/theme.rs` (cyan/violet/magenta/gold/lime).
- ratatui **0.30** (0.29 conflicts with nucleo-picker on `unicode-width`).
- Details: skill `ratatui-agent-tui` → `references/rustyline-splash.md`.

## Verify

```bash
cargo check --workspace
./target/debug/omega --help    # Usage: omega
./target/debug/omega banner    # :omega + WEB3 tagline
cargo test -p omega_main --lib -- banner theme highlighter
```
