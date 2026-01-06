use std::rc::Rc;
use crate::compiler::line_map::LineMap;
use crate::compiler::parser::parse::ExpressionKind;
use crate::compiler::parser::parse_arithmetic_expression::parse_arithmetic_expression;
use crate::compiler::parser::statement::Statement;
use crate::compiler::parser::statements::Statements;
use crate::compiler::parser::tree::node::{AssignmentNode, AssignmentSymbolNode, CodeBlockNode, IdentifierNode, Node};
use crate::compiler::tokenization::token::Token;

pub fn parse_line(tokens: Rc<Vec<Token>>, cursor: &mut usize, line_map: &mut LineMap, statements: Vec<Statements>, file_number: usize, blocks: &mut Vec<CodeBlockNode>, current_block_idx: usize) {
    let first_token: Token = tokens[*cursor].clone();
    let line_start = *cursor;
    *cursor += 1;

    match first_token.clone() {
        Token::KeywordType(keyword, _) => {
            for statement in statements.iter() {
                if let Some(statement_keyword) = statement.get_affiliated_keyword() {
                    if statement_keyword != keyword { continue; }


                    // The statement is the statement in question.
                    // Generate its arguments (starting with the header).
                    let mut arguments: Vec<Rc<dyn Node>> = vec![];
                    let argument_types = vec![statement.get_header_format(), statement.get_body_format()].concat();

                    for type_ in argument_types {
                        let is_required = type_.1;
                        let kind = type_.0;



                        match kind {
                            ExpressionKind::Value => {
                                println!("token at {} is {:?}", *cursor, tokens[*cursor]);

                                // Parse an arithmetic expression
                                if let Some(result) = parse_arithmetic_expression(tokens.clone(), file_number as u32, line_map.clone(), 0, cursor, true) {
                                    arguments.push(result);
                                } else {
                                    if is_required {
                                        // Throw an error. Something was required that wasn't given in the current statement.
                                        todo!()
                                    }
                                }
                            }
                            ExpressionKind::Assignment => {
                                match tokens[*cursor].clone() {
                                    Token::Assignment(pos) => arguments.push(Rc::new(AssignmentSymbolNode::new((file_number, pos)))),
                                    _ => {
                                        // Throw an error.
                                        todo!()
                                    }
                                }

                                *cursor += 1;
                            }
                            ExpressionKind::Keyword(_) => {
                                *cursor += 1;
                            }
                            ExpressionKind::Identifier(_) => {
                                match tokens[*cursor].clone() {
                                    Token::Identifier(id, pos) => {
                                        arguments.push(Rc::new(IdentifierNode::new(id, None, (file_number, pos))))
                                    }

                                    _ => {
                                        // Throw an error.
                                        todo!()
                                    }
                                }
                                *cursor += 1;
                            }
                        }
                    }


                    let statement_node = statement.generate_entire_node(arguments);
                    blocks[current_block_idx].push_code(statement_node.unwrap());
                }
            }
        }

        Token::Identifier(name, pos) => {
            let id_node = IdentifierNode::new(name.clone(), None, (file_number, pos.clone()));

            println!("found identifier: {:?}", name.clone());

            *cursor = line_start;
            match tokens[*cursor + 1].clone() {
                Token::Assignment(_) => {},
                _ => {
                    let value = parse_arithmetic_expression(tokens.clone(), file_number as u32, line_map.clone(), 0, cursor, false).unwrap();

                    blocks[current_block_idx].push_code(value);

                    return;
                }
            }

            *cursor = line_start + 2;
            let value = parse_arithmetic_expression(tokens.clone(), file_number as u32, line_map.clone(), 0, cursor, false).unwrap();

            let assignment_node = AssignmentNode::new(Rc::new(id_node), value, (file_number, pos));

            blocks[current_block_idx].push_code(Rc::new(assignment_node));
        }

        _ => {}
    }

    // Skip all newlines
    loop {
        match tokens[*cursor] {
            Token::SoftNewline(_) | Token::HardNewline(_) => {*cursor += 1}
            _ => {return;}
        }

        if tokens.len() <= *cursor {
            return;
        }
    }
}