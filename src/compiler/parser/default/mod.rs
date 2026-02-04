use include_dir::{include_dir, Dir};
use serde::Serialize;
use crate::compiler::context::Context;
use crate::compiler::parser::function_meta::FunctionMetaJson;

const FUNCTION_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/compiler/parser/default/functions");


pub fn load_default_functions(context: &mut Context) {
    for file in FUNCTION_DIR.files() {
        if let Some(contents) = file.contents_utf8() {
            let function_blueprint = serde_json::from_str::<FunctionMetaJson>(contents);
            let function_meta = function_blueprint.unwrap().to_function_meta(context);
            
            context.function_metas.push(function_meta);
        }
    }
}