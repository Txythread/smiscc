use derive_new::new;
use std::rc::Rc;
use crate::compiler::data_types::object::ObjectType;
use crate::compiler::line_map::LineMap;
use crate::compiler::parser::parse::ExpressionKind;
use crate::compiler::parser::parse_expression_kind::parse_multiple_expression_kinds;
use crate::compiler::parser::statements::Statements;
use crate::compiler::parser::tree::node::{CodeBlockNode, Node};
use crate::compiler::tokenization::token::Token;
use crate::config::tokenization_options::Keyword;

#[derive(Clone, Debug, new)]
pub struct Modifier {
    pub base: Keyword,
    pub arguments: Vec<Rc<dyn Node>>,
}

impl Modifier {
    pub fn keyword_is_modifier(keyword: Keyword) -> bool {
        match keyword {
            Keyword::Extern => true,
            _ => false,
        }
    }

    pub fn modifier_from(tokens: Rc<Vec<Token>>, file_number: u32, cursor: &mut usize, line_map: &mut LineMap, statements: Rc<Vec<Statements>>, datatypes: Rc<Vec<ObjectType>>, blocks: &mut Vec<CodeBlockNode>, code_block_depth: &mut u32) -> Option<Modifier> {
        let Token::KeywordType(keyword, _) = tokens[*cursor].clone() else { return None };
        if !Self::keyword_is_modifier(keyword.clone()) { return None };
        *cursor += 1;

        let mut modifier = Modifier::new(keyword, vec![]);

        modifier.arguments = parse_multiple_expression_kinds(
            tokens.clone(),
            file_number,
            cursor,
            line_map,
            modifier.get_format(),
            code_block_depth,
            blocks,
            statements,
            datatypes
        );

        println!("returning, cursor points to: {:?}", tokens[*cursor].clone());


        Some(modifier)
    }

    pub fn get_format(&self) -> Vec<(ExpressionKind, bool)> {
        match self.base {
            Keyword::Extern => {
                vec![
                    (
                        ExpressionKind::StringLiteral,
                        true,
                    ),
                    (
                        ExpressionKind::StringLiteral,
                        true,
                    )
                ]
            },
            _ => {
                todo!()
            }
        }
    }
}