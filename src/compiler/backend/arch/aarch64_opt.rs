use std::rc::Rc;
use crate::compiler::backend::arch::aarch64::Aarch64Asm;

impl Aarch64Asm {
    pub fn optimize_internal(instructions: Vec<Rc<Self>>) -> Vec<Rc<Self>> {
        instructions
    }
}