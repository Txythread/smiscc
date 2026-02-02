use std::rc::Rc;
use logos::Logos;
use strum::IntoEnumIterator;
use crate::compiler::data_types::integer::*;
use crate::compiler::data_types::object::ObjectType;
use crate::compiler::line_map::LineMap;
use crate::compiler::trimmer::trim;
use crate::compiler::tokenization::token::Token;
use crate::compiler::tokenization::token_prototype::TokenPrototype;

/// ### Turn the split strings into tokens ("classifies" them)
///
/// `let a = b`
/// as an input,
/// would be turned into the following [tokens](Token)
/// ```
/// [
///     KeywordType(Let, TokenPosition { start: 0, length: 3 }),
///     Identifier("a", TokenPosition { start: 4, length: 1 }),
///     Assignment(TokenPosition { start: 6, length: 1 }),
///     Identifier("b", TokenPosition { start: 8, length: 1 })
/// ]
/// ```
///
/// This can then be used by the [parser](crate::compiler::parser::parse::parse)
/// to perform the next step.
pub fn tokenize_file(contents: String, file_number: usize, integer_types: Rc<Vec<(IntegerType, ObjectType)>>, line_map: &mut LineMap) -> Vec<Token> {
    // Remove the comments

    let mut lines: Vec<String> = Vec::new();

    {
        let mut cursor_in_block_comment = false;
        for line in contents.lines() {
            let line_commentless = trim(line, &mut cursor_in_block_comment);

            if !line.is_empty() {
                lines.push(line_commentless)
            }
        }
    }

    // Turn it into tokens with help of logos
    let mut resulting_tokens: Vec<Token> = Vec::new();

    for line in lines.iter() {
        let mut tokens = TokenPrototype::lexer(line);
        while let Some(token_prototype) = tokens.next() {
            if token_prototype.is_err() {
                todo!("Couldn't pass token '{:?}' at: {:?}", tokens.slice(), tokens.span())
            }

            println!("current: {:?}, prot: {:?}", tokens.slice(), token_prototype);

            let token = token_prototype.unwrap().into_token(
                tokens.span(),
                tokens.slice(),
                file_number,
                integer_types.clone(),
                line_map,
            );

            resulting_tokens.push(token);
        }
    }

    resulting_tokens
}

