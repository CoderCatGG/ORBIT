use std::{fs, path::PathBuf};

use clap::Parser;

mod lexer;

#[derive(Debug, Clone, Parser)]
struct Args {
    #[arg()]
    /// The input file to compile or interpret
    input: PathBuf,
}

fn main() {
    let args = Args::parse();

    let input_text = fs::read_to_string(args.input).expect("Could not read input file");

    eprintln!("{:?}", lexer::to_tokens(&input_text));
}
