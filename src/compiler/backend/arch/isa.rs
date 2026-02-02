use std::hash::Hash;
use std::rc::Rc;
use crate::compiler::backend::assembly::AssemblyInstruction;
use crate::compiler::optimization::OptimizationFlags;

/// An ISA (or Instruction Set Architecture) holds architecture specific
/// instructions, which is important for certain optimizations.
/// They can be built from [AssemblyInstruction]s and be converted to a string
/// later.
pub trait Isa: Clone + Eq + Hash + From<AssemblyInstruction> {
    /// Produces raw assembly from the instruction.
    fn to_string(&self) -> String;

    /// Optimizes instructions.
    ///
    /// For example, not all architectures allow for the target in an
    /// add-operation to be specified independently, aarch64 does.
    /// So the architecture might optimize:
    /// ```
    /// mov     x2, x0
    /// add     x2, x1 ; x2 = x2 + x1
    /// ```
    /// (in AssemblyInstructions) to:
    /// ```
    /// add     x2, x0, x2
    /// ```
    /// in its own format.
    fn optimize(instructions: Vec<Rc<Self>>, flags: OptimizationFlags) -> Vec<Rc<Self>> { instructions }
}