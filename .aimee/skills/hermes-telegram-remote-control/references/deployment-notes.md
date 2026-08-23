# Deployment notes (this host)

Update when IDs, bot username, or paths change.

## Telegram

| Item | Value |
|------|--------|
| Control group title | Coding With an ABG / Hermes x OpenClaw |
| Group chat_id | `-1004338629579` |
| Finals / archive channel | `-1003989836403` **Aimée Codes** |
| Finals env | `TELEGRAM_FINALS_CHANNEL=-1003989836403` |
| Finals invite (user) | `https://t.me/+guXgh9xMLiRlODc1` |
| Hermes bot | `@HermesHandoverBot` (id `8913496461`) |
| OpenClaw bot | separate token / `@OpenClawHandoverBot` — never share Hermes token |
| Home channel | control group (`TELEGRAM_HOME_CHANNEL`) |

## Surfaces

| Surface | Role |
|---------|------|
| Control group | Conversation / continue prompts only (short status on stop) |
| Aimée Codes (finals) | Full **real** `hermes --tui` screenshot archive of completed work |

## Hermes home

- Profile: default → `$HERMES_HOME` = `/root/.hermes`
- Gateway: user systemd `hermes-gateway.service` (linger enabled)
- Telegram toolset: `platform_toolsets.telegram: [hermes-telegram]`
- Python: `/usr/local/lib/hermes-agent/venv/bin/python`
- Fonts: DejaVu Sans / Sans Mono under `/usr/share/fonts/truetype/dejavu/`
- Real TUI capture deps: `xvfb`, `xterm`, `xdotool`, `imagemagick` (`import`/`convert`)

## Stop-notify install paths

```
/root/.hermes/plugins/stop-notify/{plugin.yaml,__init__.py}   # ONLY notify path; on_session_end only
/root/.hermes/scripts/capture_real_tui.py                     # REAL hermes --tui under Xvfb (preferred)
/root/.hermes/scripts/render_tui_shot.py                      # synthetic TUI PNG fallback
/root/.hermes/scripts/telegram_task_wrapup.py                 # wrap-up + skip/dedup + dual Telegram send
/root/.hermes/scripts/render_terminal_shot.py                 # legacy mono dump — do not use for stop cards
/root/.hermes/agent-hooks/on-session-end-telegram.sh          # NO-OP printf '{}' — must not call wrap-up
/root/.hermes/hooks/task-finish-telegram/                     # inert (events: []); do not re-enable
/root/.hermes/cache/telegram-wrapup/                          # pngs, wrapup.jsonl, locks/
/root/.hermes/shell-hooks-allowlist.json
```

## Capture pipeline

1. `on_session_end` → plugin `stop-notify` → `telegram_task_wrapup.run`
2. Prefer `capture_real_tui.py`: Xvfb + xterm + `hermes --tui --resume <session>` + Page_Up scrolls + stitch
3. Fallback: `render_tui_shot.py` synthetic bubbles (never CLI tool-dump)
4. Send **photo/document** → finals channel Aimée Codes
5. Send **short text** → control group (reply here to continue)

## Config flags in use

- `streaming.enabled: true`
- `group_sessions_per_user: true`
- `platforms.telegram.require_mention: true`
- `platforms.telegram.exclusive_bot_mentions: true`
- `TELEGRAM_REQUIRE_MENTION=true`
- `hooks_auto_accept: true`
- `hooks.on_session_end` → on-session-end-telegram.sh (no-op body)
- `plugins.enabled` includes `stop-notify`
- `TELEGRAM_FINALS_CHANNEL=-1003989836403`

## Operator preference

- Control group = communication plane
- Finals channel = clean archive of what was actually done (real TUI visuals: tools, skills, codeblocks)
- Continue via reply in control group or `@HermesHandoverBot`
- Still use `MEDIA:` for key agent-produced files when relevant
- OpenClaw stays on its own bot

## After changes

```bash
# separate shell — not inside gateway agent
systemctl --user restart hermes-gateway.service
# restart open TUI sessions so in-memory stop-notify reloads
rm -f /root/.hermes/plugins/stop-notify/__pycache__/*.pyc
rm -f /root/.hermes/scripts/__pycache__/*.pyc
hermes hooks doctor
hermes plugins list | grep stop-notify
```

## Related skill lanes

- Pure Hermes coding control → this skill
- OpenClaw → Hermes handoffs → `receive-openclaw-handoff` (do not conflate)
