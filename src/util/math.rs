use crate::config::misc::INTEGER_CONVERSION_IGNORED_CHARACTERS;
use crate::config::tokenization_options::BASES;

/// Tries to decode a value from decimal, hexadecimal, octal or
/// binary depending on the prefix (or no prefix for decimal).
pub fn convert_to_int(input: String) -> Option<Result<i128, ConversionError>> {
    let mut input = input.trim();

    // Ignore characters if needed
    // Usually this will be a "_"
    let cleaned_input = input.replace(INTEGER_CONVERSION_IGNORED_CHARACTERS, "");
    input = cleaned_input.as_str();

    for base in BASES {
        if let Some(result) = base.integer_from_string(cleaned_input.to_string()) {
            return Some(result)
        }
    }

    None
}


/// Contains necessary information about a number system (like hexadecimal,
/// decimal, binary or octal).
///
/// Which bases are used in the [conversion process](convert_to_int) is
/// determined by the config file.
#[derive(Clone, Debug, PartialEq)]
pub struct Base {
    /// The full name of the system (e.g. "hexadecimal")
    pub name: &'static str,

    /// The base of the system (such as 16 for hexadecimal, 10 for decimal, ...).
    /// Interpretation follows [Rust's implementation](i128::from_str_radix),
    /// meaning this value has to be between 2 and 36.
    pub base: u8,

    /// Which suffix marks the use of this system in the code.
    /// For hexadecimal, that would be "0x", no suffix makes this the default
    /// system.
    pub code_prefix: &'static str,
}

impl Base {
    pub const BINARY: Base = Base { name: "binary", base: 2, code_prefix: "0b" };
    pub const OCTAL: Base = Base { name: "octal", base: 8, code_prefix: "0o" };
    pub const DECIMAL: Base = Base { name: "decimal", base: 10, code_prefix: "" };
    pub const HEXADECIMAL: Base = Base { name: "hexadecimal", base: 16, code_prefix: "0x" };


    /// Returns None if the datatype was not requested (e.g. trying to build
    /// a hexadecimal value from "0b77")
    pub fn integer_from_string(&self, string: String) -> Option<Result<i128, ConversionError>> {
        let mut stripped_value = string.strip_prefix(self.code_prefix);

        if self.code_prefix.is_empty() {
            stripped_value = Some(string.as_str());
        }

        if let Some(value) = stripped_value {
            return if let Some(value) = i128::from_str_radix(value, self.base as u32).ok() {
                Some(Ok(value))
            } else {
                // If no prefix is expected, nothing should be returned instead
                // of an error.
                if !self.code_prefix.is_empty() {
                    Some(Err(ConversionError::UnknownCharacters(self.clone())))
                }else{
                    None
                }
            }
        }

        None
    }
}

/// Potential result from trying to generate integers from strings
/// using [bases](Base).
#[derive(Debug, PartialEq)]
pub enum ConversionError {
    /// There were characters that had no meaning in the base.
    UnknownCharacters(Base),
}

impl ConversionError {
    pub fn message(&self) -> String {
        match self {
            ConversionError::UnknownCharacters(base) => {
                format!("The expression contains characters that can't be parsed in {}.", base.name)
            }
        }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub enum ArithmeticOperation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}


#[cfg(test)]
mod tests {
    use crate::util::math::convert_to_int;

    #[test]
    fn test_convert_to_int() {
        let test_subjects = [
            "0x1_0___",
            "___0b10",
            "24"
        ];

        let test_answers = [
            0x10,
            0b10,
            24
        ];

        assert_eq!(test_subjects.len(), test_answers.len(), "Test case written incorrectly");

        for i in 0..test_subjects.len() {
            let result = convert_to_int(test_subjects[i].to_string()).unwrap();
            assert_eq!(result.unwrap(), test_answers[i]);
        }
    }

    #[test]
    fn test_inconvertible() {
        let test_subjects = [
            "rumänien",
            "was geht?",
            "asdf"
        ];


        for i in 0..test_subjects.len() {
            let result = convert_to_int(test_subjects[i].to_string());
            assert_eq!(result.is_some(), false);
        }
    }
}