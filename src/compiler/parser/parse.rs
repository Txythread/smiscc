use crate::compiler::line_map::LineMap;
use crate::compiler::tokenization::token::Token;
use crate::compiler::parser::tree::node::*;
use crate::util::operator::Operation;

pub fn parse(tokens: Vec<Vec<Token>>, line_map: LineMap){
    
}

pub fn parse_arithmetic_expression(tokens: Vec<Token>, line_number: u32, line_map: LineMap) -> Option<Box<dyn Node>> {
    // If there is only one token, the principle is quite simple
    if tokens.len() == 1 {
        if let Some(node) = parse_token(tokens[0].clone(), line_number, line_map.clone()) {
            return Some(node)
        }
    }

    // Find logical parentheses and calculate stuff for them first
    // Since this is done recursively for each parenthesis, only top-level
    // ones need to be tracked.

    let mut tokens_in_parenthesis: Vec<Token> = Vec::new();
    // How many nests the current token is in.
    // Whenever a bracket is closed and this reaches 0, a new node
    // should be created by using this function recursively.
    let mut parenthesis_depth = 0;

    let mut calculated_nodes: Vec<Box<dyn Node>> = Vec::new();

    let mut current_operation: Option<Operation> = None;

    for token in tokens {
        match token {
            Token::ArithmeticParenthesisOpen(_) => {
                parenthesis_depth += 1;
            }

            Token::ArithmeticParenthesisClose(_) => {
                parenthesis_depth -= 1;

                if parenthesis_depth == 0 {
                    let solved_parenthesis = parse_arithmetic_expression(tokens_in_parenthesis.clone(), line_number, line_map.clone());
                    if let Some(solved_parenthesis) = solved_parenthesis {
                        calculated_nodes.push(solved_parenthesis);
                    } else {
                        todo!()
                    }
                }
            }

            Token::Operator(operation, _) => {
                if parenthesis_depth == 0 {
                    if let Some(previous_operation) = current_operation.clone() {
                        if operation.get_operation_order() <= previous_operation.get_operation_order() {
                            // The previous operation needs to happen first
                            
                        }

                    }
                }
            }

            _ => {
                if parenthesis_depth > 0 {
                    tokens_in_parenthesis.push(token);
                }
            }
        }
    }

    return None;
}

pub fn parse_token(token: Token, line_number: u32, line_map: LineMap) -> Option<Box<dyn Node>>{
    match token {
        Token::UnspecifiedString(_, _) => {}
        Token::StringLiteral(_, _) => {}
        Token::IntegerLiteral(int, kind, pos) => {
            let integer_literal: IntegerLiteralNode = IntegerLiteralNode::new(int, kind, (line_number as usize, pos));
            return Some(Box::new(ValueNode::Literal(LiteralValueNode::Integer(integer_literal))));
        }
        Token::BoolLiteral(boolean, pos) => {
            let boolean_literal: BoolLiteralNode = BoolLiteralNode::new(boolean, (line_number as usize, pos));
            return Some(Box::new(ValueNode::Literal(LiteralValueNode::Boolean(boolean_literal))));
        }
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