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
                .run("coderabbit review --type committed --plain")
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
