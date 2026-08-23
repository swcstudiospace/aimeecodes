use std::path::{Path, PathBuf};

use derive_setters::Setters;

/// Draft used by autonomous skill creation (`/learn`).
#[derive(Debug, Clone, PartialEq, Eq, Setters)]
#[setters(strip_option, into)]
pub struct SkillDraft {
    /// Skill slug (`lowercase-hyphens`).
    pub name: String,
    /// One-line description (≤ 60 chars preferred).
    pub description: String,
    /// Procedure body (markdown, no frontmatter).
    pub body: String,
}

impl SkillDraft {
    /// Creates a draft, sanitizing the name to a slug.
    pub fn new(
        name: impl AsRef<str>,
        description: impl Into<String>,
        body: impl Into<String>,
    ) -> Self {
        Self {
            name: slugify(name.as_ref()),
            description: description.into(),
            body: body.into(),
        }
    }

    /// Renders a SKILL.md with YAML frontmatter.
    pub fn render_markdown(&self) -> String {
        format!(
            "---\nname: {}\ndescription: {}\n---\n\n{}\n",
            self.name,
            self.description.replace('\n', " "),
            self.body.trim()
        )
    }

    /// Writes `SKILL.md` under `skills_dir/<name>/`.
    ///
    /// # Errors
    ///
    /// Returns an error when the directory cannot be created or written.
    pub fn write_to(&self, skills_dir: &Path) -> anyhow::Result<PathBuf> {
        if self.name.is_empty() {
            anyhow::bail!("skill name is empty after slugify");
        }
        let dir = skills_dir.join(&self.name);
        std::fs::create_dir_all(&dir)?;
        let path = dir.join("SKILL.md");
        std::fs::write(&path, self.render_markdown())?;
        Ok(path)
    }
}

/// Lowercase hyphen slug; strips non-alphanumerics.
pub fn slugify(raw: &str) -> String {
    let mut out = String::new();
    let mut dash = false;
    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            dash = false;
        } else if !dash && !out.is_empty() {
            out.push('-');
            dash = true;
        }
    }
    out.trim_matches('-').to_string()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_slugify() {
        let actual = slugify("Learn Telegram Gateway!!");
        let expected = "learn-telegram-gateway".to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_skill_draft_writes_skill_md() {
        let dir = std::env::temp_dir().join(format!(
            "aimee-skill-{}-{}",
            std::process::id(),
            uuid::Uuid::new_v4()
        ));
        let fixture = SkillDraft::new(
            "Goal Loop",
            "Run a standing /goal until verified.",
            "## When to Use\nUse when the user sets /goal.",
        );
        let actual_path = fixture.write_to(&dir).unwrap();
        let actual = std::fs::read_to_string(&actual_path).unwrap();
        let _ = std::fs::remove_dir_all(&dir);
        assert!(actual.starts_with("---\nname: goal-loop\n"));
        assert!(actual.contains("standing /goal"));
        assert_eq!(actual_path.file_name().unwrap(), "SKILL.md");
    }
}
