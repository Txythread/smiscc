use std::rc::Rc;
use crate::compiler::data_types::object::ObjectType;
use crate::compiler::line_map::LineMap;
use crate::compiler::parser::parser_meta::ParserMetaState;
use crate::compiler::tokenization::token::Token;

/// Parses an argument array using another parse function.
/// The array should be comma separated and be constrained by
/// arithmetic parentheses
pub fn parse_arg_array<T>(state: &mut ParserMetaState, parse_fn: &dyn Fn(Rc<Vec<Token>>, &mut usize, &mut LineMap, Rc<Vec<ObjectType>>) -> T) -> Vec<T> {
    // Look for the "("
    if !matches!(state.tokens[*state.cursor], Token::ArithmeticParenthesisOpen(_)) {
        todo!("didnt find the thing '('")
    }
    *state.cursor += 1;


    let mut data: Vec<T> = vec![];

    'data_loop: loop {


        // Look for a ',' or a ')'
        match state.tokens[*state.cursor].clone() {
            Token::Colon(_) => {
                *state.cursor += 1;
                continue 'data_loop;
            }

            Token::ArithmeticParenthesisClose(_) => {
                *state.cursor += 1;
                break 'data_loop;
            }

            _ => {
                // Find whatever is expected
                let mut map = state.line_map.clone();
                data.push(parse_fn(state.tokens.clone(), state.cursor, &mut map, state.datatypes.clone()));
                *state.line_map = map;
            }
        }
    }

    data

}