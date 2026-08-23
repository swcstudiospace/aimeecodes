# fetch

`fetch` retrieves a URL and returns its content as markdown (or raw text). Input type: `NetFetch` (`crates/aimee_domain/src/tools/catalog.rs:628-638`). Description source: `descriptions/net_fetch.md`.

## Parameters

| Parameter | Type | Required | Default | Notes |
|---|---|---|---|---|
| `url` | string | yes | — | HTTP/HTTPS URL |
| `raw` | boolean | no | `false` | Skip HTML→markdown conversion |

## Example

```json
{
  "name": "fetch",
  "arguments": {
    "url": "https://docs.rs/tokio/latest/tokio/sync/trait.Mutex.html"
  }
}
```

## Behavior

- HTML is converted to readable markdown by default; `raw: true` returns unconverted content.
- **Text only.** Binary downloads (`.tar.gz`, `.zip`, `.bin`, `.deb`, images, audio, video) are rejected with an error — the contract routes binary downloads to [shell](shell.md) with `curl -fLo <output_file> <url>`.
- Cannot access private/authenticated resources; respects `robots.txt`; anti-scraping protections may block it.
- Large pages return the first 40,000 characters and store the complete content in a temporary file for follow-up [read](read.md) / [fs_search](fs_search.md).

## Errors

| Condition | Result |
|---|---|
| Binary content type | Error (use shell + curl) |
| Auth-required resource | Error |
| Blocked by robots.txt / anti-scraping | Error |
| Oversized page | Truncated to 40k chars + temp-file pointer |

## Permissions

Gated in restricted mode as a **Fetch** operation naming the URL (`catalog.rs:1002-1006`).

## Related

- [Tool catalog](catalog.md)
- [shell](shell.md) — binary downloads via curl
- [read](read.md) — page large-page temp files
