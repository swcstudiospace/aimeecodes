# Loop autonomy (probes, XML, Linear, CoT×GoT, quality)

User intent: longer autonomous `/goal` loops **and** more HITL. Encode both.

## Five probes (mandatory)

`GOAL_PROBE_COUNT = 5`. Canonical questions in `canonical_probe_questions()`.

- Production path: `GoalStore::set_loop(text, GoalProbeSet)` — not `set`.
- TTY: `collect_goal_probes` via `OmegaWidget::input`.
- Non-TTY / tests: five `probe: …` lines in the `/goal` text (`GoalProbeSet::parse_from_text`). Partial or empty answers → refuse; do not start the loop.

## XML upgrade

Every `AppCommand::Message` (and the goal headline after probes) goes through `PromptUpgrade::wrap`. Envelope: `<omega_prompt version="1">` with `<intent>`, optional `<standing_goal>` / `<human_probes>`, `<quality>`. If the text already contains `<omega_prompt`, return unchanged. This is a **structural** best-in-class wrap, not a second LLM round-trip.

## Tool limit

`max_tool_failure_per_turn` default is **unlimited** (`unlimited_tool_failures()` = `usize::MAX`). Do not restore `unwrap_or(3)`. `ToolErrorTracker::new(0)` is **not** unlimited (limit 0 trips immediately).

## Linear + git

After each prompt/goal, `log_prompt_telemetry`:

- POST `LinearEnsureRequest` to `http://127.0.0.1:8792/v1/ensure-issue` (fail-open; log status, never tokens).
- Linear MCP is the interactive cockpit (`hermes-linear-kanban-sync`). Connector health `ok` does not mean the issue exists.
- If `cwd/.git` exists, `git commit --allow-empty` with `GitHubPrPlan`. Otherwise skip. `/goal pr` opens the PR.

## Drop CoT×GoT

Project `.mcp.json`:

```json
{ "mcpServers": { "drop": { "url": "http://127.0.0.1:7788/mcp" } } }
```

`ThoughtGraphRequest::hybrid(goal)` / `drop_mcp_url()`. Continuation tells the agent to call `reason_cot_got` (hybrid) before high-stakes steps.

## Quality gates

`QualityPolicy`: 95% line coverage, Greptile before push, CodeRabbit on PR.

- `scripts/tdd-gate.sh` — tests; `cargo llvm-cov --fail-under-lines 95` only if llvm-cov is present. Never claim whole-workspace 95% without measuring.
- `scripts/greptile-pre-push.sh` — run `greptile review` if installed; exit 0 if missing.
- `{prefix}_ci::generate_coderabbit_workflow()` → `.github/workflows/coderabbit.yml`. Edit the generator, not the YAML.

## Tests

```bash
# crate prefix is omega_* or aimee_* — read Cargo.toml
cargo test -p <prefix>_domain --lib -- loop_autonomy:: goal:: telegram::
cargo check -p <prefix>_main --bin <bin>
```
