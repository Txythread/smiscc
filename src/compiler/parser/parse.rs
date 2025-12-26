use std::rc::Rc;
use clap::arg;
use crate::compiler::line_map::{LineMap, TokenPosition};
use crate::compiler::parser::statement::Statement;
use crate::compiler::parser::statements::Statements;
use crate::compiler::tokenization::token::Token;
use crate::compiler::parser::tree::node::*;
use crate::config::tokenization_options::Keyword;
use crate::util::operator::Operation;
use strum::IntoEnumIterator;

pub fn parse(tokens: Vec<Vec<Token>>, line_map: LineMap) -> Option<Rc<dyn Node>> {
    let statements = Statements::iter().collect::<Vec<_>>();
    let mut lines: Vec<Rc<dyn Node>> = vec![];

    for x in tokens.iter().enumerate() {
        let line = x.1;
        let line_number = x.0;


        if line.is_empty() { continue; }
        let first_token = line[0].clone();


        match first_token {
            Token::KeywordType(keyword, _) => {
                println!("found keyword: {:?}", keyword);
                for statement in statements.iter() {
                    if let Some(statement_keyword) = statement.get_affiliated_keyword() {
                        if statement_keyword != keyword { continue; }

                        // The statement is the statement in question.
                        // Generate its arguments (starting with the header).
                        let mut arguments: Vec<Rc<dyn Node>> = vec![];
                        let argument_types = vec![statement.get_header_format(), statement.get_body_format()].concat();
                        let mut cursor = 1; // skip the first token

                        for type_ in argument_types {
                            let is_required = type_.1;
                            let kind = type_.0;

                            println!("looking for kind: {:?}", kind);


                            match kind {
                                ExpressionKind::Value => {
                                    // Parse an arithmetic expression
                                    if let Some(result) = parse_arithmetic_expression(line.clone(), line_number as u32, line_map.clone(), 0, &mut cursor, true) {
                                        arguments.push(result);
                                    } else {
                                        if is_required {
                                            // Throw an error. Something was required that wasn't given in the current statement.
                                            todo!()
                                        }
                                    }
                                }
                                ExpressionKind::Assignment => {
                                    match line[cursor as usize].clone() {
                                        Token::Assignment(pos ) => arguments.push(Rc::new(AssignmentNode::new((line_number, pos)))),
                                        _ => {
                                            // Throw an error.
                                            todo!()
                                        }
                                    }

                                    cursor += 1;
                                }
                                ExpressionKind::Keyword(_) => {
                                    cursor += 1;
                                }
                                ExpressionKind::Identifier(_) => {
                                    match line[cursor as usize].clone() {
                                        Token::Identifier(id, pos) => {
                                            arguments.push(Rc::new(IdentifierNode::new(id, None, (line_number, pos))))
                                        }

                                        _ => {
                                            // Throw an error.
                                            todo!()
                                        }
                                    }
                                    cursor += 1;
                                }
                            }
                        }

                        println!("arguments: {:?}", arguments);

                        let statementNode = statement.generate_entire_node(arguments);
                        lines.push(statementNode.unwrap());

                    }
                }


                println!("lines: {:#?}", lines);
            }

            _ => {}
        }

        println!("statement: {:#?}", statements.first().unwrap());
       // return parse_arithmetic_expression(line.clone(), 1, line_map, 0, &mut 0, true);
    }

    let code = CodeBlockNode::new((0, TokenPosition::new(0, 0)), lines);

    Some(Rc::new(code))
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

pub fn parse_arithmetic_expression(tokens: Vec<Token>, line_number: u32, line_map: LineMap, min_op_importance: u8, cursor: &mut u16, stop_at_unexpected_token: bool) -> Option<Rc<dyn Node>> {
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

    let mut calculated_nodes: Vec<Rc<dyn Node>> = Vec::new();

    let mut current_operation: Option<Operation> = None;

    for x in tokens.iter().enumerate() {
        let token = x.1;
        let token_number = x.0;

        if token_number < *cursor as usize {
             // The cursor is after this. Skip.
            continue;
        } else {
            *cursor += 1;
        }

        if stop_at_unexpected_token && !token.is_expected_in_arithmetic() { *cursor -= 1; break;}

        match token {
            Token::ArithmeticParenthesisOpen(_) => {
                parenthesis_depth += 1;
            }

            Token::ArithmeticParenthesisClose(_) => {
                parenthesis_depth -= 1;

                if parenthesis_depth == 0 {
                    let solved_parenthesis = parse_arithmetic_expression(tokens_in_parenthesis.clone(), line_number, line_map.clone(), 0, &mut 0, false);
                    println!("solved {:?} as {:?}", tokens_in_parenthesis, solved_parenthesis);
                    if let Some(solved_parenthesis) = solved_parenthesis {
                        calculated_nodes.push(solved_parenthesis);
                    } else {
                        todo!()
                    }
                }
            }

            Token::Operator(operation, _) => {
                if parenthesis_depth == 0 {
                    if operation.get_operation_order() < min_op_importance {
                        *cursor -= 1;
                        break;
                    }
                    if let Some(previous_operation) = current_operation.clone() {
                        if operation.get_operation_order() <= previous_operation.get_operation_order() {
                            // The previous operation needs to happen first
                            // Combine the previous two nodes using the previous operation
                            let start_pos = calculated_nodes[0].get_position().1.start;
                            let end_pos = calculated_nodes[1].get_position().1.start + calculated_nodes[1].get_position().1.length;
                            let length = end_pos - start_pos;
                            let resulting_node = ValueNode::Arithmetic(ArithmeticNode::new(previous_operation, Rc::from(calculated_nodes.remove(0)), Rc::from(calculated_nodes.remove(0)), (line_number as usize, TokenPosition::new(start_pos, length))));
                            calculated_nodes = vec![Rc::new(resulting_node)];

                            current_operation = Some(operation.clone());
                            continue;
                        }

                        let resulting_node = parse_arithmetic_expression(tokens.clone(), line_number, line_map.clone(), operation.get_operation_order() + 1, cursor, false).unwrap();
                        let start_pos = calculated_nodes.last().unwrap().get_position().1.start;
                        let end_pos = resulting_node.get_position().1.start + calculated_nodes[1].get_position().1.length;
                        let length = end_pos - start_pos;
                        let multiplication = ValueNode::Arithmetic(ArithmeticNode::new(operation.clone(), Rc::from(calculated_nodes.remove(calculated_nodes.len() - 1)), resulting_node, (line_number as usize, TokenPosition::new(start_pos, length))));

                        calculated_nodes.push(Rc::new(multiplication));

                        continue;
                    }

                    current_operation = Some(operation.clone());


                }else {
                    tokens_in_parenthesis.push(token.clone());
                }
            }

            _ => {
                if parenthesis_depth > 0 {
                    tokens_in_parenthesis.push(token.clone());
                } else {
                    calculated_nodes.push(parse_token(token.clone(), line_number, line_map.clone())?)
                }
            }
        }
    }

    if let Some(current_operation) = current_operation {
        println!("Still has to resolve {:?}", current_operation);
        // Combine the previous two nodes using the previous operation
        let start_pos = calculated_nodes[0].get_position().1.start;
        let end_pos = calculated_nodes[1].get_position().1.start + calculated_nodes[1].get_position().1.length;
        let length = end_pos - start_pos;
        let resulting_node = ValueNode::Arithmetic(ArithmeticNode::new(current_operation, Rc::from(calculated_nodes.remove(0)), Rc::from(calculated_nodes.remove(0)), (line_number as usize, TokenPosition::new(start_pos, length))));
        calculated_nodes = vec![Rc::new(resulting_node)];
    }

    if calculated_nodes.len() == 1 {
        Some(calculated_nodes[0].clone())
    } else {
        None
    }
}

pub fn parse_token(token: Token, line_number: u32, line_map: LineMap) -> Option<Rc<dyn Node>>{
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
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::compiler::data_types::integer::IntegerType;
    use crate::compiler::line_map::{LineMap, TokenPosition};
    use crate::compiler::parser::parse::{parse_arithmetic_expression, parse_token};
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
            vec![
                Token::ArithmeticParenthesisOpen(TokenPosition::test_value()),                                      // (
                Token::IntegerLiteral(10, Some(IntegerType::Unsigned32BitInteger), TokenPosition::test_value()),    // 10u32
                Token::Operator(Operation::Addition, TokenPosition::test_value()),                                  // +
                Token::IntegerLiteral(5, Some(IntegerType::Unsigned32BitInteger), TokenPosition::test_value()),     // 5u32
                Token::ArithmeticParenthesisClose(TokenPosition::test_value()),                                     // )
                Token::Operator(Operation::Subtraction, TokenPosition::test_value()),                               // -
                Token::Identifier("rumänien".to_string(), TokenPosition::test_value()),                             // rumänien

                ], 0, LineMap::new(), 0, &mut 0, true,
        );

        println!("{:#?}", node);

    }

    #[test]
    fn test_parse_multiplication() {
        let tokens1 = tokenize(split("6 + 7 * 67 - 420;".to_string()).0, &mut LineMap::test_map());
        let tokens2 = tokenize(split("(6 + (7 * 67)) - 420;".to_string()).0, &mut LineMap::test_map());

        let parsed1 = parse_arithmetic_expression(tokens1[0].clone(), 0, LineMap::test_map(), 0, &mut 0, true,);
        let parsed2 = parse_arithmetic_expression(tokens2[0].clone(), 0, LineMap::test_map(), 0, &mut 0, true);

        println!("1: {:#?}", parsed1);
        println!("2: {:#?}", parsed2);

        assert_eq!(format!("{:#?}", parsed1), format!("{:#?}", parsed2));
    }
}
