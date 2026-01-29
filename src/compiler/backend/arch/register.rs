use std::collections::HashMap;
use derive_new::new;
use uuid::Uuid;

#[derive(new, Debug, Clone, PartialEq)]
pub struct Register {
    pub name: String,
    pub kind: RegisterKind,
    pub size_bytes: u8,
    pub saving_behaviour: RegisterSavingBehaviour,
    pub options: Vec<RegisterDataType>
}

#[derive(new, Debug, Clone, PartialEq)]
pub struct RegisterMap {
    pub(crate) registers: Vec<(Register, Option<Uuid>)>,

    pub(crate) backup_reg_map: Vec<(Register, Option<Uuid>)>,

    /// The index of the scratch register in the [registers map](Self::registers)
    pub(crate) scratch_register: usize,

    /// The stack offset. Must be reset at function start.
    pub(crate) stack_offset: usize,

    /// The contents of the stack and the offset in comparison to the stack pointer.
    pub(crate) stack: HashMap</* object- */Uuid, /*stack offset: */usize>,

    /// The index of the stack pointer register in the [registers map](Self::registers)
    pub(crate) stack_pointer_register: usize,

    /// A map of where all the arguments go in a C-Style call
    /// The registers are given by their indexes in the registers.
    pub(crate) c_style_arg_map: Vec<usize>
}

#[derive(new, Debug, Clone, PartialEq)]
pub enum RegisterKind {
    GeneralPurpose,
    StackPointer,
}

#[derive(new, Debug, Clone, PartialEq)]
pub enum RegisterSavingBehaviour {
    /// Never use beyond the next function call
    Scratch,

    /// Save the register before calling the function. The function can
    /// change it any way it wants and doesn't have to restore it.
    CallerSaved,

    /// The caller can expect the callee to restore this register to be
    /// restored when the function returns.
    CalleeSaved,
}

#[derive(new, Debug, Clone, PartialEq)]
pub enum RegisterDataType {
    Integer,
    Float,
    Address
}