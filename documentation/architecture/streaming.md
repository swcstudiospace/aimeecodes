# Streaming

How LLM tokens move from a provider HTTP response to the TUI. Transport is **HTTP Server-Sent Events (SSE)** over `reqwest`. There is no WebSocket client or server in `crates/` — do not invent one.

This page is for contributors who need to pick a crate, follow a token, or diagnose a drop / cancel / backpressure failure.

## Pipeline

```
provider HTTP POST (SSE body)
        │
        ▼
aimee_infra::AimeeHttpInfra::eventsource   ── or ──  http_post + bytes_stream
        │                                              │
        ▼                                              ▼
aimee_eventsource::EventSource                 aimee_eventsource_stream
  (client + retry + Content-Type)                (byte-stream parser)
        │                                              │
        └──────────────────┬───────────────────────────┘
                           ▼
          provider adapter (openai / anthropic / google / responses)
          ChatCompletionMessage deltas
                           │
                           ▼
                 BoxStream / ResultStream
                           │
                           ▼
          Orchestrator::execute_chat_turn
          ResultStreamExt::into_full_streaming
                           │
                           ▼
                 ArcSender  (capacity 1)
                           │
                           ▼
                 MpscStream<Result<ChatResponse>>
                           │
                           ▼
          UI::on_chat  →  StreamingWriter
                           │
                           ▼
          aimee_markdown_stream::StreamdownRenderer
```

Hop by hop:

1. **Surface.** `UI::on_chat` calls `API::chat` and consumes the returned `MpscStream` (`crates/aimee_main/src/ui.rs:4693-4715`).
2. **Composition root.** `AimeeAPI::chat` resolves the active agent and delegates to `AimeeApp::chat` (`crates/aimee_api/src/aimee_api.rs:138-146`, `crates/aimee_api/src/api.rs:47`).
3. **Orchestrator spawn.** `AimeeApp::chat` builds an `Orchestrator`, then wraps `orch.run()` in `MpscStream::spawn`. The channel sender is installed as `orch.sender` (`crates/aimee_app/src/app.rs:200-221`).
4. **Provider call.** `Orchestrator::execute_chat_turn` calls `AgentService::chat_agent`, then always streams deltas through `into_full_streaming` (`crates/aimee_app/src/orch.rs:224-236`).
5. **Provider SSE.** Repositories POST JSON via `HttpInfra::http_eventsource` and map SSE messages into `ChatCompletionMessage` (`crates/aimee_repo/src/provider/openai.rs:241-250`, `crates/aimee_repo/src/provider/event.rs:12-48`).
6. **TUI render.** Each `ChatResponse::TaskMessage` markdown delta is pushed into `StreamingWriter`, which owns a `StreamdownRenderer` (`crates/aimee_main/src/ui.rs:4833-4835`, `crates/aimee_main/src/stream_renderer.rs:139-161`).

There is no second transport. `HttpInfra::http_eventsource` is a POST that returns `aimee_eventsource::EventSource` (`crates/aimee_app/src/infra.rs:242-248`). Infra implements it by cloning the `reqwest` builder and calling `.eventsource()` (`crates/aimee_infra/src/http.rs:251-268`).

## Role of each crate

| Crate | What it is | What it is not |
|---|---|---|
| `aimee_eventsource_stream` | Byte-stream SSE parser. `Eventsource` turns `Stream<Item = Result<impl AsRef<[u8]>, E>>` into `EventStream` (`crates/aimee_eventsource_stream/src/lib.rs:1-41`, `crates/aimee_eventsource_stream/src/traits.rs:6-18`). | Not an HTTP client. Does not retry, does not inspect `Content-Type`. |
| `aimee_eventsource` | reqwest wrapper. Sets `Accept: text/event-stream`, validates status + `text/event-stream`, reconnects with backoff, tracks `Last-Event-ID` (`crates/aimee_eventsource/src/lib.rs:1-6`, `crates/aimee_eventsource/src/event_source.rs:68-86`). Internally feeds `res.bytes_stream().eventsource()` (`crates/aimee_eventsource/src/event_source.rs:175-179`). | Not a domain adapter. Does not know `ChatCompletionMessage`. |
| `aimee_stream` | `MpscStream<T>`: spawn a producer task, expose a `Stream` over a capacity-1 channel, abort on drop (`crates/aimee_stream/src/mpsc_stream.rs:7-39`). | Not SSE. Not markdown. Not the provider `BoxStream`. |
| `aimee_markdown_stream` | Line-buffered markdown renderer for LLM tokens (`crates/aimee_markdown_stream/src/lib.rs:1-24`). | Does not talk to the network. Does not own the orchestrator channel. |
| `aimee_main::stream_renderer` | TUI adapter: spinner pause/resume + style switch around `StreamdownRenderer` (`crates/aimee_main/src/stream_renderer.rs:103-164`). | Not a parser. |

