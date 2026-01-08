use std::rc::Rc;
use crate::compiler::data_types::object::ObjectType;
use crate::compiler::line_map::LineMap;
use crate::compiler::tokenization::token::Token;

/// Parses an argument array using another parse function.
/// The array should be comma separated and be constrained by
/// arithmetic parentheses
pub fn parse_arg_array<T>(tokens: Rc<Vec<Token>>, cursor: &mut usize, datatypes: Rc<Vec<ObjectType>>, line_map: &mut LineMap, parse_fn: &dyn Fn(Rc<Vec<Token>>, &mut usize, &mut LineMap, Rc<Vec<ObjectType>>) -> T) -> Vec<T> {
    // Look for the "("
    if !matches!(tokens[*cursor], Token::ArithmeticParenthesisOpen(_)) {
        todo!("didnt find the thing '('")
    }
    *cursor += 1;


    println!("parsing arg array");
    let mut data: Vec<T> = vec![];

    'data_loop: loop {
        // Find whatever is expected
        data.push(parse_fn(tokens.clone(), cursor, line_map, datatypes.clone()));

        // Look for a ',' or a ')'
        match tokens[*cursor].clone() {
            Token::Colon(_) => {
                *cursor += 1;
                continue 'data_loop;
            }

            Token::ArithmeticParenthesisClose(_) => {
                *cursor += 1;
                break 'data_loop;
            }

            _ => {
                todo!("Unexpected token in argument array")
            }
        }
    }

    data

}