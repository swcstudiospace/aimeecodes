# GitHub and pull requests

Aimee integrates with GitHub at the workflow level: AI-generated commits, PR descriptions, PR comment triage, and issue creation — all through skills and commands rather than a separate login surface.

## AI-generated commits

```bash
aimee commit                        # generate message, commit staged changes
aimee commit fix typo in readme     # extra context steers the message
aimee commit --preview              # show message without committing
```

The commit generator produces conventional-style messages from your diff. A dedicated model for commit generation can be set once (`aimee config set commit <provider> <model>`) so every generated message uses your preferred cheap/fast model. The prompt template lives at `templates/aimee-commit-message-prompt.md`.

## Pull request work

| Task | Tool |
|---|---|
| Write/refresh the PR description | `/github-pr-description` command (updates an existing PR's body) |
| Draft a PR title + body from the diff | `/tpl-pr` template command |
| Review a PR or working tree | `/review`, `/tpl-review-diff` |
| Read and act on PR comments | `github-pr-comments` skill |
| Open issues | `create-github-issue` skill |
| Release notes from commits | `/tpl-release-notes` |

Run commands in-session (`: aimee /github-pr-description`) or via `aimee cmd execute`.

## Skills involved

The built-in skills shipped to `.aimee/skills/` include the GitHub trio: `create-github-issue`, `github-pr-comments`, and `github-pr-workflow`. Skills are loaded by the `skill` tool when a task needs them — see [Skills, commands, and templates](../concepts/skills-commands-templates.md).

## Conventions in this repository

Aimee-authored commits and GitHub comments carry:

```text
Co-Authored-By: AimeeCodes <noreply@aimeecodes.dev>
```

This is house policy (AGENTS.md) for work done in the Aimee tree itself; adopt the same pattern in projects you manage if you want agent contributions attributable.

## Repository CI touchpoints

GitHub workflows for the Aimee repo are **generated** by the `aimee_ci` crate (`gh-workflow`) — including release automation feeding the NPM matrix and Homebrew tap. If you fork the project, edit the generator, not the YAML. See [CI/CD of Aimee itself](../operations/cicd.md).

## Auth notes

Provider credentials for GitHub Copilot models go through `aimee provider login` like any other provider. Git push/pull rights come from your own git configuration — Aimee uses git as git; it does not broker SSH keys or tokens for remotes.

## See also

* [Slash commands](../usage/commands.md)
* [Skills, commands, and templates](../concepts/skills-commands-templates.md)
* [Security model](../operations/security.md)

<!-- sources: commands/github-pr-description.md, templates/aimee-commit-message-prompt.md, crates/aimee_main/src/cli.rs (commit/vscode), AGENTS.md, .aimee/skills listing -->
