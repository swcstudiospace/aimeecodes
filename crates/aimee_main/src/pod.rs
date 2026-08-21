//! DevPod wrap: isolated workspaces for untrusted agent PRs.
//!
//! Aimee stays the orchestrator. The Go DevPod binary is the sandbox runtime
//! (`docker`, `ssh`, cloud providers). This module maps `aimee pod …` onto
//! `devpod …` so the CLI surface is Aimee-branded without vendoring Go.

use std::path::PathBuf;
use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};
use bstr::ByteSlice;

use crate::cli::{PodCommand, PodCommandGroup};

/// Environment override for the DevPod binary (tests + custom installs).
pub const POD_BIN_ENV: &str = "AIMEE_POD_BIN";

/// Resolves the DevPod executable.
///
/// # Returns
///
/// `AIMEE_POD_BIN` when set, otherwise `devpod` on `PATH`.
pub fn binary() -> PathBuf {
    std::env::var_os(POD_BIN_ENV)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("devpod"))
}

/// Slug used as DevPod workspace id for a standing `/goal`.
///
/// ASCII alphanumerics only, `aimee-` prefix, max 40 characters.
pub fn workspace_id_for_goal(goal: &str) -> String {
    let mut slug = String::from("aimee-");
    for ch in goal.chars() {
        if slug.len() >= 40 {
            break;
        }
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
        } else if !slug.ends_with('-') {
            slug.push('-');
        }
    }
    while slug.ends_with('-') {
        slug.pop();
    }
    if slug == "aimee" {
        slug.push_str("-goal");
    }
    slug
}

/// Trailing args for `devpod up` on an agent sandbox (no local IDE on headless).
pub fn agent_up_args(id: &str, source: &str) -> Vec<String> {
    vec![
        source.to_string(),
        "--id".into(),
        id.to_string(),
        "--open-ide=false".into(),
    ]
}

/// Starts a DevPod workspace for the given goal id.
///
/// # Errors
///
/// Returns when DevPod is missing or `up` fails.
pub fn provision_for_goal(id: &str, source: &str) -> Result<()> {
    run(&PodCommandGroup { command: PodCommand::Up { args: agent_up_args(id, source) } })
}

/// Runs `command` inside an existing workspace (`devpod ssh --command`).
///
/// # Errors
///
/// Returns when DevPod is missing or the remote command fails.
pub fn exec_in_workspace(id: &str, command: &[String]) -> Result<()> {
    run(&PodCommandGroup {
        command: PodCommand::Exec { workspace: id.to_string(), command: command.to_vec() },
    })
}

/// Opens a GitHub pull request with `gh pr create --fill`.
///
/// # Errors
///
/// Returns when `gh` is missing or the create fails.
pub fn open_pull_request() -> Result<String> {
    let output = Command::new("gh")
        .args(["pr", "create", "--fill"])
        .output()
        .context("failed to spawn gh (install GitHub CLI to open a PR)")?;
    let stdout = output.stdout.to_str_lossy().trim().to_string();
    let stderr = output.stderr.to_str_lossy().trim().to_string();
    if !output.status.success() {
        bail!("gh pr create failed: {stderr}");
    }
    let url = stdout
        .lines()
        .rev()
        .find(|line| line.contains("github.com") || line.starts_with("http"))
        .unwrap_or(stdout.as_str())
        .to_string();
    Ok(url)
}

