use std::time::Duration;

/// Formats a chrono DateTime as a human-readable relative time string (e.g., "5
/// minutes ago").
pub fn humanize_time(dt: chrono::DateTime<chrono::Utc>) -> String {
    let duration = chrono::Utc::now().signed_duration_since(dt);
    let duration = Duration::from_secs((duration.num_minutes() * 60).max(0) as u64);
    if duration.is_zero() {
        "now".to_string()
    } else {
        format!("{} ago", humantime::format_duration(duration))
    }
}

/// Humanizes a number to a readable format with B/M/k suffixes.
///
/// # Arguments
/// * `n` - The number to humanize
///
/// # Examples
/// ```ignore
/// assert_eq!(humanize_number(1500), "1.5k");
/// assert_eq!(humanize_number(1500000), "1.5M");
/// assert_eq!(humanize_number(1500000000), "1.5B");
/// assert_eq!(humanize_number(500), "500");
/// ```
pub fn humanize_number(n: usize) -> String {
    match n {
        n if n >= 1_000_000_000 => format!("{:.1}B", n as f64 / 1_000_000_000.0),
        n if n >= 1_000_000 => format!("{:.1}M", n as f64 / 1_000_000.0),
        n if n >= 1_000 => format!("{:.1}k", n as f64 / 1_000.0),
        _ => n.to_string(),
    }
}

/// Terminal width probe that tolerates ptys reporting zero rows.
///
/// `terminal_size` returns `None` when either winsize dimension is 0, which
/// happens on `script`-style ptys, some CI runners, and freshly attached
/// tmux panes — the CLI then silently renders at the 80-column fallback
/// regardless of the real width. This probes stdout first (the surface the
/// UI paints on), then stderr (where the spinner draws), accepting a size
/// whose column count is positive even when rows is 0.
#[cfg(unix)]
pub fn terminal_columns() -> Option<usize> {
    for fd in [libc::STDOUT_FILENO, libc::STDERR_FILENO] {
        let mut winsize = libc::winsize { ws_row: 0, ws_col: 0, ws_xpixel: 0, ws_ypixel: 0 };
        // SAFETY: `winsize` is a plain POD struct valid for the duration of
        // the call; TIOCGWINSZ only writes into it.
        if unsafe { libc::ioctl(fd, libc::TIOCGWINSZ, &mut winsize) } == 0 && winsize.ws_col > 0 {
            return Some(winsize.ws_col as usize);
        }
    }
    None
}

/// Windows fallback: the raw `TIOCGWINSZ` probe is Unix-only. Zero-row ptys
/// are a Unix quirk, so the `terminal_size` crate's console API suffices.
#[cfg(not(unix))]
pub fn terminal_columns() -> Option<usize> {
    terminal_size::terminal_size().map(|(width, _)| width.0 as usize)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_humanize_number_billions() {
        let actual = humanize_number(1_500_000_000);
        let expected = "1.5B";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_humanize_number_millions() {
        let actual = humanize_number(2_300_000);
        let expected = "2.3M";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_humanize_number_thousands() {
        let actual = humanize_number(4_500);
        let expected = "4.5k";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_humanize_number_small() {
        let actual = humanize_number(999);
        let expected = "999";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_humanize_number_zero() {
        let actual = humanize_number(0);
        let expected = "0";
        assert_eq!(actual, expected);
    }
}