Domain types that glue the hops:

- `BoxStream<A, E>` — pinned `Send` stream of `Result<A, E>` (`crates/aimee_domain/src/error.rs:117-118`).
- `ResultStream<A, E>` — `Result<BoxStream<A, E>, E>` (`crates/aimee_domain/src/error.rs:120`).
- `ArcSender` — `tokio::sync::mpsc::Sender<anyhow::Result<ChatResponse>>` (`crates/aimee_domain/src/lib.rs:131`).
- `ChatResponse` — surface events: `TaskMessage`, `TaskReasoning`, `TaskComplete`, `ToolCallStart` / `End`, `RetryAttempt`, `Interrupt` (`crates/aimee_domain/src/chat_response.rs:55-75`).

`MpscStream` is also the workspace-index progress type (`crates/aimee_app/src/services.rs:289`). Same crate, different payload. Do not reuse it for SSE.

## SSE client vs byte-stream parser

Two crates, one protocol. Pick by whether you already have a `reqwest::Response` or you need a reconnecting client.

### `aimee_eventsource` — client

Use when the provider returns `Content-Type: text/event-stream` and you want reconnect.

- `EventSource::new` clones the `RequestBuilder` (required for retry), sets `Accept: text/event-stream`, and starts the first `send()` (`crates/aimee_eventsource/src/event_source.rs:70-85`).
- `check_response` rejects non-200 and any content type that is not `text/event-stream` (`crates/aimee_eventsource/src/event_source.rs:120-154`).
- On a good response it wraps `res.bytes_stream().eventsource()` — the parser crate (`crates/aimee_eventsource/src/event_source.rs:175-179`).
- Reconnects send `Last-Event-ID` (`crates/aimee_eventsource/src/event_source.rs:163-172`).
- Default retry is exponential: 300 ms start, factor 2, cap 5 s, unlimited retries (`crates/aimee_eventsource/src/retry.rs:114-120`).
- `close()` stops reconnect (`crates/aimee_eventsource/src/event_source.rs:93-96`).
- Invalid status / content type **do not retry**. They close the source (`crates/aimee_eventsource/src/event_source.rs:243-251`). Transport errors and `StreamEnded` go through `RetryPolicy` (`crates/aimee_eventsource/src/event_source.rs:254-285`).

`RequestBuilderExt::eventsource` is the only public constructor from a builder (`crates/aimee_eventsource/src/reqwest_ext.rs:8-16`). Infra uses it (`crates/aimee_infra/src/http.rs:262-266`).

### `aimee_eventsource_stream` — parser

Use when you already have a byte stream and must **not** validate `Content-Type`.

- Parses HTML SSE: `event`, `data`, `id`, `retry`; comments ignored; empty line dispatches (`crates/aimee_eventsource_stream/src/event_stream.rs:25-68`).
- Incomplete lines wait (`nom::Err::Incomplete` → `Ok(None)`) (`crates/aimee_eventsource_stream/src/event_stream.rs:230`).
- First chunk may strip a UTF-8 BOM (`crates/aimee_eventsource_stream/src/event_stream.rs:266-271`).
- `EventStreamError` is `Utf8` | `Parser` | `Transport` (`crates/aimee_eventsource_stream/src/event_stream.rs:169-177`).

A line that is only `data: Hello, world!\n` (no blank line) yields **no** event (`crates/aimee_eventsource_stream/src/event_stream.rs:345-351`). The blank line is the dispatch.

### When providers bypass the client

