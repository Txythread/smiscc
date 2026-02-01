use std::rc::Rc;
use crate::compiler::line_map::{LineMap, TokenPosition};
use crate::compiler::parser::statements::Statements;
use crate::compiler::tokenization::token::Token;
use crate::compiler::parser::tree::node::*;
use crate::config::tokenization_options::Keyword;
use strum::IntoEnumIterator;
use crate::compiler::data_types::object::ObjectType;
use crate::compiler::parser::parse_line::parse_line;
use crate::compiler::parser::parser_meta::ParserMetaState;

pub fn parse(files: Vec<Vec<Token>>, line_map: &mut LineMap, object_types: &mut Rc<Vec<ObjectType>>) -> Option<Rc<dyn Node>> {
    let statements = Rc::new(Statements::iter().collect::<Vec<_>>());
    let lines_in_block: Vec<Rc<dyn Node>> = vec![];

    let code = CodeBlockNode::new((0, TokenPosition::new(0, 0)), Some(Rc::new("_stray".to_string())), lines_in_block);
    let mut blocks: Vec<CodeBlockNode> = vec![code];


    for x in files.iter().enumerate() {
        let contents = x.1;
        let mut file_number = x.0;
        let mut code_block_depth = 0;
        let mut current_block_index = 0;
        let mut cursor = 0;
        let mut object_types = object_types.clone();


        if contents.is_empty() { continue; }


        while cursor < contents.len() {
            let mut parser_meta = ParserMetaState::new(
                Rc::new(files[0].clone()),
                &mut cursor,
                line_map,
                statements.clone(),
                &mut file_number,
                &mut blocks,
                &mut current_block_index,
                &mut code_block_depth,
                &mut object_types
            );


            parse_line(
                &mut parser_meta
            )
        }
    }


    Some(Rc::new(CodeBlockArray::new((0, TokenPosition::new(0, 0)), blocks.clone())))
}

#[derive(Clone, Debug)]
pub enum ExpressionKind {
    /// Something that is supposed to be parsed as an arithmetic expression
    Value,

    Assignment,

    /// **Note:** None means multiple keywords are allowed here
    Keyword(Option<Keyword>),

    /// **Note:** Only Identifiers that won't get coerced into values
    Identifier(Option<String>),

    CodeBlock,

    /// An expression kind repeated multiple times.
    /// Example: `(10, 10, 10)` would be an array of [values](Self::Value)
    Array(Box<ExpressionKind>),
    
    /// Something with a name and a type (e.g. `a: u32`)
    Parameter,

    StringLiteral,
}


