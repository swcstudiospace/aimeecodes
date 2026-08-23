# Pods and sandboxes

Aimee has two isolation knobs. They are not interchangeable.

| Flag / command | What it actually is | Process boundary | Use when |
|---|---|---|---|
| `--sandbox <name>` | A **git worktree** next to the repo | Same host, same user, same Docker/kube credentials as you | Local experiment on a branch you trust |
| `--pod <id>` / `aimee pod …` | A **DevPod workspace** (container / SSH / cloud) | Separate runtime. Aimee only shells out to the `devpod` binary | Untrusted agent PRs, untrusted code, anything that must not share your laptop |

`--sandbox` never starts a container. `--pod` never creates a worktree. The CLI says so on the flags:

```53:60:crates/aimee_main/src/cli.rs
    /// Name for an isolated git worktree to create for experimentation.
    #[arg(long)]
    pub sandbox: Option<String>,

    /// Provision a DevPod workspace (docker/ssh/cloud) before the session.
    /// Unlike `--sandbox`, this starts a container, not a git worktree.
    #[arg(long)]
    pub pod: Option<String>,
```

`AIMEE.md` records the same split: `--sandbox` is a git worktree; `aimee pod` is a rebranded DevPod wrapper.

## Sandbox = git worktree

`--sandbox <name>` is handled in `crates/aimee_main/src/main.rs:110-123` **before** the session starts. It calls `Sandbox::create()` (`crates/aimee_main/src/sandbox.rs:20-139`).

What that function does:

1. Requires a git repository (`git rev-parse --is-inside-work-tree`). Outside git it bails.
2. Resolves the repo root (`git rev-parse --show-toplevel`).
3. Places the worktree at **`<git-root-parent>/<name>`** — sibling of the repo, not inside it. A repo at filesystem root cannot create a worktree.
4. If that path already exists **and** is a worktree, it is reused (`Worktree [Reused]`).
5. If that path exists and is **not** a worktree, it bails. Remove it or pick another name.
6. If branch `refs/heads/<name>` is missing, it runs `git worktree add -b <name> <path>` from current `HEAD`. If the branch exists, it checks that branch out into the new worktree.
7. Returns the canonicalized worktree path. That path becomes the session `cwd`.

`--sandbox` combined with `-C/--directory` appends the directory onto the worktree path (`crates/aimee_main/src/main.rs:111-115`).

A sandbox does **not**:

- Start Docker, DevPod, or a VM
- Drop privileges, hide host secrets, or isolate the network
- Stop an agent from reading `~/.aimee/.credentials.json`, SSH keys, or kubeconfigs on the same machine

Treat `--sandbox` as a **branch checkout**, not a security boundary.

```bash
# Isolated branch + worktree, same machine
aimee --sandbox experiment-wallet -p "try the login flow"

# Worktree, then a relative -C path inside it
aimee --sandbox experiment-wallet -C crates/aimee_domain
```

## Pod = DevPod container

`aimee pod` is a branded wrapper around the **Go DevPod CLI**. Aimee does not vendor DevPod. It maps `aimee pod <verb> …` onto `devpod <verb> …` and execs the binary (`crates/aimee_main/src/pod.rs:1-5`, `crates/aimee_main/src/pod.rs:163-194`).

The binary is `AIMEE_POD_BIN` when set, otherwise `devpod` on `PATH` (`crates/aimee_main/src/pod.rs:15-27`). If spawn fails, the error tells you to install DevPod or set that env var. Tokens are never captured.

Top-level `--pod <id>` is **not** the `aimee pod` subcommand. After sandbox/cwd resolution, `main` calls `provision_for_goal(id, cwd)` (`crates/aimee_main/src/main.rs:125-129`), which is `devpod up <source> --id <id> --open-ide=false` (`crates/aimee_main/src/pod.rs:53-70`). The session then continues on the host; `--pod` only provisions.

Aliases: `aimee codespace` and `aimee devpod` parse as `aimee pod` (`crates/aimee_main/src/cli.rs:165-167`).

Dispatch from the TUI/CLI: `TopLevelCommand::Pod` → `pod::run` (`crates/aimee_main/src/ui.rs:836-838`).

## What is forwarded to DevPod vs Aimee-native

`pod::argv` returns `Some(devpod-argv)` for every verb that shells out, and `None` for Aimee-native commands (`crates/aimee_main/src/pod.rs:107-156`). `run` short-circuits those natives and never spawns (`crates/aimee_main/src/pod.rs:163-174`).

