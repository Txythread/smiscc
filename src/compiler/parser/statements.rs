use std::rc::Rc;
use strum_macros::EnumIter;
use crate::compiler::line_map::TokenPosition;
use crate::config::tokenization_options::Keyword;
use crate::compiler::parser::parse::ExpressionKind;
use crate::compiler::parser::statement::Statement;
use crate::compiler::parser::tree::node::{CodeBlockNode, ExitNode, FunctionDeclarationNode, IdentifierNode, LetNode, Node};

#[derive(Clone, Debug, EnumIter)]
pub enum Statements {
    LetStatement,
    VarStatement,
    ExitStatement,
    FunctionStatement,
}

impl Statement for Statements {
    fn get_affiliated_keyword(&self) -> Option<Keyword> {
        match self {
            Statements::LetStatement => Some(Keyword::Let),
            Statements::VarStatement => Some(Keyword::Var),
            Statements::ExitStatement => Some(Keyword::Exit),
            Statements::FunctionStatement => Some(Keyword::Function),
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
            Statements::VarStatement => {
                vec![
                    (
                        ExpressionKind::Identifier(None),
                        true
                    )
                ]
            }
            Statements::ExitStatement => {
                vec![
                ]
            }

            Statements::FunctionStatement => {
                vec![
                    (
                        ExpressionKind::Identifier(None),
                        true
                    ),

                    (
                        ExpressionKind::ParameterDescriptorArray,
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
            Statements::VarStatement => {
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
            Statements::ExitStatement => {
                vec![
                    (
                        ExpressionKind::Value,
                        true
                    )
                ]
            }
            Statements::FunctionStatement => {
                vec![
                    (
                        ExpressionKind::CodeBlock,
                        true
                    )
                ]
            }
        }
    }

    fn generate_header_node(&self, _arguments: Vec<Rc<dyn Node>>) -> Option<Rc<dyn Node>> {
        todo!()
    }

    fn generate_entire_node(&self, arguments: Vec<Rc<dyn Node>>) -> Option<Rc<dyn Node>> {
        match self {
            Statements::ExitStatement => {
                let arg = arguments[0].clone();

                let node = ExitNode::new(arg, (0, TokenPosition::new(0, 0)));

                return Some(Rc::new(node));
            },

            Statements::FunctionStatement => {
                let identifier_arg = arguments[0].clone();
                let identifier_node = identifier_arg.downcast_rc::<IdentifierNode>().unwrap();
                let identifier = identifier_node.identifier.clone();

                let block = arguments.last().unwrap().clone();
                let block = block.downcast_rc::<CodeBlockNode>().unwrap();

                let function_node = FunctionDeclarationNode::new(
                    (0, TokenPosition::new(0, 0)),
                    Rc::new(identifier),
                    block.clone(),
                );

                println!("Function node got args: {:#?}", arguments);


                return Some(Rc::new(function_node));
            }
            _ => {}
        }

        let identifier_arg = arguments[0].clone();
        let identifier_node = identifier_arg.downcast_rc::<IdentifierNode>().unwrap();
        let identifier = identifier_node.identifier.clone();
        
        let assigned_value = arguments[2].clone();
        
        let is_mutable = matches!(self, Statements::VarStatement);
        
        let node = LetNode::new(identifier, Some(assigned_value), is_mutable, (0, TokenPosition::new(0, 0)));
        
        Some(Rc::new(node))
    }
}

