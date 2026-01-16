use std::rc::Rc;
use crate::compiler::line_map::TokenPosition;
use crate::compiler::parser::parse::ExpressionKind;
use crate::compiler::parser::parse_arg_array::parse_arg_array;
use crate::compiler::parser::parse_arithmetic_expression::parse_arithmetic_expression;
use crate::compiler::parser::parse_datatype::{parse_parameter_descriptor, ParameterDescriptor};
use crate::compiler::parser::parse_line::parse_line;
use crate::compiler::parser::parser_meta::ParserMetaState;
use crate::compiler::parser::tree::node::*;
use crate::compiler::tokenization::token::Token;


pub fn parse_expression_kind(meta_state: &mut ParserMetaState, kind: ExpressionKind, required: bool) -> Vec<Rc<dyn Node>> {

    let mut arguments: Vec<Rc<dyn Node>> = Vec::new();
    let initial_block_depth = *meta_state.code_block_depth;
    match kind {
        ExpressionKind::Value => {
            // Parse an arithmetic expression
            if let Some(result) = parse_arithmetic_expression(meta_state, 0, true) {
                arguments.push(result);
            } else {
                if required {
                    // Throw an error. Something was required that wasn't given in the current statement.
                    todo!()
                }
            }
        }
        ExpressionKind::Assignment => {
            match meta_state.tokens[*meta_state.cursor].clone() {
                Token::Assignment(pos) => arguments.push(Rc::new(AssignmentSymbolNode::new((*meta_state.file_number, pos)))),
                _ => {
                    // Throw an error.
                    todo!()
                }
            }

            *meta_state.cursor += 1;
        }
        ExpressionKind::Keyword(_) => {
            *meta_state.cursor += 1;
        }
        ExpressionKind::Identifier(_) => {
            match meta_state.tokens[*meta_state.cursor].clone() {
                Token::Identifier(id, pos) => {
                    arguments.push(Rc::new(IdentifierNode::new(id, None, (*meta_state.file_number, pos))))
                }

                _ => {
                    // Throw an error.
                    todo!()
                }
            }
            *meta_state.cursor += 1;
        }
        ExpressionKind::CodeBlock => {
            // Locate the opening
            if !matches!(meta_state.tokens[*meta_state.cursor].clone(), Token::CodeBlockParenthesisOpen(_)) {
                todo!("expected bracket to start code block")
            }

            let initial_block_idx = *meta_state.current_block_idx;
            *meta_state.cursor += 1;
            *meta_state.code_block_depth += 1;
            *meta_state.current_block_idx = meta_state.blocks.len();


            // Create a new code block
            let block = CodeBlockNode::new((*meta_state.file_number, TokenPosition::new(0, 0)), None, vec![]);
            meta_state.blocks.push(block);

            while *meta_state.code_block_depth > initial_block_depth {
                parse_line(meta_state);
            }

            let block = meta_state.blocks.pop().unwrap();

            *meta_state.current_block_idx = initial_block_idx;
            arguments.push(Rc::new(block));
        }
        ExpressionKind::ParameterDescriptorArray => {
            arguments.push(Rc::new(
                ArgumentsNode::<ParameterDescriptor>::new(
                    (0, TokenPosition::new(0, 0)),
                    Rc::new(
                        parse_arg_array::<ParameterDescriptor>(meta_state,
                                                               &|tokens, cursor, line_map, datatypes|
                                                                   parse_parameter_descriptor(tokens, cursor, datatypes, line_map, true))
                    )
                )
            ));
        }
        ExpressionKind::StringLiteral => {
            if let Token::StringLiteral(string, pos) = meta_state.tokens[*meta_state.cursor].clone() {
                arguments.push(Rc::new(StringLiteralNode::new((*meta_state.file_number, pos), Rc::new(string))))
            } else {
                todo!("Expected string literal")
            }

            *meta_state.cursor += 1;
        }
    }


    arguments
}

pub fn parse_multiple_expression_kinds(meta_state: &mut ParserMetaState, kinds: Vec<(ExpressionKind, bool)>) -> Vec<Rc<dyn Node>> {
    let mut arguments: Vec<Rc<dyn Node>> = Vec::new();

    for kind in kinds {
        arguments.append(&mut parse_expression_kind(
            meta_state,
            kind.0,
            kind.1,
        ));
    }

    arguments
}