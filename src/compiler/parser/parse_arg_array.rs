use std::rc::Rc;
use crate::compiler::data_types::object::ObjectType;
use crate::compiler::line_map::LineMap;
use crate::compiler::parser::parser_meta::ParserMetaState;
use crate::compiler::tokenization::token::Token;

type ArrayParsingSubfunction<T> = dyn Fn(Rc<Vec<Token>>, &mut usize, &mut LineMap, Rc<Vec<ObjectType>>) -> T;

/// Parses an argument array using another parse function.
/// The array should be comma separated and be constrained by
/// arithmetic parentheses
pub fn parse_arg_array<T>(state: &mut ParserMetaState, parse_fn: &ArrayParsingSubfunction<T>) -> Vec<T> {
    // Look for the "("
    if !matches!(state.tokens[*state.cursor], Token::ArithmeticParenthesisOpen(_)) {
        todo!("didnt find the '(', got {:?} instead", state.tokens[*state.cursor])
    }
    *state.cursor += 1;


    let mut data: Vec<T> = vec![];

    let mut expected_item = true;

    println!("Starting, pointing to {:?}", state.tokens[*state.cursor]);

    'data_loop: loop {

        println!("Processing token: {:?}", state.tokens[*state.cursor]);

        // Look for a ',' or a ')'
        match state.tokens[*state.cursor].clone() {
            Token::ArgumentSeparator(_) => {
                if expected_item {
                    panic!("expected item, received a colon.")
                }

                *state.cursor += 1;
                expected_item = true;

                continue 'data_loop;
            }

            Token::ArithmeticParenthesisClose(_) => {
                if expected_item {
                    println!("just info, remove this panic to continue.")
                }

                *state.cursor += 1;
                break 'data_loop;
            }

            _ => {
                if !expected_item {
                    panic!("expected , or :")
                }

                // Find whatever is expected
                let mut map = state.line_map.clone();
                data.push(parse_fn(state.tokens.clone(), state.cursor, &mut map, state.datatypes.clone()));
                *state.line_map = map;
                expected_item = false;
            }
        }
    }

    data

}