use gh_workflow::*;

/// Create a Nix release job: bumps the flake package version in the Nix
/// channel repo so `nix run github:swcstudiospace/nix-aimee-codes` tracks
/// the published release.
pub fn release_nix_job() -> Job {
    Job::new("nix_release")
        .add_step(
            Step::new("Checkout Code")
                .uses("actions", "checkout", "v6")
                .add_with(("repository", "swcstudiospace/nix-aimee-codes"))
                .add_with(("ref", "main"))
                .add_with(("path", "nix-aimee-codes"))
                .add_with(("token", "${{ secrets.NIX_ACCESS }}")),
        )
        .add_step(
            Step::new("Update Nix Flake")
                .run("cd nix-aimee-codes && ./update-flake.sh ${{ github.event.release.tag_name }}")
                .add_env(("AUTO_PUSH", "true"))
                .add_env(("CI", "true")),
        )
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn test_nix_release_targets_swcstudiospace_channel() {
        let fixture = super::release_nix_job();
        let actual = serde_json::to_value(&fixture).unwrap();
        let expected = "swcstudiospace/nix-aimee-codes";
        assert_eq!(
            actual["steps"][0]["with"]["repository"].as_str(),
            Some(expected)
        );
        assert_eq!(
            actual["steps"][0]["with"]["path"].as_str(),
            Some("nix-aimee-codes")
        );
    }
}
