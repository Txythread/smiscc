use std::rc::Rc;
use crate::compiler::data_types::object::ObjectType;
use crate::compiler::line_map::{LineMap, TokenPosition};
use crate::compiler::parser::modifier::Modifier;
use crate::compiler::parser::parse::ExpressionKind;
use crate::compiler::parser::parse_arg_array::parse_arg_array;
use crate::compiler::parser::parse_arithmetic_expression::parse_arithmetic_expression;
use crate::compiler::parser::parse_datatype::{parse_parameter_descriptor, ParameterDescriptor};
use crate::compiler::parser::parse_expression_kind::parse_multiple_expression_kinds;
use crate::compiler::parser::statement::Statement;
use crate::compiler::parser::statements::Statements;
use crate::compiler::parser::tree::node::{ArgumentsNode, AssignmentNode, AssignmentSymbolNode, CodeBlockNode, IdentifierNode, Node};
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
pub fn parse_line(tokens: Rc<Vec<Token>>, cursor: &mut usize, line_map: &mut LineMap, statements: Rc<Vec<Statements>>, file_number: usize, blocks: &mut Vec<CodeBlockNode>, current_block_idx: usize, code_block_depth: &mut u32, datatypes: Rc<Vec<ObjectType>>) {
    let line_start = *cursor;
    let initial_block_depth = *code_block_depth;

    // Parse the modifiers
    let mut modifiers: Vec<Modifier> = vec![];

    loop {
        if let Some(modifier) = Modifier::modifier_from(
            tokens.clone(),
            file_number as u32,
            cursor,
            line_map,
            statements.clone(),
            datatypes.clone(),
            blocks,
            code_block_depth,
        ) {
            println!("Returned, cursor points to: {:?}", tokens[*cursor].clone());
            modifiers.push(modifier);
        } else {
            break;
        }
    }

    println!("modifiers: {:?}, cursor: {:?}", modifiers, tokens[*cursor].clone());

    let first_token: Token = tokens[*cursor].clone();
    *cursor += 1;

    // Parse the rest
    match first_token.clone() {
        Token::KeywordType(keyword, _) => {
            for statement in statements.iter() {
                if let Some(statement_keyword) = statement.get_affiliated_keyword() {
                    if statement_keyword != keyword { continue; }


                    // The statement is the statement in question.
                    // Generate its arguments (starting with the header).
                    let argument_types = vec![statement.get_header_format(), statement.get_body_format()].concat();

                    let arguments = parse_multiple_expression_kinds(
                        tokens.clone(),
                        file_number as u32,
                        cursor,
                        line_map,
                        argument_types,
                        code_block_depth,
                        blocks,
                        statements.clone(),
                        datatypes.clone(),
                    );

                    let statement_node = statement.generate_entire_node(arguments, &mut modifiers);
                    if let Some(statement_node) = statement_node {
                        blocks[current_block_idx].push_code(statement_node);
                    } else {
                        panic!("Statement didn't generate node")
                    }
                }
            }
        }

        Token::Identifier(name, pos) => {
            let id_node = IdentifierNode::new(name.clone(), None, (file_number, pos.clone()));

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