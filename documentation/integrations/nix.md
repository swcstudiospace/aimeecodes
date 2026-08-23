# Nix and reproducible installs

The repository ships a Nix flake as the canonical reproducible build of the `aimee` binary.

## The flake

Defined at `flake.nix` against `nixpkgs-unstable`, building for four systems:

* `x86_64-linux`
* `aarch64-linux`
* `x86_64-darwin`
* `aarch64-darwin`

Outputs:

| Output | Purpose |
|---|---|
| `packages.<system>.default` / `.aimee` | The `aimee` binary |
| `apps.<system>.default` / `.aimee` | Runnable app entry (`nix run`) |
| `formatter` | `nixfmt-rfc-style` for flake formatting |
| `devShells` | Development shell with workspace tooling |

Source filtering (`cleanSourceWith`) keeps builds to what the crate actually needs.

## Running and installing

```bash
# run without installing
nix run github:swcstudiospace/omegaloops

# install into your profile
nix profile install github:swcstudiospace/omegaloops#aimee

# development shell
nix develop github:swcstudiospace/omegaloops
```

## Why Nix here

The flake pins the entire dependency graph — compiler, crates, native libs — so the binary you get is byte-for-byte reproducible per revision. For teams standardizing on Aimee, `nix profile install` with a locked revision gives every engineer the same version without package-manager drift.

## Versioning notes

The flake's homepage points at the GitHub origin (`swcstudiospace/omegaloops`). When that repository is renamed, the flake homepage, `Cargo.toml` `repository`, README badges, and eval clone URLs must change together — house rule from AIMEE.md §17.

## See also

* [Installing](../getting-started/install.md)
* [Dev Container](devcontainer.md)
* [CLI reference](../reference/cli.md)

<!-- sources: flake.nix, README.md, AIMEE.md §2,§16,§17 -->
