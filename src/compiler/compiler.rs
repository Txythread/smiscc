use crate::compiler::line_map::LineMap;
use crate::compiler::splitter::split;
use crate::compiler::tokenization::tokenizer::tokenize;
use crate::compiler::parser::parse::parse;


pub fn compile(code: String) {
    let mut splitted = split(code);
    let tokens = tokenize(splitted.0.clone(), &mut splitted.1);
    let parsed = parse(tokens.clone(), splitted.1.clone());

    println!("Splitted: {:?}", splitted.clone().0.clone());
    println!("Tokens: {:?}", tokens);
    println!("Parsed: {:?}", parsed);
}
