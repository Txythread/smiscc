use colorize::AnsiColor;
use termimad;


#[derive(Clone, Debug)]
pub struct LineMap {
    pub lines: Vec<Line>,
    pub warning_count: u32,
    pub error_count: u32,
}


impl LineMap{
    pub fn add_line(&mut self, line: Line){
        self.lines.push(line.clone());
    }

    pub fn display_warning(&mut self, info: NotificationInfo){
        self.warning_count += 1;

        println!("{} {}", "Warning:".bold().yellow(), info.title.yellow());

        for mut notification_info in info.display_code{
            notification_info.print(self.clone());

            println!()
        }

        println!("{}", info.message);
        println!()
    }

    pub fn copy_meta_data(&mut self) -> Self {
        let mut line_map = LineMap::new();

        line_map.error_count = self.error_count;
        line_map.warning_count = self.warning_count;

        line_map
    }

    pub fn display_error(&mut self, info: NotificationInfo){

        self.error_count += 1;

        println!("{} {}", "Error:".bold().red(), info.title.red());

        for mut notification_info in info.display_code{
            notification_info.print(self.clone());

            println!()
        }

        println!("{}", info.message);
        println!()
    }

    pub fn new() -> Self {
        LineMap{lines: Vec::new(), warning_count: 0, error_count: 0 }
    }


    /// Gets the position of tokens in a line given their indexes.
    ///
    /// If the end index is negative, this means everything in the
    /// line after the start index.
    pub fn get_position_of_tokens(&self, line: u32, start_pos: u16, end_pos: i16) -> TokenPosition{
        let start_token_start = self.lines[line as usize].tokens_positions[start_pos as usize].start;

        let mut end_pos = end_pos;
        // If the end token is negative, this means the entire line should be underlined
        if end_pos.is_negative(){
            end_pos = (self.lines[line as usize].tokens_positions.len() - 1) as i16;
        }

        let end_token_position = self.lines[line as usize].tokens_positions[end_pos as usize].clone();
        let end_token_end = end_token_position.start + end_token_position.length;
        let total_length = end_token_end - start_token_start;

        TokenPosition::new(start_pos, total_length)
    }


    /// Generates a line map with several lines that
    /// with tokens that should be long enough for most
    /// testing purposes.
    #[cfg(test)]
    pub fn test_map() -> Self {
        let mut map = Self::new();

        for _ in 0..100 {
            map.add_line(Line::test_line());
        }

        map
    }
}


/// ### Contains information about an error/warning.
///
/// This information does not include whether it's an error or warning itself as that is determined
/// by the line map's functions (display error/warning).
/// 
/// Multiple display code infos can be stored to higlight multiple parts of the code.
pub struct NotificationInfo {
    pub title: String,
    pub message: String,

    pub display_code: Vec<DisplayCodeInfo>,
}

impl NotificationInfo {
    pub fn new(title: String, message: String, display_code: Vec<DisplayCodeInfo>) -> Self {
        NotificationInfo{title, message, display_code}
    }
}


/// ### Contains information about the affected line.
///
/// #### This includes:
/// 1. The number in the line map (the one from which the error is thrown)
/// 2. The first affected token, that should be underlined
/// 3. The last token that should be underlined (or -1 for all tokens afterward)
/// 4. Annotations that get printed under the first affected token.
/// 5. The kind of info (warning, error, additional information)
///
/// Multiple display code infos may be stored in a notification info.
///
#[derive(Clone)]
pub struct DisplayCodeInfo{
    pub line_number_in_map: u32,
    pub start_token: u32,
    pub end_token: i32,             // If negative, this means the rest of the line
    pub annotations: Vec<String>,   // Things like (=) note that get printed under the affected
                                    // tokens. Look at the rust compiler
    pub kind: DisplayCodeKind,      // For example: InitialError (=> red), InitialWarning (=>
                                    // yellow) or AdditionalInfo (=> green)
}


impl DisplayCodeInfo {
    pub fn new(line_number_in_map: u32, start_token: u32, end_token: i32, annotations: Vec<String>, kind: DisplayCodeKind) -> Self{
        DisplayCodeInfo { line_number_in_map, start_token, end_token, annotations, kind }
    }


