#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::compiler::data_types::datatypes_general::{Buildable};
    use crate::compiler::data_types::integer::IntegerType;
    use crate::compiler::line_map::{LineMap, TokenPosition};
    use crate::compiler::data_types::object::{Object, ObjectType};
    use crate::compiler::tokenization::tokenizer::tokenize_file;
    use crate::compiler::tokenization::token::Token;
    use crate::compiler::data_types::integer::{build_integer_types, generate_integer};
    use crate::compiler::data_types::object::generate_object;
    use crate::config::tokenization_options::Keyword;
    use crate::util::operator::Operation;

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
        /*let input_tokens = vec![
            /*0*/vec!["\"".to_string(), "Was geht ab...".to_string(), "\"".to_string()],
            /*1*/vec!["\"".to_string(), "".to_string(), "\"".to_string()],
            /*2*/vec!["\"".to_string(), "... in Rumänien?".to_string(), "\"".to_string()],
            /*3*/vec!["let".to_string(), "Was geht".to_string(), "var".to_string()],
            /*4*/vec!["var".to_string(), "true".to_string()],
            /*5*/vec!["var".to_string(), "10".to_string()],
            /*6*/vec!["var".to_string(), "rumänien".to_string(), "=".to_string(), "(".to_string(), "10u32".to_string(), "+".to_string(), "0x67".to_string(), ")".to_string()],
            /*7*/vec!["(".to_string(), "6".to_string(), "+".to_string(), "7".to_string(),
            ")".to_string(), "*".to_string(), "67".to_string()],
        ];*/

        let input_text = "\
            \"Was geht ab...\" \"... in Rumaenien?\" let Was_geht var var true var 10 var rumaenien = (10u32 + 0x67) (6 + 7) * 67
        ";

        let expected_output = [
            /*0*/vec![Token::StringLiteral("Was geht ab...".to_string(), TokenPosition::test_value()/*TokenPosition::new(0, 16 /*ends with 0 now because the tokens don't match the input*/)*/)],
            /*2*/vec![Token::StringLiteral("... in Rumaenien?".to_string(), TokenPosition::test_value())],
            /*3*/vec![Token::KeywordType(Keyword::Let, TokenPosition::test_value()), Token::Identifier("Was_geht".to_string(), TokenPosition::test_value()), Token::KeywordType(Keyword::Var, TokenPosition::test_value())],
            /*4*/vec![Token::KeywordType(Keyword::Var, TokenPosition::test_value()), Token::BoolLiteral(true, TokenPosition::test_value())],
            /*5*/vec![Token::KeywordType(Keyword::Var, TokenPosition::test_value()), Token::IntegerLiteral(10, None, TokenPosition::test_value())],
            /*6*/vec![Token::KeywordType(Keyword::Var, TokenPosition::test_value()), Token::Identifier("rumaenien".to_string(), TokenPosition::test_value()), Token::Assignment(TokenPosition::test_value()), Token::ArithmeticParenthesisOpen(TokenPosition::test_value()), Token::IntegerLiteral(10, Some(IntegerType::Unsigned32BitInteger), TokenPosition::test_value()), Token::Operator(Operation::Addition, TokenPosition::test_value()), Token::IntegerLiteral(0x67, None, TokenPosition::test_value()), Token::ArithmeticParenthesisClose(TokenPosition::test_value())],
            /*7*/vec![Token::ArithmeticParenthesisOpen(TokenPosition::test_value()), Token::IntegerLiteral(6, None, TokenPosition::test_value()), Token::Operator(Operation::Addition, TokenPosition::test_value()), Token::IntegerLiteral(7, None, TokenPosition::test_value()), Token::ArithmeticParenthesisClose(TokenPosition::test_value()), Token::Operator(Operation::Multiplication, TokenPosition::test_value()), Token::IntegerLiteral(67, None, TokenPosition::test_value())],
        ].concat();

        let actual_output = tokenize_file(input_text.to_string(), 0, Rc::new(build_integer_types()), &mut LineMap::test_map());

        for i in 0..expected_output.len() {
            //matches!(actual_output[i], expected_output[i]);
            let mut output = actual_output[i].clone();
            output.reset_position();
            assert_eq!(output, expected_output[i]);
        }

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

        let integer_types = Rc::new(build_integer_types());


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
