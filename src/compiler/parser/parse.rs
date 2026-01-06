use std::rc::Rc;
use crate::compiler::line_map::{LineMap, TokenPosition};
use crate::compiler::parser::statement::Statement;
use crate::compiler::parser::statements::Statements;
use crate::compiler::tokenization::token::Token;
use crate::compiler::parser::tree::node::*;
use crate::config::tokenization_options::Keyword;
use strum::IntoEnumIterator;
use crate::compiler::parser::parse_arithmetic_expression::parse_arithmetic_expression;

pub fn parse(files: Vec<Vec<Token>>, line_map: LineMap) -> Option<Rc<dyn Node>> {
    let statements = Statements::iter().collect::<Vec<_>>();
    let mut lines_in_block: Vec<Rc<dyn Node>> = vec![];

    let mut blocks: Vec<CodeBlockNode> = vec![];

    for x in files.iter().enumerate() {
        let contents = x.1;
        let file_number = x.0;

        let mut cursor = 0;


        if contents.is_empty() { continue; }


        'line_loop: loop {
            let first_token = contents[cursor].clone();
            let line_start = cursor;
            cursor += 1;

            match first_token.clone() {
                Token::KeywordType(keyword, _) => {
                    for statement in statements.iter() {
                        if let Some(statement_keyword) = statement.get_affiliated_keyword() {
                            if statement_keyword != keyword { continue; }


                            // The statement is the statement in question.
                            // Generate its arguments (starting with the header).
                            let mut arguments: Vec<Rc<dyn Node>> = vec![];
                            let argument_types = vec![statement.get_header_format(), statement.get_body_format()].concat();

                            for type_ in argument_types {
                                let is_required = type_.1;
                                let kind = type_.0;



                                match kind {
                                    ExpressionKind::Value => {

                                        // Parse an arithmetic expression
                                        if let Some(result) = parse_arithmetic_expression(contents.clone(), file_number as u32, line_map.clone(), 0, &mut cursor, true) {
                                            arguments.push(result);
                                        } else {
                                            if is_required {
                                                // Throw an error. Something was required that wasn't given in the current statement.
                                                todo!()
                                            }
                                        }
                                    }
                                    ExpressionKind::Assignment => {
                                        match contents[cursor as usize].clone() {
                                            Token::Assignment(pos) => arguments.push(Rc::new(AssignmentSymbolNode::new((file_number, pos)))),
                                            _ => {
                                                // Throw an error.
                                                todo!()
                                            }
                                        }

                                        cursor += 1;
                                    }
                                    ExpressionKind::Keyword(_) => {
                                        cursor += 1;
                                    }
                                    ExpressionKind::Identifier(_) => {
                                        match contents[cursor as usize].clone() {
                                            Token::Identifier(id, pos) => {
                                                arguments.push(Rc::new(IdentifierNode::new(id, None, (file_number, pos))))
                                            }

                                            _ => {
                                                // Throw an error.
                                                todo!()
                                            }
                                        }
                                        cursor += 1;
                                    }
                                }
                            }


                            let statement_node = statement.generate_entire_node(arguments);
                            lines_in_block.push(statement_node.unwrap());
                        }
                    }
                }

                Token::Identifier(name, pos) => {
                    let id_node = IdentifierNode::new(name, None, (file_number, pos.clone()));

                    match contents[1].clone() {
                        Token::Assignment(_) => {},
                        _ => {
                            cursor = line_start;
                            let value = parse_arithmetic_expression(contents.clone(), file_number as u32, line_map.clone(), 0, &mut cursor, false).unwrap();

                            lines_in_block.push(value);

                            continue;
                        }
                    }

                    cursor = line_start + 2;
                    let value = parse_arithmetic_expression(contents.clone(), file_number as u32, line_map.clone(), 0, &mut cursor, false).unwrap();

                    let assignment_node = AssignmentNode::new(Rc::new(id_node), value, (file_number, pos));

                    lines_in_block.push(Rc::new(assignment_node));
                }

                _ => {}
            }

            // Skip all newlines
            loop {
                match contents[cursor] {
                    Token::SoftNewline(_) | Token::HardNewline(_) => {cursor += 1}
                    _ => {continue 'line_loop}
                }

                if contents.len() <= cursor {
                    break 'line_loop;
                }
            }
        }
    }

    let code = CodeBlockNode::new((0, TokenPosition::new(0, 0)), Rc::new(Some("_start".to_string())), lines_in_block);

    Some(Rc::new(code))
}

#[derive(Debug)]
#[derive(Clone)]
pub enum ExpressionKind {
    /// Something that is supposed to be parsed as an arithmetic expression
    Value,

    Assignment,

    /// **Note:** None means multiple keywords are allowed here
    Keyword(Option<Keyword>),

    /// **Note:** Only Identifiers that won't get coerced into values
    Identifier(Option<String>),
}


#[cfg(test)]
mod tests {
    use crate::compiler::data_types::integer::IntegerType;
    use crate::compiler::line_map::{LineMap, TokenPosition};
    use crate::compiler::parser::parse::parse_arithmetic_expression;
    use crate::compiler::parser::parse_token::parse_token;
    use crate::compiler::tokenization::token::Token;
    use crate::compiler::parser::tree::node::*;
    use crate::compiler::splitter::split;
    use crate::compiler::tokenization::tokenizer::tokenize;
    use crate::util::operator::Operation;

    #[test]
    fn test_parse_token() {
        let node = parse_token(Token::IntegerLiteral(10, Some(IntegerType::Unsigned32BitInteger), TokenPosition::test_value()), 0, LineMap::new());
        let expected = Some(ValueNode::Literal(LiteralValueNode::Integer(IntegerLiteralNode { content: 10, kind: Some(IntegerType::Unsigned32BitInteger), position: (0, TokenPosition { start: 0, length: 0 }) })));

        assert_eq!(format!("{node:?}"), format!("{expected:?}"));
    }

    #[test]
    fn test_parse_arithmetic() {
        let node = parse_arithmetic_expression(
            vec![
                Token::ArithmeticParenthesisOpen(TokenPosition::test_value()),                                      // (
                Token::IntegerLiteral(10, Some(IntegerType::Unsigned32BitInteger), TokenPosition::test_value()),    // 10u32
                Token::Operator(Operation::Addition, TokenPosition::test_value()),                                  // +
                Token::IntegerLiteral(5, Some(IntegerType::Unsigned32BitInteger), TokenPosition::test_value()),     // 5u32
                Token::ArithmeticParenthesisClose(TokenPosition::test_value()),                                     // )
                Token::Operator(Operation::Subtraction, TokenPosition::test_value()),                               // -
                Token::Identifier("rumänien".to_string(), TokenPosition::test_value()),                             // rumänien

                ], 0, LineMap::new(), 0, &mut 0, true,
        );


    }

    #[test]
    fn test_parse_multiplication() {
        let tokens1 = tokenize(vec![split("6 + 7 * 67 - 420;".to_string(), String::from("test.txt"), &mut LineMap::test_map())], &mut LineMap::test_map());
        let tokens2 = tokenize(vec![split("(6 + (7 * 67)) - 420;".to_string(), String::from("test.txt"), &mut LineMap::test_map())], &mut LineMap::test_map());

        let parsed1 = parse_arithmetic_expression(tokens1[0].clone(), 0, LineMap::test_map(), 0, &mut 0, true,);
        let parsed2 = parse_arithmetic_expression(tokens2[0].clone(), 0, LineMap::test_map(), 0, &mut 0, true);


        assert_eq!(format!("{:#?}", parsed1), format!("{:#?}", parsed2));
    }
}
