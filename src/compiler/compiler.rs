use clap::{Arg};
use std::process::Command;
use crate::ArgumentList;
use crate::compiler::backend::assembly;
use crate::compiler::backend::context::Context;
use crate::compiler::backend::flattener::flatten;
use crate::compiler::splitter::split;
use crate::compiler::tokenization::tokenizer::tokenize;
use crate::compiler::parser::parse::parse;
use crate::compiler::backend::arch::aarch64_mac_os;
use crate::compiler::line_map::LineMap;

pub fn compile(code: String, args: ArgumentList) {
    let mut line_map: LineMap = LineMap::new();
    let mut splitted = split(code, String::from("test*.txt"), &mut line_map);
    let tokens = tokenize(vec![splitted.clone()], &mut line_map);
    if args.show_tokens {
        for line in tokens.iter().enumerate() {
            println!("{}:\t{:?}", line.0 + 1, line.1)
        }
    }

    let parsed = parse(tokens.clone(), &mut line_map);
    let mut context = Context::clear(line_map);
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




    context.line_map.display_finish();
}
