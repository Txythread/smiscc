use std::ops::Deref;
use std::rc::Rc;
use strum_macros::EnumIter;
use uuid::Uuid;
use crate::compiler::line_map::TokenPosition;
use crate::compiler::parser::modifier::Modifier;
use crate::config::tokenization_options::Keyword;
use crate::compiler::parser::parse::ExpressionKind;
use crate::compiler::parser::parse_datatype::ParameterDescriptor;
use crate::compiler::parser::statement::Statement;
use crate::compiler::parser::tree::node::{ArgumentsNode, CodeBlockNode, ExitNode, FunctionDeclarationNode, IdentifierNode, IfNode, LetNode, Node, StringLiteralNode, TokenPayloadNode, UuidPayloadNode};
use crate::compiler::tokenization::token::Token;

#[derive(Clone, Debug, EnumIter)]
pub enum Statements {
    Let,
    Var,
    Exit,
    Function,
    If,
}

impl Statement for Statements {
    fn get_affiliated_keyword(&self) -> Option<Keyword> {
        match self {
            Statements::Let => Some(Keyword::Let),
            Statements::Var => Some(Keyword::Var),
            Statements::Exit => Some(Keyword::Exit),
            Statements::Function => Some(Keyword::Function),
            Statements::If => Some(Keyword::If),
        }
    }

    fn get_header_format(&self) -> Vec<(ExpressionKind, bool)> {
        match self { 
            Statements::Let => {
                vec![
                    (
                        ExpressionKind::Identifier(None),
                        true
                    ),
                    (
                        ExpressionKind::ExpectedToken(Token::Colon(TokenPosition::new(0, 0))),
                        false
                    ),
                    (
                        ExpressionKind::Datatype,
                        false
                    ),
                ]
            }
            Statements::Var => {
                vec![
                    (
                        ExpressionKind::Identifier(None),
                        true
                    ),
                    (
                        ExpressionKind::ExpectedToken(Token::Colon(TokenPosition::new(0, 0))),
                        false
                    ),
                    (
                        ExpressionKind::Datatype,
                        false
                    ),
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
                        ExpressionKind::Array(Box::new(ExpressionKind::Parameter)),
                        true
                    )
                ]
            }

            Statements::If => {
                vec![
                    (
                        ExpressionKind::Value,
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
                        ExpressionKind::ExpectedToken(Token::Assignment(TokenPosition::new(0, 0))),
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

            Statements::If => {
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

            Statements::If => {
                let condition = arguments[0].clone();
                let code_block = arguments[1].clone().downcast_rc::<CodeBlockNode>().unwrap();
                let if_node = IfNode::new(
                    (0, TokenPosition::new(0, 0)),
                    condition,
                    code_block.clone(),
                    None
                );

                return Some(Rc::new(if_node));
            }
            _ => {}
        }

        let identifier_arg = arguments[0].clone();
        let identifier_node = identifier_arg.downcast_rc::<IdentifierNode>().unwrap();
        let identifier = identifier_node.identifier.clone();

        let mut datatype: Option<Uuid> = None;

        if let Ok(token_node) = arguments[1].clone().downcast_rc::<TokenPayloadNode>() {
            match token_node.token {
                Token::Colon(_) => {
                    if let Ok(datatype_node) = arguments[2].clone().downcast_rc::<UuidPayloadNode>() {
                        datatype = Some(datatype_node.uuid);
                    } else {
                        todo!("Expected type, couldn't be found tho")
                    }
                }

                Token::Assignment(_) => {},
                _ => todo!("(impl err) unexpected. expected  token containing : or =")
            }
        } else {
            todo!("(impl err) unexpected. expected  token containing : or =")
        }
        
        let assigned_value = arguments.last().unwrap().clone();
        
        let is_mutable = matches!(self, Statements::Var);
        
        let node = LetNode::new(identifier, datatype, Some(assigned_value), is_mutable, (0, TokenPosition::new(0, 0)));
        
        Some(Rc::new(node))
    }
}

