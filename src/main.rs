#![warn(unused_extern_crates)]

use crate::compiler::compile;
use clap::Parser;
use crate::help::print_help;

mod compiler;
mod config;
mod util;
mod help;


#[derive(Clone, Debug, PartialEq, Parser)]
pub struct ArgumentList{
    pub file: Option<String>,

    #[clap(short, long)]
    pub help: bool,                                 // -h or --help

    #[clap(long)]
    pub instruction_help: Option<Option<String>>,   // --instruction-help

    #[clap(short, long)]
    pub output_name: Option<Option<String>>,        // -o or --output

    #[clap(short, long, num_args = 0..=1)]
    pub get_micro_operation: Option<Option<String>>,// --get-micro-operation

    #[clap(long)]
    pub show_splitted: bool,                        // --show-splitted

    #[clap(short, long)]
    pub generate_instruction_table: bool,           // --generate-instructions-table

    #[clap(long)]
    pub show_tokens: bool,                          // --show-tokens
}

fn main() {
    let args = ArgumentList::parse();

    if args.help {
        print_help(args.clone())
    }

    if let Some(ref file_name) = args.file {
        let file_contents = std::fs::read_to_string(file_name).unwrap();
        compile(file_contents, args);
    } else {
        let file_contents = std::fs::read_to_string("test2.txt").unwrap();
        compile(file_contents, args);
    }
}
