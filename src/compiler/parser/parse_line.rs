use std::rc::Rc;
use crate::compiler::parser::modifier::Modifier;
use crate::compiler::parser::parse_arithmetic_expression::parse_arithmetic_expression;
use crate::compiler::parser::parse_expression_kind::parse_multiple_expression_kinds;
use crate::compiler::parser::parser_meta::ParserMetaState;
use crate::compiler::parser::statement::Statement;
use crate::compiler::parser::tree::node::{AssignmentNode, IdentifierNode};
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
pub fn parse_line(meta_state: &mut ParserMetaState) {
    let line_start = *meta_state.cursor;
    let initial_block_depth = *meta_state.code_block_depth;

    // Parse the modifiers
    let mut modifiers: Vec<Modifier> = vec![];

    while let Some(modifier) = Modifier::modifier_from(meta_state) {
        modifiers.push(modifier);
    }

    let first_token: Token = meta_state.tokens[*meta_state.cursor].clone();
    *meta_state.cursor += 1;

    // Parse the rest
    match first_token.clone() {
        Token::KeywordType(keyword, _) => {
            println!("statements: {:?}", meta_state.statements);
            for statement in meta_state.statements.clone().iter() {
                if let Some(statement_keyword) = statement.get_affiliated_keyword() {
                    if statement_keyword != keyword { continue; }


                    // The statement is the statement in question.
                    // Generate its arguments (starting with the header).
                    let argument_types = [statement.get_header_format(), statement.get_body_format()].concat();

                    let arguments = parse_multiple_expression_kinds(
                        meta_state,
                        argument_types,
                    );

                    println!("args: {:#?} for: {:?}", arguments, keyword);

                    let statement_node = statement.generate_node(arguments, &mut modifiers);
                    if let Some(statement_node) = statement_node {
                        let _position = *meta_state.current_block_idx;
                        meta_state.blocks[*meta_state.current_block_idx].push_code(statement_node);
                    } else {
                        panic!("Statement didn't generate node")
                    }
                }
            }
        }

        Token::Identifier(name, pos) => {
            let id_node = IdentifierNode::new(name.clone(), None, (*meta_state.file_number, pos.clone()));

            *meta_state.cursor = line_start;
            match meta_state.tokens[*meta_state.cursor + 1].clone() {
                Token::Assignment(_) => {},
                _ => {
                    let value = parse_arithmetic_expression(meta_state, 0, false).unwrap();

                    meta_state.blocks[*meta_state.current_block_idx].push_code(value);

                    return;
                }
            }

            *meta_state.cursor = line_start + 2;
            let value = parse_arithmetic_expression(meta_state, 0, false).unwrap();

            let assignment_node = AssignmentNode::new(Rc::new(id_node), value, (*meta_state.file_number, pos));

            meta_state.blocks[*meta_state.current_block_idx].push_code(Rc::new(assignment_node));
        }

        Token::CodeBlockParenthesisClose(_) => {
            *meta_state.code_block_depth -= 1;

            if *meta_state.code_block_depth < initial_block_depth {
                return;
            }
        }

        _ => {}
    }

    // Skip all newlines
    loop {
        if meta_state.tokens.len() <= *meta_state.cursor {
            return;
        }

        match meta_state.tokens[*meta_state.cursor] {
            Token::SoftNewline(_) | Token::HardNewline(_) => {*meta_state.cursor += 1}
            Token::CodeBlockParenthesisClose(_) => { *meta_state.code_block_depth -= 1; *meta_state.cursor += 1 }
            _ => {return;}
        }

        if *meta_state.code_block_depth < initial_block_depth {
            return;
        }
    }
}