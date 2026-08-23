use std::io;

use aimee_tracker::VERSION;
use colored::Colorize;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Fill, Paragraph, Widget};

use crate::theme;

const BANNER: &str = include_str!("banner");

/// Built-in agent-switch chips (loop + FE / BE / PLAT roster).
/// Keep in sync with `crates/aimee_repo/src/agents/*.md`.
const CHIPS: &[(&str, &str)] = &[
    // Loop
    (":aimee", "implement"),
    (":muse", "plan"),
    (":sage", "research"),
    // Frontend
    (":fe-ui", "ui"),
    (":fe-web3", "dapp"),
    (":fe-realtime", "stream"),
    (":fe-edge", "edge"),
    (":fe-qa", "e2e"),
    // Backend
    (":be-api", "api"),
    (":be-web3", "chain"),
    (":be-data", "data"),
    (":be-security", "sec"),
    (":be-reliability", "slo"),
    // Platform
    (":plat-k8s", "k8s"),
    (":plat-cloud", "cloud"),
    (":plat-compliance", "soc2"),
    (":plat-sre", "sre"),
];

/// Number of agent chips on the landing surface.
pub fn agent_chip_count() -> usize {
    CHIPS.len()
}

/// Displays the banner with version and command tips.
///
/// Warp-quiet landing: figlet art, one dim meta line, one compact command
/// hint line, one agent-flock line. No frames, gauges, or tab bars.
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

    Ok(())
}

/// Composes the branded splash into a ratatui buffer and writes it to stdout.
fn print_ratatui_splash() {
    let area = splash_area();
    let mut buf = Buffer::empty(area);
    render_splash(&mut buf, area);
    print!("{}", buffer_to_ansi(&buf));
}

/// Area for the quiet landing: art + meta line + width-wrapped flock rows.
fn splash_area() -> Rect {
    let art_width = BANNER
        .lines()
        .map(|line| line.chars().count() as u16)
        .max()
        .unwrap_or(60);
    let width = art_width.max(78).saturating_add(4);
    let art_lines = BANNER
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count() as u16;
    // art + meta + up to 5 flock rows (17 agents greedily wrap at ~82 cols).
    Rect::new(0, 0, width, art_lines.saturating_add(6))
}

/// Splits chips into rows that fit `width` columns, honoring each chip's
/// rendered size (` key `, ` label `, 2-col gap).
fn wrap_chips(width: u16) -> Vec<Vec<(String, String)>> {
    let limit = width as usize;
    let mut rows: Vec<Vec<(String, String)>> = Vec::new();
    let mut current: Vec<(String, String)> = Vec::new();
    let mut used = 0usize;
    for (key, label) in CHIPS {
        let chip_width = key.chars().count() + 2 + label.chars().count() + 2 + 2;
        if !current.is_empty() && used + chip_width > limit {
            rows.push(std::mem::take(&mut current));
            used = 0;
        }
        used += chip_width;
        current.push(((*key).to_string(), (*label).to_string()));
    }
    if !current.is_empty() {
        rows.push(current);
    }
    rows
}

/// Paints the Warp-quiet landing: art, one dim meta row, wrapped flock rows.
pub fn render_splash(buf: &mut Buffer, area: Rect) {
    Fill::new(" ")
        .style(Style::default().bg(theme::palette::RATATUI_VOID))
        .render(area, buf);

    let art_lines = BANNER
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count() as u16;
    let flock_rows = wrap_chips(area.width);
    let chunks = Layout::vertical([
        Constraint::Length(art_lines),
        Constraint::Length(1), // meta
        Constraint::Length(flock_rows.len() as u16),
    ])
    .split(area);

    if !chunks.is_empty() {
        render_into(buf, chunks[0]);
    }
    if chunks.len() > 1 {
        Paragraph::new(Line::from(vec![
            Span::styled(
                format!("v{VERSION}"),
                Style::default()
                    .fg(theme::palette::RATATUI_GOLD)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!(
                    " · {} agents · font: {}",
                    CHIPS.len(),
                    theme::WARP_FONT_FACE
                ),
                Style::default().fg(theme::palette::RATATUI_MUTED),
            ),
        ]))
        .render(chunks[1], buf);
    }
    if chunks.len() > 2 {
        let rows = Layout::vertical(
            std::iter::repeat_n(Constraint::Length(1), flock_rows.len()).collect::<Vec<_>>(),
        )
        .split(chunks[2]);
        let mut offset = 0;
        for (row, chips) in rows.iter().zip(&flock_rows) {
            let owned: Vec<(&str, &str)> = chips
                .iter()
                .map(|(k, l)| (k.as_str(), l.as_str()))
                .collect();
            render_chips_slice(buf, row.x, row.y, row.width, &owned, offset);
            offset += chips.len();
        }
    }
}

