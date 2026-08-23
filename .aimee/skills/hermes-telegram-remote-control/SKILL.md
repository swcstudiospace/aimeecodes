---
name: hermes-telegram-remote-control
description: "Use when controlling Hermes from Telegram groups."
version: 1.3.1
author: Hermes Agent
license: MIT
platforms: [linux, macos]
metadata:
  hermes:
    tags: [hermes, telegram, gateway, remote-control, hooks, messaging, stop-notify, tui-capture]
    related_skills: [hermes-agent, receive-openclaw-handoff]
---

# Hermes Telegram remote control

Operate Hermes as a **split Telegram surface**: control group for conversation, **finals channel** for clean completion archives. When the agent **finishes real work**, post a **real `hermes --tui` terminal capture** (tool trees, skills, codeblocks) to the finals channel, plus a short status in the control group so operators can reply. Suppress empty/aborted noise. Keep OpenClaw on separate tokens.

**Docs:** https://hermes-agent.nousresearch.com/docs/user-guide/messaging/telegram  
**Hooks:** https://hermes-agent.nousresearch.com/docs/user-guide/features/hooks

## User preferences (hard rules for this operator)

1. **Control group = conversation only** — continue via reply/@mention. Do not spam full completion photos into the group.
2. **Finals channel = clean archive of what was done** — full real-TUI screenshot (+ caption). This host: Aimée Codes `-1003989836403` (`TELEGRAM_FINALS_CHANNEL`).
3. **One completion card per meaningful stop** — never three-message media spam; never empty/aborted noise; never subagent cards (parent turn cards).
4. **Real TUI capture preferred** — run `hermes --tui --resume <session>` under Xvfb and screenshot the terminal (scroll + stitch). Synthetic You/Hermes bubble PNGs are **fallback only**. Operator rejected bubble mockups as “TUI view not working” when they lacked tool/skill/codeblock chrome.
5. **Never CLI/backend tool-stdout dumps** as the stop photo.
6. **Notify real work stops only**: completed, real interrupt mid-work, failed/error, budget — always with a human **Why** line. Skip ghost sessions, post-completion TUI finalize, subagents.
7. **Single notify path** — plugin `stop-notify` only. Shell hook stays pure `{}` no-op.
8. Continue via **reply in the control group** or `@HermesHandoverBot` (not the finals channel).

## When to use

- User wants to “program from Telegram” / phone control of Hermes
- Multi-bot group (Hermes + OpenClaw) needs silent-unless-mentioned behavior
- Need automatic updates when agents stop + why + **visual TUI evidence**
- Dual-channel finals archive vs conversation group
- Home channel / finals channel / group allowlist / require_mention tuning
- Debugging missing wrap-ups / shell hook JSON / plugin load / blank TUI captures

## Architecture (preferred)

| Surface | Role |
|--------|------|
| **Control supergroup** | Conversation — commands, streaming, short stop status, replies |
| **Finals / archive channel** | Clean completion archive — real TUI screenshot + caption |
| **@Hermes bot** | One token, one gateway; must be **admin** on finals channel |
| **Other bots** | Separate tokens; `exclusive_bot_mentions: true` |

```text
@Hermes <task>  →  progress in control group (if Telegram-origin)
                →  agent stops (meaningful work only)
                →  REAL hermes --tui resume + Page_Up scroll-stitch PNG
                →  photo/document → finals channel
                →  short text → control group
                →  user replies in control group / @Hermes
                →  same session continues
```

## Setup checklist

### 1. BotFather / group / channel

1. Bot can join groups; **Group Privacy OFF** *or* bot is **admin**.
2. After privacy changes: **remove + re-add** bot.
3. Confirm `getMe` → `can_read_all_group_messages: true` when privacy is off.
4. Group chat ID is negative (`-100…`). Finals channel: bot is **administrator** with `can_post_messages`.
5. Invite links rotate — resolve chat_id via `getChat` / `linked_chat_id`, not by guessing.

### 2. Secrets (`.env` only)