#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use std::rc::Rc;
    use strum::IntoEnumIterator;
    use crate::compiler::data_types::integer::{build_integer_types, IntegerType};
    use crate::compiler::data_types::object::ObjectType;
    use crate::compiler::line_map::{LineMap, TokenPosition};
    use crate::compiler::parser::parse_arithmetic_expression::parse_arithmetic_expression;
    use crate::compiler::parser::parse_token::parse_token;
    use crate::compiler::parser::parser_meta::ParserMetaState;
    use crate::compiler::parser::statements::Statements;
    use crate::compiler::tokenization::token::Token;
    use crate::compiler::parser::tree::node::*;
    use crate::compiler::tokenization::tokenizer::tokenize_file;
    use crate::util::operator::Operation;

    #[test]
    fn test_parse_token() {
        let node = parse_token(Token::IntegerLiteral(10, Some(IntegerType::Unsigned32BitInteger), TokenPosition::test_value()), 0, LineMap::new());
        let expected = Some(ValueNode::Literal(LiteralValueNode::Integer(IntegerLiteralNode { content: 10, kind: Some(IntegerType::Unsigned32BitInteger), position: (0, TokenPosition { start: 0, length: 0 }) })));

        assert_eq!(format!("{node:?}"), format!("{expected:?}"));
    }

    #[test]
    fn test_parse_arithmetic() {
        let tokens = Rc::new(vec![
            Token::ArithmeticParenthesisOpen(TokenPosition::test_value()),                                      // (
            Token::IntegerLiteral(10, Some(IntegerType::Unsigned32BitInteger), TokenPosition::test_value()),    // 10u32
            Token::Operator(Operation::Addition, TokenPosition::test_value()),                                  // +
            Token::IntegerLiteral(5, Some(IntegerType::Unsigned32BitInteger), TokenPosition::test_value()),     // 5u32
            Token::ArithmeticParenthesisClose(TokenPosition::test_value()),                                     // )
            Token::Operator(Operation::Subtraction, TokenPosition::test_value()),                               // -
            Token::Identifier("rumänien".to_string(), TokenPosition::test_value()),                             // rumänien
            Token::HardNewline(TokenPosition::test_value()),
        ]);
        let mut cursor = 0;
        let statements = Rc::new(Statements::iter().collect::<Vec<_>>());
        let mut object_types = Rc::new(ObjectType::generate_built_ins());
        let mut code_block_depth = 0;
        let mut current_block_index = 0;
        let mut file_number = 0;
        let mut blocks = vec![];
        let mut line_map = LineMap::test_map();

        let mut meta = ParserMetaState::new(
            tokens.clone(),
            &mut cursor,
            &mut line_map,
            statements,
            &mut file_number,
            &mut blocks,
            &mut current_block_index,
            &mut code_block_depth,
            &mut object_types
        );

        let node = parse_arithmetic_expression(
            &mut meta,
            0,
            false
        );


    }

    #[test]
    fn test_parse_multiplication() {
        let integer_types = Rc::new(build_integer_types());
        let tokens1 = tokenize_file("6 + 7 * 67 - 420;".to_string(), 0, integer_types.clone(), &mut LineMap::test_map(),);
        let tokens2 = tokenize_file("(6 + (7 * 67)) - 420;".to_string(), 0, integer_types, &mut LineMap::test_map(),);

        let mut parsed1;
        let mut parsed2;

        {
            let mut cursor = 0;
            let statements = Rc::new(Statements::iter().collect::<Vec<_>>());
            let mut object_types = Rc::new(ObjectType::generate_built_ins());
            let mut code_block_depth = 0;
            let mut current_block_index = 0;
            let mut file_number = 0;
            let mut blocks = vec![];
            let mut line_map = LineMap::test_map();


            let mut meta1 = ParserMetaState::new(
                Rc::new(tokens1.clone()),
                &mut cursor,
                &mut line_map,
                statements,
                &mut file_number,
                &mut blocks,
                &mut current_block_index,
                &mut code_block_depth,
                &mut object_types
            );

            let mut int = parse_arithmetic_expression(&mut meta1, 0, true,).unwrap();
            let mut int_2 = int.downcast_rc::<ValueNode>().unwrap();
            let mut int_3 = int_2.deref().clone();

            int_3.repeatedly_reset_position();

            parsed1 = int_3;
        }

        println!("-------------------------------------");

        {
            let mut cursor = 0;
            let statements = Rc::new(Statements::iter().collect::<Vec<_>>());
            let mut object_types = Rc::new(ObjectType::generate_built_ins());
            let mut code_block_depth = 0;
            let mut current_block_index = 0;
            let mut file_number = 0;
            let mut blocks = vec![];
            let mut line_map = LineMap::test_map();


            let mut meta1 = ParserMetaState::new(
                Rc::new(tokens2.clone()),
                &mut cursor,
                &mut line_map,
                statements,
                &mut file_number,
                &mut blocks,
                &mut current_block_index,
                &mut code_block_depth,
                &mut object_types
            );

            let mut int = parse_arithmetic_expression(&mut meta1, 0, true,).unwrap();
            let mut int_2 = int.downcast_rc::<ValueNode>().unwrap();
            let mut int_3 = int_2.deref().clone();

            int_3.repeatedly_reset_position();

            parsed2 = int_3;
        }





        assert_eq!(format!("{:?}", parsed1), format!("{:?}", parsed2));
    }
}
