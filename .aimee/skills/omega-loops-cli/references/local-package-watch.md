# Local debug package + watch

Keep `aimee` on PATH as a **debug symlink**, then rebuild when the working tree is newer than the binary.

## Layout

| Piece | Value |
|-------|--------|
| Checkout | `/root/src/repos/aimeecodes` |
| Script | `scripts/dev-aimee.sh` |
| Binary crate | `aimee_main` / `[[bin]] name = "aimee"` |
| Debug bin | `$REPO/target/debug/aimee` |
| PATH shim | `~/.local/bin/aimee` → debug bin |
| Watch roots | `crates/`, `templates/`, `shell-plugin/`, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml` |
| User unit | `~/.config/systemd/user/aimee-local-package.service` (`ExecStart=…/dev-aimee.sh watch`) |

`templates/` and `shell-plugin/lib` are `include_dir!` embeds — they must be in the watch set.

## Commands

```bash
scripts/dev-aimee.sh sync      # no-op when current
scripts/dev-aimee.sh install   # force cargo build -p aimee_main + relink
scripts/dev-aimee.sh watch     # inotify (else poll) + 20s quiet debounce + sync
```

`AIMEE_PACKAGE_DEBOUNCE` (default 20) and `AIMEE_PACKAGE_POLL` (default 8) override timing. `flock` on `${XDG_RUNTIME_DIR:-/tmp}/aimee-dev-package.lock` so watch + a manual install do not run two cargos.

## Durable automation

```bash
systemctl --user enable --now aimee-local-package.service
systemctl --user status aimee-local-package
journalctl --user -u aimee-local-package -f
```

Linger is already on for root on this VPS. The unit must put `/root/.cargo/bin` on `PATH`.

## Do not

- Hermes `cronjob` for the rebuild — 3-minute hard interrupt vs cargo.
- systemd `PathChanged=` on `crates/` — watches one directory level, not `*.rs` under crate dirs.
- Git `post-commit` on this checkout — there may be no `.git`; do not `git init`.
- `cargo install --path` or `cargo build --release` for local CLI testing.

Debounce exists so mid-edit saves do not thrash cargo. After a session of writes, one quiet window then one incremental build is enough.