```bash
TELEGRAM_BOT_TOKEN=...
TELEGRAM_HOME_CHANNEL=-100...              # control group
TELEGRAM_HOME_CHANNEL_NAME="Group Title"
TELEGRAM_CHAT_ID=-100...
TELEGRAM_GROUP_ALLOWED_CHATS=-100...
TELEGRAM_FINALS_CHANNEL=-100...            # archive (this host Aimée Codes: -1003989836403)
TELEGRAM_REQUIRE_MENTION=true
# TELEGRAM_ALLOWED_USERS=<numeric user id>
```

Never put tokens in `config.yaml` or chat.

### 3. Config (`config.yaml`)

```yaml
group_sessions_per_user: true
streaming:
  enabled: true

platforms:
  telegram:
    enabled: true
    home_channel:
      platform: telegram
      chat_id: "-100..."
      name: "Group Title"
    require_mention: true
    exclusive_bot_mentions: true

hooks_auto_accept: true
hooks:
  on_session_end:
    - command: "$HERMES_HOME/agent-hooks/on-session-end-telegram.sh"
      timeout: 120

plugins:
  enabled:
    - stop-notify   # required for TUI — shell hooks alone are not enough
```

### 4. Stop-notify stack (single path)

| Piece | Role |
|-------|------|
| **Plugin** `$HERMES_HOME/plugins/stop-notify/` | Only notify path; `on_session_end` only |
| **Shell hook** `on-session-end-telegram.sh` | No-op `{}` only |
| **`capture_real_tui.py`** | Preferred stop image: Xvfb + xterm + real TUI |
| **`render_tui_shot.py`** | Synthetic bubbles — fallback only |
| **`telegram_task_wrapup.py`** | Skip/dedup + dual delivery |
| **Allowlist** | shell-hooks-allowlist.json |

Host packages for real capture: `xvfb`, `xterm`, `xdotool`, ImageMagick (`import`/`convert`). Details: `references/real-tui-capture.md`.

Enable: `hermes plugins enable stop-notify`  
Health: `hermes hooks doctor` → valid JSON.

### 5. Reload after wrap-up / plugin code changes

**Stale-import failure mode (2026-08):** long-lived gateway + TUI keep the first
`import telegram_task_wrapup` forever. Disk already had dual-delivery + real-TUI
capture, but live stops still posted **synthetic bubbles to the control group
only** — no Aimée Codes, no real TUI. Operator: “TUI Output not displayed /
Aimée Codes not updated.”

**Required plugin pattern** (`stop-notify/__init__.py`):

```python
import importlib, sys
def _load_wrapup():
    name = "telegram_task_wrapup"
    if name in sys.modules:
        return importlib.reload(sys.modules[name])
    return importlib.import_module(name)
```

Log `capture_mode`, `target`, `control_target`, `results` on each fire.

**Process reload** (the plugin file itself is also cached until process restart):

```bash
# NEVER from inside a gateway agent turn (tooling blocks it)
bash $HERMES_HOME/scripts/reload-stop-notify.sh
# = rm pycache + systemctl --user restart hermes-gateway.service
# Then reopen every open hermes --tui tab
```

If the agent cannot restart mid-turn: flag
`$HERMES_HOME/cache/telegram-wrapup/NEED_PROCESS_RELOAD` and tell the operator
to run the script from SSH. One-off recovery: force-repost with a **clean**
Python process (not the gateway interpreter).

## Interaction rules

| User action | Expected |
|-------------|----------|
| `@Hermes …` | New or continued turn |
| Reply in control group | Continues session |
| `/stop@Hermes` | Abort + stop notification if real content |
| `/new@Hermes` | Fresh session |
| `@OpenClaw …` only | Hermes silent |

## Stop card format

### Finals channel (media)

```text
✅ Agent stopped — task completed
📌 <title from current user prompt>
Why: <human reason>
Origin: tui|cli|telegram · <session_tail> · shot:real_tui|synthetic

<short preview>

↪️ Continue in the control group (@HermesHandoverBot).
```

Image: real `hermes --tui` capture when possible (tall stitch → `sendDocument` if height ≳ 8500px). Fallback synthetic: full final answer (~14k chars), tools as meta only.

### Control group (short text)

