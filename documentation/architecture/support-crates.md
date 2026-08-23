# Supporting crates

The workspace's specialist crates — small, single-purpose, and boring on purpose.

## Presentation helpers

| Crate | What you notice |
|---|---|
| `aimee_display` | Syntax-highlighted code, rendered diffs, formatted grep output |
| `aimee_markdown_stream` | Answers that render as they stream, not after |
| `aimee_spinner` | Progress indication during long tool calls |
| `aimee_select` | Fuzzy pickers (nucleo) behind `aimee select` and file tagging |
| `aimee_tracker` | Telemetry and the `VERSION` constant |
| `aimee_stream` | `MpscStream` channel plumbing for the pipeline |

## File and template machinery

| Crate | Job |
|---|---|
| `aimee_fs` | Tokio filesystem ops with uniform error context |
| `aimee_walker` | Directory traversal powering discovery |
| `aimee_embed` | Embeds assets (templates, defaults) via `include_dir` + Handlebars |
| `aimee_template` | Template `Element` model |
| `aimee_json_repair` | Fixes near-miss JSON from models before tool execution |
| `aimee_snaps` | Snapshot service backing `undo` |

## Streaming

| Crate | Job |
|---|---|
| `aimee_eventsource` | SSE over reqwest |
| `aimee_eventsource_stream` | Byte-stream SSE parsing into typed events |

## CI and tests

| Crate | Job |
|---|---|
| `aimee_ci` | Generates GitHub workflows (`gh-workflow`) — edit the generator, not YAML |
| `aimee_test_kit` | `fixture!` / `json_fixture!` shared test loaders |

Each crate follows the same house rules as the core: typed errors, no service-to-service calls, smallest correct change. See [Crate map](crates.md) for placement in the architecture.

## See also

* [Crate map](crates.md)
* [Streaming pipeline](../concepts/streaming.md)
* [Testing and evals](../operations/testing-evals.md)

<!-- sources: AIMEE.md §5,§13 -->
