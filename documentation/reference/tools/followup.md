# followup

`followup` asks the human a clarifying question with tappable options. Input type: `Followup` (`crates/aimee_domain/src/tools/catalog.rs:640-670`). Description source: `descriptions/followup.md`.

## Parameters

| Parameter | Type | Required | Notes |
|---|---|---|---|
| `question` | string | yes | The question text |
| `multiple` | boolean | no | `true` = multi-select; default single-select |
| `option1` … `option5` | string | no | Up to five choices |

## Example

```json
{
  "name": "followup",
  "arguments": {
    "question": "Which deployment target should I configure?",
    "option1": "staging",
    "option2": "production",
    "multiple": false
  }
}
```

## Behavior

- **`followup` yields the turn**: after this tool runs, control returns to the human (`ToolCatalog::should_yield`, `catalog.rs:907-913`). It is the only yielding tool in the catalog.
- The contract instructs agents to use it *judiciously* — for genuine ambiguities, not as a reflex — and to prefer making a reasonable default choice and stating it when a question would be low-stakes.
- Options render as selectable rows in the TUI; the user's pick (or a typed free-form answer) arrives as the next user message.

## Errors

Malformed arguments only; there is no failure mode from the user's side — declining to answer is a valid outcome.

## Permissions

No permission gate (`catalog.rs:1010`) — it terminates in a human interaction, not an effect on the system.

## Related

- [Tool catalog](catalog.md)
- [The flock](../../flock.md) — HITL philosophy across agents
- [Loop autonomy](../../architecture/domain.md) — `/goal` probes, the other stop-and-ask mechanism
