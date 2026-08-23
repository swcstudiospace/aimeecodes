# DevPod wrap (`omega pod`)

Upstream clone: `/root/src/repos/devpod` (`loft-sh/devpod`, Go). Installed CLI: `/usr/local/bin/devpod` (v0.6.15). Omega does **not** vendor Go — `crates/omega_main/src/pod.rs` maps `omega pod …` → `devpod …`. Override binary with `OMEGA_POD_BIN`.

## Rebranded verbs

`up` `list` (`--porcelain` → `--output json`) `stop` `delete` `ssh` `status` `logs` `build` `provider` `ide` `context` `machine` `pro` `use` `upgrade` `version` plus Omega-native `ui`. Aliases: `omega codespace`, `omega devpod`.

## Headless Linux → Mac Mini

Do not launch DevPod Desktop on the VPS. `omega pod ui` prints the path:

1. DevPod Desktop on the Mac Mini
2. SSH provider pointing at this host (`omega pod provider add ssh`)
3. `omega pod ssh <workspace> -L …` or `ssh -N -L 8080:127.0.0.1:8080 user@host` for browser IDEs

## `/goal` seam

- `/goal <text>` slugs `pod_id` (`omega-…`, max 40) and persists it; does **not** spawn DevPod
- `/goal pod` → `devpod up <cwd> --id <pod_id> --open-ide=false`
- `/goal pr` → `gh pr create --fill` (needs `gh` auth); stores `pr_url`
- Continuation prompt includes `Sandbox pod:` / `Pull request:` when set
- Old `goal.json` loads via `#[serde(default)]` on `pod_id` / `pr_url`

## Not wired

Anda dTEE, auto-PR on goal set, auto-`devpod up`, running DevPod Desktop on the headless host.
