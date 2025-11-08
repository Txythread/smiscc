use crate::compiler::data_types::integer::IntegerType;
use crate::compiler::line_map::TokenPosition;
use crate::config::tokenization_options::{Keyword, BOOL_STATE_NAMES, ASSIGNMENT_OPERATION};
use crate::util::operator;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    /// A block that has yet to be classified
    /// This should only appear **inside** the tokenizer & **never
    /// returned anywhere else**.
    UnspecifiedString(String, TokenPosition),

    /// Something that has been classified as a string
    StringLiteral(String, TokenPosition),

    /// A text that could be classified (and converted to) an integer.
    IntegerLiteral(i128, Option<IntegerType>, TokenPosition),

    /// A text that could be classified as being either true or false.
    BoolLiteral(bool, TokenPosition),

    /// A keyword (such "var", "let" or "if").
    KeywordType(Keyword, TokenPosition),

    /// Texts Identifying Variables, constants, ...
    Identifier(String, TokenPosition),

    /// A mathematical operator (such as +)
    /// *Note:* This does not include assignment operations (e.g. "=", "+=")
    Operator(operator::Operation, TokenPosition),

    /// An operation that sets the left side to the right side
    /// This is like "=" in basically every (non-esoteric) programming language.
    Assignment(TokenPosition),

    //////////////////////////////////////////
    ////////////// PARENTHESES ///////////////
    //////////////////////////////////////////

    /// The arithmetic/default parenthesis ("(")
    ArithmeticParenthesisOpen(TokenPosition),
    
    /// The closing part of the arithmetic/default parenthesis (")")
    ArithmeticParenthesisClose(TokenPosition),

}

impl Token {
    pub fn get_position(&self) -> TokenPosition {
        match self {
            Token::UnspecifiedString(_, pos) => { pos.clone() }
            Token::StringLiteral(_, pos) => { pos.clone() }
            Token::IntegerLiteral(_, _, pos) => { pos.clone() }
            Token::BoolLiteral(_, pos) => { pos.clone() }
            Token::KeywordType(_, pos) => { pos.clone() }
            Token::Identifier(_, pos) => { pos.clone() }
            Token::Operator(_, pos) => { pos.clone() }
            Token::ArithmeticParenthesisOpen(pos) => { pos.clone() }
            Token::Assignment(pos) => { pos.clone() }
            Token::ArithmeticParenthesisClose(pos) => { pos.clone() }
        }
    }

    /// Gets the text a token contains if applicable
    pub fn get_raw_text(&self) -> Option<String> {
        match self {
            Token::StringLiteral(text, _) => { Some(text.clone()) }
            Token::Identifier(text, _) => { Some(text.clone()) }
            Token::BoolLiteral(value, _) => { Some( if *value { BOOL_STATE_NAMES.0.to_string().clone() } else { BOOL_STATE_NAMES.1.to_string().clone() }) }
            Token::UnspecifiedString(text, _) => { Some(text.clone()) }
            Token::Operator(op, _) => { Some(op.clone().as_ref().to_string()) }
            Token::Assignment(_) => { Some(ASSIGNMENT_OPERATION.to_string()) }

            _ => None,
        }
    }
}