/// Maps an Aimee pod command onto DevPod argv (no binary name).
///
/// # Returns
///
/// `None` for Aimee-native commands that must not call DevPod (`ui`).
pub fn argv(command: &PodCommand) -> Option<Vec<String>> {
    let prepend = |head: &str, rest: &[String]| -> Vec<String> {
        let mut out = Vec::with_capacity(rest.len() + 1);
        out.push(head.to_string());
        out.extend(rest.iter().cloned());
        out
    };

    Some(match command {
        PodCommand::Up { args } => prepend("up", args),
        PodCommand::List { porcelain, args } => {
            let mut out = vec!["list".to_string()];
            if *porcelain {
                out.push("--output".into());
                out.push("json".into());
            }
            out.extend(args.iter().cloned());
            out
        }
        PodCommand::Stop { args } => prepend("stop", args),
        PodCommand::Delete { args } => prepend("delete", args),
        PodCommand::Ssh { args } => prepend("ssh", args),
        PodCommand::Exec { workspace, command } => {
            vec![
                "ssh".into(),
                workspace.clone(),
                "--command".into(),
                command.join(" "),
            ]
        }
        PodCommand::Status { args } => prepend("status", args),
        PodCommand::Logs { args } => prepend("logs", args),
        PodCommand::Build { args } => prepend("build", args),
        PodCommand::Provider { args } => prepend("provider", args),
        PodCommand::Ide { args } => prepend("ide", args),
        PodCommand::Context { args } => prepend("context", args),
        PodCommand::Machine { args } => prepend("machine", args),
        PodCommand::Pro { args } => prepend("pro", args),
        PodCommand::Use { args } => prepend("use", args),
        PodCommand::Upgrade { args } => prepend("upgrade", args),
        PodCommand::Version { args } => prepend("version", args),
        PodCommand::Ui | PodCommand::Doctor => return None,
        PodCommand::External(args) => args.clone(),
    })
}

/// Runs a pod command. `ui` prints Mac Mini access notes and does not spawn.
///
/// # Errors
///
/// Returns when the DevPod binary is missing or exits non-zero.
pub fn run(group: &PodCommandGroup) -> Result<()> {
    match group.command {
        PodCommand::Ui => {
            print_ui_guide();
            return Ok(());
        }
        PodCommand::Doctor => {
            print_doctor(&collect_doctor());
            return Ok(());
        }
        _ => {}
    }

    let args = argv(&group.command).expect("ui is handled above");
    let bin = binary();
    let mut cmd = Command::new(&bin);
    cmd.args(&args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    let status = cmd.status().with_context(|| {
        format!(
            "failed to spawn {} (install DevPod or set {POD_BIN_ENV})",
            bin.display()
        )
    })?;
    if !status.success() {
        let code = status.code().unwrap_or(1);
        bail!("pod command exited {code}");
    }
    Ok(())
}

/// Readiness snapshot for agent sandboxes. Never includes tokens.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PodDoctor {
    /// DevPod binary on PATH (or `AIMEE_POD_BIN`).
    pub devpod: bool,
    /// Local Docker daemon reachable.
    pub docker: bool,
    /// `gh` authenticated enough to open a PR.
    pub gh: bool,
    /// Installed DevPod provider names.
    pub providers: Vec<String>,
    /// `user@host` for Mac Mini SSH / DevPod Desktop SSH provider.
    pub ssh_hint: String,
    /// Anda dTEE is not shipped in this tree.
    pub dtee: bool,
}

