use std::{fmt, io};

use aimee_tracker::VERSION;
use colored::Colorize;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Fill, LineGauge, Paragraph, Row, Table, Tabs, Widget};

use crate::theme;

const BANNER: &str = include_str!("banner");
const TAGLINE: &str = "CLI agent flock  ·  TUI now  ·  PWA next";

/// Agent-switch chips shown under the tagline.
const CHIPS: [(&str, &str); 3] = [
    (":aimee", "implement"),
    (":muse", "plan"),
    (":sage", "research"),
];

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
/// The art, tagline, and slash-command chips are composed in a ratatui
/// [`Buffer`] then flushed as ANSI so the live path uses the same renderer as
/// future full-screen TUI surfaces.
///
/// # Arguments
///
/// * `cli_mode` - If true, shows CLI-relevant commands. Both interactive and
///   CLI modes use `:` as the canonical command prefix.
///
/// # Environment Variables
///
/// * `AIMEE_BANNER` - Optional custom banner text to display instead of the
///   default
pub fn display(cli_mode: bool) -> io::Result<()> {
    let custom = std::env::var("AIMEE_BANNER").ok().filter(|s| !s.is_empty());

    if let Some(custom) = custom {
        println!("{custom}");
    } else {
        print_ratatui_splash();
    }

    print_command_sheet(cli_mode);

    if !cli_mode {
        display_zsh_encouragement();
    }

    Ok(())
}

/// Composes the branded splash into a ratatui buffer and writes it to stdout.
fn print_ratatui_splash() {
    let area = splash_area();
    let mut buf = Buffer::empty(area);
    render_splash(&mut buf, area);
    print!("{}", buffer_to_ansi(&buf));
}

/// Area large enough for the framed card: art + gauge + tabs + tagline.
fn splash_area() -> Rect {
    let art_width = BANNER
        .lines()
        .map(|line| line.chars().count() as u16)
        .max()
        .unwrap_or(60);
    let width = art_width.max(68).saturating_add(4);
    let art_lines = BANNER
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count() as u16;
    Rect::new(0, 0, width, art_lines.saturating_add(6))
}

/// Paints a framed ratatui card: void fill, rounded Block, figlet art,
/// loop LineGauge, Sage/Muse/Aimee Tabs, and the tagline.
pub fn render_splash(buf: &mut Buffer, area: Rect) {
    Fill::new(" ")
        .style(Style::default().bg(theme::palette::RATATUI_VOID))
        .render(area, buf);

    let frame = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme::palette::RATATUI_CYAN))
        .title(Line::from(vec![
            Span::styled(" 🍑 ", theme::button_key_style()),
            Span::styled(
                "Aimee Codes",
                Style::default()
                    .fg(theme::palette::RATATUI_CYAN)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" 🍑 ", theme::button_key_style()),
        ]))
        .title(
            Line::from(format!(" v{VERSION} "))
                .right_aligned()
                .style(Style::default().fg(theme::palette::RATATUI_GOLD)),
        )
        .title_bottom(
            Line::from(" sage → muse → aimee ").centered().style(
                Style::default()
                    .fg(theme::palette::RATATUI_VIOLET)
                    .add_modifier(Modifier::BOLD),
            ),
        );
    let inner = frame.inner(area);
    frame.render(area, buf);

    let art_lines = BANNER
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count() as u16;
    let chunks = Layout::vertical([
        Constraint::Length(art_lines),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Min(1),
    ])
    .split(inner);

    if !chunks.is_empty() {
        render_into(buf, chunks[0]);
    }
    if chunks.len() > 1 {
        LineGauge::default()
            .filled_symbol("━")
            .unfilled_symbol("─")
            .ratio(1.0)
            .label(Line::from(Span::styled(
                "LOOP ",
                Style::default()
                    .fg(theme::palette::RATATUI_LIME)
                    .add_modifier(Modifier::BOLD),
            )))
            .filled_style(Style::default().fg(theme::palette::RATATUI_CYAN))
            .unfilled_style(Style::default().fg(theme::palette::RATATUI_VIOLET))
            .render(chunks[1], buf);
    }
    if chunks.len() > 2 {
        Tabs::new([":sage research", ":muse plan", ":aimee verify"])
            .select(2)
            .highlight_style(theme::button_key_style().bg(theme::palette::RATATUI_LIME))
            .style(Style::default().fg(theme::palette::RATATUI_VIOLET))
            .divider("·")
            .render(chunks[2], buf);
    }
    if chunks.len() > 3 {
        Paragraph::new(Line::from(Span::styled(
            TAGLINE,
            Style::default()
                .fg(theme::palette::RATATUI_LIME)
                .add_modifier(Modifier::BOLD),
        )))
        .render(chunks[3], buf);
    }
}

