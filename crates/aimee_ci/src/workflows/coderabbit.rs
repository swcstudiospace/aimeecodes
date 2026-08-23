use gh_workflow::generate::Generate;
use gh_workflow::*;

/// Generate the CodeRabbit CLI review workflow (comments + suggested fixes on PRs).
pub fn generate_coderabbit_workflow() {
    let review = Job::new("CodeRabbit CLI")
        .permissions(
            Permissions::default()
                .contents(Level::Read)
                .pull_requests(Level::Write)
                .issues(Level::Write),
        )
        .add_step(Step::new("Checkout Code").uses("actions", "checkout", "v6"))
        .add_step(
            Step::new("Install CodeRabbit CLI")
                .run("curl -fsSL https://cli.coderabbit.ai/install.sh | sh"),
        )
        .add_step(
            Step::new("Review and comment")
                .run(coderabbit_review_command())
                .add_env(("CODERABBIT_API_KEY", "${{ secrets.CODERABBIT_API_KEY }}")),
        );

    let events = Event::default().pull_request(
        PullRequest::default()
            .add_type(PullRequestType::Opened)
            .add_type(PullRequestType::Synchronize)
            .add_type(PullRequestType::Reopened)
            .add_branch("main"),
    );

    let workflow = Workflow::default()
        .name("coderabbit")
        .on(events)
        .concurrency(
            Concurrency::default()
                .group("coderabbit-${{ github.ref }}")
                .cancel_in_progress(true),
        )
        .add_job("coderabbit", review);

    Generate::new(workflow)
        .name("coderabbit.yml")
        .generate()
        .unwrap();
}

/// CodeRabbit CLI review invocation used by the generated workflow.
///
/// Current CLI rejects `--plain` and `--type`. Non-interactive CI must pass
/// `--api-key`. Skip cleanly when the repo secret is unset so the PR is not
/// red for an org-level gap.
fn coderabbit_review_command() -> &'static str {
    concat!(
        "if [ -z \"${CODERABBIT_API_KEY:-}\" ]; then ",
        "echo 'CODERABBIT_API_KEY not set — skipping CodeRabbit review' >&2; ",
        "exit 0; ",
        "fi; ",
        "coderabbit review --committed --api-key \"$CODERABBIT_API_KEY\"",
    )
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn test_coderabbit_review_command_matches_current_cli() {
        let fixture = super::coderabbit_review_command();
        let actual = fixture;
        let expected = concat!(
            "if [ -z \"${CODERABBIT_API_KEY:-}\" ]; then ",
            "echo 'CODERABBIT_API_KEY not set — skipping CodeRabbit review' >&2; ",
            "exit 0; ",
            "fi; ",
            "coderabbit review --committed --api-key \"$CODERABBIT_API_KEY\"",
        );
        assert_eq!(actual, expected);
        assert!(
            !actual.contains("--plain"),
            "current CodeRabbit CLI rejects --plain"
        );
        assert!(
            !actual.contains("--type"),
            "current CodeRabbit CLI uses --committed, not --type committed"
        );
        assert!(
            actual.contains("--api-key"),
            "non-interactive CI must pass --api-key"
        );
    }
}
