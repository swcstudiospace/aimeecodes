//! Prebuild job for the Aimee Codes Dev Container.

use gh_workflow::*;
use indexmap::indexmap;
use serde_json::json;

/// GHCR repository for the prebuilt image (org/name/devcontainer).
pub const IMAGE_NAME: &str = "ghcr.io/${{ github.repository }}/devcontainer";

/// Command run inside the built container after Features.
pub const VERIFY_CMD: &str = "bash .devcontainer/verify.sh";

/// Push only from `main` (never from pull requests or other refs).
pub const PUSH_WHEN: &str =
    "${{ github.event_name != 'pull_request' && github.ref == 'refs/heads/main' }}";

/// Builds (and on `main`, pushes) the Dev Container image.
///
/// # Returns
///
/// A job that checks out the tree, logs in to GHCR on non-PR runs, and invokes
/// `devcontainers/ci`.
pub fn prebuild_job() -> Job {
    Job::new("Prebuild Dev Container")
        .timeout_minutes(60u32)
        .permissions(
            Permissions::default()
                .contents(Level::Read)
                .packages(Level::Write),
        )
        .add_step(Step::new("Checkout Code").uses("actions", "checkout", "v6"))
        .add_step(
            Step::new("Log in to GHCR")
                .uses("docker", "login-action", "v3")
                .if_condition(Expression::new("github.event_name != 'pull_request'"))
                .add_with(Input::from(indexmap! {
                    "registry".to_string() => json!("ghcr.io"),
                    "username".to_string() => json!("${{ github.actor }}"),
                    "password".to_string() => json!("${{ secrets.GITHUB_TOKEN }}"),
                })),
        )
        .add_step(
            Step::new("Build Dev Container")
                .uses("devcontainers", "ci", "v0.3")
                .add_with(Input::from(indexmap! {
                    "imageName".to_string() => json!(IMAGE_NAME),
                    "cacheFrom".to_string() => json!(IMAGE_NAME),
                    "push".to_string() => json!(PUSH_WHEN),
                    "runCmd".to_string() => json!(VERIFY_CMD),
                })),
        )
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn test_prebuild_pushes_only_from_main() {
        let fixture = super::PUSH_WHEN;
        let actual = fixture;
        let expected =
            "${{ github.event_name != 'pull_request' && github.ref == 'refs/heads/main' }}";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_prebuild_verify_cmd_is_repo_script() {
        let fixture = super::VERIFY_CMD;
        let actual = fixture;
        let expected = "bash .devcontainer/verify.sh";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_prebuild_image_uses_github_repository() {
        let fixture = super::IMAGE_NAME;
        let actual = fixture;
        let expected = "ghcr.io/${{ github.repository }}/devcontainer";
        assert_eq!(actual, expected);
    }
}
