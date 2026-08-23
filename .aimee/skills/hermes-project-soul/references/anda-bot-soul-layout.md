# Reference: anda-bot SOUL layout (example)

Repo: `anda-bot` (Anda Bot / package `anda_bot`). Product TUI brand: **Autonogrammer**.

## Tree produced

```
SOUL/SOUL.md
SOUL/00-overview … 17-invariants/SOUL.md   # numbered subsystems
.hermes.md
AGENTS.md   # section: Project SOUL
```

Subsystem map (task → folder): overview, architecture, rust-core, engine, brain,
channels, identity-security, config-runtime, gateway-browser, tui-cli, voice,
cron-goals, skills, chrome-extension, website-docsite, scripts-ci, development,
invariants.

## Hard invariants captured

- `ChannelMessage.external_user` untrusted; `$external_user` persona
- Route `(channel, reply_target, thread)`; WeChat `session_id` → `thread`
- `allow_external_users` cross-cutting (config, docs, prompts, tests)
- Secrets under `~/.anda`; MCP in `mcp.json` not `config.yaml`
- README EN+CN + docsite (+ zh-Hans) for public behavior

## Product surfaces — Autonogrammer

**CLI binary:** `autonogrammer` (package still `anda_bot`, home still `~/.anda`).
Full rename checklist: skill `rust-cli-binary-rename` → `references/anda-bot-autonogrammer-rename.md`.

User-facing TUI brand (strings ≠ package path):

| Surface | Path | Content |
|---------|------|---------|
| Banner | `anda_bot/src/tui/widgets.rs` | figlet **small** "Autonogrammer" (5×63) |
| Header | `status.rs` `panel_header_line` | "Autonomous intelligence, composed." + badge `Autonogrammer v{ver}` |
| Palette | `theme.rs` | electric cyan/violet/magenta/lime on void footer |
| Improve prompt | `prompt_improve.rs` + `Ctrl+P` | local structure + daemon `/side` polish |
| CLI argv0 | Cargo `[[bin]]` + installers | **`autonogrammer`** |

### Banner technique

1. pyfiglet font `small` (or `mini`/`smslant`) — width ≤ ~70, height 4–6
2. Pad lines equal width; embed as Rust `r#\"...\"#` array
3. **Byte-compare** generator output to Rust source (easy off-by-one glyph bugs, e.g. `\_,_|` vs `\__,_|`)
4. Unit-test with buffer width ≥ art width; assert distinctive mid + bottom rows
5. Creative UI: show banner+tagline for user OK; run `banner_*` tests early; hold full `make test` until taste sign-off

### Improve-prompt control (Ctrl+P)

Pattern used in anda-bot (reuse in other agent TUIs):

1. **Footer chip** — styled key badge + label (not a mouse widget; terminal “button”)
2. **Instant local rewrite** — Goal / Context / Requirements / Success criteria; preserve `/goal`, `/loop`, `/skill name`, …
3. **Async daemon polish** — `agent_run` with `/side …` instruction, unique `meta.extra.source` (e.g. `tui:prompt-improve`) so rewrite **does not join** the main conversation
4. **oneshot + poll** — same lifecycle as auto-update check / action response; separator label `improving …` while pending
5. On agent failure: keep local rewrite; never clear the draft

Detail skill: `ratatui-agent-tui` (if installed).

## Hermes wiring lesson

`.hermes.md` **replaces** AGENTS.md auto-load. Open with: must tool-read
`AGENTS.md` every non-trivial task; then SOUL index + invariant summary.

## Verification slice used

```bash
cargo test -p anda_bot -- tui:: -- --nocapture   # full TUI suite
cargo test -p anda_bot -- banner_ prompt_improve status_footer handle_key_control
make lint   # project fmt+clippy; ignore pre-existing non-TUI warnings
```

Do not treat full workspace `make test` as required for TUI-only visual slices unless user asks.
