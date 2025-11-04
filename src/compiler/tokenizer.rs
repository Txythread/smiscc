use crate::compiler::data_types::{Buildable, IntegerType};
use crate::compiler::line_map::{ LineMap, TokenPosition};
use crate::compiler::line_map::*;
use crate::config::tokenization_options::*;
use crate::config::tokenization_options::Keyword;
use strum::IntoEnumIterator;
use crate::compiler::object::ObjectType;
use crate::util::math::convert_to_int;

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


        // Build the integer types
        // Those are used for checking for specific types (like unsigned 32-bit
        // integer) later in code.
        let integer_types = build_integer_types();

        'token_loop: for y in line.iter().enumerate() {
            let token = y.1.clone();
            let token_number = y.0.clone();
            let current_token_pos = line_map.get_position_of_tokens(line_number as u32, token_number as u16, token_number as i16);


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


            // Look if it's a keyword
            for keyword in Keyword::iter() {
                let keyword: Keyword = keyword;

                if keyword.as_ref() == token {
                    let token = Token::KeywordType(keyword, current_token_pos.clone());

                    line_tokens.push(token);
                    continue 'token_loop;
                }
            }

            // Maybe it's a boolean value?
            let true_keyword = BOOL_STATE_NAMES.0;
            let false_keyword = BOOL_STATE_NAMES.1;

            if token == true_keyword {
                let token = Token::BoolLiteral(true, current_token_pos.clone());

                line_tokens.push(token);
                continue 'token_loop;
            }

            if token == true_keyword {
                let token = Token::BoolLiteral(true, current_token_pos.clone());

                line_tokens.push(token);
                continue 'token_loop;
            }


            // Check if it's an integer literal
            {
                let integer_value = generate_integer(
                    Token::UnspecifiedString(token.clone(), current_token_pos.clone()),
                    integer_types.clone(),
                    line_number as u32,
                    token_number as u32,
                    line_map,
                );

                if let Some(integer_value) = integer_value {
                    line_tokens.push(Token::IntegerLiteral(integer_value.0, integer_value.1, current_token_pos.clone()));
                    continue 'token_loop;
                }
            }

            // As it's no other option, it can only be an identifier.
            let identifier = Token::Identifier(token.clone(), current_token_pos.clone());
            line_tokens.push(identifier);
        }

        lines.push(line_tokens);
    }

    lines
}


/// Builds all integer subtypes and returns them with their corresponding
/// types
fn build_integer_types() -> Vec<(IntegerType, ObjectType)> {
    let types = vec![
        IntegerType::Unsigned32BitInteger,
        IntegerType::Signed32BitInteger,
        IntegerType::Unsigned16BitInteger,
        IntegerType::Signed16BitInteger,
        IntegerType::Unsigned8BitInteger,
        IntegerType::Signed8BitInteger,
    ];

    let mut types_and_built_object_types : Vec<(IntegerType, ObjectType)> = Vec::new();

    for kind in types {
        let object_type = kind.build_type();
        types_and_built_object_types.push((kind, object_type));
    }

    types_and_built_object_types
}


/// Try building an integer using the provided types. If none of these
/// can generate an explicit/unambiguous result, no type info will be
/// provided ("None" in that field) in case an integer can still be
/// generated.
/// If no value could be generated, but it's clear that one should've been,
/// a zero with an unspecified type will be returned. Error displaying is
/// handled automatically.
fn generate_integer(text: Token, types: Vec<(IntegerType, ObjectType)>, line_number: u32, token_number: u32, line_map: &mut LineMap) -> Option<(i128, Option<IntegerType>)> {
    let mut unambiguous_result: Option<(i128, IntegerType)> = None;

    for kind in types.iter().enumerate() {
        let result = kind.1.0.build(vec![text.clone()], types[kind.0].1.clone());

        if result.ambiguous { continue; }

        if result.result.is_err() {
            let error = result.result.unwrap_err();

            let display_info = DisplayCodeInfo::new(
                line_number,
                token_number,
                token_number as i32,
                vec![],
                DisplayCodeKind::InitialError,
            );

            let notification = NotificationInfo::new(
                "Integer Couldn't Be Built".to_string(),
                error.message,
                vec![display_info],
            );

            line_map.display_error(notification);

            break;
        }

        // This was a success
        if let Some(integer_value) = result.result.unwrap().initial_content {
            return Some((integer_value, Some(kind.1.0.clone())))
        }
    }

    // Generate a value with no specified type
    if let Some(integer_value) = convert_to_int(text.get_raw_text().unwrap()) {
        if let Some(error) = integer_value.clone().err() {
            let display_info = DisplayCodeInfo::new(
                line_number,
                token_number,
                token_number as i32,
                vec![],
                DisplayCodeKind::InitialError,
            );

            let notification = NotificationInfo::new(
                "Unspecified Integer Couldn't Be Built".to_string(),
                error.message(),
                vec![display_info],
            );

            line_map.display_error(notification);

            // Return 0 as the value should be an integer value,
            // but it couldn't be decoded. This prevents unwanted
            // errors.
            return Some((0, None));
        }

        let integer_value = integer_value.unwrap();
        return Some((integer_value, None));
    }

    None
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    /// A block that has yet to be classified
    /// This should only appear **inside** the tokenizer & **never
    /// returned anywhere else**.
    UnspecifiedString(String, TokenPosition),

    /// Something that has been classified as a string
    StringLiteral(String, TokenPosition),

    /// A text that could be classified (and converted to) an integer.
    IntegerLiteral(i128, Option<IntegerType>, TokenPosition),

    /// A text that could be classified as being either true or false.
    BoolLiteral(bool, TokenPosition),

    /// A keyword (such "var", "let" or "if").
    KeywordType(Keyword, TokenPosition),

    /// Texts Identifying Variables, constants, ...
    Identifier(String, TokenPosition),
}

