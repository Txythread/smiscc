use crate::compiler::line_map::{Line, LineMap, TokenPosition};
use crate::config::tokenization_options::*;

/// ### Splits code into lines and lines into token-like parts.
///
/// Those mostly parts align with the tokenizer's most atomic units.
///
/// #### Examples:
///
/// ```
/// /*This is in a block comment
/// And so is this!*/
/// let a = 10; // This sets a to 10
/// let b = a /* 10 */ + 3; b -= 1;
/// ```
///
/// would get passed into:
///
/// ```
/// [
///     ["let", "a", "=", "10"],
///     ["let", "b", "=", "a", "+", "3"],
///     ["b", "-", "=", "1"],
/// ]
/// ```
///
/// and a corresponding line mapping.
///
/// *Note*: The parts don't always correlate with atomic tokens. Look at -=.
pub fn split(code: String, file_name: String, line_map: &mut LineMap) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();

    let mut block_escaped: bool = false;

    let mut current_token: String = String::new();

    let ignored_single_char_matches = IGNORED_SPLIT_CHARACTERS.iter().filter(|&x| x.len() == 1).map(|&x|x.chars().collect()).collect::<Vec<Vec<char>>>();
    let ignored_double_char_matches = IGNORED_SPLIT_CHARACTERS.iter().filter(|&x| x.len() == 2).map(|&x|x.chars().collect()).collect::<Vec<Vec<char>>>();

    let unignored_single_char_matches = UNIGNORED_SPLIT_CHARACTERS.iter().filter(|&x| x.len() == 1).map(|&x|x.chars().collect()).collect::<Vec<Vec<char>>>();
    let unignored_double_char_matches = UNIGNORED_SPLIT_CHARACTERS.iter().filter(|&x| x.len() == 2).map(|&x|x.chars().collect()).collect::<Vec<Vec<char>>>();


    let mut clean_code = String::new();

    for line in code.lines() {
        clean_code += trim(line.to_string(), &mut block_escaped).as_str();
    }

    let code = clean_code;
    line_map.files.push(Line::new(file_name, 0, vec![], 0, code.clone()));

    block_escaped = false;

    for char in code.chars() {
        let mut match_: String = String::new();
        let mut match_is_included = false;

        if STRING_MARKERS.0 == char && !block_escaped {
            if !current_token.is_empty() {
                tokens.push(current_token.clone());
                current_token = String::new();
            }
            line_map.files[0].tokens_positions.push(TokenPosition::new(0, 0));
            tokens.push(char.to_string());
            line_map.files[0].tokens_positions.push(TokenPosition::new(0, 0));
            block_escaped = true;
            continue;
        }

        if STRING_MARKERS.1 == char && block_escaped {
            tokens.push(current_token.clone());
            line_map.files[0].tokens_positions.push(TokenPosition::new(0, 0));
            tokens.push(char.to_string());

            current_token = String::new();
            block_escaped = false;
            continue;
        }

        if block_escaped {
            current_token.push(char);
            continue;
        }

        match_ = ignored_single_char_matches.iter().find(|&x|x == &vec![char]).unwrap_or(&vec![]).to_vec().iter().collect();

        if match_.is_empty() {
            match_ = unignored_single_char_matches.iter().find(|&x|x == &vec![char]).unwrap_or(&vec![]).to_vec().iter().collect();

            match_is_included = true;
        }

        if match_.is_empty() {
            if let Some(previous_char) = current_token.chars().last() {
                match_is_included = false;

                // Find a double-character token that matches the previous and teh last token
                if let Some(double_char_match) = ignored_double_char_matches.iter().find(|&x| x == &vec![previous_char, char]) {
                    match_ = double_char_match.iter().collect::<String>();
                }

                if match_.is_empty() {
                    match_is_included = true;


                    // Find a double-character token that matches the previous and teh last token
                    if let Some(double_char_match) = unignored_double_char_matches.iter().find(|&x| x == &vec![previous_char, char]) {
                        match_ = double_char_match.iter().collect::<String>();
                    }
                }
            }
        }


        if match_.is_empty() {
            current_token.push(char);
        } else {
            if match_.len() == 1 {
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                }
                line_map.files[0].tokens_positions.push(TokenPosition::new(0, 0));
            } else {
                current_token.remove(current_token.len() - 1);
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                }
            }

            if match_is_included {
                tokens.push(match_.clone());
                line_map.files[0].tokens_positions.push(TokenPosition::new(0, 0));
            }

            current_token = String::new();
        }


    }


    tokens
}




/// ### Removes comments, leading and trailing whitespaces.
///
/// **For example:**
/// `"\t let a /* new value*/ = 3  "` => `"let a  = 3"`
///
/// To remove multi-line block comments, the argument in_block_comment needs to be set to a false value initially and then be re-used every time a new line is passed to the function.
pub fn trim(line: String, in_block_comment: &mut bool) -> String {
    // In this function, the romanian flag is used for 
    // remains that should be removed in the end but make
    // the functions' syntax easier (to write, not to read :) )



    let mut output = String::new();
    let mut last_character = "🇷🇴".to_string();

    for character in line.chars(){
        if last_character == "/" && character == '/' {
            last_character = "🇷🇴".to_string();
            break;
        }

        if last_character == "/" && character == '*' {
            last_character = "🇷🇴".to_string();
            *in_block_comment = true;
        }else if last_character == "*" && character == '/' {
            *in_block_comment = false;
            last_character = "🇷🇴".to_string();
            continue;
        }

        if *in_block_comment {
            last_character = character.to_string();
            continue;
        }

        output += last_character.as_str();

        last_character = format!("{}", character);
    }

    if !*in_block_comment{
        output += last_character.as_str();
    }


    output = output.replace("🇷🇴", " ");


    output.trim().to_string()
}


#[cfg(test)]
mod tests {
    use crate::compiler::line_map::LineMap;
    use crate::compiler::splitter::{split, trim};


    #[test]
    pub fn test_trim(){
        let lines = vec![
            "/*",
            "This is in a block comment",
            "And so is this!*/",
            "let a = 10 // This sets a to 10",
            "let b = a /* 10 */ + 3",
        ];

        let expected = vec![
            "",
            "",
            "",
            "let a = 10",
            "let b = a   + 3",
        ];

        let mut output: Vec<String> = vec![];


        let mut in_block_comment = false;

        for line in lines {
            output.push(trim(line.to_string(), &mut in_block_comment));
        }

        assert_eq!(expected.len(), output.len());


        for i in 0..output.len() {
            assert_eq!(expected[i], output[i]);
        }
    }


    #[test]
    pub fn test_split(){
        let lines = vec![
            "/*",
            "This is in a block comment",
            "And so is this!*/",
            "let a = 10; // This sets a to 10",
            "let b = a /* 10 */ + 3; b -= 1;",
            "let c = \"Hello, world!\";",
            "let d = \"\";",
            "(was geht);",
        ];

        let expected = vec![
            vec!["let", "a", "=", "10", ";", ],
            vec!["let", "b", "=", "a", "+", "3", ";", ],
            vec!["b", "-", "=", "1", ";", ],
            vec!["let", "c", "=", "\"", "Hello, world!", "\"", ";", ],
            vec!["let", "d", "=", "\"", "", "\"", ";",],
            vec!["(", "was", "geht", ")", ";",],
        ].concat();

        let actual = split(lines.join("\n"), String::new(), &mut LineMap::test_map());

        assert_eq!(expected.join(":"), actual.join(":"));

    }
}
