//! Warp-inspired timeline for Aimee Codes terminal output.
//!
//! Design goals (benchmark: Warp CLI + Hermes tool visibility + Super Grok
//! agent hops):
//! - Quiet gutter + short kind chip (TOOL / SKILL / AGENT / …)
//! - Near-white titles, cyan subtitles, dim clock on the right of the chip
//! - No noisy `ERROR:` / `WARNING:` prefixes when the chip already carries kind
//! - Agent handoffs read as `NAME  →  task` without dense decoration

use std::fmt;

use aimee_domain::{Category, TitleFormat};
use chrono::Local;
use nu_ansi_term::Style;

use crate::theme::palette;

/// Implementation of Display for TitleFormat in the presentation layer
pub struct TitleDisplay {
    inner: TitleFormat,
    with_colors: bool,
}

impl TitleDisplay {
    pub fn new(title: TitleFormat) -> Self {
        Self { inner: title, with_colors: true }
    }

    pub fn with_colors(mut self, with_colors: bool) -> Self {
        self.with_colors = with_colors;
        self
    }

    fn chip(&self) -> (&'static str, Style) {
        match self.inner.category {
            Category::Tool => (
                "TOOL",
                Style::new().bold().fg(palette::VOID).on(palette::CYAN),
            ),
            Category::Skill => (
                "SKIL",
                Style::new()
                    .bold()
                    .fg(palette::NEAR_WHITE)
                    .on(palette::VIOLET),
            ),
            Category::Agent => (
                "AGNT",
                Style::new()
                    .bold()
                    .fg(palette::NEAR_WHITE)
                    .on(palette::MAGENTA),
            ),
            Category::Action => (
                "ACT ",
                Style::new().bold().fg(palette::VOID).on(palette::GOLD),
            ),
            Category::Info => (
                "INFO",
                Style::new()
                    .bold()
                    .fg(palette::NEAR_WHITE)
                    .on(palette::VIOLET),
            ),
            Category::Debug => (
                "DBG ",
                Style::new().bold().fg(palette::VOID).on(palette::CYAN),
            ),
            Category::Error => (
                "ERR ",
                Style::new().bold().fg(palette::NEAR_WHITE).on(palette::RED),
            ),
            Category::Completion => (
                "DONE",
                Style::new().bold().fg(palette::VOID).on(palette::LIME),
            ),
            Category::Warning => (
                "WARN",
                Style::new().bold().fg(palette::VOID).on(palette::GOLD),
            ),
        }
    }

