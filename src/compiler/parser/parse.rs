use crate::compiler::line_map::LineMap;
use crate::compiler::tokenization::token::Token;
use crate::compiler::parser::tree::node::*;

pub fn parse(tokens: Vec<Vec<Token>>, line_map: LineMap){
    
}

pub fn parse_line(tokens: Vec<Token>, line_number: u32, line_map: LineMap) {
    // If there is only one token, the principle is quite simple
    if tokens.len() == 1 {
        let node = parse_token(tokens[0].clone(), line_number, line_map);
    }

}

pub fn parse_token(token: Token, line_number: u32, line_map: LineMap) -> Option<Box<dyn Node>>{
    match token {
        Token::UnspecifiedString(_, _) => {}
        Token::StringLiteral(_, _) => {}
        Token::IntegerLiteral(int, kind, pos) => {
            let integer_literal: IntegerLiteralNode = IntegerLiteralNode::new(int, kind, (line_number as usize, pos));
            return Some(Box::new(ValueNode::Literal(LiteralValueNode::Integer(integer_literal))));
        }
        Token::BoolLiteral(_, _) => {}
        Token::KeywordType(_, _) => {}
        Token::Identifier(_, _) => {}
        Token::Operator(_, _) => {}
        Token::Assignment(_) => {}
        Token::ArithmeticParenthesisOpen(_) => {}
        Token::ArithmeticParenthesisClose(_) => {}
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::compiler::data_types::integer::IntegerType;
    use crate::compiler::line_map::{LineMap, TokenPosition};
    use crate::compiler::parser::parse::parse_token;
    use crate::compiler::tokenization::token::Token;
    use crate::compiler::parser::tree::node::*;

    #[test]
    fn test_parse_token() {
        let node = parse_token(Token::IntegerLiteral(10, Some(IntegerType::Unsigned32BitInteger), TokenPosition::test_value()), 0, LineMap::new());
        let expected = Some(ValueNode::Literal(LiteralValueNode::Integer(IntegerLiteralNode { content: 10, kind: Some(IntegerType::Unsigned32BitInteger), position: (0, TokenPosition { start: 0, length: 0 }) })));

        assert_eq!(format!("{node:?}"), format!("{expected:?}"));
    }
}