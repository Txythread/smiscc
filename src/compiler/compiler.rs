use crate::compiler::backend::assembly;
use crate::compiler::backend::context::Context;
use crate::compiler::backend::flattener::flatten;
use crate::compiler::splitter::split;
use crate::compiler::tokenization::tokenizer::tokenize;
use crate::compiler::parser::parse::parse;
use crate::compiler::backend::arch::aarch64_mac_os;

pub fn compile(code: String) {
    let mut splitted = split(code);
    let tokens = tokenize(splitted.0.clone(), &mut splitted.1);
    println!("Tokens: {:?}", tokens);
    let parsed = parse(tokens.clone(), splitted.1.clone());
    println!("Parsed AST: {:?}", parsed);
    let mut context = Context::clear(splitted.1.clone());
    let flattened = flatten(parsed.clone().unwrap(), &mut context);
    let arch = aarch64_mac_os::generate();
    let assembly = assembly::generate_assembly_instructions(flattened, arch.clone());

    if context.line_map.error_count == 0 {
        assembly::generate_assembly(assembly, arch, "test.s".to_string())
    }

    context.line_map.display_finish();
}