| Path | Why | Crate used |
|---|---|---|
| OpenAI-compatible chat | Standard SSE | `http_eventsource` → `EventSource` → `into_chat_completion_message` (`crates/aimee_repo/src/provider/openai.rs:241-250`) |
| Google `streamGenerateContent?alt=sse` | Query flag forces SSE content type | same (`crates/aimee_repo/src/provider/google.rs:61-90`) |
| Anthropic (default) | Standard SSE | same (`crates/aimee_repo/src/provider/anthropic.rs:178-186`) |
| Anthropic `OPENCODE_ZEN` | Non-standard content type | `http_post` + `bytes_stream().eventsource()` (`crates/aimee_repo/src/provider/anthropic.rs:114-116`, `crates/aimee_repo/src/provider/anthropic.rs:189-247`) |
| OpenAI Responses (default) | Standard SSE | `http_eventsource` (`crates/aimee_repo/src/provider/openai_responses/repository.rs:201-205`) |
| Codex Responses | chatgpt.com does not send `text/event-stream`; `EventSource` would raise `InvalidContentType` | `http_post` + `bytes_stream().eventsource()` (`crates/aimee_repo/src/provider/openai_responses/repository.rs:191-198`, `crates/aimee_repo/src/provider/openai_responses/repository.rs:293-323`) |

Raw-SSE parse errors: `EventStreamError::Transport` becomes `Error::Retryable`; UTF-8 / parser errors do not (`crates/aimee_repo/src/provider/anthropic.rs:296`, `crates/aimee_repo/src/provider/openai_responses/repository.rs:378-392`).

`into_chat_completion_message` drops `Event::Open`, `[DONE]`, and empty `data`; everything else is JSON → provider DTO → `ChatCompletionMessage` (`crates/aimee_repo/src/provider/event.rs:20-48`). It also stops on `Error::StreamEnded` so a clean EOF is not an item (`crates/aimee_repo/src/provider/event.rs:21`).

## Orchestrator and `MpscStream`

`AimeeApp::chat` is the only chat producer of `MpscStream<Result<ChatResponse>>` (`crates/aimee_app/src/app.rs:60-64`, `crates/aimee_app/src/app.rs:200-223`).

```
MpscStream::spawn(|tx| async move {
    let mut orch = orch.sender(tx.clone());
    let dispatch_result = orch.run().await;
    // always persist conversation
    // then tx.send(Err(err)) if dispatch or save failed
})
```

`Orchestrator::send` is a no-op when `sender` is `None` (`crates/aimee_app/src/orch.rs:169-174`). `execute_chat_turn` always streams (`crates/aimee_app/src/orch.rs:233-236`).

`into_full_streaming` (`crates/aimee_domain/src/result_stream_ext.rs:56-147`):

- Pulls `BoxStream<ChatCompletionMessage>`.
- Forwards non-empty reasoning as `ChatResponse::TaskReasoning`.
- Forwards non-empty content as `ChatResponse::TaskMessage { Markdown { partial: true } }`.
- **Ignores send errors** — comment: “the receiver may have been dropped” (`crates/aimee_domain/src/result_stream_ext.rs:124-144`).
- Aggregates usage with provider-specific merge/replace (`crates/aimee_domain/src/result_stream_ext.rs:70-113`).
- Optionally interrupts on XML `<aimee_tool_call>` when tools are not native (`crates/aimee_domain/src/result_stream_ext.rs:153-164`). After interrupt, later **content** is ignored; later **usage** is still applied (`crates/aimee_domain/src/result_stream_ext.rs:79-116`).
- Empty completion with no tools / finish / thought signature is `Error::EmptyCompletion` → retryable (`crates/aimee_domain/src/result_stream_ext.rs:270-277`).
- `FinishReason::ContentFilter` with no tools is `Error::Refusal` and must **not** retry (`crates/aimee_domain/src/result_stream_ext.rs:262-268`).

Orchestrator retries wrap `execute_chat_turn`. Retry notifications use `try_send` so a full channel does not stall the backoff (`crates/aimee_app/src/orch.rs:282-307`). Provider-layer `into_retry` classifies transport, overloaded, and configured status codes as `Error::Retryable` (`crates/aimee_repo/src/provider/retry.rs:15-34`).

Tool start is a handshake, not fire-and-forget: orch sends `ToolCallStart { notifier }`, then `notifier.notified().await` before executing the tool (`crates/aimee_app/src/orch.rs:106-115`). The TUI `NotifyGuard` notifies on drop so a render error cannot deadlock orch (`crates/aimee_main/src/ui.rs:4837-4859`).

On a clean stop with no remaining tools, orch sends `TaskComplete` (`crates/aimee_app/src/orch.rs:446-449`).

## Markdown streaming

`StreamdownRenderer` buffers tokens until `\n`, repairs the line, parses, then renders (`crates/aimee_markdown_stream/src/lib.rs:76-94`). `finish()` flushes the tail and `parser.finalize()` (`crates/aimee_markdown_stream/src/lib.rs:96-109`).

