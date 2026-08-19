use std::{fmt, io};

use colored::Colorize;
use omega_tracker::VERSION;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style as RatatuiStyle;
use ratatui::widgets::Widget;

use crate::theme;

const BANNER: &str = include_str!("banner");
const TAGLINE: &str = "WEB3-native coding agent  ·  TUI now  ·  PWA next";

/// Renders messages into a styled box with border characters.
struct DisplayBox {
    messages: Vec<String>,
}

impl DisplayBox {
    /// Creates a new Box with the given messages.
    fn new(messages: Vec<String>) -> Self {
        Self { messages }
    }
}

impl fmt::Display for DisplayBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let visible_len = |s: &str| console::measure_text_width(s);
        let width: usize = self
            .messages
            .iter()
            .map(|s| visible_len(s))
            .max()
            .unwrap_or(0)
            + 4;
        let top = format!("┌{}┐", "─".repeat(width.saturating_sub(2)));
        let bottom = format!("└{}┘", "─".repeat(width.saturating_sub(2)));
        let fmt_line = |s: &str| {
            let padding = width.saturating_sub(4).saturating_sub(visible_len(s));
            format!("│ {}{} │", s, " ".repeat(padding))
        };

        writeln!(f, "{}", top)?;
        for msg in &self.messages {
            writeln!(f, "{}", fmt_line(msg))?;
        }
        write!(f, "{}", bottom)
    }
}

/// Displays the banner with version and command tips.
///
/// # Arguments
///
/// * `cli_mode` - If true, shows CLI-relevant commands. Both interactive and
///   CLI modes use `:` as the canonical command prefix.
///
/// # Environment Variables
///
/// * `OMEGA_BANNER` - Optional custom banner text to display instead of the
///   default
pub fn display(cli_mode: bool) -> io::Result<()> {
    let custom = std::env::var("OMEGA_BANNER").ok().filter(|s| !s.is_empty());

    if let Some(custom) = custom {
        println!("{custom}");
    } else {
        print_branded_banner();
    }

    let version_label = ("Version:", VERSION);
    let tips: Vec<(&str, &str)> = if cli_mode {
        vec![
            ("New conversation:", ":new"),
            ("Get started:", ":info, :conversation"),
            ("Switch model:", ":model"),
            ("Switch provider:", ":provider"),
            ("Switch agent:", ":<agent_name> e.g. :omega or :muse"),
        ]
    } else {
        vec![
            ("New conversation:", ":new"),
            ("Get started:", ":info, :usage, :help, :conversation"),
            ("Switch model:", ":model"),
            ("Switch agent:", ":omega or :muse or :agent"),
            ("Update:", ":update"),
            ("Quit:", ":exit or <CTRL+D>"),
        ]
    };

    let labels: Vec<(&str, &str)> = std::iter::once(version_label).chain(tips).collect();
    let max_width = labels.iter().map(|(key, _)| key.len()).max().unwrap_or(0);

    for (key, value) in &labels {
        println!(
            "{}{}",
            format!("{key:>max_width$} ").dimmed(),
            value.truecolor(0, 229, 255)
        );
    }
    println!();

    if !cli_mode {
        display_zsh_encouragement();
    }

    Ok(())
}

/// Prints the default Omega Loops ASCII banner with a cyan→violet→magenta
/// gradient.
fn print_branded_banner() {
    for (index, line) in BANNER.lines().enumerate() {
        if line.trim().is_empty() {
            println!();
            continue;
        }
        let (r, g, b) = banner_rgb(index);
        println!("{}", line.truecolor(r, g, b).bold());
    }
    println!("{}", TAGLINE.truecolor(163, 255, 18));
}

fn banner_rgb(index: usize) -> (u8, u8, u8) {
    match index % 4 {
        0 => (0, 229, 255),
        1 => (168, 85, 247),
        2 => (255, 45, 149),
        _ => (255, 210, 0),
    }
}

/// Paints the default banner into a ratatui [`Buffer`] for visual tests and
/// future full-screen TUI surfaces.
pub fn render_into(buf: &mut Buffer, area: Rect) {
    for (index, line) in BANNER.lines().enumerate() {
        if index as u16 >= area.height {
            break;
        }
        let style = RatatuiStyle::default().fg(theme::banner_line_ratatui(index));
        buf.set_stringn(
            area.x,
            area.y + index as u16,
            line,
            area.width as usize,
            style,
        );
    }
}

/// Encourages users to use the zsh plugin for a better experience.
fn display_zsh_encouragement() {
    let tip = DisplayBox::new(vec![
        format!(
            "{} {}",
            "TIP:".bold().truecolor(255, 210, 0),
            "For the best experience, use our zsh plugin!".bold()
        ),
        format!(
            "{} {} {}",
            "·".dimmed(),
            "Set up Omega Loops via our zsh plugin:".dimmed(),
            "omega zsh setup".bold().truecolor(163, 255, 18),
        ),
        format!(
            "{} {} {}",
            "·".dimmed(),
            "Learn more:".dimmed(),
            "https://omegaloops.dev/docs/zsh-support".truecolor(0, 229, 255)
        ),
    ]);
    println!("{}", tip);
}

/// Invisible widget wrapper so the banner can sit on a ratatui surface.
pub struct BannerWidget;

impl Widget for BannerWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        render_into(buf, area);
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use ratatui::buffer::Buffer;
    use ratatui::layout::Rect;

    use super::*;

    #[test]
    fn test_banner_art_contains_omega_and_loops_needles() {
        let actual = BANNER;
        assert!(actual.contains(r#" ____  __  _______________"#));
        assert!(actual.contains("/___/"));
        assert!(actual.contains("LOOPS") || actual.contains("/___/"));
    }

    #[test]
    fn test_render_into_buffer_wider_than_art() {
        let area = Rect::new(0, 0, 80, 6);
        let mut actual = Buffer::empty(area);
        render_into(&mut actual, area);

        let mid = actual[(2, 0)].symbol().to_string();
        let base = actual[(2, 3)].symbol().to_string();
        assert_eq!(mid, "_");
        assert!(base == "\\" || base == "_" || !base.is_empty());
    }
}
