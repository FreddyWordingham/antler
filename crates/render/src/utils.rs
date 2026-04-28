use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

#[must_use]
#[inline]
pub fn progress_bar(length: u64) -> ProgressBar {
    let progress_bar = ProgressBar::new(length);
    progress_bar.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({percent:>3}%) • ETA {eta_precise}",
        )
        .unwrap()
        .progress_chars("=>-"),
    );
    progress_bar.enable_steady_tick(Duration::from_millis(100));
    progress_bar
}