| `aimee pod` verb | Spawn DevPod? | Mapped argv |
|---|---|---|
| `up` | Yes | `devpod up <args…>` |
| `list` | Yes | `devpod list` (+ `--output json` when `--porcelain`) |
| `stop` | Yes | `devpod stop <args…>` |
| `delete` | Yes | `devpod delete <args…>` |
| `ssh` | Yes | `devpod ssh <args…>` (DevPod accepts `-L` / `-R`) |
| `exec` | Yes | `devpod ssh <workspace> --command "<joined command>"` |
| `status` | Yes | `devpod status <args…>` |
| `logs` | Yes | `devpod logs <args…>` |
| `build` | Yes | `devpod build <args…>` |
| `provider` | Yes | `devpod provider <args…>` |
| `ide` | Yes | `devpod ide <args…>` |
| `context` | Yes | `devpod context <args…>` |
| `machine` | Yes | `devpod machine <args…>` |
| `pro` | Yes | `devpod pro <args…>` |
| `use` | Yes | `devpod use <args…>` |
| `upgrade` | Yes | `devpod upgrade <args…>` |
| `version` | Yes | `devpod version <args…>` |
| unknown verb (`External`) | Yes | forwarded **unchanged** as DevPod argv |
| **`ui`** | **No** | prints the headless → Mac Mini guide |
| **`doctor`** | **No** | probes binaries locally; never prints tokens |

`ui` is the Aimee-native surface the CLI docs call out (`crates/aimee_main/src/cli.rs:173-176`). `doctor` is also Aimee-native (`argv` returns `None` for `Ui | Doctor` at `crates/aimee_main/src/pod.rs:153`).

`aimee doctor` (no `pod`) is a **different command**: it is an alias for `aimee zsh doctor` (`crates/aimee_main/src/cli.rs:159-160`). Use `aimee pod doctor` for DevPod/Docker/`gh` reachability.

## Every `aimee pod` subcommand

Trailing args after each forwarded verb are passed through (`trailing_var_arg`, `allow_hyphen_values`) so DevPod flags work as they would on `devpod` itself (`crates/aimee_main/src/cli.rs:185-282`).

### `up` — start or create

```bash
aimee pod up . --id agent-pr
aimee --pod agent-pr -p "run the untrusted PR checks"
```

`--pod` / `/goal pod` always add `--open-ide=false` (headless; no local IDE). Raw `aimee pod up` forwards whatever you type.

### `list` — list workspaces

```bash
aimee pod list
aimee pod list --porcelain   # becomes: devpod list --output json
```

`--porcelain` is Aimee’s JSON switch, rewritten in `argv` (`crates/aimee_main/src/pod.rs:122-129`).

### `stop` / `delete`

```bash
aimee pod stop agent-pr
aimee pod delete agent-pr
```

### `ssh` — interactive shell, port forwards

```bash
aimee pod ssh agent-pr
aimee pod ssh agent-pr -L 8080:127.0.0.1:8080
```

Docstring: forwards `-L` / `-R` (`crates/aimee_main/src/cli.rs:208`).

### `exec` — one command inside the workspace

Unlike the other verbs, `exec` is **not** a 1:1 DevPod verb. Aimee requires a workspace id (the `--id` from `up`) and a command, then runs `devpod ssh <workspace> --command "<joined>"` (`crates/aimee_main/src/cli.rs:213-220`, `crates/aimee_main/src/pod.rs:134-141`).

```bash
aimee pod exec agent-pr cargo test -p aimee_domain
```

The command vector is joined on spaces. Quote if you need a single remote argv with spaces.

### `status` / `logs` / `build`

```bash
aimee pod status agent-pr
aimee pod logs agent-pr
aimee pod build .
```

### `provider` / `ide` / `context`

```bash
aimee pod provider add docker
aimee pod provider list
aimee pod ide list
aimee pod context list
```

These nest the DevPod subcommand: `aimee pod provider add docker` → `devpod provider add docker`.

### `machine` / `pro` / `use` / `upgrade` / `version`

Forwarded the same way. Present on `PodCommand` (`crates/aimee_main/src/cli.rs:251-275`). Use them only when the installed DevPod binary supports that verb.

### `ui` — Aimee-native, no spawn

Prints how to watch pods from a Mac Mini when the Aimee host is headless (`crates/aimee_main/src/pod.rs:320-348`):

- Do not launch DevPod Desktop on the Linux box.
- Docker is the default provider on that host.
- On the Mini: install DevPod Desktop, add an SSH provider to `user@host`, or tunnel `ssh -N -L 8080:127.0.0.1:8080 user@host`.
- Workspaces started on the host: `/goal <text>` → `/goal pod` → `aimee pod ssh <id>` → `/goal pr`.
- States that **Anda dTEE is not in this Aimee tree**.

```bash
aimee pod ui
aimee codespace ui
```

### `doctor` — Aimee-native readiness

`collect_doctor` (`crates/aimee_main/src/pod.rs:213-249`) probes:

| Check | How | Tokens? |
|---|---|---|
| `devpod` | `<binary> version` exit status | Never captured |
| `docker` | `docker info` (stdout/stderr discarded) | No |
| `gh` | `gh auth status` (stdout/stderr discarded) | No — only success/fail |
| providers | `devpod provider list`, first-column names | Names only |
| Mac Mini hint | `USER` + first non-loopback `hostname -I` | Host identity, not secrets |
| Anda dTEE | Hard-coded `false` | Not shipped |

