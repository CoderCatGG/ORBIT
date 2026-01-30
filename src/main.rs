use std::{fs, path::PathBuf, process::exit};

use clap::Parser;

mod lexer;

/// ORBIT Compiler and Interpreter
#[derive(Debug, Clone, Parser)]
struct Args {
    #[arg()]
    /// The input file to compile or interpret
    input: PathBuf,

    #[arg(short, long)]
    /// Set output file. If not specified, will output to STDOUT
    output: Option<PathBuf>,

    #[arg(short, long)]
    /// Turn on pretty print when printing to STDOUT/STDERR.
    /// Only active when `--output` is undefined.
    pretty_print: bool,

    #[arg(long)]
    /// Output tokens instead of interpreting
    emit_tokens: bool,
}

fn main() {
    let args = Args::parse();

    let input_text = fs::read_to_string(args.input).expect("Could not read input file");

    let tokens = lexer::to_tokens(&input_text);

    if args.emit_tokens {
        if let Some(out) = args.output {
            fs::write(out, format!("{:?}", tokens)).expect("Could not write to output path")
        } else if args.pretty_print {
            println!("{:#?}", tokens);
        } else {
            println!("{:?}", tokens);
        }

        exit(0)
    }

    
}
