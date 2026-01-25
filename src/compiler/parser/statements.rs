use std::ops::Deref;
use std::rc::Rc;
use strum_macros::EnumIter;
use crate::compiler::line_map::TokenPosition;
use crate::compiler::parser::modifier::Modifier;
use crate::config::tokenization_options::Keyword;
use crate::compiler::parser::parse::ExpressionKind;
use crate::compiler::parser::parse_datatype::ParameterDescriptor;
use crate::compiler::parser::statement::Statement;
use crate::compiler::parser::tree::node::{ArgumentsNode, CodeBlockNode, ExitNode, FunctionDeclarationNode, IdentifierNode, LetNode, Node, StringLiteralNode};

#[derive(Clone, Debug, EnumIter)]
pub enum Statements {
    Let,
    Var,
    Exit,
    Function,
}

impl Statement for Statements {
    fn get_affiliated_keyword(&self) -> Option<Keyword> {
        match self {
            Statements::Let => Some(Keyword::Let),
            Statements::Var => Some(Keyword::Var),
            Statements::Exit => Some(Keyword::Exit),
            Statements::Function => Some(Keyword::Function),
        }
    }

    fn get_header_format(&self) -> Vec<(ExpressionKind, bool)> {
        match self { 
            Statements::Let => {
                vec![
                    (
                        ExpressionKind::Identifier(None),
                        true
                    )
                ]
            }
            Statements::Var => {
                vec![
                    (
                        ExpressionKind::Identifier(None),
                        true
                    )
                ]
            }
            Statements::Exit => {
                vec![
                ]
            }

            Statements::Function => {
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
            Statements::Let => {
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
            Statements::Var => {
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
            Statements::Exit => {
                vec![
                    (
                        ExpressionKind::Value,
                        true
                    )
                ]
            }
            Statements::Function => {
                vec![
                    (
                        ExpressionKind::CodeBlock,
                        true
                    )
                ]
            }
        }
    }

    fn generate_node(&self, arguments: Vec<Rc<dyn Node>>, modifiers: &mut Vec<Modifier>) -> Option<Rc<dyn Node>> {
        match self {
            Statements::Exit => {
                let arg = arguments[0].clone();

                let node = ExitNode::new(arg, (0, TokenPosition::new(0, 0)));

                return Some(Rc::new(node));
            },

            Statements::Function => {
                let identifier_arg = arguments[0].clone();
                let identifier_node = identifier_arg.downcast_rc::<IdentifierNode>().unwrap();
                let identifier = identifier_node.identifier.clone();

                let block = arguments.last().unwrap().clone();
                let mut block = block.downcast_rc::<CodeBlockNode>().unwrap().deref().clone();


                let argument_node = arguments[1].clone().downcast_rc::<ArgumentsNode<ParameterDescriptor>>().unwrap();
                let parameters = argument_node.args.clone();

                println!("parameters: {:?}", parameters);

                // Go through the parameters
                // 1. Find extern
                if let Some(extern_index) = modifiers.iter().position(|x|x.base == Keyword::Extern) {
                    if extern_index != modifiers.len() - 1 {
                        todo!("Throw an error: extern expect as last modifier")
                    }

                    let extern_modifier = modifiers[extern_index].clone();
                    modifiers.remove(extern_index);

                    let name_node = extern_modifier.arguments[1].clone().downcast_rc::<StringLiteralNode>().unwrap();
                    let asm_name = name_node.string.clone();

                    block.label = Some(asm_name);
                }

                let function_node = FunctionDeclarationNode::new(
                    (0, TokenPosition::new(0, 0)),
                    Rc::new(identifier),
                    Rc::new(block).clone(),
                    parameters.clone()
                );



                return Some(Rc::new(function_node));
            }
            _ => {}
        }

        let identifier_arg = arguments[0].clone();
        let identifier_node = identifier_arg.downcast_rc::<IdentifierNode>().unwrap();
        let identifier = identifier_node.identifier.clone();
        
        let assigned_value = arguments[2].clone();
        
        let is_mutable = matches!(self, Statements::Var);
        
        let node = LetNode::new(identifier, Some(assigned_value), is_mutable, (0, TokenPosition::new(0, 0)));
        
        Some(Rc::new(node))
    }
}