    /// Write the information to the screen given a line map (which contains the code).
    pub fn print(&mut self, line_map: LineMap){
        let line = line_map.lines[self.line_number_in_map as usize].clone();
        
        let mut line_number_string = format!("{} |", line.line_number);


        // Print the file name
        let mut file_name_string = format!("--> {}", line.source_file_name);

        for _ in 0..line_number_string.len()-2{
            file_name_string.insert(0, ' ');
        }

        println!("{}", file_name_string.blue());

        // The string that gets printed for separation (blue, starts with leading characters
        // followed by a |)
        let mut separation_string = String::new();

        for _ in 0..line_number_string.len()-1 {
            separation_string += " ";
        }

        separation_string = (separation_string.to_string() + "|\t").blue();

        println!("{}", separation_string);

        println!("{}\t{}", line_number_string.blue(), line.trimmed_contents);


        // Print the underlined part

        // A String up to the start position.
        let mut leading_text = separation_string.clone();

        // 1. Add the leading spaces
        let start_token_start = line.tokens_positions[self.start_token as usize].start;
        for _ in 0..start_token_start {
            leading_text += " ";
        }

        leading_text = leading_text.blue();

        // 2. Generate the underline string
        let mut underline_character: String = String::from("^");

        // 2.1 Change the colour of the character
        match self.kind {
            DisplayCodeKind::InitialError => {
                underline_character = underline_character.red();
            }
            DisplayCodeKind::InitialWarning => {
                underline_character = underline_character.yellow();
            }
            DisplayCodeKind::AdditionalInfo => {
                underline_character = underline_character.blue();
            }
        }

        let mut end_token = self.end_token;

        // If the end token is negative, this means the entire line should be underlined
        if end_token.is_negative(){
            end_token = (line.tokens_positions.len() - 1) as i32;
        }

        let end_token_position = line.tokens_positions[end_token as usize].clone();
        let end_token_end = end_token_position.start + end_token_position.length;
        let underlined_length = end_token_end - start_token_start;

        print!("{}", leading_text);

        for _ in 0..underlined_length {
            print!("{}", underline_character);
        }

        println!();

        if !self.annotations.is_empty() {
            println!("{}{}", leading_text, "|".blue());
        }

        for annotation in self.annotations.clone() {
            print!("{}", leading_text);

            println!("{} {}", "=".blue(), termimad::inline(annotation.as_str()));
        }

        println!("{}", separation_string);
    }
}


/// Controls the formatting (just the color rn) of a display code info.
#[derive(Clone)]
pub enum DisplayCodeKind{
    InitialError,                    // The error the message is about
    InitialWarning,                 // The warning the message is about
    AdditionalInfo,                 // Infos (for example function definitions when parameters are)
                                    // not called directly.
}


/// Holds all relevant information about a line for displaying it in a notification (warning/error)
/// later.
#[derive(Clone, Debug)]
pub struct Line{
    pub trimmed_contents: String,
    pub indent: u16,
    pub source_file_name: String,
    pub line_number: u32,
    pub tokens_positions: Vec<TokenPosition>,
}

impl Line {
    pub fn new(source_file_name: String, line_number: u32, tokens_positions: Vec<TokenPosition>, indent: u16, trimmed_contents: String) -> Self {
        Line { source_file_name, line_number, tokens_positions, indent, trimmed_contents }
    }

    /// Generates a long line (100 tokens) for testing purposes
    #[cfg(test)]
    pub fn test_line() -> Self {
        let mut tokens: Vec<TokenPosition> = Vec::new();

        for _ in 0..100{
            tokens.push(TokenPosition::new(0, 0));
        }

        let line = Self::new("n/a".to_string(), 0, tokens, 0, String::new());

        line
    }

}


#[derive(Clone, Debug, PartialEq)]
pub struct TokenPosition{
    pub start: u16,
    pub length: u16,
}

impl TokenPosition {
    pub fn new(start: u16, length: u16) -> Self {
        TokenPosition { start, length }
    }

    /// Creates a token for when it doesn't really matter in test cases
    #[cfg(test)]
    pub fn test_value() -> Self {
        Self::new(0, 0)
    }
}
