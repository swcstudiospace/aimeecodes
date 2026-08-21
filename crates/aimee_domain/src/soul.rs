use std::path::{Path, PathBuf};

/// A loaded SOUL document (agent identity or project knowledge).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoulDocument {
    /// Path the document was read from.
    pub path: PathBuf,
    /// Markdown body.
    pub body: String,
    /// Whether this is identity (`SOUL.md`) or a project index (`SOUL/SOUL.md`).
    pub kind: SoulKind,
}

/// Distinguishes home identity from a repo-local knowledge tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoulKind {
    /// `$AIMEE_HOME/SOUL.md` — who the agent is.
    Identity,
    /// `SOUL/SOUL.md` or `cwd/SOUL.md` — project orientation.
    Project,
}

impl SoulDocument {
    /// Candidate paths, highest priority first.
    pub fn candidate_paths(cwd: &Path, base: &Path) -> Vec<(PathBuf, SoulKind)> {
        vec![
            (cwd.join("SOUL").join("SOUL.md"), SoulKind::Project),
            (cwd.join("SOUL.md"), SoulKind::Project),
            (base.join("SOUL.md"), SoulKind::Identity),
            (base.join("SOUL").join("SOUL.md"), SoulKind::Identity),
        ]
    }

    /// Discovers the first readable SOUL document.
    pub fn discover(cwd: &Path, base: &Path) -> Option<Self> {
        Self::discover_all(cwd, base).into_iter().next()
    }

    /// Loads every readable SOUL document in priority order.
    pub fn discover_all(cwd: &Path, base: &Path) -> Vec<Self> {
        let mut out = Vec::new();
        for (path, kind) in Self::candidate_paths(cwd, base) {
            match std::fs::read_to_string(&path) {
                Ok(body) if !body.trim().is_empty() => {
                    out.push(Self { path, body, kind });
                }
                _ => {}
            }
        }
        out
    }

    /// Prefix used when injecting SOUL into custom instructions.
    pub fn instruction_block(&self) -> String {
        let label = match self.kind {
            SoulKind::Identity => "SOUL (identity)",
            SoulKind::Project => "SOUL (project)",
        };
        format!(
            "# {label}\nSource: {}\n\n{}",
            self.path.display(),
            self.body
        )
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_candidate_paths_priority() {
        let actual = SoulDocument::candidate_paths(Path::new("/proj"), Path::new("/home/.aimee"));
        let expected = vec![
            (PathBuf::from("/proj/SOUL/SOUL.md"), SoulKind::Project),
            (PathBuf::from("/proj/SOUL.md"), SoulKind::Project),
            (PathBuf::from("/home/.aimee/SOUL.md"), SoulKind::Identity),
            (
                PathBuf::from("/home/.aimee/SOUL/SOUL.md"),
                SoulKind::Identity,
            ),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_instruction_block_labels_kind() {
        let fixture = SoulDocument {
            path: PathBuf::from("/proj/SOUL.md"),
            body: "Be terse.".into(),
            kind: SoulKind::Project,
        };
        let actual = fixture.instruction_block();
        assert!(actual.contains("SOUL (project)"));
        assert!(actual.contains("Be terse."));
    }
}
