# Pod branding, first-boot hook, approval, swarm, Hermes skills

Canonical tree: `/root/src/repos/aimeecodes` (bin `aimee`).

## Clap

User-facing `///` on `aimee pod` / `--pod` must not say DevPod. Hidden alias `devpod` stays. Gate: `test_pod_help_is_aimee_branded`.

## First `aimee pod up`

`.devcontainer/post-create.sh` must `cargo install --locked cargo-nextest`. Unlocked install dies on `locked-tripwire` → `pod command exited 1`. `aimee pod up .` from repo root (or `./aimeecodes` from the parent). Default `--open-ide=false` unless the user passed `--open-ide`.

## Approval (Shift+Tab)

`ApprovalMode` in `aimee_domain`: Confirm → Auto → Yolo. rustyline `KeyCode::BackTab` (NONE and SHIFT). `Cmd::Interrupt` + flag so Ctrl+C stays Continue. Auto/Yolo skip `check_tool_permission` prompts and `should_continue`. Default Yolo.

## `/swarm`

`/swarm <text>` calls `on_goal` (five probes + `set_loop`) then the swarm command template. Loop continues until the goal judge completes.

## Hermes skills, not PyO3 tools

Do not wrap Hermes Python tools in PyO3. Load `~/.hermes/skills` at walker depth 3 (`hermes_skills_path`). `/learn` writes `SKILL.md` under `.aimee/skills`.

## Secrets

Pasted Linear/Greptile keys → `~/.config/aimee/secrets.env` mode 600. `load_aimee_secrets()` at process start. `scripts/greptile-pre-push.sh` sources that file. Never commit keys; tell the user to rotate after a chat paste.

## dTEE

Not a local pod runtime. ldclabs TEE is `anda-cloud` / `ic-tee` (ICP). Pods are Docker. See `omega-anda-pathways`.
