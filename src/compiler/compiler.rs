use crate::compiler::backend::context::Context;
use crate::compiler::backend::flattener::flatten;
use crate::compiler::splitter::split;
use crate::compiler::tokenization::tokenizer::tokenize;
use crate::compiler::parser::parse::parse;


pub fn compile(code: String) {
    let mut splitted = split(code);
    let tokens = tokenize(splitted.0.clone(), &mut splitted.1);

    println!("Splitted: {:?}", splitted.clone().0.clone());
    println!("Tokens: {:?}", tokens);

    let parsed = parse(tokens.clone(), splitted.1.clone());

    println!("flattening...");

    let flattened = flatten(parsed.clone().unwrap(), &mut Context::clear());


    println!("Parsed: {:#?}", parsed);
}
