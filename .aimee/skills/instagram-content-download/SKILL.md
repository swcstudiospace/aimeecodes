---
name: instagram-content-download
description: "Use when archiving Instagram posts/reels/stories locally."
version: 1.0.0
category: social-media
tags: [instagram, instaloader, gallery-dl, archival, scraping, bulk-download]
---

# Instagram content download (local archival)

Build or operate **local** tools that archive public (or auth-visible) Instagram media to disk. Default stack: **Instaloader** (+ optional **gallery-dl**). Project from this environment: `/root/instagram-downloader` (`igdl` CLI).

## Capability matrix (set expectations first)

| Content | Public, no login | Logged-in session | Notes |
|--------|------------------|-------------------|--------|
| Posts (photo/video/carousel) | Yes | Yes | Core path |
| Reels | Yes | Yes | Feed + reels tab when API allows |
| Profile pic | Yes | Yes | |
| Live Stories (~24h) | Sometimes | Usually required | Ephemeral |
| Highlights | Often needs login | Yes | Long-term story archive |
| Tagged / IGTV | Optional flags | Yes | |
| Private profile | No | Only if session follows them | |
| **Memories (other accounts)** | **Never** | **Never** | Private to owner |
| **Own Memories via third-party API** | **No** | **No** | Official IG data export only |
| Expired stories / DMs / close-friends | No (unless session can see) | Limited | |

**Always tell the user up front:** Memories of another person cannot be scraped. Own full history → Instagram Settings → Your activity → Download your information.

## Prerequisites

- Python 3.11+, venv or `uv`
- `instaloader>=4.14`, optional `gallery-dl`, CLI UX: `click` + `rich`
- Prefer a **secondary** IG account for login — never automate the user's main personal login without explicit consent
- Do **not** ask the user to paste passwords into chat; use interactive `getpass` / `igdl login`

## Standard build shape (CLI app)

```
project/
  src/<pkg>/
    cli.py           # click: download | login | info | about-limits
    downloader.py    # Instaloader wrapper, options dataclass, manifest
  downloads/         # default output root
  requirements.txt / pyproject.toml
  README.md          # capability matrix + rate-limit ethics
```

### Instaloader wrapper rules

1. **Output:** `downloads/<handle>/` with posts at root; subdirs `reels/`, `stories/`, `highlights/<title>/`.
2. **Sidecars:** `save_metadata=True`, caption `.txt`, run-level `download_manifest.json` (counts, errors, timestamps).
3. **Rate courtesy:** default sleep ~1.5–3s between items; expose `--sleep`.
4. **Fail fast on 429:** set `max_connection_attempts=1` (stored on `loader.context.max_connection_attempts`). Instaloader's default multi-attempt backoff can park the CLI for **10+ minutes** (e.g. ~666s) — feels hung.
5. **Session path:** `~/.config/igdl/session-<username>`; try load + `test_login()` before password; support 2FA via `TwoFactorAuthRequiredException`.
6. **Private profiles:** detect `profile.is_private` and exit with clear “login + must follow” message before bulk work.
7. **Empty handle:** validate `handle.lstrip("@").strip()` early.

### CLI commands worth shipping

- `download HANDLE` — toggles: `--posts/--reels/--stories/--highlights/--tagged/--igtv`, `--max-posts`, `--fast-update`, `--login`, `-o`
- `login USER` — interactive password, save session
- `info HANDLE` — metadata only (also `max_connection_attempts=1`)
- `about-limits` — print capability matrix (Memories callout)

## Runtime workflow

```bash
cd /root/instagram-downloader && source .venv/bin/activate   # or user machine path
igdl info <handle>                                          # smoke connectivity
igdl login SECONDARY_USER                                   # if stories/private/429 guest wall
igdl download <handle> --login SECONDARY_USER --sleep 3
# bulk sample / test
igdl download <handle> --max-posts 5 --no-stories
```

### Pre-flight connectivity checks

Before promising a full archive:

1. `curl -sI -A "Mozilla/5.0 ..." https://www.instagram.com/<handle>/`  
   - **302 → accounts/login** on datacenter/shared IPs = guest wall; **session required**
2. Unauthenticated Instaloader `Profile.from_username` may return **HTTP 429**
3. No session files under `~/.config/igdl/` → do not claim download will succeed; ask user to run `igdl login` locally

## Pitfalls

- **Memories scope creep:** Users often say “all content including memories.” Correct once, firmly; offer official export for own account only.
- **Long Instaloader retry = fake hang:** Always low `max_connection_attempts` in agent-driven/CI contexts.
- **Guest 429 / login redirect:** Common on cloud/VPS IPs. Fix = user session/cookies on a residential path — not “retry harder” in a loop.
- **Password in chat:** Refuse; interactive login or browser cookie import only.
- **Reels double-count:** Reels may appear in `get_posts()` and a reels tab; document layout; optional separate `reels/` target is fine.
- **Stories empty:** Normal if none live and highlights not requested/login missing.
- **Parallel bulk without limiter:** Multiple Instaloader instances in short sequence trigger 429; single-flight or shared rate budget.
- **ToS / ethics:** Personal archival and research framing; secondary account; no commercial abuse guidance that bypasses platform rules.

## Enterprise bulk backlog (when user asks to scale)

Implement in this order:

1. Multi-handle job queue (`handles.txt` / CSV)
2. SQLite checkpoint + resume (per-handle cursor, failed shortcodes)
3. Adaptive rate limiter (token bucket, 429 jitter backoff)
4. Multi-account session pool (round-robin, health check)
5. Filters (date range, media type, exclude list)
6. Bounded parallel workers under global QPS
7. Dedup index (shortcode + content hash) + integrity re-fetch
8. Structured archive + Parquet/CSV inventory
9. Progress dashboard / run audit log
10. Cron incremental catch-up + webhooks + optional rclone/S3

Details: `references/enterprise-bulk-features.md`, `references/igdl-project.md`.

## Verification (offline first)

Do **not** require live IG for green checks when the IP is blocked:

- Layout, imports, CLI `--help`/`--version`, missing-session exit code
- Constructor + empty-handle `ValueError`
- `compileall`
- Ad-hoc script under `/tmp/hermes-verify-*` then delete

Live download only after session exists; start with `--max-posts 3`.

## Related tools

- **Instaloader** — primary (profiles, stories, highlights, reels flags)
- **gallery-dl** — alternate extractor; often needs cookies file; `include: posts,stories,reels,highlights`
- Official IG data export — only complete path for **own** Memories and full account dump

## References

- `references/igdl-project.md` — paths, commands, session layout for the local IGDL app
- `references/enterprise-bulk-features.md` — bulk/enterprise feature definitions
