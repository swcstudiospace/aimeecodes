# FAQ

Short answers to what new users ask most. Each links the deep-dive.

## What is Aimee Codes in one sentence?

A terminal-native coding agent where three agents — Sage (research), Muse (planning), Aimee (implementation) — work as a flock under your permission controls. [Overview](../about.md)

## Do I need an API key?

For API-key providers yes; for OAuth providers like SuperGrok (`xai_oauth`) no — device login handles it. `aimee provider login` walks you through whichever you pick. [Providers](../integrations/providers.md)

## Is my code sent anywhere?

Only to your configured model provider, and to the workspace indexing service only when you explicitly sync. Conversation state stays local. [Data privacy](../operations/privacy.md)

## Can it edit files without asking?

By default within its tool permissions; set `restricted = true` and every tool execution requires your explicit grant. Role limits apply regardless: Sage never writes, Muse only writes plans. [Security model](../operations/security.md)

## How is this different from Omega Loops?

It's the same product renamed with a new flock architecture. Legacy `~/.omega` directories, `OMEGA_*` env vars, and `:omega` keep working; migrate with `aimee config migrate`. [Migration guide](migration.md)

## Does it work on Windows?

Supported install paths target Linux and macOS (x86_64/aarch64). On Windows, use WSL2 or the Dev Container. [Installing](../getting-started/install.md)

## Can I use my company's OpenAI-compatible gateway?

Yes — declare an inline `[[providers]]` entry with `response_type = "openai"` pointing at your endpoint. [Providers](../integrations/providers.md)

## How do I stop runaway loops?

Set `max_tool_failure_per_turn` in `.aimee.toml`; the doom-loop reminder also interrupts repetitive failure patterns automatically. [Autonomy](../concepts/autonomy.md)

## What's a pod vs the sandbox?

`--sandbox` is a git worktree on your machine; `aimee pod` provisions container workspaces for untrusted or reproducible work. [Pods and sandboxes](../surfaces/pods.md)

## Does the PWA run agents?

Not yet — it's an installable shell with offline support; drafts stay on-device until agent wiring lands. The TUI/CLI/ZSH surfaces are fully live today. [PWA](../surfaces/pwa.md)

## How do I add my own workflows?

Commands for repeatable flows (`.aimee/commands/*.md`), skills for task-type knowledge (`.aimee/skills/<name>/SKILL.md`). [Skills, commands, templates](../concepts/skills-commands-templates.md)

## Which models work best?

Any of the 42 built-in providers' models; wire protocols cover OpenAI, Anthropic, Google, Bedrock, Responses, and OpenCode shapes. Dedicated commit/suggest models can be cheaper picks. [Cost awareness](../operations/cost.md)

## How do I contribute?

Read AGENTS.md first — it defines the house rules, testing contract, and verification commands. CI treats warnings as errors. [Testing and evals](../operations/testing-evals.md)

## Where are the docs' sources?

Every page ends with a `sources:` comment listing the repo paths its claims came from.

## See also

* [Troubleshooting](troubleshooting.md)
* [Glossary](glossary.md)
* [What is Aimee Codes?](../about.md)

<!-- sources: AIMEE.md §1-§16 -->
