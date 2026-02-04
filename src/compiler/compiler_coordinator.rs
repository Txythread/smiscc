use std::ops::Deref;
use std::process::Command;
use log::{info, log, Level, LevelFilter};
use crate::ArgumentList;
use crate::compiler::backend::assembly;
use crate::compiler::context::Context;
use crate::compiler::backend::flattener::flatten;
use crate::compiler::tokenization::tokenizer::tokenize_file;
use crate::compiler::parser::parse::parse;
use crate::compiler::backend::arch::aarch64::aarch64_mac_os;
use crate::compiler::data_types::object::ObjectType;
use crate::compiler::line_map::LineMap;
use std::rc::Rc;
use oslog::OsLogger;
use crate::compiler::backend::arch::aarch64::Aarch64Asm;
use crate::compiler::data_types::integer::build_integer_types;
use crate::compiler::optimization::OptimizationFlags;
use crate::compiler::parser::default::load_default_functions;
use crate::compiler::parser::tree::node::{CodeBlockArray, Node};
use crate::util::instruments::InstrumentsLog;

pub fn compile(code: String, args: ArgumentList, opt_flags: Rc<OptimizationFlags>) {

    let mut inst = InstrumentsLog::new("com.txythread.smiscc", "general-smiscc");
    inst.mark("Starting Compilation");

    info!("Starting Compilation");


    let mut line_map: LineMap = LineMap::new();

    let tokens = tokenize_file(
        code,
        0usize,
        Rc::new(build_integer_types()),
        &mut line_map,
    );

    if args.show_tokens {
        println!("----- Tokens -----");
        for line in tokens.iter().enumerate() {
            println!("{}:\t{:?}", line.0 + 1, line.1)
        }
        println!("------------------");
    }

    let mut object_types = Rc::new(ObjectType::generate_built_ins());

    let mut parsed = parse(vec![tokens.clone()], &mut line_map, &mut object_types).unwrap();
    let mut context = Context::clear(line_map, opt_flags.clone());
    object_types.iter().for_each(|object_type| {
       context.datatypes.insert(object_type.type_uuid, object_type.clone());
    });

    load_default_functions(&mut context);

    let mut parsed_clone: CodeBlockArray = parsed.downcast_rc::<CodeBlockArray>().unwrap().deref().clone();
    parsed_clone.perform_early_context_changes(&mut context);
    parsed = Rc::new(parsed_clone);

    let flattened = flatten(parsed, &mut context);
    let arch = aarch64_mac_os::generate();
    let assembly = assembly::generate_assembly_instructions(flattened, arch.clone(), opt_flags);

    if context.line_map.error_count == 0 {
        assembly::generate_assembly::<Aarch64Asm>(assembly, arch, "test.s".to_string())
    }

    info!("Compilation finished");

    cc::Build::new()
        .cargo_output(false)
        .cargo_warnings(false)
        .cargo_metadata(false)
        .file("test.s")
        .out_dir("./build/")
        .target("aarch64-apple-darwin")
        .opt_level(0)
        .host("aarch64-apple-darwin")
        .opt_level(3)
        .compile("test");


    info!("Assembling finished");

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
        .arg("arm64")
        .output().unwrap();



    info!("Linking finished");

    context.line_map.display_finish();
}