Repair only splits an embedded closing fence (` ``` ` / `~~~`) when already inside a code block (`crates/aimee_markdown_stream/src/repair.rs:10-21`). Tests prove chunked Korean list/blockquote text keeps spaces (`crates/aimee_markdown_stream/src/lib.rs:147-194`).

`StreamingWriter` (`crates/aimee_main/src/stream_renderer.rs`):

- One active renderer at a time. Style change (`Normal` vs `Dimmed`) finishes the old renderer first (`crates/aimee_main/src/stream_renderer.rs:147-152`).
- Reasoning is dimmed (`write_dimmed` → `Style::Dimmed`) (`crates/aimee_main/src/ui.rs:4912-4913`, `crates/aimee_main/src/stream_renderer.rs:126-129`).
- `StreamDirectWriter` pauses the spinner, writes, flushes, and resumes only after a trailing `\n` (`crates/aimee_main/src/stream_renderer.rs:209-224`).
- `io::Write::write` returns `buf.len()`, not the styled (ANSI-expanded) length (`crates/aimee_main/src/stream_renderer.rs:226-230`).
- Width comes from `terminal_size`, default 80 (`crates/aimee_main/src/stream_renderer.rs:97-101`).

The TUI consume loop (`crates/aimee_main/src/ui.rs:4693-4715`, `crates/aimee_main/src/ui.rs:4815-4930`):

- Stream error → `writer.finish()`, spinner stop, return `Err`.
- Markdown delta → `writer.write`.
- Tool I/O and interrupts → `writer.finish()` first so a half-open fence cannot leak into the next block.

`partial` on `ChatResponseContent::Markdown` is stored but unused at the TUI match (`crates/aimee_main/src/ui.rs:4833`). Completeness is `writer.finish()`, not that flag.

## When to use which crate

| You are… | Use | Do not |
|---|---|---|
| Adding an OpenAI-compatible / Anthropic / Google provider with real SSE | `HttpInfra::http_eventsource` + `into_chat_completion_message` | Hand-roll a second SSE client |
| Talking to a host that lies about `Content-Type` (Codex, OpenCode Zen) | `http_post` + `bytes_stream().eventsource()` from `aimee_eventsource_stream` | Force `EventSource` — it will close on `InvalidContentType` (`crates/aimee_eventsource/src/event_source.rs:247-251`) |
| Parsing SSE in a test or from an in-memory byte stream | `aimee_eventsource_stream::Eventsource` | Pull in `reqwest` just to parse |
| Exposing orch / workspace progress to a surface | `aimee_stream::MpscStream` | A second unbounded channel “to keep it simple” |
| Rendering LLM markdown in the TUI | `StreamingWriter` → `StreamdownRenderer` | Print raw deltas; you will break fences and spinner ownership |
| Rendering markdown in a non-TUI writer | `StreamdownRenderer::new(writer, width)` | Depend on `aimee_main` |
| Aggregating a provider `BoxStream` | `ResultStreamExt::into_full` / `into_full_streaming` | Collect into `Vec` and re-join in the provider |

`aimee_eventsource` already depends on `aimee_eventsource_stream`. Do not add a second HTTP client or a second SSE parser.

## Best practices and failure modes

Only claims that are in the tree.

### Backpressure is capacity 1

`MpscStream::spawn` creates `tokio::sync::mpsc::channel(1)` (`crates/aimee_stream/src/mpsc_stream.rs:18`). The producer blocks on `send().await` until the TUI pulls. That is the backpressure. Do not raise the capacity without measuring: a larger buffer hides a stuck renderer and spends more memory on unread tokens.

`into_full_streaming` also `send().await`s on that same sender (`crates/aimee_domain/src/result_stream_ext.rs:125-144`). If the UI stops polling, the provider poll loop stalls. That is intended.

Retry events are the exception: `try_send` so a full channel cannot pin the retry timer (`crates/aimee_app/src/orch.rs:306`). A dropped retry notification is preferred to a stalled backoff.

### Drop cancels the producer

`Drop for MpscStream` closes the receiver and `join_handle.abort()` (`crates/aimee_stream/src/mpsc_stream.rs:34-39`). `test_drop_aborts_task` proves the spawned future does not complete after drop (`crates/aimee_stream/src/mpsc_stream.rs:64-99`).

Consequence: leaving `on_chat` (error path, user abort, or dropping the stream) aborts `orch.run()`. In-flight `send().await` then fails. `into_full_streaming` already ignores those failures (`crates/aimee_domain/src/result_stream_ext.rs:124`). `AimeeApp::chat` still tries to persist the conversation after `run()` returns (`crates/aimee_app/src/app.rs:204-210`) — but abort means that persist may not run. Do not assume a dropped stream left a saved conversation.

### Cancel vs retry vs refusal

| Signal | Behaviour | Cite |
|---|---|---|
| Drop `MpscStream` | Abort producer task | `crates/aimee_stream/src/mpsc_stream.rs:34-39` |
| `EventSource::close` | Stop reconnect, stream ends | `crates/aimee_eventsource/src/event_source.rs:93-96` |
| Transport / `StreamEnded` on `EventSource` | Retry with `DEFAULT_RETRY` | `crates/aimee_eventsource/src/event_source.rs:282-285`, `crates/aimee_eventsource/src/retry.rs:114-120` |
| Invalid status / content type on `EventSource` | Close, no retry | `crates/aimee_eventsource/src/event_source.rs:247-251` |
| Raw-SSE `Transport` | `Error::Retryable` | `crates/aimee_repo/src/provider/openai_responses/repository.rs:378-392` |
| Empty completion | Retryable | `crates/aimee_domain/src/result_stream_ext.rs:270-277` |
| `ContentFilter` / refusal | **Not** retryable | `crates/aimee_domain/src/result_stream_ext.rs:262-268` |

Do not fold refusal into `EmptyCompletion`. The test name is the contract (`crates/aimee_domain/src/result_stream_ext.rs:1232-1246`).

### Do not stampede

`EventSource` default backoff is 300 ms × 2^n, capped at 5 s (`crates/aimee_eventsource/src/retry.rs:114-120`). Orchestrator retries are a **second** loop around the whole turn (`crates/aimee_app/src/orch.rs:282-310`). A provider that already reconnects inside `EventSource` plus an orch retry can multiply requests. Prefer classifying the error once (`into_retry`) and keeping one retry owner.

`Never` exists if a caller must disable EventSource reconnect (`crates/aimee_eventsource/src/retry.rs:103-112`). No production call site currently installs it — do not assume reconnect is off.

### Do not hold UI locks across awaits

`SharedSpinner` locks, calls, unlocks (`crates/aimee_main/src/stream_renderer.rs:31-45`). `StreamDirectWriter` pauses, writes, flushes, then unlocks via the same pattern (`crates/aimee_main/src/stream_renderer.rs:209-224`). Do not hold the `Mutex<SpinnerManager>` across `stream.next().await`.

The tool handshake is the other lock-shaped hazard: orch waits on `Notify`. The TUI must notify even if `writer.finish()` fails (`crates/aimee_main/src/ui.rs:4837-4846`).

### Do not log secrets or full event payloads

Request headers are logged through `sanitize_headers`, which redacts `Authorization`, `x-api-key`, `x-goog-api-key`, and `api-key` (`crates/aimee_infra/src/http.rs:214-234`, used at `crates/aimee_infra/src/http.rs:209` and `crates/aimee_repo/src/provider/openai.rs:232`).

SSE adapters include `message.data` in parse-error context (`crates/aimee_repo/src/provider/event.rs:32-37`). That data can contain user text. Do not add `debug!(event = ?message)` of the full event. `into_chat_completion_message` already logs only a unit-style “Received completion from Upstream” on `[DONE]` (`crates/aimee_repo/src/provider/event.rs:28`).

`debug_requests` appends the **request body** to a configured path (`crates/aimee_infra/src/http.rs:237-248`). That file can contain prompts. Do not commit it.

### Invalid states

- `ReadyState` is `Connecting | Open | Closed` (`crates/aimee_eventsource/src/event_source.rs:40-47`).
- `EventStreamState` is `NotStarted | Started | Terminated` (`crates/aimee_eventsource_stream/src/event_stream.rs:117-121`).
- `ChatResponseContent` separates tool I/O from markdown (`crates/aimee_domain/src/chat_response.rs:11-17`). Do not stuff tool stdout into `Markdown`.

## Verify

From the Aimee Codes workspace:

```bash
cargo check -p aimee_stream -p aimee_eventsource -p aimee_markdown_stream
```

Targeted tests for the failure modes on this page:

```bash
cargo test -p aimee_stream
cargo test -p aimee_eventsource_stream
cargo test -p aimee_markdown_stream
cargo test -p aimee_domain into_full
```

From this docs repo:

```bash
python3 scripts/verify-docs.py
```
