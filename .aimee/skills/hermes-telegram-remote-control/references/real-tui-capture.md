# Real `hermes --tui` stop-card capture

## Why

Operator wants stop photos to show the **actual Ink TUI** (tool call trees, skills, codeblocks, session chrome) — not a synthetic bubble mockup and not a CLI tool-stdout dump.

## Preferred path

Script: `$HERMES_HOME/scripts/capture_real_tui.py`

```text
Xvfb :N -screen 0 1600x1000x24
  → xterm -geometry 180x52 -fa DejaVuSansMono
    → hermes --tui --resume <session_id> --no-restore-cwd
  → import -window <xterm> frame_N.png
  → xdotool key Page_Up (repeat)
  → convert -append … long.png
```

Called from `telegram_task_wrapup.py` before synthetic fallback. Timeout budget ~150s.

### Host packages

```bash
apt-get install -y xvfb xterm xdotool imagemagick fonts-dejavu-core
```

## Critical pitfalls

1. **Never redirect TUI stdout off the tty**  
   Wrong: `hermes --tui … >>log 2>&1` → Ink draws into the log; xterm stays blank / tiny PNG.  
   Right: `hermes --tui … 2>>log` (stderr only).

2. **Boot wait** — allow ~14–18s after xterm start before first `import` so resume finishes painting.

3. **Scroll direction** — capture bottom (latest) first, then Page_Up; reverse frames when stitching so oldest is top.

4. **Telegram height** — stitches often exceed ~8500px; wrap-up uses `sendDocument` when tall. Soft-cap convert resize if >20000px.

5. **Session must exist** in `state.db` under `$HERMES_HOME`. Ghost/empty sessions should already be skipped by wrap-up gates.

6. **Display lock** — pick free `:90–119` if `:99` is taken (`/tmp/.X*-lock`).

## Manual probe

```bash
export HERMES_HOME=~/.hermes
python $HERMES_HOME/scripts/capture_real_tui.py \
  --session-id <id> -o /tmp/real_tui.png --scrolls 8 --boot-wait 16
identify /tmp/real_tui.png   # expect multi-hundred-KB+, not 1–2KB colormap
```

## Fallback

If capture exits non-zero or PNG < ~8KB, wrap-up uses `render_tui_shot.py` (You/Hermes bubbles, full final answer text, tools as meta). Caption marks `shot:synthetic`.

## Dual delivery reminder

Real (or synthetic) media → `TELEGRAM_FINALS_CHANNEL`.  
Short status text → control group only.