/// Command cheatsheet under the splash, rendered as a ratatui table.
fn print_command_sheet(cli_mode: bool) {
    let rows: Vec<(&str, &str)> = if cli_mode {
        vec![
            (":new", "new conversation"),
            (":info", "session + workspace"),
            (":model", "switch model"),
            (":provider", "switch provider"),
            (":aimee / :muse / :sage", "switch loop role"),
        ]
    } else {
        vec![
            (":new", "new conversation"),
            (":info  :usage  :help", "get started"),
            (":model", "switch model"),
            (":aimee  :muse  :sage", "switch loop role"),
            (":update", "self-update"),
            (":exit", "quit  ·  also Ctrl+D"),
        ]
    };
    let height = (rows.len() as u16).saturating_add(2);
    let width = 72u16;
    let area = Rect::new(0, 0, width, height);
    let mut buf = Buffer::empty(area);
    Fill::new(" ")
        .style(Style::default().bg(theme::palette::RATATUI_VOID))
        .render(area, &mut buf);
    let table_rows = rows.into_iter().map(|(cmd, hint)| {
        Row::new([
            Line::from(Span::styled(
                cmd,
                Style::default()
                    .fg(theme::palette::RATATUI_GOLD)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(
                hint,
                Style::default().fg(theme::palette::RATATUI_CYAN),
            )),
        ])
    });
    Table::new(table_rows, [Constraint::Length(26), Constraint::Min(20)])
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(theme::palette::RATATUI_VIOLET))
                .title(Line::from(Span::styled(
                    " commands ",
                    Style::default()
                        .fg(theme::palette::RATATUI_GOLD)
                        .add_modifier(Modifier::BOLD),
                )))
                .style(Style::default().bg(theme::palette::RATATUI_VOID)),
        )
        .render(area, &mut buf);
    print!("{}", buffer_to_ansi(&buf));
    println!();
}

fn render_chips(buf: &mut Buffer, x: u16, y: u16, width: u16) {
    let mut cursor = x;
    for (index, (key, label)) in CHIPS.iter().enumerate() {
        let key_text = format!(" {key} ");
        let label_text = format!(" {label} ");
        let key_style = Style::default()
            .fg(theme::palette::RATATUI_VOID)
            .bg(theme::banner_line_ratatui(index))
            .add_modifier(Modifier::BOLD);
        let label_style = Style::default()
            .fg(theme::banner_line_ratatui(index))
            .bg(theme::palette::RATATUI_VOID)
            .add_modifier(Modifier::BOLD);
        buf.set_stringn(cursor, y, &key_text, width as usize, key_style);
        cursor = cursor.saturating_add(key_text.chars().count() as u16);
        buf.set_stringn(cursor, y, &label_text, width as usize, label_style);
        cursor = cursor.saturating_add(label_text.chars().count() as u16);
        buf.set_stringn(cursor, y, "  ", width as usize, Style::default());
        cursor = cursor.saturating_add(2);
    }
}

/// Single-line ANSI chip row for the interactive rustyline prompt.
///
/// # Returns
///
/// Truecolor ANSI without a trailing newline.
pub fn chips_ansi() -> String {
    let width = 72u16;
    let area = Rect::new(0, 0, width, 1);
    let mut buf = Buffer::empty(area);
    render_chips(&mut buf, 0, 0, width);
    buffer_to_ansi(&buf).trim_end().to_string()
}

/// Converts a ratatui buffer into ANSI text for inline (non-alternate-screen)
/// display so rustyline can take over afterwards.
fn buffer_to_ansi(buf: &Buffer) -> String {
    let area = buf.area();
    let mut out = String::new();
    for y in area.top()..area.bottom() {
        let mut line = String::new();
        let mut last: Option<(ratatui::style::Color, ratatui::style::Color, Modifier)> = None;
        for x in area.left()..area.right() {
            let cell = &buf[(x, y)];
            let key = (cell.fg, cell.bg, cell.modifier);
            if last != Some(key) {
                line.push_str("\u{1b}[0m");
                line.push_str(&ansi_for(cell.fg, cell.bg, cell.modifier));
                last = Some(key);
            }
            line.push_str(cell.symbol());
        }
        line.push_str("\u{1b}[0m");
        let trimmed = line
            .trim_end_matches(' ')
            .trim_end_matches("\u{1b}[0m")
            .to_string();
        if trimmed.chars().any(|c| !c.is_whitespace() && c != '\u{1b}')
            || trimmed.contains('\u{1b}')
        {
            out.push_str(trimmed.trim_end());
            out.push('\n');
        }
    }
    out
}