    fn plain_chip(&self) -> &'static str {
        match self.inner.category {
            Category::Tool => "TOOL",
            Category::Skill => "SKIL",
            Category::Agent => "AGNT",
            Category::Action => "ACT",
            Category::Info => "INFO",
            Category::Debug => "DBG",
            Category::Error => "ERR",
            Category::Completion => "DONE",
            Category::Warning => "WARN",
        }
    }

    fn format_with_colors(&self) -> String {
        let (label, chip_style) = self.chip();
        let local_time: chrono::DateTime<Local> = self.inner.timestamp.into();
        let gutter = Style::new().fg(palette::VIOLET).paint("│");
        let clock = Style::new()
            .fg(palette::VIOLET)
            .dimmed()
            .paint(format!("{}", local_time.format("%H:%M:%S")));

        let title = match self.inner.category {
            Category::Error => Style::new()
                .bold()
                .fg(palette::RED)
                .paint(&self.inner.title)
                .to_string(),
            Category::Warning => Style::new()
                .bold()
                .fg(palette::GOLD)
                .paint(&self.inner.title)
                .to_string(),
            Category::Completion => Style::new()
                .bold()
                .fg(palette::LIME)
                .paint(&self.inner.title)
                .to_string(),
            Category::Agent => Style::new()
                .bold()
                .fg(palette::MAGENTA)
                .paint(&self.inner.title)
                .to_string(),
            Category::Skill => Style::new()
                .bold()
                .fg(palette::VIOLET)
                .paint(&self.inner.title)
                .to_string(),
            Category::Tool => Style::new()
                .bold()
                .fg(palette::CYAN)
                .paint(&self.inner.title)
                .to_string(),
            Category::Debug => Style::new()
                .fg(palette::CYAN)
                .paint(&self.inner.title)
                .to_string(),
            _ => Style::new()
                .fg(palette::NEAR_WHITE)
                .paint(&self.inner.title)
                .to_string(),
        };

        let mut buf = format!(
            "{gutter} {} {}  {title}",
            chip_style.paint(format!(" {label} ")),
            clock,
        );

        if let Some(ref sub_title) = self.inner.sub_title {
            // Agent handoffs get an arrow (Super Grok Heavy style hop).
            let sep = if matches!(self.inner.category, Category::Agent) {
                Style::new()
                    .bold()
                    .fg(palette::GOLD)
                    .paint(" → ")
                    .to_string()
            } else {
                Style::new()
                    .fg(palette::VIOLET)
                    .dimmed()
                    .paint("  ")
                    .to_string()
            };
            buf.push_str(&sep);
            buf.push_str(
                &Style::new()
                    .fg(palette::NEAR_WHITE)
                    .dimmed()
                    .paint(sub_title.as_str())
                    .to_string(),
            );
        }
        buf
    }

    fn format_plain(&self) -> String {
        let label = self.plain_chip();
        let local_time: chrono::DateTime<Local> = self.inner.timestamp.into();
        let mut buf = format!(
            "| [{label}] [{}] {}",
            local_time.format("%H:%M:%S"),
            self.inner.title
        );
        if let Some(ref sub_title) = self.inner.sub_title {
            if matches!(self.inner.category, Category::Agent) {
                buf.push_str(&format!(" → {sub_title}"));
            } else {
                buf.push_str(&format!("  {sub_title}"));
            }
        }
        buf
    }
}

impl fmt::Display for TitleDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.with_colors {
            write!(f, "{}", self.format_with_colors())
        } else {
            write!(f, "{}", self.format_plain())
        }
    }
}

/// Extension trait to easily convert TitleFormat to displayable form
pub trait TitleDisplayExt {
    fn display(self) -> TitleDisplay;
    fn display_with_colors(self, with_colors: bool) -> TitleDisplay;
}

impl TitleDisplayExt for TitleFormat {
    fn display(self) -> TitleDisplay {
        TitleDisplay::new(self)
    }

    fn display_with_colors(self, with_colors: bool) -> TitleDisplay {
        TitleDisplay::new(self).with_colors(with_colors)
    }
}

#[cfg(test)]
mod tests {
    use aimee_domain::TitleFormat;

    use super::*;

    #[test]
    fn test_plain_title_uses_category_chip() {
        let fixture = TitleFormat::info("Ready").sub_title("grok-4.6");
        let actual = TitleDisplay::new(fixture).with_colors(false).to_string();
        assert!(actual.contains("[INFO]"));
        assert!(actual.contains("Ready"));
        assert!(actual.contains("grok-4.6"));
        assert!(!actual.contains("●"));
    }

    #[test]
    fn test_plain_tool_lane() {
        let fixture = TitleFormat::tool("Read").sub_title("src/main.rs");
        let actual = TitleDisplay::new(fixture).with_colors(false).to_string();
        assert!(actual.starts_with("| [TOOL]"));
        assert!(actual.contains("Read"));
        assert!(actual.contains("src/main.rs"));
    }

    #[test]
    fn test_plain_skill_lane() {
        let fixture = TitleFormat::skill("Skill").sub_title("ratatui-agent-tui");
        let actual = TitleDisplay::new(fixture).with_colors(false).to_string();
        assert!(actual.contains("[SKIL]"));
        assert!(actual.contains("ratatui-agent-tui"));
    }

    #[test]
    fn test_plain_agent_handoff_arrow() {
        let fixture = TitleFormat::agent("FE_RUST").sub_title("implement parser");
        let actual = TitleDisplay::new(fixture).with_colors(false).to_string();
        assert!(actual.contains("[AGNT]"));
        assert!(actual.contains("FE_RUST"));
        assert!(actual.contains("→ implement parser"));
    }
}
