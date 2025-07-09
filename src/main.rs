// src/main.rs
/*
 * Main executable for MLTestnetKitKitPro
 */

use clap::Parser;
use mltestnetkitkitpro::{Result, run};

#[derive(Parser)]
#[command(version, about = "MLTestnetKitKitPro - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