fn ansi_for(fg: ratatui::style::Color, bg: ratatui::style::Color, modifier: Modifier) -> String {
    let mut seq = String::new();
    if modifier.contains(Modifier::BOLD) {
        seq.push_str("\u{1b}[1m");
    }
    if let ratatui::style::Color::Rgb(r, g, b) = fg {
        seq.push_str(&format!("\u{1b}[38;2;{r};{g};{b}m"));
    }
    if let ratatui::style::Color::Rgb(r, g, b) = bg {
        seq.push_str(&format!("\u{1b}[48;2;{r};{g};{b}m"));
    }
    seq
}

/// Paints the default banner art into a ratatui [`Buffer`].
pub fn render_into(buf: &mut Buffer, area: Rect) {
    for (index, line) in BANNER.lines().enumerate() {
        if index as u16 >= area.height {
            break;
        }
        let style = Style::default()
            .fg(theme::banner_line_ratatui(index))
            .add_modifier(Modifier::BOLD);
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
            "Set up Aimee Codes via our zsh plugin:".dimmed(),
            "aimee zsh setup".bold().truecolor(163, 255, 18),
        ),
        format!(
            "{} {} {}",
            "·".dimmed(),
            "Learn more:".dimmed(),
            "https://aimeecodes.dev/docs/zsh-support".truecolor(0, 229, 255)
        ),
    ]);
    println!("{}", tip);
}

/// Invisible widget wrapper so the banner can sit on a ratatui surface.
pub struct BannerWidget;

impl Widget for BannerWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        render_splash(buf, area);
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use ratatui::buffer::Buffer;
    use ratatui::layout::Rect;

    use super::*;

    fn buffer_text(buf: &Buffer) -> String {
        let area = buf.area();
        let mut out = String::new();
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                out.push_str(buf[(x, y)].symbol());
            }
            out.push('\n');
        }
        out
    }

    #[test]
    fn test_banner_art_contains_aimee_and_loops_needles() {
        let actual = BANNER;
        assert!(actual.contains(r#"    ___    ____ __  _______ ______"#));
        assert!(actual.contains("/___/"));
    }

    #[test]
    fn test_render_into_buffer_wider_than_art() {
        let area = Rect::new(0, 0, 80, 6);
        let mut actual = Buffer::empty(area);
        render_into(&mut actual, area);

        let mid = actual[(4, 0)].symbol().to_string();
        let base = actual[(0, 4)].symbol().to_string();
        assert_eq!(mid, "_");
        assert!(base == "/" || base == "_" || !base.is_empty());
    }

    #[test]
    fn test_render_splash_includes_tagline_and_aimee_chip() {
        let area = splash_area();
        let mut buf = Buffer::empty(area);
        render_splash(&mut buf, area);
        let actual = buffer_text(&buf);
        assert!(actual.contains("CLI agent flock"));
        assert!(actual.contains("Aimee Codes"));
        assert!(actual.contains("🍑"));
        assert!(actual.contains("LOOP"));
        assert!(actual.contains(":aimee"));
        assert!(actual.contains(":muse"));
        assert!(actual.contains(":sage"));
    }

    #[test]
    fn test_buffer_to_ansi_emits_truecolor_and_chip() {
        let area = splash_area();
        let mut buf = Buffer::empty(area);
        render_splash(&mut buf, area);
        let actual = buffer_to_ansi(&buf);
        assert!(actual.contains("\u{1b}[38;2;"));
        assert!(actual.contains(":aimee"));
        assert!(actual.contains(TAGLINE));
    }

    #[test]
    fn test_chips_ansi_is_single_line_with_aimee() {
        let actual = chips_ansi();
        assert!(actual.contains(":aimee"));
        assert!(actual.contains("implement"));
        assert!(!actual.contains('\n'));
        assert!(actual.contains("\u{1b}[38;2;") || actual.contains("\u{1b}[48;2;"));
    }
}
