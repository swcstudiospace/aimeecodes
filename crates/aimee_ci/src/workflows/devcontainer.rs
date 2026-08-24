//! Dev Container prebuild workflow.

use std::collections::HashMap;

use gh_workflow::generate::Generate;
use gh_workflow::*;

/// Generate `.github/workflows/devcontainer.yml`.
///
/// Writes the file via `gh_workflow::Generate` (same path as the other
/// workflow generators). Pull requests build only; pushes to `main` may push
/// the image to GHCR.
pub fn generate_devcontainer_workflow() {
    let paths = [
        ".devcontainer/**",
        "rust-toolchain.toml",
        "package-lock.json",
        "crates/aimee_ci/src/workflows/devcontainer.rs",
        "crates/aimee_ci/src/jobs/devcontainer_job.rs",
    ];

    let mut push = Push::default().add_branch("main");
    let mut pull_request = PullRequest::default()
        .add_type(PullRequestType::Opened)
        .add_type(PullRequestType::Synchronize)
        .add_type(PullRequestType::Reopened);
    for path in paths {
        push = push.add_path(path);
        pull_request = pull_request.add_path(path);
    }

    let events = Event::default()
        .push(push)
        .pull_request(pull_request)
        .workflow_dispatch(WorkflowDispatch { inputs: HashMap::new() })
        .add_schedule(Schedule::new("0 6 * * 1"));

    let workflow = Workflow::default()
        .name("devcontainer")
        .on(events)
        .permissions(
            Permissions::default()
                .contents(Level::Read)
                .packages(Level::Write),
        )
        .concurrency(
            Concurrency::default()
                .group("devcontainer-${{ github.ref }}")
                .cancel_in_progress(true),
        )
        .add_job("prebuild", crate::jobs::prebuild_job());

    Generate::new(workflow)
        .name("devcontainer.yml")
        .generate()
        .unwrap();
}
