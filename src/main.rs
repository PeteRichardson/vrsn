use clap::Parser;

#[cfg(test)]
mod stamp;

/// The `--version` text that build.rs makes, without the program name.
const LONG_VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/long_version.txt"));

/// A dummy app to practice build stamps and releases.
#[derive(Parser)]
#[command(
    version = env!("VRSN_VERSION"),
    long_version = LONG_VERSION,
    after_help = concat!("vrsn ", include_str!(concat!(env!("OUT_DIR"), "/long_version.txt"))),
)]
struct Cli {}

fn main() {
    Cli::parse();
    println!("Hello, vrsn {}!", env!("CARGO_PKG_VERSION"));
}
