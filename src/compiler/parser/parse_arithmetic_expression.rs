use std::ops::Deref;
use std::rc::Rc;
use crate::compiler::line_map::{LineMap, TokenPosition};
use crate::compiler::parser::parse_token::parse_token;
use crate::compiler::parser::tree::node::{ArithmeticNode, FunctionCallNode, Node, ValueNode};
use crate::compiler::tokenization::token::Token;
use crate::util::operator::Operation;

pub fn parse_arithmetic_expression(tokens: Rc<Vec<Token>>, line_number: u32, line_map: LineMap, min_op_importance: u8, cursor: &mut usize, stop_at_unexpected_token: bool) -> Option<Rc<dyn Node>> {
    // If there is only one token, the principle is quite simple
    if tokens.len() -*cursor == 1 {
        if let Some(node) = parse_token(tokens[*cursor].clone(), line_number, line_map.clone()) {
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

    for x in tokens.iter().enumerate().skip(*cursor) {
        let token = x.1;
        let token_number = x.0 + *cursor;



        *cursor += 1;

        if stop_at_unexpected_token && !token.is_expected_in_arithmetic() { *cursor -= 1; break;}

        if matches!(token, Token::SoftNewline(_)) || matches!(token, Token::HardNewline(_)) {
            *cursor -= 1;
            break;
        }


        match token {
            Token::ArithmeticParenthesisOpen(_) => {
                // Detect function calls
                if current_operation.is_none() {
                    if let Some(last_node) = calculated_nodes.last() {
                        if let Some(identifier_node) = last_node.clone().downcast_rc::<ValueNode>().ok() {
                            if let ValueNode::Identifier(identifier_node) = identifier_node.deref() {
                                // This parenthesis belongs to a function call
                                let function_name = identifier_node.identifier.clone();

                                // Gather all the arguments
                                let mut args: Vec<Rc<dyn Node>> = Vec::new();

                                loop {
                                    if let Some(new_node) = parse_arithmetic_expression(tokens.clone(), line_number, line_map.clone(), 0, cursor, true) {
                                        args.push(new_node);
                                    } else {
                                        todo!()
                                    }

                                    *cursor -= 1;

                                    // Look for either "," to indicate another argument or ")" to indicate the end of the function call
                                    if let Some(token) = tokens.iter().nth(*cursor as usize) {
                                        *cursor += 1;
                                        match token {
                                            Token::ArithmeticParenthesisClose(_) => {break;},
                                            Token::ArgumentSeparator(_) => {continue},
                                            _ => {todo!("Unexpected token in function call")}
                                        }
                                    } else {
                                        todo!("Error: Unexpected line ending in function call");
                                    }
                                }

                                let function_node = FunctionCallNode::new(
                                    Rc::new(function_name),
                                    args,
                                    (line_number as usize, TokenPosition::new(0, 0))
                                );

                                calculated_nodes.remove(calculated_nodes.len() - 1);
                                calculated_nodes.push(Rc::new(function_node));

                                break;
                            }
                        }
                    }
                }

                parenthesis_depth += 1;
            }

            Token::ArithmeticParenthesisClose(_) => {
                parenthesis_depth -= 1;

                if parenthesis_depth == 0 {
                    let solved_parenthesis = parse_arithmetic_expression(Rc::new(tokens_in_parenthesis.clone()), line_number, line_map.clone(), 0, &mut 0, false);
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
                    calculated_nodes.push(parse_token(token.clone(), line_number, line_map.clone())?);
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

    Some(calculated_nodes.iter().nth(0)?.clone())
}