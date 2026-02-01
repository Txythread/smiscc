use std::rc::Rc;
use logos::{Logos, Span};
use strum::IntoEnumIterator;
use crate::compiler::data_types::integer::{generate_integer, IntegerType};
use crate::compiler::data_types::object::ObjectType;
use crate::compiler::line_map::{LineMap, TokenPosition};
use crate::compiler::tokenization::token::Token;
use crate::compiler::tokenization::token::Token::{IntegerLiteral, UnspecifiedString};
use crate::config::tokenization_options::Keyword;
use crate::util::operator;

#[derive(Clone, Copy, Debug, Logos, PartialEq)]
#[logos(skip r"[ \t\f]+")]
pub enum TokenPrototype {
    /// A string literal in the original code.
    /// At this stage, this still contains the leading and trailing delimiters.
    #[regex("\"[^\"]*\"")]
    StringLiteral,

    /// An integer literal. Type might be specified in the corresponding &str.
    #[regex("[0-9](x|d|o)?[0-9a-fA-Fu]*")]
    IntegerLiteral,

    #[token("true")]
    BoolLiteralTrue,

    #[token("false")]
    BoolLiteralFalse,

    /// An assignment (=), not to be confused with an equal operator (==)
    #[token("=")]
    Assignment,

    /// A newline with no ';'
    #[token("\n")]
    SoftNewline,

    /// Any operator (+-*/%, etc.)
    #[regex(r"\+|-|\*|/|(<<?<?)|(>>?>?)|(==)")]
    Operation,

    /// A newline that always terminates a line (';')
    #[token(";")]
    HardNewline,

    #[cfg(test)]
    #[token("test")]
    Test,

    /// A comma, separating values (or types) in tuples, arrays and such.
    #[token(",")]
    ArgumentSeparator,

    #[token("(")]
    ArithmeticParenthesisOpen,
    
    #[token(")")]
    ArithmeticParenthesisClose,
    
    #[token("{")]
    CurlyParenthesisOpen,
    
    #[token("}")]
    CurlyParenthesisClose,
    
    #[token(":")]
    Colon,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
}


impl TokenPrototype {
    pub fn into_token(self, pos: Span, string_contents: &str, file_number: usize, integer_types: Rc<Vec<(IntegerType, ObjectType)>>, line_map: &mut LineMap) -> Token {
        let position = TokenPosition::new(pos.start, pos.end - pos.start);
        match self {
            TokenPrototype::BoolLiteralTrue => Token::BoolLiteral(true, position),
            TokenPrototype::BoolLiteralFalse => Token::BoolLiteral(false, position),
            TokenPrototype::Assignment => Token::Assignment(position),
            TokenPrototype::SoftNewline => Token::SoftNewline(position),
            TokenPrototype::HardNewline => Token::HardNewline(position),
            TokenPrototype::ArgumentSeparator => Token::ArgumentSeparator(position),
            TokenPrototype::ArithmeticParenthesisOpen => Token::ArithmeticParenthesisOpen(position),
            TokenPrototype::ArithmeticParenthesisClose => Token::ArithmeticParenthesisClose(position),
            TokenPrototype::CurlyParenthesisOpen => Token::CodeBlockParenthesisOpen(position),
            TokenPrototype::CurlyParenthesisClose => Token::CodeBlockParenthesisClose(position),
            TokenPrototype::Colon => Token::Colon(position),
            
            #[cfg(test)]
            TokenPrototype::Test => panic!("test token is not meant to be converted from a prototype into a token"),
            
            TokenPrototype::Operation => {
                let operations = operator::Operation::iter(); // TODO: Would be better with a hash map

                for operation in operations {
                    if operation.as_ref() != string_contents { continue }

                    let token = Token::Operator(operation, position);

                    return token;
                }

                todo!("Unknown operation: {} passed regex", string_contents)
            }

            TokenPrototype::StringLiteral => {
                // Cut the leading and trailing delimiters from the string
                // to receive its true/expected contents.
                let inner_string = &string_contents[1..string_contents.len() - 1];

                let token = Token::StringLiteral(inner_string.to_string(), position);

                token
            }

            TokenPrototype::IntegerLiteral => {
                let integer_value = generate_integer(
                    UnspecifiedString(string_contents.to_string(), position),
                    integer_types.clone(),
                    file_number as u32,
                    0u32,
                    line_map,
                );

                if let Some(integer_value) = integer_value {
                    IntegerLiteral(integer_value.0, integer_value.1, position)
                } else {
                    panic!("todo: implement error, integer expected, found {string_contents}")
                }
            }


            TokenPrototype::Identifier => {
                // Check if it's a keyword
                for keyword in Keyword::iter() { // TODO: This can be done using a hash map
                    if keyword.as_ref() != string_contents { continue }

                    let token = Token::KeywordType(keyword, position);

                    return token;
                }

                Token::Identifier(string_contents.to_string(), position)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logos_tokens() {
        let test_text = "\
        \"was geht\"\
        asfasdfasdfasdf test \
        test\
        = ==\
        0d5 \
        0xf \
        134356 \
        ";

        let mut tokens = TokenPrototype::lexer(test_text);
        println!("{:?}", tokens);

        assert_eq!(tokens.next().unwrap().unwrap(), TokenPrototype::StringLiteral);
        assert_eq!(tokens.slice(), "\"was geht\"");
        assert_eq!(tokens.next().unwrap().ok(), Some(TokenPrototype::Identifier));
        assert_eq!(tokens.slice(), "asfasdfasdfasdf");
        assert_eq!(tokens.next().unwrap().ok(), Some(TokenPrototype::Test));
        assert_eq!(tokens.slice(), "test");
        assert_eq!(tokens.next().unwrap().ok(), Some(TokenPrototype::Test));
        assert_eq!(tokens.slice(), "test");
        assert_eq!(tokens.next().unwrap().ok(), Some(TokenPrototype::Assignment));
        assert_eq!(tokens.slice(), "=");
        assert_eq!(tokens.next().unwrap().ok(), Some(TokenPrototype::Operation));
        assert_eq!(tokens.slice(), "==");
        assert_eq!(tokens.next().unwrap().ok(), Some(TokenPrototype::IntegerLiteral));
        assert_eq!(tokens.slice(), "0d5");
        assert_eq!(tokens.next().unwrap().ok(), Some(TokenPrototype::IntegerLiteral));
        assert_eq!(tokens.slice(), "0xf");
        assert_eq!(tokens.next().unwrap().ok(), Some(TokenPrototype::IntegerLiteral));
        assert_eq!(tokens.slice(), "134356");
        assert_eq!(tokens.next(), None);
    }
}