```text
✅ Agent stopped — task completed
📌 <title>
Why: …
Origin: …

📷 Full TUI capture posted to Aimée Codes (finals channel).
↪️ Reply here to continue.
```

**Title:** prefer current user prompt first line — not sticky session `display_name`.

### Why mapping

| Internal signal | Status | Card? |
|-----------------|--------|-------|
| `text_response(...)` | completed | Yes |
| interrupt / `/stop` mid-work | stopped | Yes if content |
| max_iterations / budget | budget | Yes |
| error | failed | Yes |
| `subagent_*` | — | **No** |
| empty ghost / post-complete finalize | stopped | **No** |

## Automatic finish pipeline

See `references/wrapup-pipeline.md`.

1. `on_session_end` → plugin → `telegram_task_wrapup.run`
2. Prefer `capture_real_tui.py` (real terminal)
3. Fallback `render_tui_shot.py`
4. Media → finals; short text → control
5. If finals send fails, media falls back to control

**Shell hook stdout** must stay pure `{}`. Dual-path (shell + plugin both calling wrap-up) was the 2026-08 spam root cause.

**Dedup:** turn fingerprint 120s; session-stop cooldown 300s after successful post; skip empty_stop / subagent.

## Multi-bot / OpenClaw

- One bot token → one polling gateway.
- Hermes coding → Hermes bot only.
- OpenClaw handoffs → `receive-openclaw-handoff`; do not conflate.
- `require_mention` + `exclusive_bot_mentions` for multi-bot groups.

## Pitfalls

1. **Dual path spam** — shell hook must remain pure `printf '{}\n'`; plugin alone calls wrap-up.
2. **Empty/aborted spam** — skip `empty_stop`; do not “always notify empty stops.”
3. **TUI finalize after completed turn** — `session_stop_cooldown` 300s.
4. **Non-atomic dedup** — use `O_CREAT|O_EXCL` locks.
5. **Subagent spam** — no `subagent_stop` hook.
6. **Stale in-memory wrap-up** — plain `import telegram_task_wrapup` is sticky
   for the life of gateway/TUI. Use `importlib.reload` every fire **and** restart
   gateway + TUIs after editing `stop-notify` itself. Symptom: live
   `wrapup.jsonl` lacks `capture_mode`/`finals` while forced Python runs have them.
7. **Shell hook stdout pollution** — non-JSON → silent notification death.
8. **TUI without plugin** — plugin required for TUI stops.
9. **Gateway restart from inside gateway** — blocked.
10. **Privacy cache** — privacy change needs kick/rejoin.
11. **Polling conflict** — second process on same token.
12. **Redirecting TUI stdout to a log** — Ink paints the log; xterm blank. Capture: `2>>log` only.
13. **Bubble mockups sold as “the TUI”** — operator rejected; real capture preferred.
14. **Photo spam in control group** — control is conversation-only.
15. **Sticky session title** — title from current user prompt.
16. **Truncated synthetic answers** — assistant body ~14k (1.8k caused `… [truncated]`).
17. **Invite link ≠ chat_id** — resolve IDs; bot must be channel admin.
18. **Truncated hook fields** — rebuild from `state.db` via `session_id`.

## Verification matrix

| Check | Pass |
|-------|------|
| `hermes hooks doctor` | valid JSON |
| `hermes plugins list` | stop-notify, `on_session_end` only |
| Shell hook stdout | exactly `{}` |
| Turn completes | Media on **finals** + short text on **control** |
| Ghost open/close | No card |
| Real capture probe | Multi-hundred-KB+ PNG, not 1–2KB blank |
| Reply in control | Continues session |
| Capture fail | `shot:synthetic` still posts |

## Support files

- `references/wrapup-pipeline.md` — triggers, dual delivery, dedup
- `references/deployment-notes.md` — this host’s IDs and paths
- `references/real-tui-capture.md` — Xvfb/xterm recipe, Ink stdout pitfall, stitch limits
- Host helper: `$HERMES_HOME/scripts/reload-stop-notify.sh` — gateway reload outside agent turn

## Related

- Bundled `hermes-agent` for general CLI/gateway (reference only)
- `receive-openclaw-handoff` for OpenClaw → Hermes handoff lane
