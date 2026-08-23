use std::sync::atomic::{AtomicU8, Ordering};

/// Session tool-approval mode. Cycle with Shift+Tab in the TUI.
///
/// Confirm prompts on policy `Confirm`. Auto and Yolo skip prompts so swarms
/// are not blocked on every tool call. Yolo also skips the restricted-mode
/// check entirely.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ApprovalMode {
    /// Prompt on policy Confirm (restricted mode).
    #[default]
    Confirm,
    /// Allow Confirm without prompting; still honor Deny.
    Auto,
    /// Skip permission checks. Autonomous / swarm default.
    Yolo,
}

static MODE: AtomicU8 = AtomicU8::new(2); // default Yolo — matches unrestricted config

impl ApprovalMode {
    /// Current process-wide mode.
    pub fn current() -> Self {
        match MODE.load(Ordering::Relaxed) {
            0 => Self::Confirm,
            1 => Self::Auto,
            _ => Self::Yolo,
        }
    }

    /// Stores the mode for this process.
    pub fn set(self) {
        MODE.store(self.as_u8(), Ordering::Relaxed);
    }

    /// Confirm → Auto → Yolo → Confirm.
    pub fn cycle() -> Self {
        let next = Self::current().next();
        next.set();
        next
    }

    /// Next mode in the Shift+Tab cycle.
    pub fn next(self) -> Self {
        match self {
            Self::Confirm => Self::Auto,
            Self::Auto => Self::Yolo,
            Self::Yolo => Self::Confirm,
        }
    }

    /// Skip the interactive permission prompt.
    pub fn skips_prompts(self) -> bool {
        !matches!(self, Self::Confirm)
    }

    /// Skip the restricted-mode permission check entirely.
    pub fn skips_permission_check(self) -> bool {
        matches!(self, Self::Yolo)
    }

    /// Footer / status label.
    pub fn label(self) -> &'static str {
        match self {
            Self::Confirm => "confirm",
            Self::Auto => "auto",
            Self::Yolo => "yolo",
        }
    }

    fn as_u8(self) -> u8 {
        match self {
            Self::Confirm => 0,
            Self::Auto => 1,
            Self::Yolo => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_approval_mode_cycle_order() {
        let fixture = ApprovalMode::Confirm;
        let actual = [fixture, fixture.next(), fixture.next().next()];
        let expected = [
            ApprovalMode::Confirm,
            ApprovalMode::Auto,
            ApprovalMode::Yolo,
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_yolo_skips_permission_check() {
        let actual = (
            ApprovalMode::Yolo.skips_permission_check(),
            ApprovalMode::Auto.skips_prompts(),
            ApprovalMode::Confirm.skips_prompts(),
        );
        let expected = (true, true, false);
        assert_eq!(actual, expected);
    }
}
