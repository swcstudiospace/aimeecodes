---
name: ratatui-agent-tui
description: "Brand and extend ratatui agent TUIs (hotkeys, prompts)."
version: 1.0.0
author: Hermes Agent
license: MIT
platforms: [linux, macos, windows]
metadata:
  hermes:
    tags: [ratatui, tui, branding, prompt-improve, crossterm, agent-ui]
    related_skills: [hermes-project-soul, plan, test-driven-development, rust-cli-binary-rename]
---

# Ratatui Agent TUI

## Overview

Brand, theme, and extend **ratatui + crossterm** agent terminals: ASCII banners,
electric palettes, footer “button” chips, and draft-prompt improve controls that
do not pollute the main chat. Grounded in the Autonogrammer / anda-bot TUI work.

## When to Use

- Rebrand a TUI (banner, tagline, version badge) without renaming the binary
- Make a terminal UI more vibrant (palette, divider shimmer, badges)
- Add a hotkey that rewrites the compose draft into a stronger agent prompt
- Wire oneshot background work (improve/update/action) into the TUI poll loop
- Redesign **agent run timeline** (tool/skill/agent hops) toward Warp-clean lanes
  (Aimee Codes / multi-agent CLIs)

Don't use for: CLI/binary renames across installer/docs/launcher
→ `rust-cli-binary-rename`; full crate/folder/type rebrand
→ `rust-workspace-rebrand`; web UIs → `sveltekit-marketing-landing`;
Hermes home identity SOUL.md; Telegram stop-photo capture
→ `hermes-telegram-remote-control`.

## Prerequisites

- Repo uses ratatui + crossterm (or equivalent)
- For agent-backed rewrite: local daemon/gateway `agent_run` (or similar) available
- Optional: `pyfiglet` only to *generate* banner art offline; do not require it at runtime

## Procedure

### 1. Scope the brand surface

List **user-visible** strings only: banner art, header tagline, version chip,
footer help, separator labels. Leave package/binary names alone unless asked.

Done when: touch list is TUI-only paths.

### 2. Banner art

1. Generate with pyfiglet font `small` (fallback `mini` / `smslant`); target width ≤ ~70, height 4–6. Two-word names (`OMEGA LOOPS`) fit `smslant` at ~58×4.
2. Pad lines to equal width; embed `const BANNER_ART` or `include_str!("banner")`.
3. Diff bytes against the generator (one-character glyph bugs are common)
4. Unit-test render into a buffer **wider than the art**; assert mid-row + base-row needles
5. **Hybrid rustyline session:** do **not** EnterAlternateScreen / raw mode for the splash. Compose art+tagline+chips in a ratatui `Buffer`, convert cells to truecolor ANSI, print, then return. See `references/rustyline-splash.md`.

Done when: banner tests pass and user has seen the art (creative UI).

### 3. Vibrant palette

- High-chroma accents (cyan / violet / magenta / lime) on a deep footer/void base
- Keep body text near-white for legibility
- Style helpers: `accent`, `success`, `warn`, `danger`, `button_style`, `button_key_style`
- Divider shimmer: blend label vs track colors over `animation_tick`

Done when: theme unit tests lock the new constants; no washed-out dim-on-dark body.

### 4. Footer “button” chip (no mouse required)

Terminal UX = keyboard. Show a chip:

`[ Ctrl+P ][ Improve prompt ]` + short hint

- Use distinct bg/fg for key vs label (`button_key_style` / `button_style`)
- Mirror the binding in the compose separator when draft non-empty
- Bump footer max lines if you add a chip row above help lines

Done when: help footer shows the chip and height tests match.

### 5. Improve-prompt control (Ctrl+P pattern)

**Instant path (always):**

- Local rewrite: Goal / Context / Requirements / Success criteria
- Preserve slash prefixes: `/goal`, `/loop`, `/side`, `/steer`, `/skill <name>`, `/stop`, `/cancel`, `/new`

**Async polish (when daemon healthy):**

1. `oneshot` + `tokio::spawn` calling gateway `agent_run` with a `/side` instruction
2. Set unique request `meta.extra.source` (e.g. `tui:prompt-improve`) so the run **does not join** the main conversation session
3. Poll `try_recv` in the TUI loop alongside other pending work
4. Sanitize model output (strip fences, “Improved prompt:” preambles, wrapping quotes)
5. On empty/error: **keep local rewrite**; never blank the draft
6. While pending: block draft edits (allow quit); separator label `improving …`