Empty providers print `(none — aimee pod provider add docker)`. Ready loop printed: `/goal <text> → /goal pod → /goal pr`.

```bash
aimee pod doctor
```

### Unknown verbs

`PodCommand::External` forwards the argv unchanged (`crates/aimee_main/src/cli.rs:280-282`, `crates/aimee_main/src/pod.rs:154`). Use this only for DevPod verbs Aimee has not named.

## When to use a pod for untrusted agent PRs

The product comment on the command is the policy: **isolated DevPod workspace for untrusted agent PRs** (`crates/aimee_main/src/cli.rs:165`, `crates/aimee_main/src/pod.rs:1`).

Use a **pod** when the agent will:

- Check out or apply a third-party / agent-authored PR
- Run builds, tests, or package installs from that tree
- Need a throwaway filesystem that you can `aimee pod delete`
- Must not inherit your laptop’s Docker socket, cloud creds, or home directory by default

Stay on a **sandbox (worktree)** when you:

- Already trust the branch (your own experiment)
- Only need a second checkout, not a container
- Accept that the process still runs as you

`--sandbox` is the wrong tool for untrusted PRs. It shares the host.

### Goal loop (TUI)

`/goal` slugs a workspace id (`aimee-` + ASCII alphanumerics, max 40 chars; empty slug becomes `aimee-goal`) and can provision/exec/PR (`crates/aimee_main/src/pod.rs:29-51`, `crates/aimee_main/src/ui.rs:2572-2621`):

```text
/goal <text>          # set loop; attach sandbox id; does not start DevPod
/goal pod             # devpod up <cwd> --id <id> --open-ide=false
/goal exec <command>  # same as aimee pod exec <id> …
/goal pr              # gh pr create --fill  (requires authenticated gh)
/goal status          # shows pod id and PR URL when attached
```

`/goal exec` fails until `/goal pod` (or an attached `pod_id`) exists. `/goal pr` runs `gh` on the **host**, not inside the workspace (`crates/aimee_main/src/pod.rs:83-105`). Review the diff before you treat that PR as trusted.

## Cluster / Kubernetes

The **aimeecodes** tree does **not** talk to Kubernetes. There is no kube client crate, no in-tree manifests, and `aimee pod` never calls `kubectl`. Isolation here is DevPod (typically Docker) or a git worktree.

`plat-k8s` and `k8s-review` are **agent/command personas** for reviewing cluster YAML in *other* repositories. They are not a cluster runtime for Aimee.

EKS / kind for Spectrum Web Co lives in the sibling **`spectrum/`** repo. Do not look for those charts inside aimeecodes.

## Best practices and safety

**Do**

- Run `aimee pod doctor` before the first untrusted PR. Fix `devpod` / `docker` / `gh` / provider gaps it reports.
- Give every untrusted run a distinct `--id`. Goal ids are slugged; raw `up --id` is whatever you pass.
- Prefer `aimee pod exec <id> …` over copying secrets into the workspace.
- `aimee pod stop` when idle; `aimee pod delete` when the PR is merged or abandoned.
- Keep provider credentials in DevPod / `gh` / Docker’s own stores. Aimee’s doctor explicitly **never includes tokens** (`crates/aimee_main/src/pod.rs:196`).
- On a headless Linux host, use Docker as the provider and `aimee pod ui` for Mini access. Do not expect a local desktop IDE (`--open-ide=false` on agent `up`).
- Set `AIMEE_POD_BIN` only for tests or a pinned DevPod install.

**Do not**

- Confuse `--sandbox` with a container. Worktrees are not a sandbox runtime.
- Confuse `aimee doctor` (zsh) with `aimee pod doctor` (DevPod).
- Claim Anda dTEE is wired. `PodDoctor.dtee` is always `false`; both `ui` and `doctor` say it is not in this tree (`crates/aimee_main/src/pod.rs:209-210`, `crates/aimee_main/src/pod.rs:341`).
- Put API keys, `.env`, or `.credentials.json` in pod YAML, docs, or `exec` command lines.
- Treat `gh pr create --fill` as a security review. Isolation ends when the PR lands on a machine you trust.
- Invent Kubernetes manifests or `kubectl` workflows inside aimeecodes. They are not there.
- Forward unknown `External` verbs you have not checked against your DevPod version.

## Verify

From the product tree (not this docs repo):

```bash
cargo test -p aimee_main --lib -- pod::tests
# covers argv forwarding, --porcelain → json, ui/doctor native, slug, exec → ssh --command
```

From this docs repo:

```bash
python3 scripts/verify-docs.py
```

DevPod itself is an external binary. Aimee’s tests do not start Docker.
