use std::borrow::Cow;
use std::fmt::Write;
use std::path::PathBuf;

use aimee_api::{AgentId, Effort, ModelId, Usage};
use convert_case::{Case, Casing};
use derive_setters::Setters;
use nu_ansi_term::{Color, Style};

use crate::display_constants::markers;
use crate::utils::humanize_number;

// Nerd font symbols — right prompt (ZSH rprompt)
const AGENT_SYMBOL: &str = "\u{f167a}";
const MODEL_SYMBOL: &str = "\u{ec19}";

/// Warp's prompt opens with a solid accent block (two half-block glyphs).
const WARP_INPUT_BLOCK: &str = "\u{258c}\u{258c}"; // ▌▌

/// Terminal width at which the reasoning effort label switches from the
/// compact three-letter form (e.g. `MED`) to the full uppercase label
/// (e.g. `MEDIUM`). Matches [`crate::zsh::rprompt`] so the CLI and zsh
/// integration render identically on equivalent terminals.
const WIDE_TERMINAL_THRESHOLD: usize = 100;

/// Very Specialized Prompt for the Agent Chat
#[derive(Clone, Setters)]
#[setters(strip_option, borrow_self)]
pub struct AimeePrompt {
    pub cwd: PathBuf,
    pub usage: Option<Usage>,
    pub agent_id: AgentId,
    pub model: Option<ModelId>,
    /// Context window of the active model, in tokens. When both this and
    /// `usage` are set, the right prompt renders a context-fill percentage
    /// (Grok Build status-line pattern: pressure beats raw counts).
    pub context_window: Option<u64>,
    /// Currently configured reasoning effort level for the active model,
    /// rendered to the right of the model when set. `Effort::None` is
    /// suppressed (see [`AimeePrompt::render_prompt_right`]).
    pub reasoning_effort: Option<Effort>,
    pub git_branch: Option<String>,
}

impl AimeePrompt {
    /// Creates a new `AimeePrompt`, resolving the git branch once at
    /// construction time.
    pub fn new(cwd: PathBuf, agent_id: AgentId) -> Self {
        let git_branch = get_git_branch();
        Self {
            cwd,
            usage: None,
            agent_id,
            model: None,
            context_window: None,
            reasoning_effort: None,
            git_branch,
        }
    }

    pub fn refresh(&mut self) -> &mut Self {
        let git_branch = get_git_branch();
        self.git_branch = git_branch;
        self
    }

    pub fn render_prompt_left(&self) -> Cow<'_, str> {
        // Warp-style input block:
        //
        //   ▌▌ dir  branch
        //
        // Colors (Warp):
        //   block   → accent blue, bold
        //   dir     → near-white
        //   branch  → dim muted

        let current_dir = self
            .cwd
            .file_name()
            .and_then(|name| name.to_str())
            .map(String::from)
            .unwrap_or_else(|| markers::EMPTY.to_string());

        let mut result = String::with_capacity(120);

        // Warp's input starts with a solid accent block on the prompt line.
        write!(
            result,
            "{}",
            crate::theme::warp_input_block_style().paint(WARP_INPUT_BLOCK)
        )
        .unwrap();

        // Directory — plain near-white after the block.
        write!(
            result,
            " {}",
            crate::theme::prompt_dir_style().paint(&current_dir)
        )
        .unwrap();

        // Git branch — dim muted (Warp shows branch quietly).
        if let Some(branch) = self.git_branch.as_deref()
            && branch != current_dir.as_str()
        {
            write!(
                result,
                " {}",
                crate::theme::prompt_branch_style().paint(branch)
            )
            .unwrap();
        }

