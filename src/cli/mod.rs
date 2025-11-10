mod context;

use clap::Parser;
use colored::Colorize;
pub use context::ServerContext;
use std::{
    fs,
    path::{Path, PathBuf},
    process::exit,
};

#[derive(Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    author,
    version,
    about,
    help_template = r#"{about}

Version: {version}
Author: {author}

Usage: {usage}

{all-args}
"#
)]
pub struct Args {
    #[arg(
        short,
        long,
        default_value = ".",
        help = "path to be distributed can be a file or a directory., e.g. '~/desktop/temp'"
    )]
    path: String,

    #[arg(long, default_value = "80", help = "port used by the service")]
    port: u16,
}

fn parse_path(path: String) -> Result<(PathBuf, Vec<PathBuf>), String> {
    let path: &Path = Path::new(&path);

    if !path.exists() {
        return Err("given path is invalid".to_string());
    }

    let mut files: Vec<PathBuf> = Vec::new();
    let base_dir: PathBuf = path.to_path_buf();

    if !path.is_dir() {
        files.push(path.to_path_buf());
        return Ok((base_dir.parent().unwrap().to_path_buf(), files));
    }

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap().path();
        if entry.is_dir() {
            continue;
        }

        files.push(entry);
    }

    Ok((base_dir, files))
}

pub fn parse() -> ServerContext {
    let args = Args::parse();

    let (base_dir, files) = match parse_path(args.path) {
        Ok((base_dir, files)) => (base_dir, files),
        Err(err) => {
            eprintln!("{}: {}", "error".red().bold(), err);
            exit(1);
        }
    };

    let port = args.port;

    ServerContext {
        base_dir: base_dir,
        files: files,
        port: port,
    }
}
