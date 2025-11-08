use crate::cli::parse;

mod cli;

fn main() {
    let (dir, port): (String, i32) = parse();
    println!(r"{dir}, {port}");
}