        Cow::Owned(result)
    }

    pub fn render_prompt_right(&self) -> Cow<'_, str> {
        // Right prompt layout: agent · tokens · cost · model
        // Active (tokens > 0): bright white for agent/tokens, green for cost
        // Inactive (no tokens): all segments dimmed

        let total_tokens = self.usage.as_ref().map(|u| u.total_tokens);
        let active = total_tokens.map(|t| *t > 0).unwrap_or(false);

        let agent_color = if active {
            Color::LightGray
        } else {
            Color::DarkGray
        };
        let mut result = String::with_capacity(64);

        // Agent name with nerd font symbol
        let agent_str = format!(
            "{AGENT_SYMBOL} {}",
            self.agent_id.as_str().to_case(Case::UpperSnake)
        );
        write!(
            result,
            " {}",
            Style::new().bold().fg(agent_color).paint(&agent_str)
        )
        .unwrap();

        // Token count (only shown when active)
        if let Some(tokens) = total_tokens
            && active
        {
            let prefix = match tokens {
                aimee_api::TokenCount::Actual(_) => "",
                aimee_api::TokenCount::Approx(_) => "~",
            };
            let count_str = format!("{}{}", prefix, humanize_number(*tokens));
            write!(
                result,
                " {}",
                Style::new().bold().fg(Color::LightGray).paint(&count_str)
            )
            .unwrap();
        }

        // Context fill — borrowed from Grok Build's status-line contract: a
        // fill percentage communicates context pressure better than raw
        // counts. Color escalates as the window fills (muted → gold → red);
        // hidden entirely until both usage and a known window exist.
        if let Some(window) = self.context_window
            && active
            && let Some(tokens) = total_tokens
        {
            let percent = context_percent(*tokens, window);
            let label = format!("ctx {percent}%");
            write!(result, " {}", context_style(percent).paint(&label)).unwrap();
        }

        // Cost (only shown when active)
        if let Some(cost) = self.usage.as_ref().and_then(|u| u.cost)
            && active
        {
            let cost_str = format!("\u{f155}{cost:.2}");
            write!(
                result,
                " {}",
                Style::new().bold().fg(Color::Green).paint(&cost_str)
            )
            .unwrap();
        }

        // Model with nerd font symbol
        if let Some(model) = self.model.as_ref() {
            let model_str = model.to_string();
            let short_model = model_str.split('/').next_back().unwrap_or(model.as_str());
            let model_label = format!("{MODEL_SYMBOL} {short_model}");
            let color = if active {
                Color::LightMagenta
            } else {
                Color::DarkGray
            };
            write!(result, " {}", Style::new().fg(color).paint(&model_label)).unwrap();
        }

        // Reasoning effort — rendered to the right of the model, matching the
        // ZSH rprompt. `Effort::None` is suppressed (see zsh/rprompt.rs). On
        // narrow terminals the label collapses to its first three characters
        // so the prompt stays compact.
        if let Some(ref effort) = self.reasoning_effort
            && !matches!(effort, Effort::None)
        {
            let effort_label = effort_label(effort, term_width());
            let color = if active {
                Color::Yellow
            } else {
                Color::DarkGray
            };
            write!(result, " {}", Style::new().fg(color).paint(&effort_label)).unwrap();
        }

        Cow::Owned(result)
    }

    pub fn render_prompt_indicator(&self) -> Cow<'_, str> {
        Cow::Borrowed("")
    }
}

/// Gets the current git branch name if available
fn get_git_branch() -> Option<String> {
    let repo = gix::discover(".").ok()?;
    let head = repo.head().ok()?;
    head.referent_name().map(|r| r.shorten().to_string())
}

/// Returns the current terminal width in columns, falling back to 80 when
/// the size cannot be detected.
fn term_width() -> usize {
    terminal_size::terminal_size()
        .map(|(w, _)| w.0 as usize)
        .unwrap_or(80)
}

/// Formats an [`Effort`] as its uppercase label, collapsing to the first three
/// characters on narrow terminals (< [`WIDE_TERMINAL_THRESHOLD`] columns).
fn effort_label(effort: &Effort, width: usize) -> String {
    let full = effort.to_string().to_uppercase();
    if width >= WIDE_TERMINAL_THRESHOLD {
        full
    } else {
        // `chars().take(3)` rather than `&full[..3]` to satisfy the
        // `clippy::string_slice` lint denied in CI.
        full.chars().take(3).collect()
    }
}

/// Context fill as a whole percentage, clamped to 0–100. Saturating math so
/// an oversized count can never panic or wrap.
fn context_percent(tokens: usize, window: u64) -> u8 {
    let window = window.max(1) as usize;
    (tokens.saturating_mul(100) / window).min(100) as u8
}

/// Color for the context-fill label: muted while there is room, Warp gold
/// past 70% as a heads-up, red past 90% as pressure.
fn context_style(percent: u8) -> Style {
    if percent > 90 {
        Style::new().bold().fg(Color::Red)
    } else if percent > 70 {
        Style::new().bold().fg(Color::Yellow)
    } else {
        Style::new().fg(Color::DarkGray)
    }
}

#[cfg(test)]
mod tests {
    use nu_ansi_term::Style;
    use pretty_assertions::assert_eq;

    use super::*;

