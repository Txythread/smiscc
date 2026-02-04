use std::rc::Rc;
use crate::ArgumentList;
use crate::compiler::optimization::OptimizationFlags;

mod compiler_coordinator;
pub mod data_types;
pub mod line_map;
pub mod trimmer;
pub mod tokenization;
pub mod parser;
mod backend;
pub mod optimization;
pub mod context;

pub fn compile(code: String, args: ArgumentList, opt_flags: OptimizationFlags) {
    compiler_coordinator::compile(code, args, Rc::new(opt_flags));
}