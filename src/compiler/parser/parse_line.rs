use std::rc::Rc;
use crate::compiler::line_map::{LineMap, TokenPosition};
use crate::compiler::parser::parse::ExpressionKind;
use crate::compiler::parser::parse_arithmetic_expression::parse_arithmetic_expression;
use crate::compiler::parser::statement::Statement;
use crate::compiler::parser::statements::Statements;
use crate::compiler::parser::tree::node::{AssignmentNode, AssignmentSymbolNode, CodeBlockNode, IdentifierNode, Node};
use crate::compiler::tokenization::token::Token;

/// Generates nodes for one logical line or statement.
///
/// For example:
/// Tokens describing `let a = 10` would be turned into a let node containing
/// "a" as the identifier and an assigned value of a ValueNode containing a Literal
/// which points to a IntegerLiteralNode.
///
/// Unlike what the name suggests, this function doesn't necessarily stop after one
/// line. It can handle multiple code blocks as long as those were themselves started
/// within the **logical** line. Meaning something like a "func"-statement can be fully
/// parsed while this function gets invoked only once.
pub fn parse_line(tokens: Rc<Vec<Token>>, cursor: &mut usize, line_map: &mut LineMap, statements: Rc<Vec<Statements>>, file_number: usize, blocks: &mut Vec<CodeBlockNode>, current_block_idx: usize, code_block_depth: &mut u32) {
    let first_token: Token = tokens[*cursor].clone();
    let line_start = *cursor;
    *cursor += 1;
    let initial_block_depth = *code_block_depth;

    println!("Started line parsing process with first token: {:?}", first_token.clone());
    match first_token.clone() {
        Token::KeywordType(keyword, _) => {
            for statement in statements.iter() {
                if let Some(statement_keyword) = statement.get_affiliated_keyword() {
                    if statement_keyword != keyword { continue; }


                    // The statement is the statement in question.
                    // Generate its arguments (starting with the header).
                    let mut arguments: Vec<Rc<dyn Node>> = vec![];
                    let argument_types = vec![statement.get_header_format(), statement.get_body_format()].concat();

                    println!("argument_types: {:?}", argument_types);

                    for type_ in argument_types {
                        let is_required = type_.1;
                        let kind = type_.0;



                        match kind {
                            ExpressionKind::Value => {
                                println!("token at {} is {:?}", *cursor, tokens.clone()[*cursor]);

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
                            ExpressionKind::CodeBlock => {
                                // Locate the opening
                                if !matches!(tokens[*cursor].clone(), Token::CodeBlockParenthesisOpen(_)) {
                                    todo!("expected bracket to start code block")
                                }
                                *cursor += 1;
                                *code_block_depth += 1;

                                // Create a new code block
                                let block = CodeBlockNode::new((file_number, TokenPosition::new(0, 0)), Rc::new(None), vec![]);
                                blocks.push(block);

                                while *code_block_depth > initial_block_depth {
                                    parse_line(tokens.clone(), cursor, line_map, statements.clone(), file_number, blocks, blocks.len() - 1, code_block_depth);
                                }

                                let block = blocks.pop().unwrap();


                                arguments.push(Rc::new(block));
                            }
                        }
                    }


                    let statement_node = statement.generate_entire_node(arguments);
                    if let Some(statement_node) = statement_node {
                        blocks[current_block_idx].push_code(statement_node);
                    } else {
                        eprintln!("Statement didn't generate node")
                    }
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

        Token::CodeBlockParenthesisClose(_) => {
            *code_block_depth -= 1;

            if *code_block_depth < initial_block_depth {
                return;
            }
        }

        _ => {}
    }

    // Skip all newlines
    loop {
        if tokens.len() <= *cursor {
            return;
        }

        match tokens[*cursor] {
            Token::SoftNewline(_) | Token::HardNewline(_) => {*cursor += 1}
            Token::CodeBlockParenthesisClose(_) => { *code_block_depth -= 1; *cursor += 1 }
            _ => {return;}
        }

        if *code_block_depth < initial_block_depth {
            return;
        }
    }
}