    impl Default for AimeePrompt {
        fn default() -> Self {
            AimeePrompt {
                cwd: PathBuf::from("."),
                usage: None,
                agent_id: AgentId::default(),
                model: None,
                context_window: None,
                reasoning_effort: None,
                git_branch: None,
            }
        }
    }

    enum PromptHistorySearchStatus {
        Passing,
        Failing,
    }

    struct PromptHistorySearch {
        status: PromptHistorySearchStatus,
        term: String,
    }

    fn render_prompt_history_search_indicator(
        history_search: PromptHistorySearch,
    ) -> Cow<'static, str> {
        let prefix = match history_search.status {
            PromptHistorySearchStatus::Passing => "",
            PromptHistorySearchStatus::Failing => "failing ",
        };

        let mut result = String::with_capacity(32);
        if history_search.term.is_empty() {
            write!(result, "({prefix}reverse-search) ").unwrap();
        } else {
            write!(
                result,
                "({}reverse-search: {}) ",
                prefix, history_search.term
            )
            .unwrap();
        }

        Cow::Owned(Style::new().fg(Color::White).paint(&result).to_string())
    }

    #[test]
    fn test_render_prompt_left() {
        let prompt = AimeePrompt::default();
        let actual = prompt.render_prompt_left();

        // Warp input block leads the line.
        assert!(actual.contains(WARP_INPUT_BLOCK));
        // Test cwd "." has no file_name → placeholder marker renders.
        assert!(actual.contains(markers::EMPTY));
        // No chip row above the prompt anymore (Warp-quiet).
        assert!(!actual.contains(":muse"));
    }

    #[test]
    fn test_render_prompt_left_with_branch() {
        let prompt = AimeePrompt { git_branch: Some("main".to_string()), ..Default::default() };
        let actual = prompt.render_prompt_left();

        // Branch renders quietly after the directory.
        assert!(actual.contains("main"));
    }

    #[test]
    fn test_render_prompt_right_inactive() {
        // No tokens → dimmed agent + model, no token/cost segments
        let mut prompt = AimeePrompt::default();
        let _ = prompt.model(ModelId::new("gpt-4"));

        let actual = prompt.render_prompt_right();
        // Agent symbol and name present
        assert!(actual.contains(AGENT_SYMBOL));
        assert!(actual.contains("AIMEE"));
        // Model symbol and name present
        assert!(actual.contains(MODEL_SYMBOL));
        assert!(actual.contains("gpt-4"));
        // No token count text in inactive state (no humanized number segment)
        assert!(!actual.contains("1k") && !actual.contains("~"));
    }

    #[test]
    fn test_render_prompt_right_active_with_tokens() {
        // Tokens > 0 → active colours; approx tokens show "~" prefix
        let usage = Usage {
            prompt_tokens: aimee_api::TokenCount::Actual(10),
            completion_tokens: aimee_api::TokenCount::Actual(20),
            total_tokens: aimee_api::TokenCount::Approx(30),
            ..Default::default()
        };
        let mut prompt = AimeePrompt::default();
        let _ = prompt.usage(usage);

        let actual = prompt.render_prompt_right();
        assert!(actual.contains("~30"));
        assert!(actual.contains(AGENT_SYMBOL));
    }

    #[test]
    fn test_render_prompt_history_search_indicator_passing() {
        let history_search = PromptHistorySearch {
            status: PromptHistorySearchStatus::Passing,
            term: "test".to_string(),
        };
        let actual = render_prompt_history_search_indicator(history_search);
        let expected = Style::new()
            .fg(Color::White)
            .paint("(reverse-search: test) ")
            .to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_render_prompt_history_search_indicator_failing() {
        let history_search = PromptHistorySearch {
            status: PromptHistorySearchStatus::Failing,
            term: "test".to_string(),
        };
        let actual = render_prompt_history_search_indicator(history_search);
        let expected = Style::new()
            .fg(Color::White)
            .paint("(failing reverse-search: test) ")
            .to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_render_prompt_history_search_indicator_empty_term() {
        let history_search = PromptHistorySearch {
            status: PromptHistorySearchStatus::Passing,
            term: "".to_string(),
        };
        let actual = render_prompt_history_search_indicator(history_search);
        let expected = Style::new()
            .fg(Color::White)
            .paint("(reverse-search) ")
            .to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_render_prompt_right_strips_provider_prefix() {
        // Model ID like "anthropic/claude-3" should show only "claude-3"
        let usage = Usage {
            prompt_tokens: aimee_api::TokenCount::Actual(10),
            completion_tokens: aimee_api::TokenCount::Actual(20),
            total_tokens: aimee_api::TokenCount::Actual(30),
            ..Default::default()
        };
        let mut prompt = AimeePrompt::default();
        let _ = prompt.usage(usage);
        let _ = prompt.model(ModelId::new("anthropic/claude-3"));

        let actual = prompt.render_prompt_right();
        assert!(actual.contains("claude-3"));
        assert!(!actual.contains("anthropic/claude-3"));
        assert!(actual.contains("30"));
    }

    #[test]
    fn test_render_prompt_right_with_cost() {
        // Cost shown when active
        let usage = Usage {
            total_tokens: aimee_api::TokenCount::Actual(1500),
            cost: Some(0.01),
            ..Default::default()
        };
        let mut prompt = AimeePrompt::default();
        let _ = prompt.usage(usage);

        let actual = prompt.render_prompt_right();
        assert!(actual.contains("0.01"));
        assert!(actual.contains("1.5k"));
    }

    #[test]
    fn test_render_prompt_right_with_reasoning_effort() {
        // When reasoning effort is set, its uppercase label appears after the
        // model segment.
        let mut prompt = AimeePrompt::default();
        let _ = prompt.model(ModelId::new("gpt-4"));
        let _ = prompt.reasoning_effort(Effort::High);

        let actual = prompt.render_prompt_right();
        assert!(actual.contains("HIGH") || actual.contains("HIG"));
    }

    #[test]
    fn test_render_prompt_right_hides_effort_none() {
        // `Effort::None` carries no useful info — it must not be rendered.
        let mut prompt = AimeePrompt::default();
        let _ = prompt.model(ModelId::new("gpt-4"));
        let _ = prompt.reasoning_effort(Effort::None);

        let actual = prompt.render_prompt_right();
        assert!(!actual.to_uppercase().contains("NONE"));
    }

    #[test]
    fn test_effort_label_narrow_vs_wide() {
        assert_eq!(effort_label(&Effort::Medium, 80), "MED");
        assert_eq!(
            effort_label(&Effort::Medium, WIDE_TERMINAL_THRESHOLD),
            "MEDIUM"
        );
    }

    #[test]
    fn test_render_prompt_right_with_context_meter() {
        // Known window + usage → fill percentage between tokens and cost.
        let usage = Usage {
            total_tokens: aimee_api::TokenCount::Actual(2_000),
            ..Default::default()
        };
        let mut prompt = AimeePrompt::default();
        let _ = prompt.usage(usage);
        prompt.context_window = Some(100_000);

        let actual = prompt.render_prompt_right();
        assert!(actual.contains("ctx 2%"));
    }

    #[test]
    fn test_render_prompt_right_hides_context_without_window() {
        // Usage without a known window → no ctx segment (no fake baseline).
        let usage = Usage {
            total_tokens: aimee_api::TokenCount::Actual(2_000),
            ..Default::default()
        };
        let mut prompt = AimeePrompt::default();
        let _ = prompt.usage(usage);

        let actual = prompt.render_prompt_right();
        assert!(!actual.contains("ctx "));
    }

    #[test]
    fn test_render_prompt_right_hides_context_when_inactive() {
        // Window known but zero tokens → inactive prompt stays dim and bare.
        let mut prompt = AimeePrompt::default();
        prompt.context_window = Some(100_000);

        let actual = prompt.render_prompt_right();
        assert!(!actual.contains("ctx "));
    }

    #[test]
    fn test_context_percent_clamps() {
        let actual = (
            context_percent(0, 100_000),
            context_percent(50_000, 100_000),
            context_percent(200_000, 100_000),
            context_percent(1_000, 0), // degenerate window must not divide by zero
        );
        let expected = (0u8, 50u8, 100u8, 100u8);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_context_style_escalates() {
        // The three bands must render as three distinct styles, escalating to
        // bold at the warn and pressure thresholds (calm stays plain).
        let calm = context_style(10);
        let warm = context_style(75);
        let hot = context_style(95);
        assert_ne!(format!("{warm:?}"), format!("{calm:?}"));
        assert_ne!(format!("{hot:?}"), format!("{warm:?}"));
        assert!(format!("{warm:?}").contains("bold"));
        assert!(format!("{hot:?}").contains("bold"));
        assert!(!format!("{calm:?}").contains("bold"));
    }
}
