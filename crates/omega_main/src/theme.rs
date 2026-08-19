//! Omega Loops TUI palette and style helpers.
//!
//! High-chroma cyan / violet / magenta / lime accents on a deep void base.
//! Body text stays near-white for legibility. Shared by the rustyline prompt
//! and the ratatui banner so both surfaces stay on-brand.

use nu_ansi_term::{Color, Style};
use ratatui::style::Color as RatatuiColor;

/// High-chroma Omega Loops palette.
pub mod palette {
    use super::{Color, RatatuiColor};

    /// Electric cyan used for mentions, values, and banner line 1.
    pub const CYAN: Color = Color::Rgb(0, 229, 255);
    /// Hot magenta used for shell pass-through and banner line 3.
    pub const MAGENTA: Color = Color::Rgb(255, 45, 149);
    /// Violet used for banner line 2 and secondary accents.
    pub const VIOLET: Color = Color::Rgb(168, 85, 247);
    /// Lime used for success / chevron / branch.
    pub const LIME: Color = Color::Rgb(163, 255, 18);
    /// Gold used for slash/colon commands.
    pub const GOLD: Color = Color::Rgb(255, 210, 0);
    /// Near-white body text.
    pub const NEAR_WHITE: Color = Color::Rgb(240, 244, 255);

    /// Ratatui counterpart of [`CYAN`].
    pub const RATATUI_CYAN: RatatuiColor = RatatuiColor::Rgb(0, 229, 255);
    /// Ratatui counterpart of [`MAGENTA`].
    pub const RATATUI_MAGENTA: RatatuiColor = RatatuiColor::Rgb(255, 45, 149);
    /// Ratatui counterpart of [`VIOLET`].
    pub const RATATUI_VIOLET: RatatuiColor = RatatuiColor::Rgb(168, 85, 247);
    /// Ratatui counterpart of [`LIME`].
    pub const RATATUI_LIME: RatatuiColor = RatatuiColor::Rgb(163, 255, 18);
    /// Ratatui counterpart of [`GOLD`].
    pub const RATATUI_GOLD: RatatuiColor = RatatuiColor::Rgb(255, 210, 0);
    /// Deep void background for ratatui surfaces.
    pub const RATATUI_VOID: RatatuiColor = RatatuiColor::Rgb(8, 6, 18);
}

/// Style for `:command` / `/command` tokens.
pub fn command_style() -> Style {
    Style::new().bold().fg(palette::GOLD)
}

/// Style for `@[path]` file mentions.
pub fn mention_style() -> Style {
    Style::new().bold().fg(palette::CYAN)
}

/// Style for `!shell` pass-through commands.
pub fn shell_style() -> Style {
    Style::new().fg(palette::MAGENTA)
}

/// Style for directory segments in the left prompt.
pub fn dir_style() -> Style {
    Style::new().bold().fg(palette::CYAN)
}

/// Style for git branch segments.
pub fn branch_style() -> Style {
    Style::new().bold().fg(palette::LIME)
}

/// Style for the prompt chevron.
pub fn chevron_style() -> Style {
    Style::new().bold().fg(palette::LIME)
}

/// Banner line colors cycling cyan → violet → magenta → gold.
pub fn banner_line_color(index: usize) -> Color {
    match index % 4 {
        0 => palette::CYAN,
        1 => palette::VIOLET,
        2 => palette::MAGENTA,
        _ => palette::GOLD,
    }
}

/// Ratatui banner line colors cycling cyan → violet → magenta → gold.
pub fn banner_line_ratatui(index: usize) -> RatatuiColor {
    match index % 4 {
        0 => palette::RATATUI_CYAN,
        1 => palette::RATATUI_VIOLET,
        2 => palette::RATATUI_MAGENTA,
        _ => palette::RATATUI_GOLD,
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_palette_locks_high_chroma_accents() {
        let actual = (
            palette::CYAN,
            palette::MAGENTA,
            palette::VIOLET,
            palette::LIME,
            palette::GOLD,
        );
        let expected = (
            Color::Rgb(0, 229, 255),
            Color::Rgb(255, 45, 149),
            Color::Rgb(168, 85, 247),
            Color::Rgb(163, 255, 18),
            Color::Rgb(255, 210, 0),
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_command_style_is_gold_bold() {
        let actual = command_style();
        let expected = Style::new().bold().fg(palette::GOLD);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_banner_line_cycle() {
        let actual = [
            banner_line_color(0),
            banner_line_color(1),
            banner_line_color(2),
            banner_line_color(3),
        ];
        let expected = [
            palette::CYAN,
            palette::VIOLET,
            palette::MAGENTA,
            palette::GOLD,
        ];
        assert_eq!(actual, expected);
    }
}
