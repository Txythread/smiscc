use std::collections::HashMap;
use uuid::Uuid;
use crate::compiler::backend::arch::{Architecture, Register, RegisterDataType, RegisterKind, RegisterMap, RegisterSavingBehaviour};
use crate::compiler::backend::flattener::InstructionMeta;


pub fn generate() -> Architecture {
    let mut instructions: HashMap<InstructionMeta, String> = HashMap::new();

    instructions.insert(InstructionMeta::MoveReg, String::from("\tmov\t$a, $b\n"));
    instructions.insert(InstructionMeta::MoveImm, String::from("\tmov\t$a, #$b\n"));
    instructions.insert(InstructionMeta::AddReg, String::from("\tadd\t$a, $a, $b\n"));
    instructions.insert(InstructionMeta::SubReg, String::from("\tsub\t$a, $a, $b\n"));
    instructions.insert(InstructionMeta::MulReg, String::from("\tmul\t$a, $a, $b\n"));
    instructions.insert(InstructionMeta::DivReg, String::from("\tsdiv\t$a, $a, $b\n"));
    instructions.insert(InstructionMeta::StackStore, String::from("\tstr\t$a, [$sp, #$b]\n"));
    instructions.insert(InstructionMeta::StackLoad, String::from("\tldr\t$a, [$sp, #$b]\n"));
    instructions.insert(InstructionMeta::Exit, String::from("\tmov\tx16, #1\n\tmov\tx0, $a\n\tsvc\t#0x80\n"));
    instructions.insert(InstructionMeta::Call, String::from("\tb\t$a\n"));

    Architecture::new(
        "aarch64_macOS".to_string(),
        instructions,
        RegisterMap::new(
            vec![
                (Register::new("x0".to_string(), RegisterKind::GeneralPurpose, 8, RegisterSavingBehaviour::CallerSaved, vec![RegisterDataType::Address, RegisterDataType::Integer]), None),
                (Register::new("x1".to_string(), RegisterKind::GeneralPurpose, 8, RegisterSavingBehaviour::CallerSaved, vec![RegisterDataType::Address, RegisterDataType::Integer]), None),
                (Register::new("x2".to_string(), RegisterKind::GeneralPurpose, 8, RegisterSavingBehaviour::CallerSaved, vec![RegisterDataType::Address, RegisterDataType::Integer]), None),
                (Register::new("x3".to_string(), RegisterKind::GeneralPurpose, 8, RegisterSavingBehaviour::CallerSaved, vec![RegisterDataType::Address, RegisterDataType::Integer]), None),
                (Register::new("x4".to_string(), RegisterKind::GeneralPurpose, 8, RegisterSavingBehaviour::CallerSaved, vec![RegisterDataType::Address, RegisterDataType::Integer]), None),
                (Register::new("x5".to_string(), RegisterKind::GeneralPurpose, 8, RegisterSavingBehaviour::CallerSaved, vec![RegisterDataType::Address, RegisterDataType::Integer]), None),
                (Register::new("x6".to_string(), RegisterKind::GeneralPurpose, 8, RegisterSavingBehaviour::CallerSaved, vec![RegisterDataType::Address, RegisterDataType::Integer]), None),
                (Register::new("x7".to_string(), RegisterKind::GeneralPurpose, 8, RegisterSavingBehaviour::CallerSaved, vec![RegisterDataType::Address, RegisterDataType::Integer]), None),

                (Register::new("x8".to_string(), RegisterKind::GeneralPurpose, 8, RegisterSavingBehaviour::Scratch, vec![RegisterDataType::Address, RegisterDataType::Integer]), None),

                (Register::new("x9".to_string(), RegisterKind::GeneralPurpose, 8, RegisterSavingBehaviour::CallerSaved, vec![RegisterDataType::Address, RegisterDataType::Integer]), None),
                (Register::new("x10".to_string(), RegisterKind::GeneralPurpose, 8, RegisterSavingBehaviour::CallerSaved, vec![RegisterDataType::Address, RegisterDataType::Integer]), None),
                (Register::new("x11".to_string(), RegisterKind::GeneralPurpose, 8, RegisterSavingBehaviour::CallerSaved, vec![RegisterDataType::Address, RegisterDataType::Integer]), None),
                (Register::new("x12".to_string(), RegisterKind::GeneralPurpose, 8, RegisterSavingBehaviour::CallerSaved, vec![RegisterDataType::Address, RegisterDataType::Integer]), None),
                (Register::new("x13".to_string(), RegisterKind::GeneralPurpose, 8, RegisterSavingBehaviour::CallerSaved, vec![RegisterDataType::Address, RegisterDataType::Integer]), None),
                (Register::new("x14".to_string(), RegisterKind::GeneralPurpose, 8, RegisterSavingBehaviour::CallerSaved, vec![RegisterDataType::Address, RegisterDataType::Integer]), None),
                (Register::new("x15".to_string(), RegisterKind::GeneralPurpose, 8, RegisterSavingBehaviour::CallerSaved, vec![RegisterDataType::Address, RegisterDataType::Integer]), None),

                (Register::new("x19".to_string(), RegisterKind::GeneralPurpose, 8, RegisterSavingBehaviour::CalleeSaved, vec![RegisterDataType::Address, RegisterDataType::Integer]), None),
                (Register::new("x20".to_string(), RegisterKind::GeneralPurpose, 8, RegisterSavingBehaviour::CalleeSaved, vec![RegisterDataType::Address, RegisterDataType::Integer]), None),
                (Register::new("x21".to_string(), RegisterKind::GeneralPurpose, 8, RegisterSavingBehaviour::CalleeSaved, vec![RegisterDataType::Address, RegisterDataType::Integer]), None),
                (Register::new("x22".to_string(), RegisterKind::GeneralPurpose, 8, RegisterSavingBehaviour::CalleeSaved, vec![RegisterDataType::Address, RegisterDataType::Integer]), None),
                (Register::new("x23".to_string(), RegisterKind::GeneralPurpose, 8, RegisterSavingBehaviour::CalleeSaved, vec![RegisterDataType::Address, RegisterDataType::Integer]), None),
                (Register::new("x24".to_string(), RegisterKind::GeneralPurpose, 8, RegisterSavingBehaviour::CalleeSaved, vec![RegisterDataType::Address, RegisterDataType::Integer]), None),
                (Register::new("x25".to_string(), RegisterKind::GeneralPurpose, 8, RegisterSavingBehaviour::CalleeSaved, vec![RegisterDataType::Address, RegisterDataType::Integer]), None),
                (Register::new("x26".to_string(), RegisterKind::GeneralPurpose, 8, RegisterSavingBehaviour::CalleeSaved, vec![RegisterDataType::Address, RegisterDataType::Integer]), None),
                (Register::new("x27".to_string(), RegisterKind::GeneralPurpose, 8, RegisterSavingBehaviour::CalleeSaved, vec![RegisterDataType::Address, RegisterDataType::Integer]), None),
                (Register::new("x28".to_string(), RegisterKind::GeneralPurpose, 8, RegisterSavingBehaviour::CalleeSaved, vec![RegisterDataType::Address, RegisterDataType::Integer]), None),


                (Register::new("sp".to_string(), RegisterKind::StackPointer, 8, RegisterSavingBehaviour::CalleeSaved, vec![RegisterDataType::Address]), Some(Uuid::new_v4())),
            ],
            8,
            0,
            HashMap::new(),
            26,
            vec![0,1,2,3,4,5,6,7]
         ),
        include_str!("aarch64_macOS_header_bp.s"),
        ""
    )
}