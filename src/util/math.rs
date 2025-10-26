use crate::config::misc::INTEGER_CONVERSION_IGNORED_CHARACTERS;

/// Tries to decode a value from decimal, hexadecimal, octal or
/// binary depending on the prefix (or no prefix for decimal).
pub fn convert_to_int(input: String) -> Option<i128> {
    let mut input = input.trim();

    // Ignore characters if needed
    // Usually this will be a "_"
    let cleaned_input = input.replace(INTEGER_CONVERSION_IGNORED_CHARACTERS, "");
    input = cleaned_input.as_str();


    // Try to convert from decimal (no prefix) first
    if let Some(value) = input.parse::<i128>().ok() {
        return Some(value)
    }

    if let Some(hex_value) = input.strip_prefix("0x") {
        if let Some(value) = i128::from_str_radix(hex_value, 16).ok() {
            return Some(value)
        }
    }

    if let Some(bin_value) = input.strip_prefix("0b") {
        if let Some(value) = i128::from_str_radix(bin_value, 2).ok() {
            return Some(value)
        }
    }

    if let Some(octal_value) = input.strip_prefix("0o") {
        if let Some(value) = i128::from_str_radix(octal_value, 8).ok() {
            return Some(value)
        }
    }

    // Couldn't convert from binary, hexadecimal and octal, just return nothing
    None
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
            assert_eq!(result, test_answers[i]);
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
            assert_eq!(result, None);
        }
    }
}