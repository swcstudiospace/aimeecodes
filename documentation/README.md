# Aimee Codes documentation

Welcome to the official documentation for **Aimee Codes** (`aimee`) — a WEB3-aware, terminal-native coding agent for design engineers and backend engineers. Sage researches. Muse plans. Aimee implements and verifies.

Aimee Codes is developed and maintained by **Spectrum Web Co LLC** (brand **@swcstudio**). Formerly Omega Loops.

| | |
|---|---|
| Command | `aimee` (`crates/aimee_main`) |
| Surfaces | TUI · one-shot CLI · ZSH `:` prefix · PWA |
| Config | `~/.aimee/.aimee.toml` (schema: `aimee.schema.json`) |
| Services API | `https://api.aimeecodes.dev/` |
| License | Apache-2.0 (Spectrum Web Co LLC) |
| GitHub | [swcstudiospace/omegaloops](https://github.com/swcstudiospace/omegaloops) |
| Studio | [swcstudio.space](https://www.swcstudio.space/) |
| Contact | [ovesheng@spectrumweb.co](mailto:ovesheng@spectrumweb.co) |

## Start here

1. [Quickstart](quickstart.md) — install, log in to a provider, first run
2. [The flock](flock.md) — Sage / Muse / Aimee and what each one does
3. [How to use](howto.md) — everyday workflows: research → plan → implement
4. [CLI reference](cli.md) — every command and flag
5. [Configuration](configuration.md) — `.aimee.toml`, env vars, credentials

Going deeper:

- **Architecture** — how the 27-crate workspace fits together ([overview](architecture/overview.md))
- **Tools** — the 16 built-in tools the agent can call ([catalog](reference/tools/catalog.md), then one page per tool)
- **Providers** — 42 built-in model providers and how login works ([providers.md](providers.md))
- **WEB3** — Anda/KIP session pathways and ICP durability ([web3/anda.md](web3/anda.md))

## How this documentation is organised

The space has two halves so readers can enter at their own level:

| Group | Audience | What it covers |
|---|---|---|
| Getting started & usage | Everyone — especially new users | Install, flock, daily workflows, CLI, config, ZSH, PWA |
| Architecture & reference | Engineers extending or embedding Aimee | Crates, composition root, persistence, tool internals, gRPC proto, evals |

Every page ends with a **Related** section linking its neighbours, so you can navigate without returning to this page.

## How this site is built

This space lives inside the product repository (`documentation/`) and syncs to GitBook through site-wide Git Sync. The mapping is declared in [`docs.yaml`](../docs.yaml) at the repository root; this directory carries the per-space structure files (`.gitbook.yaml`, `SUMMARY.md`, `README.md`). Markdown is the source of truth.

House rules for editing these pages live in [Contributing to documentation](resources/contributing.md).

## Related

- [Quickstart](quickstart.md)
- [The flock](flock.md)
- [Architecture overview](architecture/overview.md)
