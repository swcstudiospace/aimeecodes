# IGDL local project (Hermes environment)

## Location

- Root: `/root/instagram-downloader`
- Package: `src/igdl/` (`cli.py`, `downloader.py`)
- Entrypoint: `igdl` (after `pip install -e .` / `uv pip install -e .`)
- Launcher: `./igdl.sh`
- Default output: `downloads/<handle>/`
- Sessions: `~/.config/igdl/session-<username>`

## Install

```bash
cd /root/instagram-downloader
python3 -m venv .venv && source .venv/bin/activate
# or: uv venv .venv && source .venv/bin/activate
uv pip install -r requirements.txt && uv pip install -e .
# deps: instaloader, rich, click (+ optional gallery-dl)
```

## Commands

```bash
igdl --help
igdl about-limits
igdl info <handle>
igdl login <secondary_user>          # interactive password; 2FA supported
igdl download <handle> [options]
igdl download aimeekimm --login USER --sleep 3 -o ./downloads
igdl download handle --max-posts 5 --no-stories --no-highlights
```

## Implementation notes

- `InstagramDownloader` wraps Instaloader; options via `DownloadOptions`.
- `max_connection_attempts=1` passed into Instaloader; live value is `loader.context.max_connection_attempts` (not on the Instaloader instance itself).
- Missing `--login` session → exit code **2** quickly (do not hang).
- Manifest: `downloads/<handle>/download_manifest.json`.

## Observed host behavior (do not treat as permanent global truth)

On some cloud/shared IPs:

- Unauthenticated profile fetch → HTTP **429**
- Browser-like GET → **302** to `/accounts/login/`

Mitigation: valid session from `igdl login` on a less-blocked network; raise `--sleep`; avoid parallel scrapers. Re-check connectivity each session rather than assuming permanent block.

## Offline verification pattern

Ad-hoc script under `/tmp/hermes-verify-igdl-*.py`: layout, imports, defaults, constructor, CLI help, missing-session exit, `compileall`. Delete after run. Live IG not required for offline pass.
