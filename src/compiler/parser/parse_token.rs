use std::rc::Rc;
use crate::compiler::line_map::LineMap;
use crate::compiler::parser::tree::node::{BoolLiteralNode, IdentifierNode, IntegerLiteralNode, LiteralValueNode, Node, ValueNode};
use crate::compiler::tokenization::token::Token;

pub fn parse_token(token: Token, line_number: u32, _line_map: LineMap) -> Option<Rc<dyn Node>>{
    match token {
        Token::UnspecifiedString(_, _) => {}
        Token::StringLiteral(_, _) => {}
        Token::IntegerLiteral(int, kind, pos) => {
            let integer_literal: IntegerLiteralNode = IntegerLiteralNode::new(int, kind, (line_number as usize, pos));
            return Some(Rc::new(ValueNode::Literal(LiteralValueNode::Integer(integer_literal))));
        }
        Token::BoolLiteral(boolean, pos) => {
            let boolean_literal: BoolLiteralNode = BoolLiteralNode::new(boolean, (line_number as usize, pos));
            return Some(Rc::new(ValueNode::Literal(LiteralValueNode::Boolean(boolean_literal))));
        }
        Token::KeywordType(_, _) => {}
        Token::Identifier(identifier, pos) => {
            let identifier_node: IdentifierNode = IdentifierNode::new(identifier, None, (line_number as usize, pos));
            return Some(Rc::new(ValueNode::Identifier(identifier_node)));
        }
        Token::Operator(_, _) => {}
        Token::Assignment(_) => {}
        Token::ArithmeticParenthesisOpen(_) => {}
        Token::ArithmeticParenthesisClose(_) => {}
        Token::ArgumentSeparator(_) => {}
        Token::SoftNewline(_) => {}
        Token::HardNewline(_) => {}
        Token::CodeBlockParenthesisClose(_) => {}
        Token::CodeBlockParenthesisOpen(_) => {}
    }

    None
}