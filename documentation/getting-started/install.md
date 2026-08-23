# Installing

Three supported paths: Nix, Cargo from a checkout, or an existing package channel. Requirements: Rust 1.97 toolchain (pinned in `rust-toolchain.toml`) if you build yourself; nothing but Nix if you don't.

## Nix (Linux and macOS)

The flake builds the `aimee` binary for `x86_64-linux`, `aarch64-linux`, `x86_64-darwin`, and `aarch64-darwin`.

Run without installing:

```bash
nix run github:swcstudiospace/omegaloops
```

Add to your profile:

```bash
nix profile install github:swcstudiospace/omegaloops#aimee
```

## From a local checkout

```bash
git clone https://github.com/swcstudiospace/omegaloops
cd omegaloops
cargo install --path crates/aimee_main --locked --bin aimee
```

`--locked` respects `Cargo.lock`. The workspace MSRV is 1.94; the pinned toolchain is 1.97.

## Package channels

| Channel | Source |
|---|---|
| Homebrew | tap `antinomyhq/homebrew-aimee-codes` |
| NPM release matrix | `swcstudiospace/npm-aimee-codes` |
| GitHub releases | [swcstudiospace/omegaloops](https://github.com/swcstudiospace/omegaloops) |

## First run

On first launch Aimee checks for stored credentials and walks you through provider login when none exist:

```bash
aimee provider login
```

Credentials are written to `.credentials.json` under your config base directory. Never commit this file.

## Where things live

| Path | Purpose |
|---|---|
| `~/.aimee` | Config base directory |
| `~/.aimee/.aimee.toml` | Primary configuration file (schema: [aimee.schema.json](../reference/schema.md)) |
| `~/.aimee/.credentials.json` | Provider credentials — keep out of git |
| `~/.aimee/agents/` | Global custom agents |
| `~/.aimee/skills/` | Global skills |
| `~/.aimee/.mcp.json` | Global MCP servers |

The base directory resolves in this order: `$AIMEE_CONFIG`, then legacy `$OMEGA_CONFIG`, then whichever of `~/aimee`, `~/.aimee`, `~/omega`, `~/.omega` exists (Forge-legacy `~/forge` / `~/.forge` after that), defaulting to `~/.aimee`.

## Coming from Omega Loops

Existing `~/.omega` installs keep working; Aimee detects them automatically. Consolidate onto the new layout with:

```bash
aimee config migrate
```

Full mapping: [Migrating from Omega Loops](../help/migration.md).

## Updating

```bash
aimee update    # self-update to the latest release
```

## See also

* [Quickstart](quickstart.md)
* [Nix and reproducible installs](../integrations/nix.md)
* [Dev Container](../integrations/devcontainer.md)

<!-- sources: README.md, flake.nix, AIMEE.md §2,§6, crates/aimee_config/src/reader.rs -->
