use std::collections::HashMap;
use crate::compiler::data_types::{BuildResult, Buildable, IntegerType};
use crate::compiler::line_map::{DisplayCodeInfo, LineMap, TokenPosition};
use crate::compiler::object::{Object, ObjectType};
use crate::compiler::line_map::*;
use crate::config::tokenization_options::*;
use crate::util::math::ArithmeticOperation;

/// Turn the split string into tokens ("classify" them)
///
/// A simple `"10"` would be turned into an object with the parent
/// of an object type describing an u32 or whatever the default
/// number system is set to.
pub fn tokenize(separated: Vec<Vec<String>>, line_map: &mut LineMap) -> Vec<Vec<Token>> {
    let mut lines: Vec<Vec<Token>> = Vec::new();


    for x in separated.iter().enumerate() {
        let line = x.1.clone();

        // The line number in the line map, not in the
        // original file.
        let line_number = x.0.clone();


        // Set to "None" whenever no string is being built.
        // Set to an empty string whenever the string building process started
        // Has contents when the string was started, filled but not yet been
        // delimited.
        let mut current_string: Option<String> = None;
        let mut current_string_start: Option<u16> = None;

        let mut line_tokens: Vec<Token> = Vec::new();

        for y in line.iter().enumerate() {
            let token = y.1.clone();
            let token_number = y.0.clone();

            // Append to string if necessary or close it.
            if let Some(string) = current_string.clone() {
                // Check if the token is the delimiter
                if token == STRING_MARKERS.1.to_string() {
                    // Calculate the position of the resulting string
                    let start_token_number = current_string_start.unwrap_or((token_number - 1) as u16 /*Fallback, one token before current*/);
                    let new_position = line_map.get_position_of_tokens(line_number as u32, start_token_number, token_number as i16);

                    let token = Token::StringLiteral(string, new_position);
                    line_tokens.push(token);

                    current_string_start = None;
                    current_string = None;
                } else {
                    current_string = Some(current_string.unwrap() + token.as_str());
                }

                continue;
            }

            if token == STRING_MARKERS.0.to_string() {
                current_string = Some(String::new());
                current_string_start = Some(token_number as u16);

                continue;
            }
        }

        lines.push(line_tokens);
    }

    lines
}



/// ### Generates Unclassified Tokens up to the Delimiter
///
/// [Tokens](Token) get generated from the original code (`from`)
/// until the delimiter is found or the line ended (when the
/// delimiter is not enforced, in which case an error would be
/// thrown).
///
/// If the delimiting tokens appears to be a parenthesis (defined
/// [here](LOGICAL_PARENTHESES)), it will be ignored once if it was
/// opened another time within the function.
fn generate_unclassified_tokens(from: Vec<String>, delimiting_token: String, enforce_delimiter: bool, line_map: &mut LineMap) -> Option<Vec<Token>> {
todo!()
}



#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    /// A block that has yet to be classified
    UnspecifiedString(String, TokenPosition),

    /// Something that has been classified as a string
    StringLiteral(String, TokenPosition),

    /// A text that could be classified (and converted to) an integer.
    Integer(i128, IntegerType, TokenPosition),

    /// A keyword changing the interpretation of the entire line (such
    /// as "var", "let" or "if").
    BehaviouralKeyword(String, TokenPosition),

    /// A keyword changing minor parts about interpretation (such as
    /// "as".
    LogicalKeyword(String, TokenPosition),
}

impl Token {
    pub fn get_position(&self) -> TokenPosition {
        match self {
            Token::UnspecifiedString(_, pos) => { pos.clone() }
            Token::StringLiteral(_, pos) => { pos.clone() }
            Token::Integer(_, _, pos) => { pos.clone() }
            Token::BehaviouralKeyword(_, pos) => { pos.clone() }
            Token::LogicalKeyword(_, pos) => { pos.clone() }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::compiler::data_types::{Buildable, IntegerType};
    use crate::compiler::line_map::{LineMap, TokenPosition};
    use crate::compiler::object::{Object, ObjectType};
    use crate::compiler::tokenizer::{tokenize, Token};
    use crate::compiler::object::generate_object;

    #[test]
    fn test_generate_object() {
        let i32_type = IntegerType::Signed32BitInteger.build_type();
        let object_types: Vec<(ObjectType, Box<dyn Buildable>)> = vec![
            (i32_type.clone(), Box::new(IntegerType::Signed32BitInteger)),
        ];

        let token = Token::UnspecifiedString(String::from("10"), TokenPosition::new(0, 5));

        let mut line_map = LineMap::new();

        let result = generate_object(&mut vec![token], object_types, &mut line_map, 0, 0, 0);

        assert_eq!(result, /*Some(Token::Object(*/Some(Object::new(i32_type.type_uuid.clone(), String::new(), Some(10))/*, TokenPosition::new(0, 5)))*/))
    }

    #[test]
    fn test_tokenization() {
        let input_tokens = vec![
            vec!["\"".to_string(), "Was geht ab...".to_string(), "\"".to_string()],
            vec!["\"".to_string(), "".to_string(), "\"".to_string()],
            vec!["\"".to_string(), "... in Rumänien?".to_string(), "\"".to_string()],
        ];

        let expected_output = vec![
            vec![Token::StringLiteral("Was geht ab...".to_string(), TokenPosition::new(0, 0 /*ends with 0 now because the tokens don't match the input*/))],
            vec![Token::StringLiteral("".to_string(), TokenPosition::new(0, 0))],
            vec![Token::StringLiteral("... in Rumänien?".to_string(), TokenPosition::new(0, 0))],
        ];

        let actual_output = tokenize(input_tokens, &mut LineMap::test_map());

        assert_eq!(actual_output, expected_output);
    }
}