use clap::Parser;

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
        help = "Directory to be distrubute, e.g. '~/desktop/temp'"
    )]
    dir: String,

    #[arg(short, long, default_value = "80", help = "port used by the service")]
    port: i32,
}

pub fn parse() -> (String, i32) {
    let args = Args::parse();

    let dir = args.dir;
    let port = args.port;

    (dir, port)
}
