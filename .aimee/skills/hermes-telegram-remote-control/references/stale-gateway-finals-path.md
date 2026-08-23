# Stale gateway → silent finals channel (2026-08)

## Symptom

- Operator: “Telegram channel is not getting finish notifications” / Aimée Codes quiet
- Control group may still get stop photos
- Live `wrapup.jsonl` lines lack `capture_mode` and `target` is control only
- Clean one-shot Python posts work: `target=-1003989836403`, `capture_mode=real_tui`

## Root cause

1. Gateway process keeps the **plugin register() callback** from boot forever.
2. Even `importlib.reload(telegram_task_wrapup)` does nothing if the **old** plugin never reloads.
3. Disk already had dual-delivery + real-TUI; live path did not.

## Durable fix (preferred)

| Layer | Behavior |
|-------|----------|
| **Shell hook** | Re-exec script from disk every fire → `python wrapup.py --from-hook` with `WRAPUP_FINALS_ONLY=1` + force → **Aimée Codes only** |
| **Plugin** | **Subprocess** wrapup every fire (not sticky import); dual delivery when process is current |
| **Hook timeout** | **180s** (real TUI stitch is 30–150s; 120 kills it) |
| **Gateway reload** | Still needed for dual-path plugin freshness; run `reload-stop-notify.sh` **outside** agent turns |

## Smoke

```bash
export HERMES_HOME=~/.hermes WRAPUP_FINALS_ONLY=1
python $HERMES_HOME/scripts/telegram_task_wrapup.py --test completed
# finals_ok=true target=-1003989836403 capture_mode=real_tui|synthetic

printf '%s' '{"hook_event_name":"on_session_end","session_id":"<sid>","extra":{"completed":true,"platform":"telegram","user_message":"x","response":"y","turn_id":"t1"}}' \
  | $HERMES_HOME/agent-hooks/on-session-end-telegram.sh
tail -3 $HERMES_HOME/cache/telegram-wrapup/hook.err
```

## Anti-patterns

- Permanent shell no-op while relying only on in-process plugin (fails when plugin is stale)
- Dual full photo posts to **control** from shell + plugin (use finals_only on shell)
- Claiming reload alone fixed finals without verifying `wrapup.jsonl` target + capture_mode

See also: `references/wrapup-pipeline.md` (full pipeline; update when editing stack).
