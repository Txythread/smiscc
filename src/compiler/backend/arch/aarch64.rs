// Contains general, non-eabi specific definitions for AArch64.
// Some general definitions might be in the eabi specific files, too.
pub mod aarch64_mac_os;
mod aarch64_opt;

use std::rc::Rc;
use crate::compiler::backend::arch::Register;
use crate::compiler::backend::arch::isa::Isa;
use crate::compiler::backend::assembly::AssemblyInstruction;
use crate::compiler::backend::flattener::ComparisonType;
use crate::compiler::optimization::OptimizationFlags;

/// The [Isa] implementation for Aarch64.
#[derive(Clone, Eq, Hash, PartialEq)]
pub enum Aarch64Asm {
    MoveReg(Register, Register),
    MoveImm(Register, i64),
    Load(Register, Register, u8),
    Store(Register, Register, u8),
    AddReg(Register, Register),
    AddImm(Register, i64),
    SubReg(Register, Register),
    SubImm(Register, i64),
    MulReg(Register, Register),
    DivReg(Register, Register),
    StackLoad(Register, u64),
    StackStore(Register, u64),
    Exit(Register),
    Call(Rc<String>),
    Label(Rc<String>),
    Compare(Register, Register),
    Jump(Rc<String>),
    JumpEqual(Rc<String>),
    JumpNotEqual(Rc<String>),
    CSet(Register, &'static str)
}


impl Isa for Aarch64Asm {
    fn to_string(&self) -> String {
        use Aarch64Asm as A;
        match self {
            A::MoveReg(a, b) => format!("\tmov\t{}, {}\n", a.name, b.name),
            A::MoveImm(a, b) => format!("\tmov\t{}, #{}\n", a.name, b),
            A::Load(_, _, _) => todo!(),
            A::Store(_, _, _) => todo!(),
            A::AddReg(a, b) => format!("\tadd\t{}, {}, {}\n", a.name, a.name, b.name),
            A::AddImm(a, b) => format!("\tadd\t{}, {}, #{}\n", a.name, a.name, b),
            A::SubReg(a, b) => format!("\tsub\t{}, {}, {}\n", a.name, a.name, b.name),
            A::SubImm(a, b) => format!("\tsub\t{}, {}, #{}\n", a.name, a.name, b),
            A::MulReg(a, b) => format!("\tmul\t{}, {}, {}\n", a.name, a.name, b.name),
            A::DivReg(a, b) => format!("\tdiv\t{}, {}, {}\n", a.name, a.name, b.name),
            A::StackLoad(a, b) => format!("\tldr\t{}, [sp, #{}]\n", a.name, b),
            A::StackStore(a, b) => format!("\tstr\t{}, [sp, #{}]\n", a.name, b),
            A::Exit(a) => format!("\tmov\tx16, #1\n\tmov\tx0, {}\n\tsvc\t#0x80\n", a.name),
            A::Call(a) => format!("\tbl\t{}\n", a),
            A::Label(a) => format!("\n{}:\n", a),
            A::Compare(a, b) => format!("\tcmp\t{}, {}\n", a.name, b.name),
            A::Jump(a) => format!("\tb\t{}\n", a),
            A::JumpEqual(a) => format!("\tbeq\t{}\n", a),
            A::JumpNotEqual(a) => format!("\tbne\t{}\n", a),
            A::CSet(a, cmp) => format!("\tcset\t{}, {}\n", a.name, cmp)
        }
    }

    fn optimize(instructions: Vec<Rc<Self>>, _flags: OptimizationFlags) -> Vec<Rc<Self>> {
        Self::optimize_internal(instructions)
    }
}

impl From<AssemblyInstruction> for Aarch64Asm {
    fn from(asm: AssemblyInstruction) -> Self {
        use AssemblyInstruction as AI;
        use Aarch64Asm as AA;

        match asm {
            AI::MoveReg(a, b) => AA::MoveReg(a, b),
            AI::MoveImm(a, i) => AA::MoveImm(a, i),
            AI::Load(dest, adr, len) => AA::Load(dest, adr, len),
            AI::Store(data, adr, len) => AA::Store(data, adr, len),
            AI::AddReg(a, b) => AA::AddReg(a, b),
            AI::AddImm(a, i) => AA::AddImm(a, i),
            AI::SubReg(a, b) => AA::SubReg(a, b),
            AI::SubImm(a, i) => AA::SubImm(a, i),
            AI::MulReg(a, b) => AA::MulReg(a, b),
            AI::DivReg(a, b) => AA::DivReg(a, b),
            AI::StackLoad(a, b) => AA::StackLoad(a, b),
            AI::StackStore(a, b) => AA::StackStore(a, b),
            AI::Exit(a) => AA::Exit(a),
            AI::Call(label) => AA::Call(Rc::new(label)),
            AI::Label(name) => AA::Label(name),
            AI::Compare(a, b) => AA::Compare(a, b),
            AI::Jump(a) => AA::Jump(a),
            AI::JumpEqual(a) => AA::JumpEqual(a),
            AI::JumpNotEqual(a) => AA::JumpNotEqual(a),
            AssemblyInstruction::ExtractCompare(a, b) => {
                let mut comparison: &'static str;
                use ComparisonType as CT;
                match b {
                    CT::Equal => comparison = "eq",
                    CT::NotEqual => comparison = "ne",
                    CT::Greater => comparison = "gt",
                    CT::GreaterOrEqual => comparison = "ge",
                    CT::Less => comparison = "lt",
                    CT::LessOrEqual => comparison = "le",
                }

                AA::CSet(a, comparison)
            }
        }
    }
}