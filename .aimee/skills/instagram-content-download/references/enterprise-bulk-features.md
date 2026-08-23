# Enterprise bulk features (Instagram archival)

When the user asks for “enterprise grade” or “bulk download all content at once,” prefer this ordered backlog. Ship 1–4 before dashboard chrome.

## 1. Multi-handle job queue

- Input: `handles.txt`, CSV (`handle,priority,tags`), or CLI nargs
- States: pending → running → done / failed / skipped
- One active profile job at a time unless workers (6) are enabled under a global QPS cap

## 2. Checkpoint + resume (SQLite)

- Tables: `jobs`, `job_items` (shortcode, type, status, path, error)
- Store last successful cursor / newest shortcode per handle
- `--resume` continues without re-downloading existing files (pair with Instaloader skip-existing)

## 3. Adaptive rate limiter

- Token bucket per session + global budget
- On 429: exponential backoff with jitter; mark session cooling
- Never spin multiple Instaloader processes back-to-back without shared limiter

## 4. Multi-account session pool

- Directory of session files; round-robin or least-recently-rate-limited
- Health check (`test_login`) before dequeue
- Isolate banned sessions; alert operator

## 5. Content-type matrix + filters

- Boolean matrix: posts, reels, stories, highlights, tagged, igtv, profile_pic
- Filters: since/until date, media type, min duration, exclude shortcodes file
- Document Memories still out of scope for third-party APIs

## 6. Bounded parallel workers

- N workers ≤ session pool size; shard by handle (not by media across one profile without care)
- Global max in-flight requests

## 7. Dedup + integrity

- Index: shortcode → path, sha256, size, mtime
- Skip if hash matches; optional `--verify` re-hash pass and re-fetch corrupt

## 8. Structured archive layout

- `org/campaign/handle/YYYY/mm/...` optional prefix
- Sidecar JSON per item + rollup CSV/Parquet inventory
- Per-run `manifest.json` and machine-readable exit codes

## 9. Observability

- CLI progress: counts, bytes, ETA, 429 events
- Optional local web UI or log drain
- Exportable audit log for compliance narratives

## 10. Scheduling and delivery

- Cron / incremental “fast-update” catch-up
- Webhooks (Slack etc.) on complete/fail
- Optional post-job `rclone` sync to S3/Drive

## Extras

- Browser cookie import (`--cookies-from-browser` pattern via gallery-dl)
- Proxy egress (only when user supplies lawful proxy infra)
- Dry-run estimate: post count × avg size → disk forecast

## Product messaging

Lead with capability matrix and Memories limitation before promising “download everything.” Full personal history including Memories = official Instagram data export only.
