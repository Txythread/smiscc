use crate::compiler::line_map::*;

mod compiler;
mod config;

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


    let mut info = DisplayCodeInfo::new(
        0,
        1,
        1,
        vec![
            "note: This is an example note".to_string(),
            "hint: Change this variable".to_string(),
        ],
        DisplayCodeKind::InitialError
    );

    let notification = NotificationInfo::new(String::from("Test"), String::from("This is a test notification."), vec![info.clone()]);

    line_map.display_error(notification);
}
