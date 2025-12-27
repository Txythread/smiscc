use std::rc::Rc;
use crate::compiler::line_map::{LineMap, TokenPosition};
use crate::compiler::parser::parse_token::parse_token;
use crate::compiler::parser::tree::node::{ArithmeticNode, Node, ValueNode};
use crate::compiler::tokenization::token::Token;
use crate::util::operator::Operation;

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