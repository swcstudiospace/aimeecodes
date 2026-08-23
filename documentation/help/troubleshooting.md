# Troubleshooting

Symptom → cause → fix for the failure modes you're most likely to hit.

## Provider and auth

**"No credentials stored" / first-run loop**

You skipped or failed provider login. Run `aimee provider login` and complete the flow; verify with `aimee provider list`. Credentials land in `~/.aimee/.credentials.json`.

**401s after working fine**

Token expired (OAuth providers) or was rotated. `aimee provider logout <id>` then log in again. For SuperGrok (`xai_oauth`) re-run the device flow.

**Wrong model answering**

A session-scoped model override is active. The ZSH dispatcher sets session model/provider per shell; start a fresh shell or switch back via the dispatcher's model action.

## Config and paths

**Config changes not taking effect**

Check resolution order: `AIMEE_CONFIG` beats `OMEGA_CONFIG` beats existing directories. You may be editing a file that isn't in your resolved base. Confirm with `aimee info`, which shows the active config path.

**Old Omega Loops install interfering**

Legacy directories are picked up intentionally. Consolidate: `aimee config migrate`. See [Migrating from Omega Loops](migration.md).

## Shell plugin

**`:` lines not routing to aimee**

The plugin isn't loaded. Re-run `aimee setup`, then `exec zsh`. Diagnose with `aimee doctor`.

**Key bindings vanished after installing zsh-vi-mode**

vi-mode rebuilds keymaps, but the plugin re-applies its bindings automatically on vi-mode init. If they're still missing, your plugin load order changed — ensure aimee's plugin sources after zsh-vi-mode, then reload.

**Pasted paths not wrapping in @[]]**

Wrapping only happens on lines starting with `:`. Check that bracketed-paste mode is enabled in your terminal; the handler delegates formatting to `aimee zsh format --buffer`.

## Sessions

**Can't resume a conversation**

IDs are exact — list them first (`aimee conversation list`) rather than retyping. If the database moved (new config base), old IDs live under the old base until migrated.

**Context quality degrading in long sessions**

Compact: `aimee conversation compact --cid <id>`. Tune `[compact]` thresholds if it recurs.

## Tools and execution

**"Permission required" errors everywhere**

Restricted mode is on (`restricted = true`). Grant per prompt, or turn it off consciously in `.aimee.toml` — never silently in scripts.

**Tool calls timing out on big operations**

Raise `tool_timeout_secs`. If one command legitimately needs minutes, split the task instead of removing all bounds.

## Environment

**Build fails on Rust version mismatch**

The workspace pins 1.97 (`rust-toolchain.toml`) with MSRV 1.94. Ensure rustup has the pinned toolchain; Nix users get it automatically.

**Pod won't start**

Check runtime availability with `aimee pod doctor`, then `aimee pod status`. Image builds go through `aimee pod build`.

Still stuck? See [FAQ](faq.md) or file an issue at [swcstudiospace/omegaloops](https://github.com/swcstudiospace/omegaloops).

## See also

* [FAQ](faq.md)
* [Reliability and recovery](../operations/reliability.md)
* [CLI reference](../reference/cli.md)

<!-- sources: AIMEE.md §2,§6,§15, shell-plugin/README.md, doctor.zsh -->
