// Termimad is for Markdown formatting in the terminal.
use termimad;
use include_dir::{include_dir, Dir};
use crate::ArgumentList;
use crate::util::exit::{exit, ExitCode};

const HELP_STRING: &str = include_str!("help.md");
const COMMAND_HELP_FILES: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/help/commands");


pub fn print_help(arguments: ArgumentList) {
    if arguments.show_tokens {
        print_help_file("show-tokens".to_string());
        std::process::exit(0);
    }

    println!("{}", termimad::inline(HELP_STRING));
}


fn print_help_file(name: String) {
    let contents = COMMAND_HELP_FILES.get_file(format!("{}.md", name));

    if let Some(contents) = contents {
        println!("{}", termimad::inline(contents.contents_utf8().unwrap()));
    }else{
        exit(format!("Tried to open help file commands/{}, but failed.", name), ExitCode::Internal);
    }
}