impl Token {
    pub fn get_position(&self) -> TokenPosition {
        match self {
            Token::UnspecifiedString(_, pos) => { pos.clone() }
            Token::StringLiteral(_, pos) => { pos.clone() }
            Token::IntegerLiteral(_, _, pos) => { pos.clone() }
            Token::BoolLiteral(_, pos) => { pos.clone() }
            Token::KeywordType(_, pos) => { pos.clone() }
            Token::Identifier(_, pos) => { pos.clone() }
        }
    }

    /// Gets the text a token contains if applicable
    pub fn get_raw_text(&self) -> Option<String> {
        match self {
            Token::StringLiteral(text, _) => { Some(text.clone()) }
            Token::Identifier(text, _) => { Some(text.clone()) }
            Token::BoolLiteral(value, _) => { Some( if *value { BOOL_STATE_NAMES.0.to_string().clone() } else { BOOL_STATE_NAMES.1.to_string().clone() }) }
            Token::UnspecifiedString(text, _) => { Some(text.clone()) }

            _ => None,
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::compiler::data_types::{Buildable, IntegerType};
    use crate::compiler::line_map::{LineMap, TokenPosition};
    use crate::compiler::object::{Object, ObjectType};
    use crate::compiler::tokenizer::{build_integer_types, generate_integer, tokenize, Token};
    use crate::compiler::object::generate_object;
    use crate::config::tokenization_options::Keyword;

    #[test]
    fn test_generate_object() {
        let i32_type = IntegerType::Signed32BitInteger.build_type();
        let object_types: Vec<(ObjectType, Box<dyn Buildable>)> = vec![
            (i32_type.clone(), Box::new(IntegerType::Signed32BitInteger)),
        ];

        let token = Token::UnspecifiedString(String::from("10"), TokenPosition::new(0, 5));

        let mut line_map = LineMap::new();

        let result = generate_object(&mut vec![token], object_types, &mut line_map, 0, 0, 0);

        assert_eq!(result, Some(Object::new(i32_type.type_uuid.clone(), String::new(), Some(10))/*, TokenPosition::new(0, 5)))*/))
    }

    #[test]
    fn test_tokenization() {
        let input_tokens = vec![
            /*0*/vec!["\"".to_string(), "Was geht ab...".to_string(), "\"".to_string()],
            /*1*/vec!["\"".to_string(), "".to_string(), "\"".to_string()],
            /*2*/vec!["\"".to_string(), "... in Rumänien?".to_string(), "\"".to_string()],
            /*3*/vec!["let".to_string(), "Was geht".to_string(), "var".to_string()],
            /*4*/vec!["var".to_string(), "true".to_string()],
            /*5*/vec!["var".to_string(), "10".to_string()],
            /*6*/vec!["var".to_string(), "10u32".to_string()],
        ];

        let expected_output = vec![
            /*0*/vec![Token::StringLiteral("Was geht ab...".to_string(), TokenPosition::new(0, 0 /*ends with 0 now because the tokens don't match the input*/))],
            /*1*/vec![Token::StringLiteral("".to_string(), TokenPosition::test_value())],
            /*2*/vec![Token::StringLiteral("... in Rumänien?".to_string(), TokenPosition::test_value())],
            /*3*/vec![Token::KeywordType(Keyword::Let, TokenPosition::test_value()), Token::Identifier("Was geht".to_string(), TokenPosition::test_value()), Token::KeywordType(Keyword::Var, TokenPosition::test_value())],
            /*4*/vec![Token::KeywordType(Keyword::Var, TokenPosition::test_value()), Token::BoolLiteral(true, TokenPosition::test_value())],
            /*5*/vec![Token::KeywordType(Keyword::Var, TokenPosition::test_value()), Token::IntegerLiteral(10, None, TokenPosition::test_value())],
            /*6*/vec![Token::KeywordType(Keyword::Var, TokenPosition::test_value()), Token::IntegerLiteral(10, Some(IntegerType::Unsigned32BitInteger), TokenPosition::test_value())],
        ];

        let actual_output = tokenize(input_tokens, &mut LineMap::test_map());

        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn test_generate_integer() {
        let types = vec![IntegerType::Signed32BitInteger, IntegerType::Signed32BitInteger];
        let test_cases = ["0x45u32", "0b1011i32", "57", "rumänien"];
        let expected_results: [Option<(i128, Option<IntegerType>)>; 4] = [
            Some((0x45, Some(IntegerType::Unsigned32BitInteger))),
            Some((0b1011, Some(IntegerType::Signed32BitInteger))),
            Some((57, None)),
            None,
        ];

        let integer_types = build_integer_types();


        for case in test_cases.iter().enumerate() {
            let result = generate_integer(
                Token::UnspecifiedString(case.1.to_string(), TokenPosition::test_value()),
                integer_types.clone(),
                0,
                0,
                &mut LineMap::test_map(),
            );

            assert_eq!(result, expected_results[case.0]);
        }
    }
}