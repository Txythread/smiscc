use crate::compiler::backend::assembly;
use crate::compiler::backend::context::Context;
use crate::compiler::backend::flattener::flatten;
use crate::compiler::splitter::split;
use crate::compiler::tokenization::tokenizer::tokenize;
use crate::compiler::parser::parse::parse;
use crate::compiler::backend::arch::aarch64_macOS;

pub fn compile(code: String) {
    let mut splitted = split(code);
    let tokens = tokenize(splitted.0.clone(), &mut splitted.1);
    let parsed = parse(tokens.clone(), splitted.1.clone());
    let flattened = flatten(parsed.clone().unwrap(), &mut Context::clear());
    let arch = aarch64_macOS::generate();
    let assembly = assembly::generate_assembly_instructions(flattened, arch.clone());

    if splitted.1.error_count == 0 {
        assembly::generate_assembly(assembly, arch, "test.s".to_string())
    }

    println!("\nFinished with {} errors", splitted.1.error_count)
}
