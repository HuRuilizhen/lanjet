use crate::cli::parse;
use std::path::PathBuf;
mod cli;

fn main() {
    let (files, _): (Vec<PathBuf>, i32) = parse();
    for file in files {
        println!("[FILE] {}", file.file_name().unwrap().to_str().unwrap());
    }
}
