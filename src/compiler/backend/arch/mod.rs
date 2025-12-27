pub mod aarch64_macOS;
mod register;

use std::cmp::PartialEq;
use std::collections::HashMap;
use derive_new::new;
use uuid::Uuid;
use crate::compiler::backend::arch::register::{RegisterDataType, RegisterKind, RegisterSavingBehaviour};
use crate::compiler::backend::assembly::AssemblyInstruction;
use crate::compiler::backend::flattener::{Instruction, InstructionMeta};

#[derive(new, Debug, Clone, PartialEq)]
pub struct Architecture {
    pub name: String,
    pub instructions: HashMap<InstructionMeta, String>,
    pub register_map: RegisterMap,
}

impl Architecture {
    /// Gets the register where a variable lives. If the variable is currently
    /// on the stack, it is retrieved from there. Preserved objects won't be damaged
    /// so pass them as any objects needed in unmittelbarer future.
    pub fn get_object(&mut self, object: Uuid, preserving: Vec<Uuid>) -> (Register, Vec<AssemblyInstruction>) {
        let mut instructions = Vec::new();

        // Check if a register has the object in question
        for register in self.register_map.registers.clone() {
            if let Some(contents) = register.1 {
                if contents == object {
                    return (register.0, vec![])
                }
            }
        }

        // Make room for it in the registers as it's needed regardless of the value being on stack or not.
        let mut result_reg = self.provide_empty_register(preserving);
        println!("storing resulting register: {:?}", result_reg);
        instructions.append(result_reg.1.as_mut());

        let result_reg = result_reg.0;

        // Load the data from the stack if possible
        let stack_info = self.register_map.stack.iter().find(|x| *x.0 == object);

        if let Some(stack_info) = stack_info {
            let stack_pos = *stack_info.1 as u64;

            instructions.push(AssemblyInstruction::StackLoad(result_reg.clone(), stack_pos));
        } else {
            // Set the register's contents to the object UUID
            let register_index = self.register_map.registers.iter().position(|x| x.0 == result_reg);

            self.register_map.registers[register_index.unwrap()].1 = Some(object);
        }

        println!("instructions: {:?}", instructions);


        (result_reg, instructions)
    }

    /// Provides an empty register. If there is one available right away, that one
    /// is chosen. When all registers are used, the stack is used to store the data.
    /// Registers containing the values of ignored objects will not be chosen.
    pub fn provide_empty_register(&mut self, ignoring: Vec<Uuid>) -> (Register, Vec<AssemblyInstruction>) {
        // Try to locate an empty register if possible.
        for register in self.register_map.registers.clone() {
            if register.1.is_none() {
                return (register.0, vec![])
            }
        }


        // Take a random register and move its contents to the stack
        for i in 0..self.register_map.registers.len() {
            let register = self.register_map.registers[i].clone();


            if ignoring.contains(&register.1.unwrap()) {
                continue;
            }

            if matches!(register.0.kind, RegisterKind::GeneralPurpose) {
                // Found a general purpose register
                // If it's been stored on the stack already, re-use this position.

                let stack_pos = self.register_map.stack.iter().find(|x|x.0.clone() == register.1.unwrap());

                let mut move_function: Vec<AssemblyInstruction> = Vec::new();

                if let Some(stack_pos) = stack_pos {
                    move_function.push(AssemblyInstruction::StackStore(register.0.clone(), *stack_pos.1 as u64));
                } else {
                    self.register_map.stack.insert(register.1.unwrap(), self.register_map.stack_offset);
                    move_function.push(AssemblyInstruction::StackStore(register.0.clone(), self.register_map.stack_offset as u64));
                    self.register_map.stack_offset += register.0.size_bytes as usize;
                }

                // Remove the object uuid to prevent it being mistaken later
                self.register_map.registers[i].1 = None;


                return (register.0, move_function)
            }
        }

        panic!("No register found")
    }
}

#[derive(new, Debug, Clone, PartialEq)]
pub struct RegisterMap {
    registers: Vec<(Register, Option<Uuid>)>,

    /// The index of the scratch register in the [registers map](registers)
    scratch_register: usize,

    /// The stack offset. Must be reset at function start.
    stack_offset: usize,

    /// The contents of the stack and the offset in comparison to the stack pointer.
    stack: HashMap</* object- */Uuid, /*stack offset: */usize>
}

#[derive(new, Debug, Clone, PartialEq)]
pub struct Register {
    pub name: String,
    pub kind: RegisterKind,
    pub size_bytes: u8,
    pub saving_behaviour: RegisterSavingBehaviour,
    pub options: Vec<RegisterDataType>
}