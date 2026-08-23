# High-Quality Full-Session Handoffs / Compaction

## Summary of improvements (Hermes connector)
- `compact_messages` default `max_messages` raised from 40 to 120 (larger recent window).
- Truncation made far less destructive: threshold 12 000 chars (was 4 000).
- Truncation logic now explicitly preserves code blocks (```...```) and the tail of the message (conclusions, final state).
- For very long messages: extract recent code blocks + tail when truncating.
- `build_brief` default also 120.
- `MAX_MESSAGES` in connector config raised to 300 (with comment).
- Extraction improved to surface more artifacts (files, commands, code presence) from the broader transcript.
- Brief section headers updated for clarity: "Full Recent Transcript (high fidelity from session)", "Key Decisions & Rationale (extracted from entire session)".

## Why this matters
Previous compaction dropped most of a long Grok session. When the user wants the "entire session" handed over (for complex multi-turn reasoning, code context, earlier decisions), the resulting pack now retains significantly more usable raw transcript + structured extraction.

## Recommended practice
- On the Grok side, prompt to pass the full relevant conversation history in the `messages` array to `handoff_to_hermes`.
- Example: "hand over this entire session to Hermes including the full history so the brief is high fidelity."
- The connector will now produce a much richer brief instead of a truncated tail.

## Limits still apply
- Body size checks (`MAX_BODY_BYTES` ~512 KiB raw).
- For extremely long histories, older parts rely on extraction rather than verbatim text.
- Always supply a clear `goal`.

## Implementation locations (Hermes connector)
- `src/grok_hermes_connector/compact.py` (core logic)
- `src/grok_hermes_connector/config.py` (MAX_MESSAGES)
- `src/grok_hermes_connector/server.py` (tool description)
- `README.md` (high-quality handoff section)

See also `references/sibling-mcp-domain-swap.md` for the dual-connector domain configuration that was active when these improvements landed.

This change makes the "scrape the entire session" handoff pattern practical and high-signal.
