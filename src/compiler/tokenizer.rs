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
pub fn tokenize(separated: Vec<Vec<String>>, line_map: &mut LineMap) {
    let mut constants: Vec<Object> = Vec::new();
    let mut variables: Vec<Object> = Vec::new();

    let mut current_namespace = String::new();

    let mut lines: Vec<Token> = Vec::new();

    for x in separated.iter().enumerate() {
        let line = x.1.clone();

        // The line number in the line map, not in the
        // original file.
        let line_number = x.0.clone();

        // The first token, might not necessarily be a keyword,
        // but it has to make some sorta sense.
        let initiating_keyword = line.iter().nth(0);

        if initiating_keyword.is_none() {
            // For some reason, there is no code in the line.
            continue;
        }

        let initiating_keyword = initiating_keyword.unwrap().as_str();

        match initiating_keyword {
            UNMODIFIABLE_OBJECT_DECLARATION_KEYWORD => {

            }

            MODIFIABLE_OBJECT_DECLARATION_KEYWORD => {

            }

            _ => {}
        }
    }
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
    use crate::compiler::tokenizer::Token;
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
}