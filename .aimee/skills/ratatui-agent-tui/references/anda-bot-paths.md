# anda-bot TUI paths (Autonogrammer slice)

Quick map after the Autonogrammer brand + Ctrl+P improve work.

| Concern | Path |
|---------|------|
| Banner art | `anda_bot/src/tui/widgets.rs` (`BANNER_ART`) |
| Theme / buttons | `anda_bot/src/tui/theme.rs` |
| Footer chip + header | `anda_bot/src/tui/status.rs` |
| Divider shimmer / label | `anda_bot/src/tui/input.rs` |
| Ctrl+P + oneshot poll | `anda_bot/src/tui/app.rs` |
| Poll loop hook | `anda_bot/src/tui/terminal.rs` |
| Local + agent improve | `anda_bot/src/tui/prompt_improve.rs` |
| Orientation doc | `SOUL/09-tui-cli/SOUL.md` |

Agent rewrite source key: `tui:prompt-improve` (must not equal main TUI chat source).

Verify:

```bash
cargo test -p anda_bot -- tui:: -- --nocapture
make lint
```