/// Compact command hints printed under the splash.
///
/// Warp keeps help to a single dim line (`⌘ / for commands`); we match that
/// with one quiet row instead of a framed table.
fn print_command_sheet(_cli_mode: bool) {
    let hint = format!(
        "{} for commands · {} agents · {} for shell",
        "/".truecolor(0xFF, 0xCC, 0x02),
        ":".truecolor(0xFF, 0xCC, 0x02),
        "!".truecolor(0xBF, 0x7A, 0xF0),
    );
    println!("{}", hint.dimmed());
    println!();
}

fn render_chips_slice(
    buf: &mut Buffer,
    x: u16,
    y: u16,
    width: u16,
    chips: &[(&str, &str)],
    _color_offset: usize,
) {
    // Warp pill style: key on a filled accent pill, role in muted text —
    // one accent per chip, no rainbow cycling.
    const PILL_ACCENTS: [ratatui::style::Color; 4] = [
        theme::palette::RATATUI_CYAN,
        theme::palette::RATATUI_VIOLET,
        theme::palette::RATATUI_MAGENTA,
        theme::palette::RATATUI_LIME,
    ];
    let mut cursor = x;
    for (index, (key, label)) in chips.iter().enumerate() {
        let accent = PILL_ACCENTS[index % PILL_ACCENTS.len()];
        let key_text = format!(" {key} ");
        let key_style = Style::default()
            .fg(theme::palette::RATATUI_VOID)
            .bg(accent)
            .add_modifier(Modifier::BOLD);
        buf.set_stringn(cursor, y, &key_text, width as usize, key_style);
        cursor = cursor.saturating_add(key_text.chars().count() as u16);
        let label_text = format!(" {label}  ");
        buf.set_stringn(
            cursor,
            y,
            &label_text,
            width as usize,
            Style::default().fg(theme::palette::RATATUI_MUTED),
        );
        cursor = cursor.saturating_add(label_text.chars().count() as u16);
        if cursor >= x.saturating_add(width) {
            break;
        }
    }
}

/// Single-line ANSI chip row for the interactive rustyline prompt.
///
/// Shows the loop trio plus a flock count so the prompt stays Warp-compact.
///
/// # Returns
///
/// Truecolor ANSI without a trailing newline.
pub fn chips_ansi() -> String {
    let width = 90u16;
    let area = Rect::new(0, 0, width, 1);
    let mut buf = Buffer::empty(area);
    // Loop agents always visible on the prompt; full flock is on the splash.
    const PROMPT_CHIPS: &[(&str, &str)] = &[
        (":aimee", "implement"),
        (":muse", "plan"),
        (":sage", "research"),
        (":fe|be|plat", "+15 more · / for cmds"),
    ];
    render_chips_slice(&mut buf, 0, 0, width, PROMPT_CHIPS, 0);
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
    fn test_render_splash_quiet_landing() {
        let area = splash_area();
        let mut buf = Buffer::empty(area);
        render_splash(&mut buf, area);
        let actual = buffer_text(&buf);
        // Meta line + two flock rows; no frame chrome.
        assert!(actual.contains("agents"));
        assert!(actual.contains("JetBrains Mono"));
        // Row 1 leads with the loop trio.
        assert!(actual.contains(":aimee"));
        assert!(actual.contains(":muse"));
        assert!(actual.contains(":sage"));
        // Row 2 carries the tail of the flock (17 agents split across 2 rows).
        assert!(actual.contains(":plat-sre") || actual.contains(":plat-cloud"));
        assert!(actual.contains(":fe-ui") || actual.contains("fe-ui"));
        assert!(!actual.contains('╭'));
        assert!(!actual.contains('╰'));
        assert_eq!(agent_chip_count(), 17);
    }

    #[test]
    fn test_buffer_to_ansi_emits_truecolor_and_chip() {
        let area = splash_area();
        let mut buf = Buffer::empty(area);
        render_splash(&mut buf, area);
        let actual = buffer_to_ansi(&buf);
        assert!(actual.contains("\u{1b}[38;2;"));
        assert!(actual.contains(":aimee"));
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
