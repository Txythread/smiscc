use crate::compiler::data_types::{BuildResult, Buildable};
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


fn tokenize_arithmetic_expression_series(line: Vec<String>, mut constants: &Vec<Object>, mut variables: &Vec<Object>) {

}

/// ### Generates objects if possible
///
/// This is done using the ["Buildable" trait](Buildable), which
/// works by exposing both the struct implementing the trait and a
/// one-time generated ObjectType. The tokens can usually be left
/// unclassified for simple objects and the line map and the line
/// number are required for producing token positions.
///
/// The resulting token will always contain an object.
fn generate_object<T: Buildable + ?Sized>(tokens: &mut Vec<Token>, object_types: Vec<(ObjectType, Box<T>)>, line_map: &mut LineMap, line_number: u32,  first_token_index: u32, last_token_index: u32) -> Option<Token> {
    /// Where ambiguous results get stored for later.
    /// If there is more than one element in here in the end,
    /// that's an error that should be displayed (except when
    /// there's an explicit one in there, than that one should
    /// be selected).
    let mut successful_ambiguous_results: Vec<Object> = Vec::new();

    /// Where explicit results get stored for later.
    /// If there is more than one element in here in the end,
    /// that's an error that should be displayed in the end.
    /// If there are none, then successful_ambiguous_results
    /// should be used instead.
    let mut successful_explicit_results: Vec<Object> = Vec::new();

    // Calculate the position of the resulting token
    let result_start_pos = tokens[0].get_position().start;
    let last_token_pos = tokens.last().unwrap().get_position();
    let result_end_pos = last_token_pos.start + last_token_pos.length;
    let result_length = result_end_pos - result_start_pos;
    let result_position = TokenPosition::new(result_start_pos, result_end_pos);


    // Was geht ab in Rumänien?

    for object_type in object_types.iter().clone() {
        let object_type = object_type.clone();
        let parent_type = object_type.0.clone();
        let build_result = object_type.1.build(tokens.clone(), parent_type);

        if build_result.ambiguous {
            if build_result.result.is_ok() {
                successful_ambiguous_results.push(build_result.result.unwrap());
            }
        } else {
            if build_result.result.is_ok() {
                successful_explicit_results.push(build_result.result.unwrap());
            } else {
                // The datatype was explicitly requested by the code but
                // couldn't be built. Throw an error.
                let error_info = DisplayCodeInfo::new(
                    line_number,
                    first_token_index,
                    last_token_index as i32,
                    vec![
                        format!("**note:** this couldn't be parsed as a {}", build_result.result.clone().err().unwrap().expected_object)
                    ],
                    DisplayCodeKind::InitialError
                );

                let error = NotificationInfo::new(
                    "Failed to Decode Object".to_string(),
                    build_result.result.err().unwrap().message.clone(),
                    vec![error_info],
                );

                line_map.display_error(error);
            }
        }
    }

    // All objects have had a chance to build their versions.
    // Now, return whatever is available.
    if successful_explicit_results.len() > 0 {
        if successful_explicit_results.len() != 1 {
            // There are too many results all claiming to be unambiguous.
            // The datatype was explicitly requested by the code but
            // couldn't be built. Throw an error.
            let error_info = DisplayCodeInfo::new(
                line_number,
                first_token_index,
                last_token_index as i32,
                vec![],
                DisplayCodeKind::InitialError
            );

            let mut message = String::from("There are multiple ways to decode this object: ");

            for result in successful_explicit_results.clone() {
                let type_uuid = result.type_uuid;
                let type_with_uuid = object_types.iter().filter(|&x| x.0.type_uuid == type_uuid).collect::<Vec<&(ObjectType, Box<T>)>>().iter().nth(0).unwrap().0.clone();
                message += type_with_uuid.name.clone().as_str();
                message += "  ";
            }

            let error = NotificationInfo::new(
                "Statement Ambiguous".to_string(),
                message,
                vec![error_info],
            );

            line_map.display_error(error);

        } else {
            let token = Token::Object(successful_explicit_results[0].clone(), result_position);
            return Some(token);
        }
    }

    // All objects have had a chance to build their versions.
    // Now, return whatever is available.
    if successful_ambiguous_results.len() > 0 {
        if successful_ambiguous_results.len() != 1 {
            // There are too many results all claiming to be unambiguous.
            // The datatype was explicitly requested by the code but
            // couldn't be built. Throw an error.
            let error_info = DisplayCodeInfo::new(
                line_number,
                first_token_index,
                last_token_index as i32,
                vec![],
                DisplayCodeKind::InitialError
            );

            let mut message = String::from("There are multiple ways to decode this object: ");

            for result in successful_ambiguous_results {
                let type_uuid = result.type_uuid;
                let type_with_uuid = object_types.iter().filter(|&x| x.0.type_uuid == type_uuid).collect::<Vec<&(ObjectType, Box<T>)>>().iter().nth(0).unwrap().0.clone();
                message += type_with_uuid.name.clone().as_str();
                message += "  ";
            }

            let error = NotificationInfo::new(
                "Statement Ambiguous".to_string(),
                message,
                vec![error_info],
            );

            line_map.display_error(error);

        } else {
            let token = Token::Object(successful_ambiguous_results[0].clone(), result_position);
            return Some(token)
        }
    }

    None

}


/// **Note:** A token always includes the position from
/// which it stems. This is important for producing debug
/// information about the user's code.
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    /// A static/constant object that can be accessed instantly.
    Object(Object, TokenPosition),

    /// A variable that might be changed. It therefore can't be
    /// optimized away.
    Variable(Object, TokenPosition),

    /// ### Equation operation
    /// An equation operation that sets a variable to another token.
    /// This token should be an object of the same type in the end.
    Set(Object, Box<Token>, TokenPosition),

    /// ### A block of tokens/code
    /// This might, for example, be a function body, a loop body or an if body.
    Block(Vec<Token>, TokenPosition),


    /// A block that has yet to be classified
    UnspecifiedString(String, TokenPosition),

    ArithmeticOperation(Box<Token>, Box<Token>, ArithmeticOperation, TokenPosition),
}

impl Token {
    pub fn get_position(&self) -> TokenPosition {
        match self {
            Token::Object(_, pos) => { pos.clone() }
            Token::Variable(_, pos) => { pos.clone() }
            Token::Set(_, _, pos) => { pos.clone() }
            Token::Block(_, pos) => { pos.clone() }
            Token::UnspecifiedString(_, pos) => { pos.clone() }
            Token::ArithmeticOperation(_, _, _, pos) => { pos.clone() }
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::compiler::data_types::{Buildable, IntegerType};
    use crate::compiler::line_map::{LineMap, TokenPosition};
    use crate::compiler::object::{Object, ObjectType};
    use crate::compiler::tokenizer::{generate_object, Token};

    #[test]
    fn test_generate_object() {
        let i32_type = IntegerType::Signed32BitInteger.build_type();
        let object_types: Vec<(ObjectType, Box<dyn Buildable>)> = vec![
            (i32_type.clone(), Box::new(IntegerType::Signed32BitInteger)),
        ];

        let token = Token::UnspecifiedString(String::from("10"), TokenPosition::new(0, 5));

        let mut line_map = LineMap::new();

        let result = generate_object(&mut vec![token], object_types, &mut line_map, 0, 0, 0);

        assert_eq!(result, Some(Token::Object(Object::new(i32_type.type_uuid.clone(), String::new(), Some(10)), TokenPosition::new(0, 5))))
    }
}