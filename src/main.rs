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

    #[clap(short, long)]
    pub output_name: Option<Option<String>>,        // -o or --output

    #[clap(long)]
    pub show_tokens: bool,                          // --show-tokens

    #[arg(long = "ol")]
    pub optimization_level: Option<Option<u8>>,     // --ol

    #[arg(long)]                                    // --optimizations
    pub optimizations: Option<Vec<String>>,
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