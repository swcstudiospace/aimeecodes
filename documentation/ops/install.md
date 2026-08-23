# Install and Nix

How to install the `aimee` CLI from the product tree. This page is grounded in `aimeecodes`. It does not invent release URLs, Homebrew formulae, or npm package names beyond what the tree names.

The CLI binary is **`aimee`** from `crates/aimee_main`. Workspace version is `0.1.0` (`Cargo.toml:7`). Live GitHub remote is [swcstudiospace/omegaloops](https://github.com/swcstudiospace/omegaloops) (`Cargo.toml:13`, `README.md:390`).

## Quickstart

From `README.md:53-63`:

```bash
# Nix (Linux and macOS: x86_64 and aarch64)
nix run github:swcstudiospace/omegaloops

# From a local checkout
cargo install --path crates/aimee_main --locked --bin aimee

aimee provider login    # interactive provider credentials
aimee                   # start the TUI
aimee setup             # optional: install the ZSH `:` prefix plugin
```

On first run, Aimee walks you through provider login if no credentials are stored (`README.md:65`). Config lives in `~/.aimee`. Existing `~/.omega` directories are still picked up until you migrate.

## Nix

The flake description and homepage match the live GitHub origin (`flake.nix:2`, `flake.nix:91`).

### Run without a checkout

```bash
nix run github:swcstudiospace/omegaloops
```

That is the default flake app. It points at `${self.packages.${system}.default}/bin/aimee` (`flake.nix:104-112`). Systems: `x86_64-linux`, `aarch64-linux`, `x86_64-darwin`, `aarch64-darwin` (`flake.nix:10-15`).

### Dev shell

```bash
nix develop
```

The default `devShell` includes `cargo`, `clippy`, `rustc`, `rustfmt`, `rust-analyzer`, `cargo-insta`, `cargo-llvm-cov`, `protobuf`, `sqlite`, `cmake`, `nasm`, `perl`, and `pkg-config` (`flake.nix:115-156`). It sets `PROTOC`, `PROTOC_INCLUDE`, and `APP_VERSION = "0.1.0-dev"`.

The package build uses `pkgs.rustPlatform.buildRustPackage` with `cargoBuildFlags` / `cargoInstallFlags` of `-p aimee_main --bin aimee` (`flake.nix:47-58`). `doCheck = false` (`flake.nix:87`). Package version in the flake is `0.1.0-dev` (`flake.nix:39`).

`README.md:364` restates both entry points: `nix run github:swcstudiospace/omegaloops` or `nix develop`.

## Cargo install from a checkout

```bash
cargo install --path crates/aimee_main --locked --bin aimee
```

`--locked` uses `Cargo.lock`. `--bin aimee` is the only binary the README documents for this path (`README.md:58`).

`crates/aimee_main/build.rs:7-16` injects `CARGO_PKG_VERSION` from `APP_VERSION` when set (CI/CD), otherwise `0.1.0-dev`.

## Toolchain pin vs MSRV

Two different numbers. Do not collapse them.

| Kind | Value | Source |
|---|---|---|
| Pinned toolchain | `1.97` | `rust-toolchain.toml:1-3` (`channel = "1.97"`, `profile = "default"`) |
| MSRV | `1.94` | workspace `rust-version` (`Cargo.toml:8`) |
| Edition | `2024` | `Cargo.toml:9` |
| Workspace version | `0.1.0` | `Cargo.toml:7` |

`README.md:51` and `AIMEE.md:18` state the same split: pinned toolchain Rust `1.97`, MSRV `1.94`.

`rustup` will use `rust-toolchain.toml` in a checkout. CI's `build` job currently requests `toolchain: stable` (`aimeecodes/.github/workflows/ci.yml:49-52`). Autofix uses nightly for `fmt` / `clippy` (`aimeecodes/.github/workflows/autofix.yml:47-57`).

## First-run provider login

Credentials are stored under the config base path as `.credentials.json`. Do not put API keys in git (`README.md:262`).

Interactive login:

```bash
aimee provider login            # picker when no provider is named
aimee provider login <id>       # specific ProviderId
aimee provider logout
aimee provider list
```

`aimee provider login` is `ProviderCommand::Login` (`crates/aimee_main/src/cli.rs:980-987`). With no argument it fetches providers and opens a picker; with an ID it configures that provider (`crates/aimee_main/src/ui.rs:1268-1300`). Re-auth is allowed even when already configured (`crates/aimee_main/src/ui.rs:1289-1291`).

On first interactive session, `init_state` migrates env credentials, then if there is no session config it prompts for provider selection (`crates/aimee_main/src/ui.rs:4584-4596`). That is the code behind the README first-run sentence.

`aimee provider list` is the source of truth for built-in provider IDs (`README.md:264`). There are 42 built-in IDs (`ProviderId::built_in_providers()`).

Env keys are no longer the long-term store. `init_state` calls `migrate_env_credentials` and, if anything moved, warns that Aimee no longer reads API keys from environment variables (`crates/aimee_main/src/ui.rs:5911-5932`).

## `aimee setup` / `doctor` / `update`

### `aimee setup`

Top-level `Setup` is an alias for `zsh setup` (`crates/aimee_main/src/cli.rs:155-157`). It updates `.zshrc` with the plugin and theme.

The handler asks whether Nerd Fonts render, optionally writes `NERD_FONT=0`, then offers an editor for `AIMEE_EDITOR` (`crates/aimee_main/src/ui.rs:2019-2096`). It writes between `# >>> aimee initialize >>>` and `# <<< aimee initialize <<<` in `$ZDOTDIR/.zshrc` (or `$HOME/.zshrc`) and backs up an existing file (`crates/aimee_main/src/zsh/plugin.rs:249-350`). Then it runs doctor and tells you to `exec zsh` and try `: Hi` (`crates/aimee_main/src/ui.rs:2109-2119`).

Equivalent: `aimee zsh setup` (`crates/aimee_main/src/cli.rs:626-627`).

### `aimee doctor`

Top-level `Doctor` is an alias for `zsh doctor` (`crates/aimee_main/src/cli.rs:159-160`). It streams `shell-plugin/doctor.zsh` (`crates/aimee_main/src/ui.rs:1968-1976`, `crates/aimee_main/src/zsh/plugin.rs:175-183`).

`README.md:187`: shell diagnostics are `aimee doctor`; keyboard shortcuts are `aimee zsh keyboard`.

### `aimee update`

```bash
aimee update
aimee update --no-confirm
```

`--no-confirm` skips the confirmation prompt (`crates/aimee_main/src/cli.rs:1080-1086`). The UI maps that flag to `Update.auto_update` (`crates/aimee_main/src/ui.rs:818-821`).

What the updater actually does (`crates/aimee_main/src/update.rs:73-97`):

1. Skip if `[updates].frequency` is `never`.
2. Skip if `VERSION` contains `dev` or equals `0.1.0` (current workspace version — so a checkout of `0.1.0` will not self-update).
3. Ask GitHub via `update_informer` for registry `aimeecodes/aimeecodes`.
4. If a newer version exists and you confirm (or `auto_update` is on), run:

   ```bash
   curl -fsSL https://aimeecodes.dev/cli | sh
   ```

That is the only install URL the updater hard-codes (`crates/aimee_main/src/update.rs:16`). This page does not claim that URL is currently serving a binary.

Embedded defaults (`crates/aimee_config/.aimee.toml:68-70`):

```toml
[updates]
auto_update = true
frequency = "daily"
```

Frequencies are `daily`, `weekly`, `never`, `always` (`crates/aimee_config/src/compact.rs:13-19`). The type default when unset is `Always` (`crates/aimee_config/src/compact.rs:17-18`). Interactive first init also calls `on_update` with `config.updates` (`crates/aimee_main/src/ui.rs:4611-4612`).

## Compat config migrate

Primary config file: `~/.aimee/.aimee.toml` (`README.md:268`).

Base-path resolution (`AIMEE_CONFIG` wins, then `OMEGA_CONFIG`) (`README.md:270-276`, `crates/aimee_config/src/reader.rs:56-86`):

1. `AIMEE_CONFIG` if set
2. `OMEGA_CONFIG` if set
3. First existing home candidate: `~/aimee`, `~/.aimee`, `~/omega`, `~/.omega`, `~/forge`, `~/.forge`
4. Otherwise `~/.aimee`

A leftover `.omega.toml` in the same directory is loaded before `.aimee.toml` (`crates/aimee_config/src/reader.rs:145-153`). Legacy `~/.aimee/.config.json` is still read (`crates/aimee_config/src/reader.rs:156-165`). `AIMEE_`-prefixed env vars map onto `.aimee.toml`; legacy `OMEGA_` vars are still read (`crates/aimee_config/src/reader.rs:105-124`).

To rename a legacy directory onto the new default:

```bash
aimee config migrate
```

That is `ConfigCommand::Migrate` (`crates/aimee_main/src/cli.rs:772`). It looks for `~/aimee`, `~/.omega`, or `~/omega`, then `rename`s the first hit to `~/.aimee` (`crates/aimee_main/src/ui.rs:5199-5240`). It errors if none of those exist, or if `~/.aimee` already exists.

`aimee config path` prints the resolved global config file path (`crates/aimee_main/src/ui.rs:5188-5190`).

`AIMEE.md:43-50` lists the same compat surfaces. Do not delete them.

## Cloud endpoints

Workspace/indexing URL, Vertex ADC, and Bedrock profile login are documented on [Cloud and services](cloud.md). They are **user provider logins**, not Spectrum tenancy.

## What this page does not claim

- The README does not document `brew install` or `npm i -g` as user install paths. Release-channel repo names live on the [CI/CD](cicd.md) page.
- `nix run` / `cargo install` are the documented ways to get a local `aimee` binary.
- Do not commit `.credentials.json`, `.env`, or `target/`.

## Verify

From a product checkout:

```bash
nix develop
cargo check -p aimee_main
aimee --version
aimee provider list
aimee config path
aimee doctor
```

Development commands from `README.md:357-361`:

```bash
cargo fmt
cargo check -p aimee_main
cargo clippy -p aimee_main --all-targets -- -D warnings
cargo insta test --accept -p aimee_main
```
