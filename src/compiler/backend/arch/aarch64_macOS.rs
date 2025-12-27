use std::collections::HashMap;
use crate::compiler::backend::arch::{Architecture, Register, RegisterDataType, RegisterKind, RegisterMap, RegisterSavingBehaviour};
use crate::compiler::backend::flattener::InstructionMeta;

pub fn generate() -> Architecture {
    let mut instructions: HashMap<InstructionMeta, String> = HashMap::new();

    instructions.insert(InstructionMeta::MoveReg, String::from("\tmov\t$a, $b"));
    instructions.insert(InstructionMeta::MoveImm, String::from("\tmov\t$a, #$b"));
    instructions.insert(InstructionMeta::AddReg, String::from("\tadd\t$a, $a, $b"));

    Architecture::new(
        "aarch64_macOS".to_string(),
        instructions,
        RegisterMap::new(
            vec![
                (Register::new("x0".to_string(), RegisterKind::GeneralPurpose, 8, RegisterSavingBehaviour::CalleeSaved, vec![RegisterDataType::Address, RegisterDataType::Integer]), None),
                (Register::new("x1".to_string(), RegisterKind::GeneralPurpose, 8, RegisterSavingBehaviour::CalleeSaved, vec![RegisterDataType::Address, RegisterDataType::Integer]), None),
                (Register::new("x2".to_string(), RegisterKind::GeneralPurpose, 8, RegisterSavingBehaviour::CalleeSaved, vec![RegisterDataType::Address, RegisterDataType::Integer]), None),
                (Register::new("x3".to_string(), RegisterKind::GeneralPurpose, 8, RegisterSavingBehaviour::CalleeSaved, vec![RegisterDataType::Address, RegisterDataType::Integer]), None),
                (Register::new("x4".to_string(), RegisterKind::GeneralPurpose, 8, RegisterSavingBehaviour::CalleeSaved, vec![RegisterDataType::Address, RegisterDataType::Integer]), None),
                (Register::new("x5".to_string(), RegisterKind::GeneralPurpose, 8, RegisterSavingBehaviour::CalleeSaved, vec![RegisterDataType::Address, RegisterDataType::Integer]), None),
                (Register::new("x6".to_string(), RegisterKind::GeneralPurpose, 8, RegisterSavingBehaviour::CalleeSaved, vec![RegisterDataType::Address, RegisterDataType::Integer]), None),
                (Register::new("x7".to_string(), RegisterKind::GeneralPurpose, 8, RegisterSavingBehaviour::CalleeSaved, vec![RegisterDataType::Address, RegisterDataType::Integer]), None),
                (Register::new("x8".to_string(), RegisterKind::GeneralPurpose, 8, RegisterSavingBehaviour::CalleeSaved, vec![RegisterDataType::Address, RegisterDataType::Integer]), None),
                (Register::new("x9".to_string(), RegisterKind::GeneralPurpose, 8, RegisterSavingBehaviour::CalleeSaved, vec![RegisterDataType::Address, RegisterDataType::Integer]), None),
            ],
            0,
            0,
            HashMap::new()
         )
    )
}