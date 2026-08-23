# Hermes Agent → Omega Loops (Rust port)

Upstream clone: `/root/src/repos/hermes-agent` (`NousResearch/hermes-agent`).
Port primitives into `omega_domain`, wire slash commands in `omega_main`.
Do **not** copy Hermes Python (`goals.py` judge, gateway long-poll) in the first cut.

## Commands

| Slash | Domain | Persist |
|-------|--------|---------|
| `/goal [text\|status\|pause\|resume\|clear]` | `GoalStore` / `GoalState` | `~/.omega/goal.json` |
| `/subgoal <criterion>` | `GoalState.subgoals` | same file |
| `/soul` | `SoulDocument::discover_all` | none (read-only) |
| `/team [list]` | `Team`, `AgentWorkflow` | none (roster is in-memory default) |
| `/team run` | `WorkflowRun` | in-memory on `UIState.workflow` |
| `/learn name \| description` | `SkillDraft` | `cwd/.omega/skills/<slug>/SKILL.md` |
| `/channel [list\|send <text>\|telegram <chat-id>]` | `ChannelStore`, `TelegramSendRequest` | `~/.omega/channels.json` (address only; token in `TELEGRAM_BOT_TOKEN`) |

Aliases: `:goal` / `/goal` (Omega uses both `:` and `/`). Reserved names in `AppCommand::is_reserved_command`.

## Loop contract (`/goal`)

1. User `/goal <text>` writes `GoalState` (active, `max_turns=30`).
2. User turns: if `continuation_prompt()` exists and the text is **not** already `GoalState::is_continuation`, prepend the marker. Do **not** `tick()` here.
3. On `ChatResponse::TaskComplete`, call `after_turn_loops`:
   - If a `WorkflowRun` is live, `maybe_advance_workflow` (takes precedence).
   - Else `maybe_continue_goal`:
     1. Read last assistant text (`api.conversation` → flatten `Option`). Fail-open if missing.
     2. `GoalStore::judge` / `judge_goal` — fail-open (no marker → continue).
     3. If complete, print `/goal complete — {reason}` and **do not** tick or auto-send.
     4. Else `tick()`, then if still active auto-send continuation via `Box::pin(self.on_message(Some(prompt)))`.
4. Auto-pause when `turns_used >= max_turns`. `/goal pause|clear` stops the loop. `GOAL_COMPLETE:` or a `stop when:` match marks `GoalStatus::Done`.

Parse `outcome:` / `verify:` / `constraints:` / `boundaries:` / `stop when:` into `GoalContract`; incidental colons (`Fix bug: the parser`) stay in the headline.

**Judge (deterministic, not Hermes' LLM judge):** `GOAL_COMPLETE: <reason>` on its own line (case-insensitive), or last reply contains `contract.stop_when`. Continuation prompt tells the agent to emit that line when verified. Empty reply / no marker → continue. Do not log or store tokens.

## Workflow runner (`/team run`)

`AgentWorkflow::engineering_ship()` is muse (plan) → omega (build) → sage (review).

- `/team run` → `WorkflowRun::start` at index 0 → `dispatch_workflow_step` (`on_agent_change` + `on_message` with `step_prompt()`).
- Each `TaskComplete` calls `run.advance()`. If another step exists, dispatch it; else clear `UIState.workflow` and print `Workflow complete`, then fall through to `/goal`.
- Recursive async (`on_message` → `TaskComplete` → `dispatch_workflow_step` → `on_message`) **must** be `Box::pin(...)`.

## SOUL load order

1. `cwd/SOUL/SOUL.md` (project)
2. `cwd/SOUL.md` (project)
3. `base_path/SOUL.md` (identity)
4. `base_path/SOUL/SOUL.md` (identity)

`OmegaCustomInstructionsService` appends these after `AGENTS.md`. `/soul` previews the first 8 lines.

## Adding a slash command

Touch all of: `AppCommand` variant + `name()` + `is_reserved_command` + `ui.rs` match arm + handler. Adding `GoalStore` / `WorkflowRun` to `UIState` **drops** `Default` — construct via `UIState::new(env)` only.

## Telegram send (`/channel send`)

- Bind: `/channel telegram <chat-id>` → `ChannelStore::upsert` (kind + address only).
- Send: `/channel send <text>` builds `TelegramSendRequest` from `TELEGRAM_BOT_TOKEN` + first enabled telegram endpoint, POSTs `sendMessage`. Logs use `TelegramSendRequest::redacted_url()` — never print the token URL.
- List: `/channel` / `/channel list` dumps `ChannelStore` items.

## Still later (do not fake)

- Inbound Telegram / Discord long-poll (needs bot token + a running process). Outbound send exists; do not claim a live gateway.
- LLM auxiliary judge like Hermes `goals.py` — Omega uses the deterministic marker judge above.
- `/goal` → PR → Anda dTEE agent inside a DevPod sandbox. `omega pod` wraps DevPod today; the loop is not connected. See `references/devpod-wrap.md`.

## UI borrow

Clone `goal.goal` / `subgoals.len()` **before** `writeln_title(&mut self, …)`. Holding `&GoalState` from `self.state.goal` across a mutable write is E0502.

`patch` lint on `ui.rs` reports edition-2015 `async fn` errors — ignore those; `cargo check -p omega_main --bin omega` is the source of truth.

CI is `-D warnings`. Drop unused imports after extracting a helper (e.g. `WorkflowStep` after `/team` started using `engineering_ship()`).

## Tests

```bash
cargo test -p omega_domain --lib -- goal:: soul:: team:: channel:: skill_author:: telegram::
cargo check -p omega_main --bin omega
```
