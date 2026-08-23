# Streaming pipeline

Model responses stream — token by token, rendered live. Four crates make that happen without blocking the interface.

## The path

```text
provider (SSE)
   ↓
aimee_eventsource        SSE client over reqwest
   ↓
aimee_eventsource_stream byte-stream parser → typed events
   ↓
aimee_stream             MpscStream channel plumbing
   ↓
aimee_markdown_stream    incremental markdown renderer → terminal
```

Each stage is its own crate with a narrow job:

* **`aimee_eventsource`** speaks server-sent events to providers over reqwest.
* **`aimee_eventsource_stream`** parses raw byte streams into event types — the boundary where wire format becomes domain data.
* **`aimee_stream`** provides `MpscStream`, the channel abstraction connecting producer to consumer.
* **`aimee_markdown_stream`** renders markdown incrementally, so you watch the answer form instead of waiting for a block of text.

## Why it matters to you

* **Responsiveness:** long answers appear as they generate; tool activity interleaves in the timeline.
* **Cancellation:** streaming through channels means a cancelled turn stops cleanly at the pipeline level.
* **Protocol diversity:** all six wire protocols (OpenAI, OpenAI Responses, Anthropic, Bedrock, Google, OpenCode) normalize into the same internal event flow — surfaces don't care which provider answered.

## Related rendering

The presentation layer adds syntax highlighting, diffs, and grep output formatting (`aimee_display`), spinners/progress (`aimee_spinner`), and fuzzy pickers (`aimee_select`) — but those are display concerns, not streaming.

## See also

* [How Aimee thinks: the loop](loop.md)
* [Terminal UI](../surfaces/tui.md)
* [Architecture overview](../architecture/overview.md)

<!-- sources: AIMEE.md §5, crates/aimee_eventsource*, crates/aimee_markdown_stream -->
