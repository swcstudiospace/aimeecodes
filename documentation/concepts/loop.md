# How Aimee thinks: the loop

Every agent turn is a loop: receive intent, think, act with tools, observe results, repeat until the goal is met or a guardrail stops it. Understanding this loop explains most of Aimee's behavior — why it verifies, why it asks, why it sometimes stops.

## One turn, end to end

1. **Input** arrives from any surface (TUI, CLI `-p`, ZSH `:`, command execution).
2. **Context assembly** builds the prompt: system frame (title generation, summary frames, tool-use examples), project policy (`AGENTS.md`), skills in scope, and conversation history.
3. **Prompt uplift** can restate and sharpen the request before execution begins.
4. **Model call** goes to the active provider over its wire protocol; responses stream back through the markdown renderer.
5. **Tool use**: the model's chosen tools execute through the registry — reads, patches, shell, searches — each with a timeout (`tool_timeout_secs`) and permission checks.
6. **Observation** feeds results back as the next model input. Failures are reflected on: a tool-error reflection template turns errors into corrective context rather than dead ends.
7. **Repeat** until done, budget exhausted, or a HITL probe fires.

## The templates behind the behavior

The loop's personality is assembled from explicit prompt partials in `templates/`. The important ones:

| Template | Effect you notice |
|---|---|
| `aimee-partial-verification.md` | Claims come with evidence |
| `aimee-partial-tool-error-reflection.md` | Errors trigger retry-with-reflection, not surrender |
| `aimee-pending-todos-reminder.md` | Unfinished todos resurface mid-run |
| `aimee-doom-loop-reminder.md` | Repetitive failure loops get interrupted |
| `aimee-tool-retry-message.md` | Transient failures retry deliberately |
| `aimee-system-prompt-title-generation.md` | Sessions get useful titles |

## Failure budgets

By default the tool-failure budget per turn is unlimited (`usize::MAX`). Set `max_tool_failure_per_turn` in `.aimee.toml` to stop runaway loops early. Combined with the doom-loop reminder, the loop prefers stopping and asking over thrashing.

## Where the loop stops

Three stop conditions: goal satisfied (with verification), guardrail tripped (autonomy levels, failure budget), or human intervention. What happens next is defined by [Autonomy levels and guardrails](autonomy.md).

## See also

* [Autonomy levels and guardrails](autonomy.md)
* [Tools: how agents touch your system](tools-overview.md)
* [Streaming pipeline](streaming.md)

<!-- sources: AIMEE.md §7,§11, templates/*.md -->
