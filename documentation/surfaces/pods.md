# Pods and sandboxes

Two isolation mechanisms, two different tools. Knowing which is which matters: one is a git worktree on your machine, the other is a container workspace.

## Sandbox: a git worktree

The `--sandbox` flag runs Aimee against a **git worktree** — a separate checkout of your repo sharing the same object store. No containers involved.

```bash
aimee --sandbox
```

Use it when you want the flock's changes isolated from your working tree but are fine with everything happening locally. Fast to start, no images to build.

## Pods: container workspaces

`aimee pod` provisions isolated **container** workspaces — the right tool for untrusted PRs, risky experiments, or reproducible environments:

```bash
aimee pod up          # start or create an isolated workspace
aimee pod list        # list workspaces (--porcelain for JSON)
aimee pod status      # workspace status
aimee pod stop        # stop a workspace
aimee pod delete      # delete a workspace
aimee pod ssh         # SSH in; forwards -L / -R tunnels
aimee pod exec        # run a command inside a workspace
aimee pod logs        # workspace logs
aimee pod build       # build a workspace image
```

Workspace IDs derive deterministically from the goal (`workspace_id_for_goal`), so re-running provisioning for the same goal targets the same workspace. There's also an Aimee-native surface:

```bash
aimee pod ui
```

Beyond the core lifecycle, `pod` carries provider/IDE/context/machine subcommands and hosted-platform commands (`pro`), plus `use` (select default resources), `upgrade` (pod runtime), and `doctor` diagnostics (`PodDoctor`).

## Attaching to an existing workspace

```bash
aimee pod connect <workspace>
```

`connect` attaches your Aimee TUI to an existing workspace — it does not SSH. Under the hood this is the Anda bridge: `prepare_connect` probes the workspace's Anda nexus endpoint and then connects through it. The probe result is rendered as a readable status before attachment.

This is the extent of the integration: a KIP/Anda-aware connect path plus SSH-style access via `pod ssh`. Nothing more should be assumed about execution guarantees inside workspaces.

## Choosing between them

| | Sandbox | Pod |
|---|---|---|
| Mechanism | Git worktree | Container workspace |
| Startup | Instant | Image pull/build |
| Isolation | Working-tree level | Machine-level |
| Best for | Parallel local changes | Untrusted code, reproducible envs |
| Remote access | None | SSH with tunnel forwarding |

## See also

* [Terminal UI](tui.md)
* [Security model](../operations/security.md)
* [Anda / KIP pathways](../integrations/anda-kip.md)

<!-- sources: crates/aimee_main/src/cli.rs (pod group), crates/aimee_main/src/pod.rs, src/sandbox.rs, AIMEE.md §6 -->
