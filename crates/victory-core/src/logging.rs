//! One tracing setup for every binary.
//!
//! Filter with `VICTORY_LOG` (falls back to `RUST_LOG`), e.g.
//! `VICTORY_LOG=victory_loader=trace,info`. Debug and trace output exist
//! only in builds with the `diagnostics` feature; a plain release build
//! logs info and above whatever the filter asks for.

use tracing_subscriber::EnvFilter;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

/// The most verbose level this build can log.
const CAP: LevelFilter = if cfg!(feature = "diagnostics") {
    LevelFilter::TRACE
} else {
    LevelFilter::INFO
};

/// Starts logging. `log_file` (see [`crate::logfile::log_file_from_env`])
/// adds a copy of every line to that file, flushed as it is written, so a run
/// that dies without unwinding still has its log on disk.
pub fn init_to(default_level: &str, log_file: Option<std::path::PathBuf>) {
    let filter = std::env::var("VICTORY_LOG")
        .or_else(|_| std::env::var("RUST_LOG"))
        .ok()
        .and_then(|v| EnvFilter::try_new(v).ok())
        .unwrap_or_else(|| EnvFilter::new(default_level));
    let console = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_writer(std::io::stderr);
    let registry = tracing_subscriber::registry().with(CAP).with(filter);
    // A log file that cannot be opened must not stop the run: say so on
    // stderr (which still gets everything) and carry on without it.
    match log_file {
        Some(path) => match crate::logfile::ToFile::open(&path) {
            Ok(file) => {
                let file_layer = tracing_subscriber::fmt::layer()
                    .with_target(true)
                    // No escape codes in a file: they make it unreadable in
                    // an editor and unsearchable by grep.
                    .with_ansi(false)
                    .with_writer(file);
                let _ = registry.with(console).with(file_layer).try_init();
                // Announced after the subscriber exists so it lands in both
                // places: the reader needs the path, and it is the first line
                // of the file.
                tracing::info!(path = %path.display(), "logging to this file as well as stderr");
            }
            Err(e) => {
                let _ = registry.with(console).try_init();
                tracing::warn!(
                    path = %path.display(),
                    error = %e,
                    "cannot write a log file; logging to stderr only"
                );
            }
        },
        None => {
            let _ = registry.with(console).try_init();
        }
    }
}

/// Starts logging to stderr only (tests, tools that run no title).
pub fn init(default_level: &str) {
    init_to(default_level, None);
}
