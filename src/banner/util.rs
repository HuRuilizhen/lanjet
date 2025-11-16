use std::path::{Path, PathBuf};
use unicode_width::UnicodeWidthStr;

#[cfg(windows)]
use dunce::canonicalize;

const LABEL_PAD: usize = 15;

pub fn label(s: &str) -> String {
    let width = UnicodeWidthStr::width(s);
    let padding = if LABEL_PAD > width {
        LABEL_PAD - width
    } else {
        1
    };
    format!("{}{}", s, " ".repeat(padding))
}

pub fn human_size(bytes: u64) -> String {
    const UNITS: [&str; 4] = ["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut idx = 0;
    while size >= 1024.0 && idx < UNITS.len() - 1 {
        size /= 1024.0;
        idx += 1;
    }
    format!("{:.2} {}", size, UNITS[idx])
}

#[cfg(windows)]
pub fn canon(path: &Path) -> PathBuf {
    canonicalize(path).unwrap_or(path.to_path_buf())
}

#[cfg(not(windows))]
pub fn canon(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or(path.to_path_buf())
}