Instruction skeleton: output **only** improved prompt text; preserve slash command prefixes; add concrete success criteria; no essay.

Done when: unit tests cover local structure + prefix preservation + sanitize; Ctrl+P key test exists.

### 6. Warp-clean agent timeline (tools / skills / handoffs)

**Operator benchmark (Aimee Codes):** Warp quietness + Hermes tool/skill
visibility + Super Grok Heavy agent hops. Dense `ERROR:` prefixes and long
prose tool titles fail the bar.

| Chip | Use |
|------|-----|
| TOOL | Read / Shell / Search / MCP / Todos |
| SKIL | Skill load |
| AGNT | Agent handoff or switch (`NAME → task`) |
| DONE | Turn finished |

**Pattern:** domain categories `Tool`/`Skill`/`Agent` + presentation gutter/chip
+ short tool verbs + agent `→` subtitle. Full map:
`references/warp-agent-timeline.md`.

Done when: plain lane tests green; live session shows TOOL/SKIL/AGNT/DONE.

### 7. Verify (slice, then broaden)

```bash
# Targeted — prefer this for TUI-only slices
cargo test -p <pkg> -- tui:: -- --nocapture
cargo test -p <pkg> -- banner_ prompt_improve status_footer handle_key_control title_display

# Aimee Codes display slice
cargo test -p aimee_main --lib title_display
cargo test -p aimee_app --lib fmt::

# Project lint if Makefile defines it
make lint
```

Creative UI: after visual changes, show banner/tagline/chip **and sample
timeline lines** to the user; run banner/theme/title tests immediately; hold
full workspace `make test` until taste OK unless the user asks for full suite.

Done when: TUI filter suite green; no new clippy hits in touched TUI files.

## Pitfalls

1. **Byte drift in banner strings** — always regenerate or byte-diff; do not hand-edit figlet by eye.
2. **ratatui 0.29 + nucleo-picker** — 0.29 pins `unicode-width =0.2.0`; nucleo-picker wants `^0.2.2`. Use ratatui **0.30+**.
3. **Alt-screen splash before rustyline** — raw/`Terminal` steals the session. Buffer → ANSI only.
2. **Agent rewrite joining main chat** — wrong/missing `source` meta collides sessions; use a dedicated source key.
3. **Blocking the UI on agent_run** — always oneshot + poll; apply local rewrite first for snappy UX.
4. **Clearing draft on agent failure** — keep local structured text.
5. **Forgetting slash prefixes** — `/goal foo` must stay `/goal …` after improve.
6. **Full-product rebrand by accident** — TUI strings ≠ installer, locales, binary names.
7. **Assuming mouse buttons** — chips are reminders; hotkeys do the work.
8. **Clippy `-D warnings` on whole crate** — pre-existing warnings elsewhere fail CI; scope lint to project `make lint` or touched paths.
9. **Prose tool titles** — long “Search for X in Y at Z” fails Warp bar; short verb + subtitle.
10. **Agent hop in Debug chip** — use AGNT lane + `→`, not `NAME [Agent]` soup.
11. **Category rename without tests** — Debug→Tool breaks fmt assertions; update together.

## Verification

- [ ] Banner tests pass at art width
- [ ] Theme tests lock palette + button styles
- [ ] Footer shows Ctrl+P chip; height tests updated
- [ ] Local improve structures plain drafts; preserves `/goal` and `/skill name`
- [ ] Sanitize strips fences/preambles
- [ ] Ctrl+P keypath covered
- [ ] Timeline plain tests lock TOOL/SKIL/AGNT chips + handoff arrow
- [ ] `cargo test -p <pkg> -- tui::` green

## References

- Path map (anda-bot Autonogrammer slice): `references/anda-bot-paths.md`
- Hybrid rustyline splash (Omega Loops): `references/rustyline-splash.md`
- Warp-style tool/skill/agent timeline (Aimee Codes): `references/warp-agent-timeline.md`
- Broader SOUL + brand notes: skill `hermes-project-soul` → `references/anda-bot-soul-layout.md`
