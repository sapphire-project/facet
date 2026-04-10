use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

/// Spinner for indeterminate operations (API calls, etc.).
pub fn spinner(msg: impl Into<String>) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::with_template("{spinner:.cyan} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    pb.set_message(msg.into());
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
}

/// Progress bar for a download of known size.
pub fn download_bar(total_bytes: u64, name: impl Into<String>) -> ProgressBar {
    let pb = ProgressBar::new(total_bytes);
    pb.set_style(
        ProgressStyle::with_template(
            "{msg}\n{wide_bar:.cyan/blue} {bytes:>10} / {total_bytes} \
             {bytes_per_sec:>12}  eta {eta}",
        )
        .unwrap()
        .progress_chars("█▉▊▋▌▍▎▏ "),
    );
    pb.set_message(format!("  Downloading {}", name.into()));
    pb
}

/// Spinner for a download of unknown size.
pub fn download_spinner(name: impl Into<String>) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::with_template("{spinner:.cyan} {msg}  {bytes} {bytes_per_sec}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    pb.set_message(format!("Downloading {}", name.into()));
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
}

/// Print a success line with a green checkmark, clearing any active bar.
pub fn success(pb: &ProgressBar, msg: impl Into<String>) {
    pb.println(format!("  {} {}", console_green("✓"), msg.into()));
}

fn console_green(s: &str) -> String {
    // Use ANSI only when stderr is a TTY.
    if std::io::IsTerminal::is_terminal(&std::io::stderr()) {
        format!("\x1b[32m{s}\x1b[0m")
    } else {
        s.to_string()
    }
}
