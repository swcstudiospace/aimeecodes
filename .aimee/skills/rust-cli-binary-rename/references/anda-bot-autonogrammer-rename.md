# anda → autonogrammer rename map

Package stays `anda_bot`. Home stays `~/.anda`. Launcher stays `anda_launcher`.

## Must-change paths

| Layer | Path | Note |
|-------|------|------|
| Cargo | `anda_bot/Cargo.toml` | `default-run` + `[[bin]] name = "autonogrammer"` |
| Updater | `anda_bot/src/cli/updater.rs` | `BINARY_NAME`; asset unit tests |
| CLI UX | `anda_bot/src/main.rs` | clap about/examples/status; `try_parse_from` |
| Auto-update tests | `anda_bot/src/auto_update.rs` | asset name fixtures |
| Launcher discover | `anda_bot/src/bin/anda_launcher/core.rs` | sibling + PATH candidates |
| Launcher tests | `…/macos.rs` | temp `autonogrammer` paths |
| Install | `scripts/install.sh`, `install.ps1` | `BINARY_NAME=autonogrammer` |
| Homebrew | `scripts/publish-homebrew.sh` | asset URLs + `bin.install =>` |
| Windows pack | `scripts/build-windows-installer.ps1` | staging `autonogrammer.exe` |
| CI | `.github/workflows/release.yml` | `cp …/autonogrammer$ext release/autonogrammer-$target$ext` |
| Docs | README*, docsite/**, AGENTS, SOUL | command forms only |
| Website unstuck | `website/src/lib/content/branding.ts` | `` `autonogrammer status` `` |
| Skills eval | `skills/skill-creator/scripts/run_eval.py` | default `ANDA_COMMAND` |

## Protect from greedy replace

- `anda_bot` package
- `anda-bot` GitHub repo slug
- `anda_launcher`
- `~/.anda` / `ANDA_HOME`
- `anda-skills.zip`
- Internal agent `name`/`handle` test fixtures (product identity ≠ CLI argv0)

## Verify used

```bash
cargo build -p anda_bot --bin autonogrammer
./target/debug/autonogrammer --help   # Usage: autonogrammer
cargo test -p anda_bot -- release_target_matches_published_assets status_report
make lint
make test   # 962 passed in the rename session
```

## Related product surfaces (not this rename)

- TUI brand strings → `ratatui-agent-tui`
- Marketing dual-brand → `sveltekit-marketing-landing`
- Project SOUL map → `hermes-project-soul` (update CLI one-liner to `autonogrammer`)
