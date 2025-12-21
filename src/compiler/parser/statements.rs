use std::rc::Rc;
use strum_macros::EnumIter;
use crate::compiler::line_map::TokenPosition;
use crate::config::tokenization_options::Keyword;
use crate::compiler::parser::parse::ExpressionKind;
use crate::compiler::parser::statement::Statement;
use crate::compiler::parser::tree::node::{IdentifierNode, LetNode, Node};

#[derive(Debug, EnumIter)]
pub enum Statements {
    LetStatement    
}

impl Statement for Statements {
    fn get_affiliated_keyword(&self) -> Option<Keyword> {
        match self {
            Statements::LetStatement => Some(Keyword::Let)
        }
    }

    fn get_header_format(&self) -> Vec<(ExpressionKind, bool)> {
        match self { 
            Statements::LetStatement => {
                vec![
                    (
                        ExpressionKind::Identifier(None),
                        true
                    )
                ]
            }
        }
    }

    fn get_body_format(&self) -> Vec<(ExpressionKind, bool)> {
        match self {
            Statements::LetStatement => {
                vec![
                    (
                        ExpressionKind::Assignment,
                        true
                    ),

                    (
                        ExpressionKind::Value,
                        true
                    )
                ]
            } 
        }
    }

    fn generate_header_node(&self, arguments: Vec<Rc<dyn Node>>) -> Option<Rc<dyn Node>> {
        todo!()
    }

    fn generate_entire_node(&self, arguments: Vec<Rc<dyn Node>>) -> Option<Rc<dyn Node>> {
        let identifierArg = arguments[0].clone();
        let identifierNode = identifierArg.downcast_rc::<IdentifierNode>().unwrap();
        let identifier = identifierNode.identifier.clone();
        
        let assignedValue = arguments[2].clone();
        
        let node = LetNode::new(identifier, Some(assignedValue), (0, TokenPosition::new(0,0)));
        
        Some(Rc::new(node))
    }
}

