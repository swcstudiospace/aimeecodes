---
name: hermes-project-soul
description: "Use when building SOUL/ project KBs for Hermes."
version: 1.1.0
author: Hermes Agent
license: MIT
platforms: [linux, macos, windows]
metadata:
  hermes:
    tags: [hermes, soul, agents-md, project-context, onboarding, knowledge-base]
    related_skills: [hermes-agent, plan, hermes-agent-skill-authoring, ratatui-agent-tui]
---

# Hermes Project SOUL Knowledge Bases

## Overview

Build an on-demand **project understanding layer** so Hermes (and other agents) orient quickly without stuffing the entire codebase into one context file. Distinct from `$HERMES_HOME/SOUL.md` (agent **identity**). This skill covers **repo-local** `SOUL/` trees + correct Hermes project-context wiring.

## When to Use

- User asks for a SOUL folder, nested SOUL.md maps, or project understanding docs for Hermes
- Onboarding agents to a large multi-subsystem repo
- Wiring or fixing `.hermes.md` / `AGENTS.md` so rules actually load
- After major architecture work when subsystem docs must stay current

Don't use for: Hermes home identity (`~/.hermes/SOUL.md`), one-off README rewrites, or runtime skill packages (`SKILL.md` under `skills/`).

## Critical Hermes loading rule

Hermes injects **exactly one** project context source (first match wins):

| Priority | File | Discovery |
|----------|------|-----------|
| 1 | `.hermes.md` / `HERMES.md` | Walk parents to git root |
| 2 | `AGENTS.md` | Cwd only |
| 3 | `CLAUDE.md` / `.cursorrules` | Cwd only |

**Pitfall:** Adding `.hermes.md` **disables auto-injection of `AGENTS.md`**. If both exist, `.hermes.md` must explicitly require reading `AGENTS.md` (tool read on every non-trivial task) and summarize hard invariants. Do not assume AGENTS.md still lands in the system prompt.

`$HERMES_HOME/SOUL.md` is independent (identity) and is **not** a substitute for repo SOUL/.

Context files are size-capped (~20k chars head+tail truncate). Nested SOUL files stay small and **on-demand**.

## Target layout

```
REPO_ROOT/
  AGENTS.md                 # coding policy (all agents)
  .hermes.md                # Hermes pointer + force-read AGENTS.md + SOUL index
  SOUL/
    SOUL.md                 # master index + load protocol
    00-overview/SOUL.md
    01-architecture/SOUL.md
    …                       # numbered subsystem folders
    NN-invariants/SOUL.md   # security/routing hard rules
```

Number folders so the index sorts by reading order. Keep each nested file ~1–6 KB (orientation, not source dumps).

## Workflow

1. **Survey the tree** — top-level dirs, binaries, monorepo packages, docs, config templates, tests. Done when you can name subsystems and entrypoints.
2. **Read existing agent rules** — `AGENTS.md`, README, internal docs. Extract hard invariants (security, routing, secrets). Done when invariants are listed.
3. **Write `SOUL/SOUL.md` first** — product one-liner, repo map table, nested index (task → path), hard-rules summary, verify commands, load protocol. Done when an agent can pick the right nested file from the table alone.
4. **Write nested SOUL.md per subsystem** — purpose, key paths, contracts, do-not-break, edit guidance, tests. Point to source paths; do not paste whole modules. Done when a stranger can start work in that subsystem without rereading the whole repo.
5. **Wire Hermes**
   - Update `AGENTS.md` with a short "Project SOUL" section (index + usage).
   - Create/update `.hermes.md`: **must** say Hermes loads this *instead of* AGENTS.md; **must** require `read AGENTS.md`; include SOUL index + invariant summary + verify commands.
6. **Maintenance rule** — durable contract changes update the matching nested SOUL in the same change set. Policy stays in AGENTS.md; knowledge stays in SOUL/.

## Nested SOUL content recipe

Each nested file should answer:

- What is this subsystem for?
- Where does code live (paths)?
- What are the public contracts / types / configs?
- What must never break (invariants)?
- How to verify (targeted tests/commands)?
- What to open next (related SOUL or source)?

Prefer tables and path references over narrative essays.

## Size and loading discipline

- Root `SOUL/SOUL.md`: index + protocol only
- Nested files: one subsystem each; load **only** task-relevant files
- Never dump the whole `SOUL/` tree into one turn
- If a nested file grows past ~8–10 KB, split further

## Separation of concerns

| Artifact | Owns |
|----------|------|
| `AGENTS.md` | Coding policy, verify commands, portable agent rules |
| `.hermes.md` | Hermes entry: force AGENTS + SOUL map |
| `SOUL/**` | Subsystem knowledge / architecture orientation |
| `$HERMES_HOME/SOUL.md` | Agent identity (not project) |
| Skills | Reusable *procedures* across repos |

## Reference implementation

See `references/anda-bot-soul-layout.md` for the anda-bot tree shape, Autonogrammer
TUI brand paths, Ctrl+P improve-prompt pattern, CLI binary name, and verify commands.

- TUI brand/hotkeys → `ratatui-agent-tui`; keep SOUL `09-tui-cli` updated.
- CLI command rename (`anda` → `autonogrammer`) → `rust-cli-binary-rename`;
  keep SOUL overview / rust-core one-liners on the current binary name.
- Marketing dual-brand → `sveltekit-marketing-landing`.

## Common Pitfalls

1. **`.hermes.md` without force-read AGENTS.md** — coding rules silently drop out of the system prompt.
2. **One giant SOUL.md** — hits truncation; agents miss the middle. Nest and index instead.
3. **Copying source into SOUL** — rots immediately. Paths + contracts only.
4. **Treating repo SOUL as Hermes identity** — different file, different purpose.
5. **Skipping invariants** — security/routing rules must appear in SOUL *and* AGENTS.
6. **Not updating SOUL after contract changes** — stale orientation is worse than none; update in the same change.

## Verification Checklist

- [ ] `SOUL/SOUL.md` index maps every nested folder
- [ ] Each nested `SOUL.md` is small and path-grounded
- [ ] Hard invariants appear in AGENTS.md and SOUL invariants doc
- [ ] If `.hermes.md` exists: explicit AGENTS.md read requirement + SOUL pointer
- [ ] No secrets or credential-bearing paths
- [ ] Maintenance note: update nested SOUL when contracts change
