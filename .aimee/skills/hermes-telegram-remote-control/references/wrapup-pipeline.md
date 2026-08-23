# Task-stop wrap-up pipeline

## Goal

When a Hermes agent **finishes real work**, post **exactly one** Telegram message to the control group:

- Titled TUI-style screenshot (conversation view)
- Caption: status badge + title + **Why** + preview + reply CTA

Suppress empty/aborted noise and dual-path doubles.

## Triggers (single path)

| Source | Event | Entry | Surfaces |
|--------|-------|--------|----------|
| **Plugin** `stop-notify` | `on_session_end` **only** | `$HERMES_HOME/plugins/stop-notify/` | TUI + CLI + gateway (in-process) |
| **Shell hook** | `on_session_end` | `$HERMES_HOME/agent-hooks/on-session-end-telegram.sh` | **No-op** — prints `{}` only; must **not** call wrap-up |

Only the plugin calls `$HERMES_HOME/scripts/telegram_task_wrapup.py`.

`on_session_end` from `turn_finalizer` fires at the end of **every** `run_conversation` turn (each user message), with kwargs: `session_id`, `turn_id`, `completed`, `failed`, `interrupted`, `turn_exit_reason`, `model`, `platform`.

TUI session-close finalize may fire with `completed=False`, `interrupted=True` and empty content — **skip** via `empty_stop` / `session_stop_cooldown` (do not treat as a real card).

### Why plugin is required

TUI agent processes do not always register shell hooks the same way as CLI/gateway. Plugin `stop-notify` with `hooks: [on_session_end]` (no `subagent_stop`) + `hermes plugins enable stop-notify` is the reliable path. Shell hook alone is **not** enough for TUI stop cards — and must not be a second caller.

### Do not re-enable dual path

Having shell hook **and** plugin both call wrap-up was the primary spam source (2026-08 fix). Shell stays pure JSON no-op.

## CRITICAL: shell hook JSON protocol

Hermes shell hooks must print **one JSON object** on stdout (usually `{}`).

**Current live shell hook:**

```bash
#!/usr/bin/env bash
set -u
printf '%s\n' '{}'
```

If you ever temporarily re-enable shell-side wrap-up (discouraged), wrap it so only `{}` hits stdout:

```bash
python "$HERMES_HOME/scripts/telegram_task_wrapup.py" \
  >>"$LOGDIR/hook.stdout.log" 2>>"$LOGDIR/hook.stderr.log" || true
printf '%s\n' '{}'
```

Inside Python render children:

```python
subprocess.check_call([...], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
```

Validate: `hermes hooks doctor` must report valid JSON.

Allowlist `$HERMES_HOME/shell-hooks-allowlist.json` + `hooks_auto_accept: true`.

## Payload → status (plugin)

| Flags / reason | status |
|----------------|--------|
| `interrupted` | interrupted → labeled stopped |
| `failed` | failed |
| reason contains `max_iteration` / `budget` | budget |
| `completed` | completed |
| else | stopped |

Always rebuild transcript from `state.db` when `session_id` is set — do not rely on truncated hook fields alone.

## Skip gates (`telegram_task_wrapup.run`)

| `skipped` value | When |
|-----------------|------|
| `empty` | No session_id and no content |
| `empty_stop` | `stopped`/`interrupted`, msg_count < 2, no response/user text |
| `subagent` | reason contains `subagent_` or `platform=subagent` |
| `session_stop_cooldown` | stopped with no response within 300s of any successful post for that session |
| `dedup` | Same turn fingerprint within 120s |
| `completed_cooldown` | Completed without `turn_id`, same last-msg fp within 25s (dual-hook remnant) |

## TUI render (real terminal first)

Scripts: `capture_real_tui.py` (preferred) + `render_tui_shot.py` (fallback) + `telegram_task_wrapup.py`

**Preferred:** spawn real `hermes --tui --resume <session_id>` under Xvfb/xterm, Page_Up scroll, stitch PNGs. Shows tool trees, skills, codeblocks — the actual TUI.

**Fallback:** synthetic conversation bubbles if Xvfb capture fails (still not a CLI tool-dump).

### Delivery

| Destination | Content |
|-------------|---------|
| **Aimée Codes** `-1003989836403` (`TELEGRAM_FINALS_CHANNEL`) | Full TUI capture photo/document + caption |
| **Control group** `-1004338629579` | Short status text only — reply here to continue |

### Synthetic fallback limits

| Field | Limit |
|-------|-------|
| Assistant body | ~14k chars |
| User body | ~2.5k after chrome strip |
| Render body lines | 260 |
| Message load | last 250 rows |

Final assistant bubble (fallback path) = hook `response` for this stop.

## Telegram delivery (one message)

Always `TELEGRAM_HOME_CHANNEL`. Prefer `sendPhoto` plain caption; fall back to one `sendDocument`. Never three-message spam.

Caption includes: status line, title, **Why**, origin, preview, reply CTA.

## Dedup implementation

```text
fp = turn:<turn_id> | msg:<last_db_msg_id:role:len>|<status> | hash:<sha1(msg|resp|reason|st)[:16]>
key = <session_id>|<fp>
lock = cache/telegram-wrapup/locks/<sha1(key)>.lock
claim with O_CREAT|O_EXCL; TTL DEDUP_SECONDS=120
after successful post: refresh session-stop|<session_id> (SESSION_STOP_COOLDOWN=300)
```

Not session-only for completed turns (that suppressed consecutive completions). Prefer `turn_id` when present.

## Artifacts

`$HERMES_HOME/cache/telegram-wrapup/` — `tui_*.png`, `bubbles_*.json`, `wrapup.jsonl`, hook logs, `locks/`

## Manual tests

```bash
# Live send (force=True) — use sparingly
python $HERMES_HOME/scripts/telegram_task_wrapup.py --test finished
python $HERMES_HOME/scripts/telegram_task_wrapup.py --test interrupted
python $HERMES_HOME/scripts/telegram_task_wrapup.py --test budget

# Offline gate checks: monkeypatch _tg_api + subprocess.check_call, assert skipped values
hermes hooks doctor
```

## Restart after edits

```bash
# separate shell — not inside gateway agent turn
systemctl --user restart hermes-gateway.service
# open TUI processes keep old in-memory plugin — restart TUI tabs too
rm -f $HERMES_HOME/plugins/stop-notify/__pycache__/*.pyc
rm -f $HERMES_HOME/scripts/__pycache__/telegram_task_wrapup*.pyc
```

## Gateway restart caveat

Do not restart gateway from inside a gateway agent turn — blocked. Use a separate shell.
