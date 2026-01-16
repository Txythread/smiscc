use derive_new::new;
use std::rc::Rc;
use crate::compiler::parser::parse::ExpressionKind;
use crate::compiler::parser::parse_expression_kind::parse_multiple_expression_kinds;
use crate::compiler::parser::parser_meta::ParserMetaState;
use crate::compiler::parser::tree::node::Node;
use crate::compiler::tokenization::token::Token;
use crate::config::tokenization_options::Keyword;

#[derive(Clone, Debug, new)]
pub struct Modifier {
    pub base: Keyword,
    pub arguments: Vec<Rc<dyn Node>>,
}

impl Modifier {
    pub fn keyword_is_modifier(keyword: Keyword) -> bool {
        matches!(keyword, Keyword::Extern)
    }

    pub fn modifier_from(state: &mut ParserMetaState) -> Option<Modifier> {
        let Token::KeywordType(keyword, _) = state.tokens[*state.cursor].clone() else { return None };
        if !Self::keyword_is_modifier(keyword.clone()) { return None };
        *state.cursor += 1;

        let mut modifier = Modifier::new(keyword, vec![]);

        modifier.arguments = parse_multiple_expression_kinds(
            state,
            modifier.get_format()
        );

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