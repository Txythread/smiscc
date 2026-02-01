use crate::ArgumentList;

mod compiler_coordinator;
pub mod data_types;
pub mod line_map;
pub mod trimmer;
pub mod tokenization;
pub mod parser;
mod backend;

pub fn compile(code: String, args: ArgumentList) {
    compiler_coordinator::compile(code, args);
}