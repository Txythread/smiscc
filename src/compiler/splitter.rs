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
pub fn split(code: String) -> (Vec<Vec<String>>, LineMap) {
    let file_name = "n/a";


    let mut line_map = LineMap::new();
    let mut splitted_code: Vec<Vec<String>> = Vec::new();

    let mut in_block_comment: bool = false;

    for x in code.lines().enumerate() {
        let line = x.1.to_string();
        let line_number = x.0 + 1;

        // Remove the comments
        let mut trimmed_line = trim(line, &mut in_block_comment);

        // Skip ahead in case the trimmed line contains nothing.
        if trimmed_line.is_empty() { continue; }

        // The logic line's seperated contents
        // One line in the code may produce multiple lines in the result in case there are multiple statements
        let mut splitted_line: Vec<String> = Vec::new();

        // The tokens for the logical line (see above)
        let mut line_tokens: Vec<TokenPosition> = Vec::new();

        let mut current_token: TokenPosition = TokenPosition::new(0, 0);


        let mut current_token_text: String = String::new();

        // Separate into (unclassified) tokens
        for x in trimmed_line.clone().chars().enumerate() {
            let character = x.1.clone();
            let i = x.0;

            /// Store the position (i) as the end position of the current token.
            let mut end_token = |line_tokens: &mut Vec<TokenPosition>, splitted_line: &mut Vec<String>| {
                current_token.length = i as u16 - current_token.clone().start;

                // Store the token if (and only if) it has a positive length
                if current_token.length != 0 {
                    line_tokens.push(current_token.clone());
                    splitted_line.push(current_token_text.clone());
                }

                current_token = TokenPosition::new(i as u16 + 1, 0);
                current_token_text = String::new();
            };

            if IGNORED_SPLIT_CHARACTERS.contains(&character.to_string().as_str()) {
                end_token(&mut line_tokens, &mut splitted_line);

                // Skip to prevent the addition of the character to the line
                continue;
            }

            if UNIGNORED_SPLIT_CHARACTERS.contains(&character.to_string().as_str()) {
                end_token(&mut line_tokens, &mut splitted_line);
                let token = TokenPosition::new(i as u16, 1);
                line_tokens.push(token.clone());
                splitted_line.push(String::from(character));
            }

            if NEW_LOGICAL_LINE_CHARACTERS.contains(&character.to_string().as_str()) {
                // 1. Save the current line's tokens to split code.
                end_token(&mut line_tokens, &mut splitted_line);

                if !splitted_line.is_empty() {
                    splitted_code.push(splitted_line.clone());
                    splitted_line = Vec::new();

                    line_map.add_line(
                        Line::new(
                            file_name.to_string(),
                            line_number as u32,
                            line_tokens.clone(),
                            0 /* serves no functionality rn */,
                            trimmed_line.clone(),
                        )
                    );

                    current_token_text = String::new();

                    current_token = TokenPosition::new(i as u16 + 1, 0);
                    line_tokens = Vec::new();
                }
            }

            current_token_text = current_token_text.clone() + &character.to_string();
        }


        if !splitted_line.is_empty() {
            // There was no line ending in the end, throw an error

            // TODO: Throw an error!
        }


    }

    (splitted_code, line_map)
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
    // syntax easier (to write, not to read :) )



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
        ];

        let expected = vec![
            vec!["let", "a", "=", "10"],
            vec!["let", "b", "=", "a", "+", "3"],
            vec!["b", "-", "=", "1"],
        ];

        let actual = split(lines.join("\n"));

        assert_eq!(expected, actual.0);

    }
}
