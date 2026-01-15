use std::rc::Rc;
use crate::compiler::data_types::object::ObjectType;
use crate::compiler::line_map::{LineMap, TokenPosition};
use crate::compiler::parser::parse::ExpressionKind;
use crate::compiler::parser::parse_arg_array::parse_arg_array;
use crate::compiler::parser::parse_arithmetic_expression::parse_arithmetic_expression;
use crate::compiler::parser::parse_datatype::{parse_parameter_descriptor, ParameterDescriptor};
use crate::compiler::parser::parse_line::parse_line;
use crate::compiler::parser::statements::Statements;
use crate::compiler::parser::tree::node::*;
use crate::compiler::tokenization::token::Token;


pub fn parse_expression_kind(tokens: Rc<Vec<Token>>, file_number: u32, cursor: &mut usize, line_map: &mut LineMap, kind: ExpressionKind, required: bool, code_block_depth: &mut u32, blocks: &mut Vec<CodeBlockNode>, statements: Rc<Vec<Statements>>, datatypes: Rc<Vec<ObjectType>>) -> Vec<Rc<dyn Node>> {
    let mut arguments: Vec<Rc<dyn Node>> = Vec::new();
    let initial_block_depth = *code_block_depth;
    match kind {
        ExpressionKind::Value => {
            // Parse an arithmetic expression
            if let Some(result) = parse_arithmetic_expression(tokens.clone(), file_number, line_map.clone(), 0, cursor, true) {
                arguments.push(result);
            } else {
                if required {
                    // Throw an error. Something was required that wasn't given in the current statement.
                    todo!()
                }
            }
        }
        ExpressionKind::Assignment => {
            match tokens[*cursor].clone() {
                Token::Assignment(pos) => arguments.push(Rc::new(AssignmentSymbolNode::new((file_number as usize, pos)))),
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
                    arguments.push(Rc::new(IdentifierNode::new(id, None, (file_number as usize, pos))))
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
            let block = CodeBlockNode::new((file_number as usize, TokenPosition::new(0, 0)), None, vec![]);
            blocks.push(block);

            while *code_block_depth > initial_block_depth {
                parse_line(tokens.clone(), cursor, line_map, statements.clone(), file_number as usize, blocks, blocks.len() - 1, code_block_depth, datatypes.clone());
            }

            let block = blocks.pop().unwrap();


            arguments.push(Rc::new(block));
        }
        ExpressionKind::ParameterDescriptorArray => {
            arguments.push(Rc::new(
                ArgumentsNode::<ParameterDescriptor>::new(
                    (0, TokenPosition::new(0, 0)),
                    Rc::new(
                        parse_arg_array::<ParameterDescriptor>(tokens.clone(), cursor, datatypes.clone(), line_map,
                                                               &|tokens, cursor, line_map, data_types|
                                                                   parse_parameter_descriptor(tokens, cursor, datatypes.clone(), line_map, true))
                    )
                )
            ));
        }
        ExpressionKind::StringLiteral => {
            if let Token::StringLiteral(string, pos) = tokens[*cursor].clone() {
                arguments.push(Rc::new(StringLiteralNode::new((file_number as usize, pos), Rc::new(string))))
            } else {
                todo!("Expected string literal")
            }

            *cursor += 1;
        }
    }


    arguments
}

pub fn parse_multiple_expression_kinds(tokens: Rc<Vec<Token>>, file_number: u32, cursor: &mut usize, line_map: &mut LineMap, kinds: Vec<(ExpressionKind, bool)>, code_block_depth: &mut u32, blocks: &mut Vec<CodeBlockNode>, statements: Rc<Vec<Statements>>, datatypes: Rc<Vec<ObjectType>>) -> Vec<Rc<dyn Node>> {
    let mut arguments: Vec<Rc<dyn Node>> = Vec::new();

    for kind in kinds {
        arguments.append(&mut parse_expression_kind(
            tokens.clone(),
            file_number,
            cursor,
            line_map,
            kind.0,
            kind.1,
            code_block_depth,
            blocks,
            statements.clone(),
            datatypes.clone(),
        ));
    }

    arguments
}