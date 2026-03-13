//! Key press logger that writes to the terminal (stdout) or to a file.
//!
//! Only available when both `brkbx` and `key_log_std` features are enabled.
//! Use [`StdKeyLog::terminal()`] or [`StdKeyLog::to_file`] to configure the destination.

use crate::brkbx::KeyPressLog;
use std::io::Write;

/// Destination for key press logs: terminal (stdout) or a file.
pub enum StdKeyLog {
    Terminal,
    File(std::fs::File),
}

impl StdKeyLog {
    /// Log key events to the terminal (stdout).
    pub fn terminal() -> Self {
        StdKeyLog::Terminal
    }

    /// Log key events to a file. Creates or truncates the file at `path`.
    pub fn to_file(path: &std::path::Path) -> std::io::Result<Self> {
        std::fs::File::create(path).map(StdKeyLog::File)
    }
}

impl KeyPressLog for StdKeyLog {
    fn log_key(&mut self, row: u8, col: u8, pressed: bool) {
        let action = if pressed { "press" } else { "release" };
        let line = format!("brkbx key {} row={} col={}\n", action, row, col);
        let _ = match self {
            StdKeyLog::Terminal => std::io::stdout().lock().write_all(line.as_bytes()),
            StdKeyLog::File(f) => f.write_all(line.as_bytes()),
        };
    }
}
