use clap::Parser;
use serde::Serialize;

#[cfg(test)]
mod stamp;

/// The `--version` text that build.rs makes, without the program name.
const LONG_VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/long_version.txt"));

/// A dummy app to practice build stamps and releases.
#[derive(Parser)]
#[command(
    version = VERSION,
    long_version = LONG_VERSION,
    after_help = concat!("vrsn ", include_str!(concat!(env!("OUT_DIR"), "/long_version.txt"))),
)]
struct Cli {
    /// Print the greeting as JSON
    #[arg(long)]
    json: bool,
}

const GREETING: &str = "Hello";
const GREETEE: &str = "vrsn";
const VERSION: &str = env!("VRSN_VERSION");

/// The `--json` output. serde writes the fields in this order.
#[derive(Serialize)]
struct Greeting {
    greeting: &'static str,
    greetee: &'static str,
    version: &'static str,
}

fn main() {
    let cli = Cli::parse();
    if cli.json {
        let greeting = Greeting {
            greeting: GREETING,
            greetee: GREETEE,
            version: VERSION,
        };
        println!(
            "{}",
            serde_json::to_string_pretty(&greeting).expect("Greeting has only strings")
        );
    } else {
        println!("{GREETING}, {GREETEE} {VERSION}!");
    }
}
