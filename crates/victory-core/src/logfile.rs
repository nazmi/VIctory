//! A copy of everything this process logs, on disk while it runs.
//!
//! `VICTORY_LOG_FILE` names a file (or `=1` for the default) and every log
//! line is appended there *and* still printed. This exists for a run that
//! ends badly: a crash, a device removal, or a `TerminateProcess` from the
//! title never unwinds, so anything buffered in memory is lost. Each line is
//! written and flushed as it is logged, so the file holds everything up to
//! the moment the process stops, not what it managed to flush on the way out.
//!
//! The default is `victory.log` beside `crashes.log`, in the title's data
//! directory: `<data>/<title id>/victory.log`. That directory is checked
//! against the asset policy like every other output path.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use tracing_subscriber::fmt::MakeWriter;

/// Appends to one file, flushing after every write.
///
/// A `Mutex<File>` rather than an `Arc<File>`: the subscriber writes from
/// every worker thread at once, and a write split across threads would
/// interleave two lines. `tracing` reuses one `MakeWriter` value per event.
#[derive(Clone, Debug)]
pub(crate) struct ToFile {
    file: std::sync::Arc<Mutex<std::fs::File>>,
}

impl ToFile {
    /// Opens (creating, appending to) `path`, which must be outside the
    /// repository. Returns the error rather than panicking: a run that cannot
    /// write its log should still run.
    pub(crate) fn open(path: &Path) -> std::io::Result<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        Ok(ToFile {
            file: std::sync::Arc::new(Mutex::new(file)),
        })
    }
}

impl<'a> MakeWriter<'a> for ToFile {
    type Writer = ToFileWriter;

    fn make_writer(&'a self) -> Self::Writer {
        ToFileWriter {
            file: self.file.clone(),
        }
    }
}

/// One event's write, held under the file's lock for its whole length.
pub(crate) struct ToFileWriter {
    file: std::sync::Arc<Mutex<std::fs::File>>,
}

impl Write for ToFileWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut file = self
            .file
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        file.write_all(buf)?;
        // Flushed per line, not per buffer: the point of this file is that it
        // survives a process that never runs another instruction.
        file.flush()?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let mut file = self
            .file
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        file.flush()
    }
}

/// The file to log to, given what this command would use by default.
///
/// - unset, empty, `1`, `true`, `yes`: the default (`None` default means no file)
/// - `0`, `false`, `no`, `off`: no file, even where a default exists
/// - anything else: that path
///
/// `default` is `Some` for a title run, which keeps its log without being
/// asked, and `None` for everything else.
pub fn log_file_from_env(default: Option<PathBuf>) -> Option<PathBuf> {
    let value = std::env::var_os("VICTORY_LOG_FILE");
    resolve(
        value.as_deref().map(|v| v.to_string_lossy()).as_deref(),
        default,
    )
}

/// [`log_file_from_env`] with the variable's value passed in, so the rule can
/// be tested without touching process-wide environment.
fn resolve(value: Option<&str>, default: Option<PathBuf>) -> Option<PathBuf> {
    let Some(value) = value else {
        // Unset is the ordinary case, and for a run it still means "keep the
        // log": the default is what happens when nobody asked otherwise.
        return default;
    };
    let value = value.trim();
    if value.is_empty()
        || value == "1"
        || value.eq_ignore_ascii_case("true")
        || value.eq_ignore_ascii_case("yes")
    {
        return default;
    }
    if value == "0" || value.eq_ignore_ascii_case("false") || value.eq_ignore_ascii_case("off") {
        return None;
    }
    Some(PathBuf::from(value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_file_receives_whole_lines_from_several_writers() {
        // Two writers at once, as tracing makes on worker threads: no line
        // may interleave with another.
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("victory.log");
        let writer = ToFile::open(&path).unwrap();
        let threads: Vec<_> = (0..4)
            .map(|t| {
                let writer = writer.clone();
                std::thread::spawn(move || {
                    for i in 0..50 {
                        let line = format!(
                            "thread {t} line {i}
"
                        );
                        writer.make_writer().write_all(line.as_bytes()).unwrap();
                    }
                })
            })
            .collect();
        for t in threads {
            t.join().unwrap();
        }
        let text = std::fs::read_to_string(&path).unwrap();
        assert_eq!(text.lines().count(), 200, "every line arrived");
        for line in text.lines() {
            assert!(
                line.starts_with("thread ") && line.matches("line").count() == 1,
                "interleaved line: {line:?}"
            );
        }
    }

    #[test]
    fn the_switch_rules() {
        let default = || Some(PathBuf::from("data/victory.log"));
        // Unset: the command's default, which for a run is a file.
        assert_eq!(resolve(None, default()), default());
        assert_eq!(resolve(None, None), None);
        // On, spelled any way: the default.
        for on in ["", "1", "true", "TRUE", "yes", " 1 "] {
            assert_eq!(resolve(Some(on), default()), default(), "{on:?}");
        }
        // Off, even where a default exists.
        for off in ["0", "false", "OFF"] {
            assert_eq!(resolve(Some(off), default()), None, "{off:?}");
        }
        // Any other value is a path, taken as written.
        assert_eq!(
            resolve(Some("D:/logs/run.log"), default()),
            Some(PathBuf::from("D:/logs/run.log"))
        );
    }

    #[test]
    fn a_line_reaches_the_file_without_being_dropped() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested").join("victory.log");
        let writer = ToFile::open(&path).unwrap();
        writer
            .make_writer()
            .write_all(
                b"first line
",
            )
            .unwrap();
        writer
            .make_writer()
            .write_all(
                b"second line
",
            )
            .unwrap();

        // Read back through a separate handle: the point is that the bytes
        // are already on disk, not held in a buffer the process still owns.
        let text = std::fs::read_to_string(&path).unwrap();
        assert_eq!(
            text,
            "first line
second line
"
        );
    }

    #[test]
    fn opening_creates_the_directory_and_appends() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("a").join("b").join("victory.log");
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(
            &path,
            "earlier run
",
        )
        .unwrap();
        let writer = ToFile::open(&path).unwrap();
        writer
            .make_writer()
            .write_all(
                b"this run
",
            )
            .unwrap();
        let text = std::fs::read_to_string(&path).unwrap();
        assert_eq!(
            text,
            "earlier run
this run
",
            "append, not truncate"
        );
    }
}
