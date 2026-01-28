use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Clone, Parser)]
struct Args {
    #[arg()]
    /// The input file to compile or interpret
    input: PathBuf,
}

fn main() {
    let args = Args::parse();
}
