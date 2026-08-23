# Troubleshooting and FAQ

Common failure modes and their fixes, grounded in how the tree actually works. For configuration background start at [Configuration](../configuration.md); for loop-behavior knobs see [Reliability](../reliability.md).

## Install and startup

**`nix run github:swcstudiospace/omegaloops` fails**
Check the flake supports your platform (Linux and macOS, `x86_64` and `aarch64`). From a local checkout, `cargo install --path crates/aimee_main --locked --bin aimee` is the fallback. See [Install and Nix](../ops/install.md).

**First run asks for a provider login**
Expected: with no stored credentials Aimee walks you through provider login. Run `aimee provider login`, pick a provider, then start the TUI. Credentials land in `.credentials.json` under the config base — never in git.

**Where is my config?**
`aimee config path` prints it. Resolution order: `AIMEE_CONFIG` → `OMEGA_CONFIG` → first existing of `~/aimee`, `~/.aimee`, `~/omega`, `~/.omega` → Forge-legacy paths → `~/.aimee`. Coming from Omega Loops or Forge? See [Migration](migration.md).

## Providers

**"Provider not found" / model list empty**
Run `aimee provider list` — it is the source of truth for the 42 built-in IDs. Inline `[[providers]]` entries in `.aimee.toml` merge over built-ins when the `id` matches. OpenAI-compatible endpoints use the `openai_compatible` provider with a custom base URL.

**Auth errors after changing keys**
Re-run `aimee provider login` for that provider (it rewrites `.credentials.json`). Keys are not read from generic env vars.

## Tools and turns

**A tool call "hangs" then fails**
Every tool call is bounded by `tool_timeout_secs` (default 300 s); on expiry you get a timeout error reported in minutes. Lower/raise via config; details in [Reliability](../reliability.md).

**Agent keeps using `cat` / `grep` / `cd &&` instead of the tools**
That violates the tool contracts, and the eval suite tests against exactly this (`read_over_cat`, `search_over_find`, `redundant_cd_with_cwd`). Strong models follow the contracts; if you see drift, try a stronger model or check that tool descriptions weren't locally edited (see the repo's `docs/tool-guidelines.md, [Tool catalog](../reference/tools/catalog.md)).

**Restricted mode keeps prompting me**
Expected. In restricted mode every file write, delete, shell command, and fetch requires an explicit grant before execution. Disable with `restricted = false` only on machines you trust.

**`sem_search` returns nothing or times out**
The workspace must be indexed first (`aimee workspace sync`) and reachable at `services_url`. Queries that are too broad ("authentication") also degrade results — use specific phrasing plus intent, per [sem_search](../reference/tools/sem_search.md). Falls back to `fs_search` outside the workspace.

## Conversations

**Lost a thread / want yesterday's session back**
`aimee conversation list` then `aimee conversation resume <id>`. Conversations persist in local SQLite (`conversations` table) until deleted — see [Persistence](../architecture/persistence.md).

**Context too long mid-task**
`:compact` (or the `[compact]` config) shrinks the prompt window. Note `compact.retention_window` is a *prompt-window* knob, not a legal data-retention policy ([Reliability](../reliability.md)).

## ZSH plugin

**`:` lines aren't being rewritten**
Run `aimee setup` to install the plugin, restart the shell, confirm `AIMEE_BIN` points at a real binary. Diagnostics: `aimee doctor`. Full reference: [ZSH plugin](../zsh.md).

**File tagging `@` + Tab doesn't complete**
It shells out to `aimee select file`; run that command directly to see the error. Keyboard map lives at `aimee zsh keyboard`.

## PWA

**Service worker doesn't register**
Serve from inside `pwa/` (`python3 -m http.server 4173`) — serving the repo root 404s `./sw.js`. `file://` never registers a worker; localhost counts as a secure context. The PWA is a static shell: drafts stay on-device and there is no live agent API yet ([PWA](../surfaces/pwa.md)).

## FAQ

**Is my code sent anywhere?**
Only what you opt into: provider calls carry your prompts/tool results by definition; workspace indexing (`workspace sync`) uploads content to the services endpoint for semantic search; everything else (conversations, credentials, PWA drafts) stays local. Restricted mode gates all side effects.

**How do I add a custom agent or skill?**
Project-level `.aimee/agents/` and `.aimee/skills/<name>/SKILL.md`; global equivalents live under the config base. See [Skills and commands](../skills.md) and [The flock](../flock.md).

**Windows support?**
The documented surfaces are Linux/macOS (Nix matrix) plus the browser PWA. Check `aimee doctor` on your platform for concrete diagnostics.

**Still stuck?**
`aimee info` prints resolved config, version, and timeouts; `aimee logs` shows recent structured logs (`AIMEE_LOG=aimee=debug` for verbosity). Contact: ovesheng@spectrumweb.co.

## Related

- [Configuration](../configuration.md) · [Reliability](../reliability.md) · [Security](../security.md)
- [Migrating from Omega Loops](migration.md)
- [Glossary](glossary.md)
