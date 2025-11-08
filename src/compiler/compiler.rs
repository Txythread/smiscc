use crate::compiler::line_map::LineMap;
use crate::compiler::splitter::split;
use crate::compiler::tokenization::tokenizer::tokenize;

pub fn compile(code: String) {
    let mut splitted = split(code);
    let tokens = tokenize(splitted.0, &mut splitted.1);

    println!("Tokens: {:?}", tokens);
}