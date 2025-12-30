use std::rc::Rc;
use crate::compiler::line_map::{LineMap, TokenPosition};
use crate::compiler::parser::statement::Statement;
use crate::compiler::parser::statements::Statements;
use crate::compiler::tokenization::token::Token;
use crate::compiler::parser::tree::node::*;
use crate::config::tokenization_options::Keyword;
use strum::IntoEnumIterator;
use crate::compiler::parser::parse_arithmetic_expression::parse_arithmetic_expression;

pub fn parse(tokens: Vec<Vec<Token>>, line_map: LineMap) -> Option<Rc<dyn Node>> {
    let statements = Statements::iter().collect::<Vec<_>>();
    let mut lines: Vec<Rc<dyn Node>> = vec![];

    for x in tokens.iter().enumerate() {
        let line = x.1;
        let line_number = x.0;


        if line.is_empty() { continue; }
        let first_token = line[0].clone();

        println!("First token: {:?}", first_token);


        match first_token {
            Token::KeywordType(keyword, _) => {
                for statement in statements.iter() {
                    if let Some(statement_keyword) = statement.get_affiliated_keyword() {
                        if statement_keyword != keyword { continue; }

                        println!("Found statement: {:?}", statement);

                        // The statement is the statement in question.
                        // Generate its arguments (starting with the header).
                        let mut arguments: Vec<Rc<dyn Node>> = vec![];
                        let argument_types = vec![statement.get_header_format(), statement.get_body_format()].concat();
                        let mut cursor = 1; // skip the first token

                        for type_ in argument_types {
                            let is_required = type_.1;
                            let kind = type_.0;

                            println!("Type: {:?}", kind);


                            match kind {
                                ExpressionKind::Value => {
                                    println!("Tokens: {:?}, cursor: {}", line, cursor);

                                    // Parse an arithmetic expression
                                    if let Some(result) = parse_arithmetic_expression(line.clone(), line_number as u32, line_map.clone(), 0, &mut cursor, true) {
                                        arguments.push(result);
                                    } else {
                                        if is_required {
                                            // Throw an error. Something was required that wasn't given in the current statement.
                                            todo!()
                                        }
                                    }
                                }
                                ExpressionKind::Assignment => {
                                    match line[cursor as usize].clone() {
                                        Token::Assignment(pos ) => arguments.push(Rc::new(AssignmentSymbolNode::new((line_number, pos)))),
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
                                    match line[cursor as usize].clone() {
                                        Token::Identifier(id, pos) => {
                                            arguments.push(Rc::new(IdentifierNode::new(id, None, (line_number, pos))))
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
                        lines.push(statement_node.unwrap());

                    }
                }
            }

            Token::Identifier(name, pos) => {
                let id_node = IdentifierNode::new(name, None, (line_number, pos.clone()));

                match line[1].clone() {
                    Token::Assignment(_) => {},
                    _ => {
                        println!("Parsing function ");
                        let value = parse_arithmetic_expression(line.clone(), line_number as u32, line_map.clone(), 0, &mut 0, false).unwrap();

                        lines.push(value);

                        continue;
                    }
                }

                let value = parse_arithmetic_expression(line.clone(), line_number as u32, line_map.clone(), 0, &mut 2, false).unwrap();

                let assignment_node = AssignmentNode::new(Rc::new(id_node), value, (line_number, pos));

                lines.push(Rc::new(assignment_node));
            }

            _ => {}
        }
    }

    let code = CodeBlockNode::new((0, TokenPosition::new(0, 0)), lines);

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

        println!("{:#?}", node);

    }

    #[test]
    fn test_parse_multiplication() {
        let tokens1 = tokenize(split("6 + 7 * 67 - 420;".to_string()).0, &mut LineMap::test_map());
        let tokens2 = tokenize(split("(6 + (7 * 67)) - 420;".to_string()).0, &mut LineMap::test_map());

        let parsed1 = parse_arithmetic_expression(tokens1[0].clone(), 0, LineMap::test_map(), 0, &mut 0, true,);
        let parsed2 = parse_arithmetic_expression(tokens2[0].clone(), 0, LineMap::test_map(), 0, &mut 0, true);

        println!("1: {:#?}", parsed1);
        println!("2: {:#?}", parsed2);

        assert_eq!(format!("{:#?}", parsed1), format!("{:#?}", parsed2));
    }
}
