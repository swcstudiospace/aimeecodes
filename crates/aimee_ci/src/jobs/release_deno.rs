use gh_workflow::*;

/// Create a Deno/JSR release job: republishes the `deno install` launcher
/// package from the Deno channel repo against the released binary version.
pub fn release_deno_job() -> Job {
    Job::new("deno_release")
        .add_step(
            Step::new("Checkout Code")
                .uses("actions", "checkout", "v6")
                .add_with(("repository", "swcstudiospace/deno-aimee-codes"))
                .add_with(("ref", "main"))
                .add_with(("token", "${{ secrets.DENO_ACCESS }}")),
        )
        .add_step(
            Step::new("Update Deno Launcher Package")
                .run("./update-deno.sh ${{ github.event.release.tag_name }}")
                .add_env(("AUTO_PUSH", "true"))
                .add_env(("CI", "true"))
                .add_env(("JSR_TOKEN", "${{ secrets.JSR_TOKEN }}"))
                .add_env(("NPM_TOKEN", "${{ secrets.NPM_TOKEN }}")),
        )
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn test_deno_release_targets_swcstudiospace_channel() {
        let fixture = super::release_deno_job();
        let actual = serde_json::to_value(&fixture).unwrap();
        let expected = "swcstudiospace/deno-aimee-codes";
        assert_eq!(
            actual["steps"][0]["with"]["repository"].as_str(),
            Some(expected)
        );
    }
}
