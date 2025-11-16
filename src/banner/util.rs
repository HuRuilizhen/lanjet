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

#[cfg(windows)]
pub fn canon(path: &Path) -> PathBuf {
    canonicalize(path).unwrap_or(path.to_path_buf())
}

#[cfg(not(windows))]
pub fn canon(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or(path.to_path_buf())
}
