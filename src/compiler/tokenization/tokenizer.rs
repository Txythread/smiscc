use strum::IntoEnumIterator;
use crate::compiler::data_types::integer::*;
use crate::compiler::line_map::LineMap;
use crate::compiler::tokenization::token::{Token, Token::* };
use crate::config::tokenization_options::*;
use crate::config::tokenization_options::Keyword;
use crate::util::operator;

/// ### Turn the split strings into tokens ("classify" them)
///
/// `["let", "a", "=", "b"]`
/// which is produced by the [splitter](crate::compiler::splitter::split),
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
pub fn tokenize(separated: Vec<Vec<String>>, line_map: &mut LineMap) -> Vec<Vec<Token>> {
    let mut lines: Vec<Vec<Token>> = Vec::new();


    for x in separated.iter().enumerate() {
        let line = x.1.clone();

        // The line number in the line map, not in the
        // original file.
        let line_number = x.0;


        // Set to "None" whenever no string is being built.
        // Set to an empty string whenever the string building process started
        // Has contents when the string was started, filled but not yet been
        // delimited.
        let mut current_string: Option<String> = None;
        let mut current_string_start: Option<u16> = None;

        let mut line_tokens: Vec<Token> = Vec::new();


        // Build the integer types
        // Those are used for checking for specific types (like unsigned 32-bit
        // integer) later in code.
        let integer_types = build_integer_types();

        'token_loop: for y in line.iter().enumerate() {
            let token = y.1.clone();
            let token_number = y.0;
            let current_token_pos = line_map.get_position_of_tokens(line_number as u32, token_number as u16, token_number as i16);


            // Append to string if necessary or close it.
            if let Some(string) = current_string.clone() {
                // Check if the token is the delimiter
                if token == STRING_MARKERS.1.to_string() {
                    // Calculate the position of the resulting string
                    let start_token_number = current_string_start.unwrap_or((token_number - 1) as u16 /*Fallback, one token before current*/);
                    let new_position = line_map.get_position_of_tokens(line_number as u32, start_token_number, token_number as i16);

                    let token = Token::StringLiteral(string, new_position);
                    line_tokens.push(token);

                    current_string_start = None;
                    current_string = None;
                } else {
                    current_string = Some(current_string.unwrap() + token.as_str());
                }

                continue;
            }

            if token == STRING_MARKERS.0.to_string() {
                current_string = Some(String::new());
                current_string_start = Some(token_number as u16);

                continue;
            }


            // Look if it's a keyword
            for keyword in Keyword::iter() {
                let keyword: Keyword = keyword;

                if keyword.as_ref() == token {
                    let token = KeywordType(keyword, current_token_pos.clone());

                    line_tokens.push(token);
                    continue 'token_loop;
                }
            }

            // Check for boolean literals, assignments,
            const TRUE_KEYWORD: &str = BOOL_STATE_NAMES.0;
            const FALSE_KEYWORD: &str = BOOL_STATE_NAMES.1;
            const ARITHMETIC_PARENTHESIS_OPEN: &str = ARITHMETIC_PARENTHESES.0;
            const ARITHMETIC_PARENTHESIS_CLOSE: &str = ARITHMETIC_PARENTHESES.1;
            const CODE_BLOCK_PARENTHESIS_OPEN: &str = CODE_BLOCK_PARENTHESES.1;
            const CODE_BLOCK_PARENTHESIS_CLOSE: &str = CODE_BLOCK_PARENTHESES.1;
            const OTHER: &str = CODE_BLOCK_PARENTHESES.0;
            match token.as_str() {
                // Look if is a parenthesis
                OTHER => {
                    let token = CodeBlockParenthesisOpen(current_token_pos.clone());
                    line_tokens.push(token);
                    continue 'token_loop;
                }

                // Look if it's true
                 TRUE_KEYWORD=> {
                    let token = BoolLiteral(true, current_token_pos.clone());

                    line_tokens.push(token);
                    continue 'token_loop;
                }

                // Look if it's false
                FALSE_KEYWORD => {
                    let token = BoolLiteral(false, current_token_pos.clone());

                    line_tokens.push(token);
                    continue 'token_loop;
                }

                ARITHMETIC_PARENTHESIS_OPEN => {
                    let token = ArithmeticParenthesisOpen(current_token_pos.clone());

                    line_tokens.push(token);
                    continue 'token_loop;
                }

                ARITHMETIC_PARENTHESIS_CLOSE => {
                    let token = ArithmeticParenthesisClose(current_token_pos.clone());

                    line_tokens.push(token);
                    continue 'token_loop;
                }


                // Check if it's an assigment
                ASSIGNMENT_OPERATION => {
                    let token = Assignment(current_token_pos.clone());

                    line_tokens.push(token);
                    continue 'token_loop;
                }

                SEPARATOR_CHARACTER => {
                    let token = ArgumentSeparator(current_token_pos.clone());

                    line_tokens.push(token);
                    continue 'token_loop;
                }



                CODE_BLOCK_PARENTHESIS_CLOSE => {
                    let token = CodeBlockParenthesisClose(current_token_pos.clone());

                    line_tokens.push(token);
                    continue 'token_loop;
                }



                ";" => {
                    let token = HardNewline(current_token_pos.clone());

                    line_tokens.push(token);
                    continue 'token_loop;
                }

                "\n" => {
                    let token = SoftNewline(current_token_pos.clone());

                    line_tokens.push(token);
                    continue 'token_loop;
                }
                
                ":" => {
                    let token = Colon(current_token_pos.clone());
                    
                    line_tokens.push(token);
                    continue 'token_loop;
                }

                _ => {}
            }



            // Check if it's an integer literal
            {
                let integer_value = generate_integer(
                    UnspecifiedString(token.clone(), current_token_pos.clone()),
                    integer_types.clone(),
                    line_number as u32,
                    token_number as u32,
                    line_map,
                );

                if let Some(integer_value) = integer_value {
                    line_tokens.push(IntegerLiteral(integer_value.0, integer_value.1, current_token_pos.clone()));
                    continue 'token_loop;
                }
            }

            // Check if it's an operator
            for operation in operator::Operation::iter() {
                let name: &str = operation.as_ref();

                if name != token { continue; }


                let operator: Token = Operator(operation, current_token_pos.clone());
                line_tokens.push(operator);

                continue 'token_loop;
            }

            // As it's no other option, it can only be an identifier.
            let identifier = Token::Identifier(token.clone(), current_token_pos.clone());
            line_tokens.push(identifier);
        }

        lines.push(line_tokens);
    }

    lines
}






