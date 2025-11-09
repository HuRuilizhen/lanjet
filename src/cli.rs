use crate::context::ServerContext;
use clap::Parser;
use colored::Colorize;
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

fn parse_path(path: String) -> Result<Vec<PathBuf>, String> {
    let path: &Path = Path::new(&path);

    if !path.exists() {
        return Err("given path is invalid".to_string());
    }

    let mut files: Vec<PathBuf> = Vec::new();

    if !path.is_dir() {
        files.push(path.to_path_buf());
        return Ok(files);
    }

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap().path();
        if entry.is_dir() {
            continue;
        }

        files.push(entry);
    }

    Ok(files)
}

pub fn parse() -> ServerContext {
    let args = Args::parse();

    let files = match parse_path(args.path) {
        Ok(files) => files,
        Err(err) => {
            println!("{}: {err}", "error".red().bold());
            exit(1);
        }
    };

    let port = args.port;

    ServerContext {
        files: files,
        port: port,
    }
}
