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
            Category::Action => (
                "ACT",
                Style::new()
                    .bold()
                    .fg(palette::NEAR_WHITE)
                    .on(palette::GOLD),
            ),
            Category::Info => (
                "INF",
                Style::new()
                    .bold()
                    .fg(palette::NEAR_WHITE)
                    .on(palette::VIOLET),
            ),
            Category::Debug => (
                "DBG",
                Style::new().bold().fg(palette::VOID).on(palette::CYAN),
            ),
            Category::Error => (
                "ERR",
                Style::new()
                    .bold()
                    .fg(palette::NEAR_WHITE)
                    .on(palette::MAGENTA),
            ),
            Category::Completion => (
                "OK ",
                Style::new().bold().fg(palette::VOID).on(palette::LIME),
            ),
            Category::Warning => (
                "WRN",
                Style::new().bold().fg(palette::VOID).on(palette::GOLD),
            ),
        }
    }

    fn format_with_colors(&self) -> String {
        let (label, chip_style) = self.chip();
        let local_time: chrono::DateTime<Local> = self.inner.timestamp.into();
        let timestamp = Style::new()
            .fg(palette::VIOLET)
            .paint(format!(" {} ", local_time.format("%H:%M:%S")));

        let title = match self.inner.category {
            Category::Error => Style::new()
                .bold()
                .fg(palette::MAGENTA)
                .paint(format!("ERROR: {}", self.inner.title))
                .to_string(),
            Category::Warning => Style::new()
                .bold()
                .fg(palette::GOLD)
                .paint(format!("WARNING: {}", self.inner.title))
                .to_string(),
            Category::Debug => Style::new()
                .fg(palette::CYAN)
                .paint(&self.inner.title)
                .to_string(),
            Category::Completion => Style::new()
                .bold()
                .fg(palette::LIME)
                .paint(&self.inner.title)
                .to_string(),
            _ => Style::new()
                .fg(palette::NEAR_WHITE)
                .paint(&self.inner.title)
                .to_string(),
        };

        let mut buf = format!(
            "{}{} {}",
            chip_style.paint(format!(" {label} ")),
            timestamp,
            title
        );
        if let Some(ref sub_title) = self.inner.sub_title {
            buf.push_str(&format!(
                " {}",
                Style::new().fg(palette::CYAN).paint(sub_title.as_str())
            ));
        }
        buf
    }

    fn format_plain(&self) -> String {
        let (label, _) = self.chip();
        let local_time: chrono::DateTime<Local> = self.inner.timestamp.into();
        let mut buf = format!(
            "[{label}] [{}] {}",
            local_time.format("%H:%M:%S"),
            self.inner.title
        );
        if let Some(ref sub_title) = self.inner.sub_title {
            buf.push_str(&format!(" {sub_title}"));
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
        assert!(actual.contains("[INF]"));
        assert!(actual.contains("Ready"));
        assert!(actual.contains("grok-4.6"));
        assert!(!actual.contains("●"));
    }
}
