#![warn(unused_extern_crates)]

use std::rc::Rc;
use crate::compiler::compile;
use clap::Parser;
use crate::compiler::optimization::OptimizationFlags;
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

    #[arg(long = "ol", default_value_t = 1)]
    pub optimization_level: u8,                     // -o

    #[arg(long)]                                    // --optimizations
    pub optimizations: Vec<String>
}

fn main() {
    let args = ArgumentList::parse();

    if args.help {
        print_help(args.clone())
    }

    if let Some(ref file_name) = args.file {
        let file_contents = std::fs::read_to_string(file_name).unwrap();
        let optimizations = OptimizationFlags::new(&args);
        compile(file_contents, args, optimizations);
    } else {
        println!("Nothing to do!");
    }
}