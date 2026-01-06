use std::rc::Rc;
use crate::compiler::line_map::{LineMap, TokenPosition};
use crate::compiler::parser::statement::Statement;
use crate::compiler::parser::statements::Statements;
use crate::compiler::tokenization::token::Token;
use crate::compiler::parser::tree::node::*;
use crate::config::tokenization_options::Keyword;
use strum::IntoEnumIterator;
use crate::compiler::parser::parse_arithmetic_expression::parse_arithmetic_expression;
use crate::compiler::parser::parse_line::parse_line;

pub fn parse(files: Vec<Vec<Token>>, line_map: &mut LineMap) -> Option<Rc<dyn Node>> {
    let statements = Statements::iter().collect::<Vec<_>>();
    let mut lines_in_block: Vec<Rc<dyn Node>> = vec![];

    let code = CodeBlockNode::new((0, TokenPosition::new(0, 0)), Rc::new(Some("_stray".to_string())), lines_in_block);
    let mut blocks: Vec<CodeBlockNode> = vec![code];


    for x in files.iter().enumerate() {
        let contents = x.1;
        let file_number = x.0;

        let mut cursor = 0;


        if contents.is_empty() { continue; }


        while cursor < contents.len() {
            parse_line(
                Rc::new(files[0].clone()),
                &mut cursor,
                line_map,
                statements.clone(),
                0,
                &mut blocks,
                0,
            )
        }
    }


    Some(Rc::new(blocks[0].clone()))
}

#[derive(Debug)]
#[derive(Clone)]
pub enum ExpressionKind {
    /// Something that is supposed to be parsed as an arithmetic expression
    Value,

    Assignment,

    /// **Note:** None means multiple keywords are allowed here
    Keyword(Option<Keyword>),

    /// **Note:** Only Identifiers that won't get coerced into values
    Identifier(Option<String>),
}


#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::compiler::data_types::integer::IntegerType;
    use crate::compiler::line_map::{LineMap, TokenPosition};
    use crate::compiler::parser::parse::parse_arithmetic_expression;
    use crate::compiler::parser::parse_token::parse_token;
    use crate::compiler::tokenization::token::Token;
    use crate::compiler::parser::tree::node::*;
    use crate::compiler::splitter::split;
    use crate::compiler::tokenization::tokenizer::tokenize;
    use crate::util::operator::Operation;

    #[test]
    fn test_parse_token() {
        let node = parse_token(Token::IntegerLiteral(10, Some(IntegerType::Unsigned32BitInteger), TokenPosition::test_value()), 0, LineMap::new());
        let expected = Some(ValueNode::Literal(LiteralValueNode::Integer(IntegerLiteralNode { content: 10, kind: Some(IntegerType::Unsigned32BitInteger), position: (0, TokenPosition { start: 0, length: 0 }) })));

        assert_eq!(format!("{node:?}"), format!("{expected:?}"));
    }

    #[test]
    fn test_parse_arithmetic() {
        let node = parse_arithmetic_expression(
            Rc::new(vec![
                Token::ArithmeticParenthesisOpen(TokenPosition::test_value()),                                      // (
                Token::IntegerLiteral(10, Some(IntegerType::Unsigned32BitInteger), TokenPosition::test_value()),    // 10u32
                Token::Operator(Operation::Addition, TokenPosition::test_value()),                                  // +
                Token::IntegerLiteral(5, Some(IntegerType::Unsigned32BitInteger), TokenPosition::test_value()),     // 5u32
                Token::ArithmeticParenthesisClose(TokenPosition::test_value()),                                     // )
                Token::Operator(Operation::Subtraction, TokenPosition::test_value()),                               // -
                Token::Identifier("rumänien".to_string(), TokenPosition::test_value()),                             // rumänien

                ]), 0, LineMap::new(), 0, &mut 0, true,
        );


    }

    #[test]
    fn test_parse_multiplication() {
        let tokens1 = tokenize(vec![split("6 + 7 * 67 - 420;".to_string(), String::from("test.txt"), &mut LineMap::test_map())], &mut LineMap::test_map());
        let tokens2 = tokenize(vec![split("(6 + (7 * 67)) - 420;".to_string(), String::from("test.txt"), &mut LineMap::test_map())], &mut LineMap::test_map());

        let parsed1 = parse_arithmetic_expression(Rc::new(tokens1[0].clone()), 0, LineMap::test_map(), 0, &mut 0, true,);
        let parsed2 = parse_arithmetic_expression(Rc::new(tokens2[0].clone()), 0, LineMap::test_map(), 0, &mut 0, true);


        assert_eq!(format!("{:#?}", parsed1), format!("{:#?}", parsed2));
    }
}
