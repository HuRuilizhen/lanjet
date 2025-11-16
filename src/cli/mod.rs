mod context;

use crate::matcher::Matcher;
use clap::Parser;
use colored::Colorize;
pub use context::{BannerContext, ServerContext};
use std::fs::{self, read_dir};
use std::net::SocketAddr;
use std::{
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

    #[arg(
        short,
        long,
        default_value = ".lanjetignore",
        help = "ignore rule file when finding files"
    )]
    ignore: String,

    #[arg(long, help = "server on local machine")]
    local_only: bool,

    #[arg(long, help = "generate qr code in banner")]
    show_qrcode: bool,
}

fn get_files(path: &Path, files: &mut Vec<PathBuf>, matcher: &Matcher) {
    if !path.is_dir() {
        return;
    }

    for entry in read_dir(path).into_iter().flatten() {
        let entry = entry.unwrap().path();

        if matcher.is_matched(&entry) {
            continue;
        }

        if entry.is_dir() {
            get_files(&entry, files, matcher);
        } else {
            files.push(entry);
        }
    }
}

fn parse_path(path: String, ignore_file: String) -> Result<(PathBuf, Vec<PathBuf>), String> {
    let path: &Path = Path::new(&path);

    if !path.exists() {
        return Err("given path is invalid".to_string());
    }

    let mut files: Vec<PathBuf> = Vec::new();
    let base_dir: PathBuf = path.to_path_buf();
    let matcher = Matcher::new(&base_dir, ignore_file);

    if !path.is_dir() {
        files.push(path.to_path_buf());
        return Ok((base_dir.parent().unwrap().to_path_buf(), files));
    }

    get_files(path, &mut files, &matcher);

    Ok((base_dir, files))
}

pub fn parse() -> (BannerContext, ServerContext) {
    let args = Args::parse();

    let ignore = PathBuf::from(&args.ignore);

    let (base_dir, files) = match parse_path(args.path, args.ignore) {
        Ok((base_dir, files)) => (base_dir, files),
        Err(err) => {
            eprintln!("{}: {}", "error".red().bold(), err);
            exit(1);
        }
    };
    let total_size = files
        .iter()
        .filter_map(|path| fs::metadata(path).ok())
        .map(|meta| meta.len())
        .sum();

    let port = args.port;
    let local_only = args.local_only;
    let addr = match local_only {
        true => SocketAddr::from(([127, 0, 0, 1], port)),
        false => SocketAddr::from(([0, 0, 0, 0], port)),
    };
    let show_qrcode = args.show_qrcode;

    (
        BannerContext {
            addr,
            base_dir: base_dir.clone(),
            ignore,
            files_count: files.len(),
            total_size,
            local_only,
            show_qrcode,
        },
        ServerContext {
            base_dir,
            files,
            addr,
        },
    )
}
