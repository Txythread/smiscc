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
    
    /// Something that seperates two arguments, e.g. in a function call or array.
    /// This correlates with a "," in most languages.
    ArgumentSeparator(TokenPosition),

    //////////////////////////////////////////
    ////////////// PARENTHESES ///////////////
    //////////////////////////////////////////

    /// The arithmetic/default parenthesis ("(")
    ArithmeticParenthesisOpen(TokenPosition),
    
    /// The closing part of the arithmetic/default parenthesis (")")
    ArithmeticParenthesisClose(TokenPosition),

    /// A newline that doesn't force a logical newline (e.g. \n)
    SoftNewline(TokenPosition),

    /// A newline that should lead to a new line in the parsing state (e.g. ";")
    HardNewline(TokenPosition),

    /// An opening code block parenthesis ("{")
    CodeBlockParenthesisOpen(TokenPosition),

    /// A closing code block parenthesis ("}")
    CodeBlockParenthesisClose(TokenPosition),

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
            Token::ArgumentSeparator(pos) => { pos.clone() },
            Token::SoftNewline(pos) | Token::HardNewline(pos) => { pos.clone() },
            Token::CodeBlockParenthesisOpen(pos) | Token::CodeBlockParenthesisClose(pos) => pos.clone(),
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

    /// Whether this is a reasonable token to expect in an arithmetic operation.
    /// If it's not, it might interrupt/stop tokens from being parsed in the parser.
    pub fn is_expected_in_arithmetic(&self) -> bool {
        match self {
            Token::KeywordType(_, _) => false,
            Token::Assignment(_) => false,
            Token::SoftNewline(_) => false,
            Token::HardNewline(_) => false,

            _ => true
        }
    }

    pub fn is_line_delimiting(&self) -> bool {
        match self {
            Token::HardNewline(_) => true,
            Token::SoftNewline(_) => true,
            Token::CodeBlockParenthesisOpen(_) => true,
            Token::CodeBlockParenthesisClose(_) => true,

            _ => false
        }
    }
}