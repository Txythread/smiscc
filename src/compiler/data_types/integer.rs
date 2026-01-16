use uuid::Uuid;
use crate::compiler::data_types::datatypes_general::{BuildResult, Buildable, ObjectBuildingError};
use crate::compiler::data_types::object::{Object, ObjectType, Trait};
use crate::compiler::line_map::{DisplayCodeInfo, DisplayCodeKind, LineMap, NotificationInfo};
use crate::compiler::tokenization::token::Token;
use crate::config::target::ADDRESS_INTEGER_TYPE;
use crate::util::math::convert_to_int;

/// Builds all integer subtypes and returns them with their corresponding
/// types
pub fn build_integer_types() -> Vec<(IntegerType, ObjectType)> {
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
pub fn generate_integer(text: Token, types: Vec<(IntegerType, ObjectType)>, line_number: u32, token_number: u32, line_map: &mut LineMap) -> Option<(i128, Option<IntegerType>)> {
    let _unambiguous_result: Option<(i128, IntegerType)> = None;

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

/// An enum that holds information about any basic integer type.
#[derive(Clone, Debug, PartialEq)]
pub enum IntegerType {
    Unsigned8BitInteger,
    Signed8BitInteger,
    Unsigned16BitInteger,
    Signed16BitInteger,
    Unsigned32BitInteger,
    Signed32BitInteger,
    Address
}


impl IntegerType {
    /// ### The maximum value that the integer is allowed to contain.
    ///
    /// Right now, no checks are performed. This might, however, be changed
    /// in the future.
    ///
    /// For an u32, this would be 2^32 - 1.
    /// For an i32, this would be 2^31 - 1.
    pub fn get_upper_bound(&self) -> u64 {
        match self {
            IntegerType::Unsigned8BitInteger =>     0xff,
            IntegerType::Signed8BitInteger =>       0x7f,
            IntegerType::Unsigned16BitInteger =>    0xff_ff,
            IntegerType::Signed16BitInteger =>      0x7f_ff,
            IntegerType::Unsigned32BitInteger =>    0xff_ff_ff_ff,
            IntegerType::Signed32BitInteger =>      0x7f_ff_ff_ff,

            IntegerType::Address =>                 ADDRESS_INTEGER_TYPE.get_upper_bound(),
        }
    }

    /// ### The minimum value that the integer is allowed to contain.
    ///
    /// Right now, no checks regarding over- or underflows are implemented.
    /// This might, however, be changed in the future.
    ///
    /// For an u32, this would be 0.
    /// For an i32, this would be 2^31.
    ///
    /// **Note:** The number returned here is always positive or zero.
    /// The bound is inverted when checking.
    pub fn get_lower_bound(&self) -> u64 {
        match self {
            IntegerType::Signed8BitInteger =>   0x80,
            IntegerType::Signed16BitInteger =>  0x80_00,
            IntegerType::Signed32BitInteger =>  0x80_00_00_00,

            IntegerType::Address =>             ADDRESS_INTEGER_TYPE.get_lower_bound(),

            _ => /* Unsigned - no negatives */  0x0
        }
    }


    /// ### The name used for errors & warnings.
    ///
    /// This should usually correlate with
    /// [get_code_name](IntegerType::get_code_name)
    pub fn display_name(&self) -> String {
        // In this language, this is just the same as the code name.
        self.get_code_name()
    }

    /// ### The name that refers to the type in the user's code
    ///
    /// **Note:** This should correlate with
    /// [display_name](IntegerType::display_name)
    /// in most cases.
    pub fn get_code_name(&self) -> String {
        match self {
            IntegerType::Unsigned8BitInteger =>     "u8".to_string(),
            IntegerType::Signed8BitInteger =>       "i8".to_string(),
            IntegerType::Unsigned16BitInteger =>    "u16".to_string(),
            IntegerType::Signed16BitInteger =>      "i16".to_string(),
            IntegerType::Unsigned32BitInteger =>    "u32".to_string(),
            IntegerType::Signed32BitInteger =>      "i32".to_string(),

            IntegerType::Address =>                 ADDRESS_INTEGER_TYPE.get_code_name(),
        }
    }


    /// ### Amount of bytes
    ///
    /// The amount of bytes that need to be stored or loaded
    /// when an instance of a datatype is stored.
    ///
    /// #### Examples
    ///
    /// `u8`  | `i8`  -> 1
    /// `u16` | `i16` -> 2
    /// `u32` | `i32` -> 4
    /// `u64` | `i64` -> 8
    pub fn get_memory_size(&self) -> u8 {
        match self {
            IntegerType::Unsigned8BitInteger  | IntegerType::Signed8BitInteger =>   1,
            IntegerType::Unsigned16BitInteger | IntegerType::Signed16BitInteger =>  2,
            IntegerType::Unsigned32BitInteger | IntegerType::Signed32BitInteger =>  4,


            IntegerType::Address => ADDRESS_INTEGER_TYPE.get_memory_size(),
        }
    }
}


impl Buildable for IntegerType {
    fn build(&self, tokens: Vec<Token>, parent_type: ObjectType) -> BuildResult {
        // The type is ambiguous unless a marker is found later.
        let mut ambiguous = true;

        let standard_error = ObjectBuildingError::new(self.display_name(), format!("Couldn't infer an {} from this", self.display_name()));


        if tokens.len() != 1 {
            // Numbers always can be built from one token.
            // As there are multiple tokens, no integer can be
            // built from this => throw an error.

            return BuildResult::new(Err(standard_error), ambiguous);
        }

        let token = tokens.first().unwrap();

        match token {
            Token::UnspecifiedString(token_content, _) => {
                let mut token_content = token_content.clone();
                if token_content.ends_with(self.get_code_name().as_str()) && token_content.len() != self.get_code_name().len() {
                    token_content = token_content.strip_suffix(self.get_code_name().as_str()).unwrap().parse().unwrap();
                    ambiguous = false;
                }

                let value = convert_to_int(token_content);

                if value.is_none() {
                    // Failed to build integer from the value
                    return BuildResult::new(Err(standard_error), ambiguous);
                }

                let value = value.unwrap();

                if let Err(err) = value {
                    let error = ObjectBuildingError::new(self.display_name(), err.message());

                    return BuildResult::new(Err(error), ambiguous);
                }

                let value = value.ok().unwrap();

                if value > self.get_upper_bound() as i128 {
                    let error = ObjectBuildingError::new(self.display_name(), format!("Couldn't infer an {} from this because the upper bound of {} was exceeded.", self.display_name(), self.get_upper_bound()));
                    return BuildResult::new(Err(error), ambiguous);
                }

                if value < -(self.get_lower_bound() as i128) {
                    let error = ObjectBuildingError::new(self.display_name(), format!("Couldn't infer an {} from this because the value was lower than the lower bound of {} .", self.display_name(), self.get_upper_bound()));
                    return BuildResult::new(Err(error), ambiguous);
                }

                let object = Object::new(parent_type.type_uuid, String::new(), Some(value));

                BuildResult::new(Ok(object), ambiguous)
            }
            _ => {
                BuildResult::new(Err(standard_error), ambiguous)
            }
        }
    }

    fn build_type(&self) -> ObjectType {
        let object_uuid = Uuid::new_v4();

        let mut type_ = ObjectType::new(self.get_code_name(), object_uuid);

        type_.add_trait(Trait::ARITHMETIC_COMPATIBLE);
        type_.add_trait(Trait::VALUE_TYPE);
        type_.add_trait(Trait::INTEGER);
        type_.add_complex_trait(Trait::SIZED, vec![self.get_memory_size().to_string()]);


        type_
    }

    fn get_name(&self) -> String {
        self.get_code_name()
    }
}

#[cfg(test)]
mod tests {
    use crate::compiler::data_types::datatypes_general::{Buildable};
    use crate::compiler::data_types::integer::*;
    use crate::compiler::line_map::TokenPosition;
    use crate::compiler::tokenization::token::Token;

    #[test]
    fn test_build_integer_type() {
        let u32_ = IntegerType::Unsigned32BitInteger;

        let u32_type = u32_.build_type();

        println!("{:?}", u32_type);

        let ambiguous_subjects = [
            Token::UnspecifiedString("hello".to_string(), TokenPosition::test_value()),
            Token::UnspecifiedString("36".to_string(), TokenPosition::test_value()),
            Token::UnspecifiedString("2_5".to_string(), TokenPosition::test_value()),
            Token::UnspecifiedString("0x10".to_string(), TokenPosition::test_value()),
        ];

        let ambiguous_answers = [
            None,
            Some(36_i128),
            Some(25_i128),
            Some(0x10_i128),
        ];

        assert_eq!(ambiguous_subjects.len(), ambiguous_answers.len(), "test case written incorrectly");

        for i in 0..ambiguous_subjects.len() {
            let subject = ambiguous_subjects[i].clone();
            let expected_answer = ambiguous_answers[i].clone();

            let result = u32_.build(vec![subject], u32_type.clone());

            assert_eq!(result.ambiguous, true);

            assert_eq!(result.result.is_ok(), expected_answer.is_some());

            if result.result.is_ok() {
                let result = result.result.unwrap().initial_content;

                assert_eq!(result, expected_answer);
            }
        }


        let unambiguous_subjects = [
            Token::UnspecifiedString("hellou32".to_string(), TokenPosition::test_value()),
            Token::UnspecifiedString("36u32".to_string(), TokenPosition::test_value()),
            Token::UnspecifiedString("2_5u32".to_string(), TokenPosition::test_value()),
            Token::UnspecifiedString("0x10_u32".to_string(), TokenPosition::test_value()),
        ];

        let unambiguous_answers = [
            None,
            Some(36_i128),
            Some(25_i128),
            Some(0x10_i128),
        ];

        assert_eq!(unambiguous_subjects.len(), unambiguous_answers.len(), "test case written incorrectly");

        for i in 0..unambiguous_subjects.len() {
            let subject = unambiguous_subjects[i].clone();
            let expected_answer = unambiguous_answers[i].clone();

            let result = u32_.build(vec![subject], u32_type.clone());

            assert_eq!(result.ambiguous, false);

            assert_eq!(result.result.is_ok(), expected_answer.is_some());

            if result.result.is_ok() {
                println!("{:?}", result.clone().result.unwrap());

                let result = result.result.unwrap().initial_content;



                assert_eq!(result, expected_answer);
            }
        }


    }
}
