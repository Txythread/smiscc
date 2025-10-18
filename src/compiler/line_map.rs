#[derive(Clone, Debug)]
pub struct LineMap{
    pub lines: Vec<Line>
}


impl LineMap{
    pub fn add_line(line: Line){
        lines.append(line.clone());
    }

    pub fn display_warning(NotificationInfo){
        
    }
}


pub struct NotificationInfo{
    pub title: String,
    pub message: String,

    pub display_code: Vec<DisplayCodeInfo>,
}


#[derive(Clone)]
pub struct DisplayCodeInfo{
    pub line_number_in_map: u32,
    pub start_token: u32,
    pub end_token: u32,             // If negative, this means the rest of the line
    pub anotations: Vec<String>,    // Things like (=) note that get printed under the affected
                                    // tokens. Look at the rust compiler
    pub kind: DisplayCodeKind,      // For example: InitialError (=> red), InitialWarning (=>
                                    // yellow) or AdditionalInfo (=> green)
}


impl DisplayCodeInfo{
    pub fn print(&mut self, line_map: LineMap){
        let line = line_map.lines[self.line_number_in_map].clone();
        
        let line_number_string = format!("{} |", line.line_number).blue();

        // The string that gets printed for seperation (blue, starts with leading characters
        // followed by a |)
        let mut seperation_string = "";

        for _ in 0..seperation_string.len()-1 {
            seperation_string += " ";
        }

        seperation_string = (seperation_string + "|").blue();

        println!("{}", seperation_string);

        println!("{}\t{}", line_number_string, line.trimmed_contents);
    }
}


pub enum DisplayCodeKind{
    IntialError,                    // The error the message is about
    InitialWarning,                 // The warning the message is about
    AdditionalInfo,                 // Infos (for example function definitions when parameters are
                                    // not called directly.
}


#[derive(Clone, Debug)]
pub struct Line{
    pub trimmed_contents: String,
    pub indent: u16,
    pub source_file_name: String,
    pub line_number: u32,
    pub tokens_positions: Vec<TokenPosition>,
}


#[derive(Clone, Debug)]
pub struct TokenPosition{
    pub start: u16,
    pub length: u16,
}