/// Probe binaries and providers. Tokens are never captured.
pub fn collect_doctor() -> PodDoctor {
    let devpod = Command::new(binary())
        .arg("version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    let docker = Command::new("docker")
        .args(["info"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    let gh = Command::new("gh")
        .args(["auth", "status"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    let providers = Command::new(binary())
        .args(["provider", "list"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|t| parse_provider_names(&t))
        .unwrap_or_default();
    PodDoctor {
        devpod,
        docker,
        gh,
        providers,
        ssh_hint: ssh_hint(),
        dtee: false,
    }
}

/// First column of `devpod provider list` table rows.
pub fn parse_provider_names(table: &str) -> Vec<String> {
    table
        .lines()
        .filter(|line| line.contains('|') && !line.contains("---"))
        .filter_map(|line| {
            let name = line.split('|').next()?.trim();
            if name.is_empty()
                || name.eq_ignore_ascii_case("name")
                || !name
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
            {
                None
            } else {
                Some(name.to_string())
            }
        })
        .collect()
}

fn ssh_hint() -> String {
    let user = std::env::var("USER").unwrap_or_else(|_| "root".into());
    let ip = first_non_loopback_ip().unwrap_or_else(|| "<host-ip>".into());
    format!("{user}@{ip}")
}

fn first_non_loopback_ip() -> Option<String> {
    let output = Command::new("hostname").arg("-I").output().ok()?;
    String::from_utf8(output.stdout)
        .ok()?
        .split_whitespace()
        .find(|ip| !ip.starts_with("127.") && ip != &"::1")
        .map(str::to_string)
}

fn print_doctor(report: &PodDoctor) {
    let flag = |ok: bool| if ok { "ok" } else { "missing" };
    let providers = if report.providers.is_empty() {
        "(none — aimee pod provider add docker)".to_string()
    } else {
        report.providers.join(", ")
    };
    println!(
        "\
Aimee pod doctor
  devpod     {}
  docker     {}
  gh         {}
  providers  {}
  mac mini   ssh -N -L 8080:127.0.0.1:8080 {}
  anda dTEE  {}

Ready loop: /goal <text>  →  /goal pod  →  /goal pr
dTEE is not in this Aimee tree; do not claim it is wired.
",
        flag(report.devpod),
        flag(report.docker),
        flag(report.gh),
        providers,
        report.ssh_hint,
        if report.dtee {
            "ok"
        } else {
            "missing (not in this tree)"
        },
    );
}

/// Headless-server → Mac Mini access for the DevPod desktop / browser IDE.
fn print_ui_guide() {
    let hint = ssh_hint();
    println!(
        "\
Aimee pod UI (headless host → Mac Mini)

This Linux box has no desktop. Do not try to launch DevPod Desktop here.
Docker is the default provider on this host.

  1. On the Mac Mini install DevPod Desktop
     https://devpod.sh/docs/getting-started/install
  2. Providers → Add → SSH → {hint}
  3. Or tunnel a browser IDE from the Mini:
       ssh -N -L 8080:127.0.0.1:8080 {hint}
  4. Workspaces started here:
       /goal <text>
       /goal pod
       aimee pod ssh <id>
       /goal pr

Anda dTEE is not in this Aimee tree.

Current DevPod binary: {}
SSH target: {hint}
",
        binary().display()
    );
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_argv_up_forwards_workspace_and_flags() {
        let fixture = PodCommand::Up { args: vec![".".into(), "--id".into(), "agent-pr".into()] };
        let actual = argv(&fixture).unwrap();
        let expected = vec![
            "up".to_string(),
            ".".into(),
            "--id".into(),
            "agent-pr".into(),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_argv_list_porcelain_uses_json() {
        let fixture = PodCommand::List { porcelain: true, args: vec![] };
        let actual = argv(&fixture).unwrap();
        let expected = vec!["list".to_string(), "--output".into(), "json".into()];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_argv_provider_nests_devpod_subcommand() {
        let fixture = PodCommand::Provider { args: vec!["add".into(), "docker".into()] };
        let actual = argv(&fixture).unwrap();
        let expected = vec!["provider".to_string(), "add".into(), "docker".into()];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_argv_ui_is_aimee_native() {
        let fixture = PodCommand::Ui;
        let actual = argv(&fixture);
        let expected = None;
        assert_eq!(actual, expected);
        assert_eq!(argv(&PodCommand::Doctor), None);
    }

    #[test]
    fn test_parse_provider_names_skips_header() {
        let fixture = "\n    NAME | VERSION | DEFAULT\n  -------+---------+--------\n    docker | v0.0.1  | true\n";
        let actual = parse_provider_names(fixture);
        let expected = vec!["docker".to_string()];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_workspace_id_for_goal_slugs_headline() {
        let fixture = "Ship PWA wallet login!";
        let actual = workspace_id_for_goal(fixture);
        let expected = "aimee-ship-pwa-wallet-login";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_agent_up_args_disable_ide() {
        let actual = agent_up_args("aimee-ship", ".");
        let expected = vec![
            ".".to_string(),
            "--id".into(),
            "aimee-ship".into(),
            "--open-ide=false".into(),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_argv_exec_uses_ssh_command() {
        let fixture = PodCommand::Exec {
            workspace: "aimee-ship".into(),
            command: vec![
                "cargo".into(),
                "test".into(),
                "-p".into(),
                "aimee_domain".into(),
            ],
        };
        let actual = argv(&fixture).unwrap();
        let expected = vec![
            "ssh".to_string(),
            "aimee-ship".into(),
            "--command".into(),
            "cargo test -p aimee_domain".into(),
        ];
        assert_eq!(actual, expected);
    }
}
