// src/main.rs
/*
 * Main executable for FluidSource
 */

use clap::Parser;
use fluidsource::{Result, run};

#[derive(Parser)]
#[command(version, about = "FluidSource - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
