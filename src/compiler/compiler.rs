use clap::{Arg};
use std::process::Command;
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

    cc::Build::new()
        .file("test.s")
        .out_dir("./build/")
        .target("aarch64-apple-darwin")
        .opt_level(0)
        .host("aarch64-apple-darwin")
        .compile("test");

    let xc_command = Command::new("xcrun")
        .args(["-sdk", "macosx", "--show-sdk-path"])
        .output()
        .unwrap();

    let syslibroot = String::from_utf8(xc_command.stdout).unwrap();

    println!("syslibroot: {}", syslibroot);

    let mut command = Command::new("ld");

    for entry in glob::glob("./build/*-test.o").unwrap() {
        command.arg(entry.unwrap().to_string_lossy().to_string());
    }

    command
        .arg("-o")
        .arg("test")
        .arg("-lSystem")
        .arg("-syslibroot")
        .arg(syslibroot.trim())
        .arg("-e")
        .arg("_start")
        .arg("-arch")
        .arg("arm64");

    println!("output: {}", String::from_utf8_lossy(&command.output().unwrap().stdout));;



    context.line_map.display_finish();
}
