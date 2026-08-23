# What is Aimee Codes?

Aimee Codes is a terminal-native coding agent organized as a **flock** of three cooperating agents. It reads and edits the files in your repository, runs commands, and reports evidence for what it did — under permissions you control.

## The one-paragraph version

You address the flock from your shell or the built-in TUI. Sage (`:ask`) researches your codebase and answers questions without writing anything. Muse (`:plan`) turns intent into a checkbox plan saved under `plans/`. Aimee (`:act`) implements the plan, verifies the result with tests or commands, and shows you the evidence. Specialists (frontend, backend, platform) are dispatched by Aimee when a task needs a specialty.

## Ways to run it

| Surface | Command | Best for |
|---|---|---|
| Interactive TUI | `aimee` | Long working sessions in the terminal |
| One-shot prompt | `aimee -p "…"` | Scripting, quick questions, piped input (`cat file \| aimee`) |
| ZSH prefix | `: sage …`, `: aimee …` | Staying in your existing shell |
| PWA | installable app shell | Drafts on mobile/browser; agent wiring pending |

## What makes it different

**Flock, not monolith.** Research, planning, and implementation are separate roles with separate write permissions. Sage cannot edit files. Muse only writes plan files. Aimee implements and verifies.

**WEB3-aware, opt-in.** Anda/KIP session pathways add hash-chained checkpoints and chat-only rollback to conversations. Wallet login exists beside provider auth. Payments and spend stay human-in-the-loop. See [Anda / KIP pathways](integrations/anda-kip.md).

**Guardrails by default.** Restricted mode requires explicit permission grants before tool execution. Credentials live outside git. Plans are reviewable artifacts. See [Security model](operations/security.md).

**42 providers.** OpenAI, Anthropic, xAI (including SuperGrok via `xai_oauth`), Google, Bedrock, Azure, OpenRouter, local-compatible endpoints and more. See [Providers](integrations/providers.md).

## Compatibility

Aimee Codes was formerly **Omega Loops**. Existing `~/.omega` directories, `OMEGA_*` environment variables, and the `:omega` alias keep working. Migration is one command: `aimee config migrate`. See [Migrating from Omega Loops](help/migration.md).

## Under the hood

A Rust 2024 Cargo workspace with clean architecture: domain types and policies, application services over injected infrastructure, Diesel/SQLite persistence, and a composition root that wires it together. The full map is in [Architecture overview](architecture/overview.md); the machine-readable contract is [gRPC (aimee.proto)](reference/proto.md).

## See also

* [Quickstart](getting-started/quickstart.md) — install and first run
* [The flock: Sage, Muse, Aimee](getting-started/the-flock.md)
* [Three modes](getting-started/modes.md)

<!-- sources: AIMEE.md §1,§3,§10, README.md -->
