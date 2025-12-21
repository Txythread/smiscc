use crate::compiler::compiler::compile;
use crate::compiler::line_map::*;
use clap::Parser;

mod compiler;
mod config;
mod util;

#[derive(Debug, PartialEq, Parser)]
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

    #[clap(short, long)]
    pub generate_instruction_table: bool,           // --generate-instructions-table
}

fn main() {
    let mut line_map = LineMap::new();

    line_map.add_line(Line::new(
        "main.txt".to_string(),
        1,
        vec![
            TokenPosition::new(0, 3),
            TokenPosition::new(4, 4),
            TokenPosition::new(8, 1),
            TokenPosition::new(10, 2),
            TokenPosition::new(13, 1),
        ],
        0,
        "let test = 50;".to_string(),
    ));


    let info = DisplayCodeInfo::new(
        0,
        1,
        1,
        vec![
            "**note:** This is an example note".to_string(),
            "*hint:* Change this variable".to_string(),
        ],
        DisplayCodeKind::InitialError
    );

    let notification = NotificationInfo::new(String::from("Test"), String::from("This is a test notification."), vec![info.clone()]);

    let args = ArgumentList::parse();

    if let Some(file_name) = args.file {
        let file_contents = std::fs::read_to_string(file_name).unwrap();
        compile(file_contents);
    } else {
        let file_contents = std::fs::read_to_string("test2.txt").unwrap();
        compile(file_contents);
    }


    line_map.display_error(notification);
}
