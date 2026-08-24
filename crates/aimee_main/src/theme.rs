//! Aimee Codes TUI palette — **1:1 with Warp CLI dark theme**.
//!
//! Colors match Warp's shipped dark terminal theme (accent blue, mint green,
//! soft white body on near-black). Font cannot be forced from the app; set
//! your terminal profile to **IBM Plex Mono** (the monospaced face used by
//! os.swcstudio.space) for the house look.
//!
//! Shared by the rustyline prompt, title timeline, and ratatui banner.

use nu_ansi_term::{Color, Style};
use ratatui::style::Color as RatatuiColor;

/// Warp CLI dark theme palette (truecolor RGB).
pub mod palette {
    use super::{Color, RatatuiColor};

    // --- Warp dark terminal ---
    /// Warp accent blue (`#01A4FF`).
    pub const CYAN: Color = Color::Rgb(0x01, 0xA4, 0xFF);
    /// Warp magenta / purple accent (`#BF7AF0`).
    pub const MAGENTA: Color = Color::Rgb(0xBF, 0x7A, 0xF0);
    /// Secondary violet (`#7C5CFF`) — tabs / gutters.
    pub const VIOLET: Color = Color::Rgb(0x7C, 0x5C, 0xFF);
    /// Warp success green (`#00D67E`).
    pub const LIME: Color = Color::Rgb(0x00, 0xD6, 0x7E);
    /// Warp yellow / command gold (`#FFCC02`).
    pub const GOLD: Color = Color::Rgb(0xFF, 0xCC, 0x02);
    /// Soft near-white body (`#E6E6E6`).
    pub const NEAR_WHITE: Color = Color::Rgb(0xE6, 0xE6, 0xE6);
    /// Warp void background (`#0B0D12`).
    pub const VOID: Color = Color::Rgb(0x0B, 0x0D, 0x12);
    /// Dim muted gray for secondary text (`#8B949E`).
    pub const MUTED: Color = Color::Rgb(0x8B, 0x94, 0x9E);
    /// Error red (`#F14C4C`).
    pub const RED: Color = Color::Rgb(0xF1, 0x4C, 0x4C);

    pub const RATATUI_CYAN: RatatuiColor = RatatuiColor::Rgb(0x01, 0xA4, 0xFF);
    pub const RATATUI_MAGENTA: RatatuiColor = RatatuiColor::Rgb(0xBF, 0x7A, 0xF0);
    pub const RATATUI_VIOLET: RatatuiColor = RatatuiColor::Rgb(0x7C, 0x5C, 0xFF);
    pub const RATATUI_LIME: RatatuiColor = RatatuiColor::Rgb(0x00, 0xD6, 0x7E);
    pub const RATATUI_GOLD: RatatuiColor = RatatuiColor::Rgb(0xFF, 0xCC, 0x02);
    pub const RATATUI_VOID: RatatuiColor = RatatuiColor::Rgb(0x0B, 0x0D, 0x12);
    pub const RATATUI_MUTED: RatatuiColor = RatatuiColor::Rgb(0x8B, 0x94, 0x9E);
    pub const RATATUI_RED: RatatuiColor = RatatuiColor::Rgb(0xF1, 0x4C, 0x4C);
    pub const RATATUI_NEAR_WHITE: RatatuiColor = RatatuiColor::Rgb(0xE6, 0xE6, 0xE6);
}

/// Recommended monospaced face (os.swcstudio.space house font). Terminals
/// must set this themselves — the CLI cannot load a font file into the host
/// emulator.
pub const WARP_FONT_FACE: &str = "IBM Plex Mono";

/// Inverted key chip used on splash tabs and footer buttons.
pub fn button_key_style() -> ratatui::style::Style {
    ratatui::style::Style::default()
        .fg(palette::RATATUI_VOID)
        .bg(palette::RATATUI_CYAN)
        .add_modifier(ratatui::style::Modifier::BOLD)
}

/// Label sitting next to a key chip.
pub fn button_style() -> ratatui::style::Style {
    ratatui::style::Style::default()
        .fg(palette::RATATUI_CYAN)
        .bg(palette::RATATUI_VOID)
        .add_modifier(ratatui::style::Modifier::BOLD)
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

/// Style for the Warp-style solid input block at the start of the prompt.
pub fn warp_input_block_style() -> Style {
    Style::new().bold().fg(palette::CYAN)
}

/// Directory text after the input block (Warp: plain body color).
pub fn prompt_dir_style() -> Style {
    Style::new().fg(palette::NEAR_WHITE)
}

/// Git branch text (Warp: quiet, dim).
pub fn prompt_branch_style() -> Style {
    Style::new().fg(palette::MUTED)
}

/// Banner line colors cycling Warp blue → violet → magenta → gold.
pub fn banner_line_color(index: usize) -> Color {
    match index % 4 {
        0 => palette::CYAN,
        1 => palette::VIOLET,
        2 => palette::MAGENTA,
        _ => palette::GOLD,
    }
}

/// Ratatui banner line colors cycling Warp blue → violet → magenta → gold.
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
    fn test_palette_locks_warp_dark_accents() {
        let actual = (
            palette::CYAN,
            palette::MAGENTA,
            palette::VIOLET,
            palette::LIME,
            palette::GOLD,
            palette::VOID,
        );
        let expected = (
            Color::Rgb(0x01, 0xA4, 0xFF),
            Color::Rgb(0xBF, 0x7A, 0xF0),
            Color::Rgb(0x7C, 0x5C, 0xFF),
            Color::Rgb(0x00, 0xD6, 0x7E),
            Color::Rgb(0xFF, 0xCC, 0x02),
            Color::Rgb(0x0B, 0x0D, 0x12),
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

    #[test]
    fn test_button_key_style_is_void_on_cyan() {
        let actual = button_key_style();
        let expected = ratatui::style::Style::default()
            .fg(palette::RATATUI_VOID)
            .bg(palette::RATATUI_CYAN)
            .add_modifier(ratatui::style::Modifier::BOLD);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_warp_font_face_is_ibm_plex_mono() {
        assert_eq!(WARP_FONT_FACE, "IBM Plex Mono");
    }
}
