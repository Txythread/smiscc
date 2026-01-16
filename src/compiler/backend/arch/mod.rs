pub mod aarch64_mac_os;
pub mod smiscc_none;
mod register;

use std::cmp::PartialEq;
use std::collections::HashMap;
use derive_new::new;
use uuid::Uuid;
use crate::compiler::backend::arch::register::{RegisterDataType, RegisterKind, RegisterSavingBehaviour};
use crate::compiler::backend::assembly::AssemblyInstruction;
use crate::compiler::backend::flattener::InstructionMeta;
use crate::compiler::parser::function_meta::FunctionStyle;

#[derive(new, Debug, Clone, PartialEq)]
pub struct Architecture {
    pub name: String,
    /// The instructions. What the string is for is specified by the
    /// (InstructionMeta)[InstructionMeta] provided. The associated string will get
    /// some parts replaced.
    /// 1. Regular arguments are provided as $a, $b, ...
    /// 1. The stack pointer is provided as $sp.
    /// 1. The scratch register is provided $scratch
    pub instructions: HashMap<InstructionMeta, String>,
    pub register_map: RegisterMap,
    pub leading_boilerplate: &'static str,
    pub trailing_boilerplate: &'static str,
    pub address_alignment: usize,
}

impl Architecture {
    /// Gets the register where a variable lives. If the variable is currently
    /// on the stack, it is retrieved from there. Preserved objects won't be damaged
    /// so pass them as any objects needed in unmittelbarer future.
    pub fn get_object(&mut self, object: Uuid, preserving: Vec<Uuid>) -> (Register, Vec<AssemblyInstruction>) {
        let mut instructions = Vec::new();

        // Check if a register has the object in question
        for register in self.register_map.registers.clone() {
            if let Some(contents) = register.1
                && contents == object {
                    return (register.0, vec![])
                }
        }

        // Make room for it in the registers as it's needed regardless of the value being on stack or not.
        let mut result_reg = self.provide_empty_register(preserving, &|_|true);
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


        (result_reg, instructions)
    }

    /// Removes an object from the list of maintained objects, freeing the space and
    /// the registers it uses. This is done by removing all the references to it.
    pub fn delete_object(&mut self, object: Uuid) {
        // Remove register references
        for i in 0..self.register_map.registers.len() {
            let register = self.register_map.registers[i].clone();

            if register.1 == Some(object) {
                self.register_map.registers[i].1 = None;
            }
        }

        // Remove stack references
        self.register_map.stack.remove(&object);
    }


    /// Moves data into a register without updating the code, so just
    /// the register map will be affected.
    pub fn move_into_reg_no_code(&mut self, object: Uuid, register: Register) {
        let index = self.register_map.registers.iter().position(|x| x.0 == register);
        self.register_map.registers[index.unwrap()].1 = Some(object);
    }

    /// Moves an object into a specified register
    pub fn move_into_reg(&mut self, object: Uuid, register: Register, preserving: Vec<Uuid>) -> Vec<AssemblyInstruction> {
        let mut instructions: Vec<AssemblyInstruction> = Vec::new();



        // Store the register's contents if needed
        if let Some(reg_contents) = self.register_map.registers.clone().iter().find(|x| x.0==register) {
            if reg_contents.1 == Some(object) {
                return instructions;
            }

            if let Some(reg_contents) = reg_contents.1 {
                let mut movement = self.provide_empty_register(preserving, &|_| true);
                instructions.append(movement.1.as_mut());
                instructions.push(AssemblyInstruction::MoveReg(register.clone(), movement.0.clone()));

                if let Some(index) = self.register_map.registers.iter().position(|x| x.0 == movement.0) {
                    self.register_map.registers[index].1 = Some(reg_contents);
                }
            }
        }

        // Update the register state
        if let Some(index) = self.register_map.registers.iter().position(|x| x.1==Some(object)) {
            self.register_map.registers[index].1 = Some(object);
        }

        // Load the register value into the register
        // 1. Try to find it in the registers
        for other_register in self.register_map.registers.clone().iter().enumerate() {
            if other_register.1.1 == Some(object) {
                self.register_map.registers[other_register.0].1 = None;

                instructions.push(AssemblyInstruction::MoveReg(register.clone(), other_register.1.0.clone()));
                return instructions;
            }
        }


        // 2. Try to find it in the stack, as it couldn't be located inside a register
        for stack_info in self.register_map.stack.clone().iter().enumerate() {
            if *stack_info.1.0 == object {
                instructions.push(AssemblyInstruction::StackLoad(register.clone(), *stack_info.1.1 as u64));
                return instructions;
            }
        }

        todo!()
    }

    /// Get the argument register for an architecture and calling convention
    /// A return value of None means that it should be stored on the stack.
    pub fn get_register_for_argument(&self, argument_index: usize, calling_convention: FunctionStyle) -> Option<Register> {
        match calling_convention {
            FunctionStyle::C => {
                Some(self.register_map.registers.get(self.register_map.c_style_arg_map[argument_index])?.0.clone())
            }
            FunctionStyle::Smisc => todo!(),
        }
    }

    /// Prepares the data for a new function.
    /// This means that:
    /// 1. All register contents will be forgotten
    /// 2. The stack data will be cleared.
    /// 3. The callee saved regs get backed up
    pub fn prepare_new_function(&mut self) {
        self.clear_registers_and_stack();
        self.backup_callee_saved_regs();
    }

    /// Stops the current function and generates heading and trailing
    /// code.
    /// This means that:
    /// 1. The stack pointer gets set (header)
    /// 2. The stack pointer gets restored (trailer)
    /// 3. The callee-saved registers get restored
    /// 4. [A new function gets prepared](Self::prepare_new_function)
    ///
    /// The first result of this function is the header, the second the
    /// trailer code.
    pub fn end_function(&mut self) -> (Vec<AssemblyInstruction>, Vec<AssemblyInstruction>) {
        let stack_offset = (self.register_map.stack_offset + (self.register_map.stack_offset % 16)) as i64;
        let header: Vec<AssemblyInstruction> = vec![
            AssemblyInstruction::SubImm(self.get_stack_pointer(), stack_offset),
        ];

        let mut trailer: Vec<AssemblyInstruction> = vec![
            AssemblyInstruction::AddImm(self.get_stack_pointer(), stack_offset),
        ];


        trailer.append(&mut self.restore_backup_map());
        self.prepare_new_function();

        (header, trailer)
    }

    /// Creates UUIDs for the callee-saved registers and writes to
    /// the backup register map for restoring the state later.
    pub fn backup_callee_saved_regs(&mut self) {
        for x in self.register_map.registers.clone().iter().enumerate() {
            let register = x.1.0.clone();
            let i = x.0;

            if matches!(register.saving_behaviour, RegisterSavingBehaviour::CalleeSaved) {
                let uuid = Some(Uuid::new_v4());
                self.register_map.registers[i].1 = uuid;
                self.register_map.backup_reg_map.push((register, uuid));
            }
        }
    }

    /// Restores the register map that was backed up if possible and
    /// deletes all the other UUIDs.
    pub fn restore_backup_map(&mut self) -> Vec<AssemblyInstruction> {
        let mut instructions: Vec<AssemblyInstruction> = Vec::new();
        let backup_uuids: Vec<Uuid> = self.register_map.backup_reg_map.iter().map(|x|x.1.unwrap()).collect();

        // Delete all unlisted objects in the registers
        for object in self.register_map.clone().registers.iter().filter_map(|x|x.1) {
            if !backup_uuids.contains(&object) {
                self.delete_object(object);
            }
        }

        // Delete all unlisted objects in the stack
        for object in self.register_map.clone().stack.keys() {
            if !backup_uuids.contains(object) {
                self.delete_object(*object);
            }
        }

        let mut correctly_placed_uuids: Vec<Uuid> = Vec::new();
        while let Some(item) = self.register_map.backup_reg_map.pop() {
            let object = item.1.unwrap();
            instructions.append(&mut self.move_into_reg(object, item.0, correctly_placed_uuids.clone()));
            correctly_placed_uuids.push(object);
        }

        instructions
    }

    pub fn clear_registers_and_stack(&mut self) {
        for i in 0..self.register_map.registers.len() {
            self.register_map.registers[i].1 = None;
        }

        self.register_map.stack = HashMap::new();
        self.register_map.stack_offset = 0;
    }




    /// Stores all the caller saved regs to other empty registers or the stack
    pub fn backup_caller_saved_regs(&mut self) -> Vec<AssemblyInstruction> {
        let mut instructions: Vec<AssemblyInstruction> = vec![];
        for reg_info in self.register_map.registers.clone().iter().enumerate() {
            let _i = reg_info.0;
            let register = reg_info.1.clone();

            if register.0.saving_behaviour != RegisterSavingBehaviour::CallerSaved { continue; }

            if let Some(object) = register.1 {
                // Look for an empty reg to put stuff in or put it on the stack
                if let Some(mut empty_register) = self.conditionally_provide_empty_register(vec![]) {
                    // Put the register in the register
                    instructions.append(empty_register.1.as_mut());
                    instructions.push(AssemblyInstruction::MoveReg(empty_register.0.clone(), register.0.clone()));

                    if let Some(index) = self.register_map.registers.iter().position(|x| x.0==empty_register.0) {
                        self.register_map.registers[index].1 = Some(object);
                    }
                } else {
                    // Put the data in the stack
                    let mut movement = self.push_object_to_stack(object);

                    instructions.append(movement.as_mut().unwrap())
                }

                if let Some(index) = self.register_map.registers.iter().position(|x| x.0==register.0) {
                    self.register_map.registers[index].1 = None;
                }
            }
        }

        instructions
    }

    /// Takes a register and throws its data onto the stack either at the location
    /// it was stored in already or creates a new position.
    /// This expects a register to currently possess the data.
    pub fn push_object_to_stack(&mut self, object: Uuid) -> Option<Vec<AssemblyInstruction>> {
        // Try to locate the object on the stack
        let register = self.register_map.registers.iter().find(|x| x.1==Some(object))?.0.clone();

        for stack_info in self.register_map.stack.clone() {
            if stack_info.0 == object {
                return Some(vec![
                    AssemblyInstruction::StackStore(register, stack_info.1 as u64)
                ])
            }
        }

        let stack_top = self.register_map.stack_offset;
        self.register_map.stack_offset += register.size_bytes as usize;

        self.register_map.stack.insert(object, stack_top);

        Some(vec![
            AssemblyInstruction::StackStore(register, stack_top as u64)
        ])
    }


    /// Gets the default stack pointer in use for this architecture
    pub fn get_stack_pointer(&self) -> Register {
        let sp = self.register_map.registers[self.register_map.stack_pointer_register].0.clone();

        if sp.options != vec![RegisterDataType::Address] {
            panic!("Stack pointer register was expected, other type of register was found")
        }

        if sp.kind != RegisterKind::StackPointer {
            panic!("Stack pointer register was expected, other type of register was found")
        }

        sp
    }

    /// Gets the default scratch register in use for this architecture
    pub fn get_scratch_register(&self) -> Register {
        let scratch = self.register_map.registers[self.register_map.scratch_register].0.clone();

        if scratch.options.contains(&RegisterDataType::Float) {
            panic!("Scratch register was expected, other type of register was found")
        }

        if scratch.kind != RegisterKind::GeneralPurpose {
            panic!("Scratch register was expected, other type of register was found")
        }

        if scratch.saving_behaviour != RegisterSavingBehaviour::Scratch {
            panic!("Scratch register was expected, other type of register was found")
        }

        scratch
    }

    /// Provides an empty register using [this function](Self::provide_empty_register) IF
    /// it can be achieved with an empty register. Returns None if there is no register
    /// free and the data should potentially be stored on the stack instead.
    pub fn conditionally_provide_empty_register(&mut self, ignoring: Vec<Uuid>) -> Option<(Register, Vec<AssemblyInstruction>)> {
        let mut other_self = self.clone();
        let result = other_self.provide_empty_register(ignoring, &|x| x.saving_behaviour==RegisterSavingBehaviour::CalleeSaved);

        // Look if the register is empty
        let register = self.register_map.registers.iter().find(|&x| x.0==result.0).unwrap();

        if register.clone().1.is_some() {
            // Operation failed, no empty register was returned
            return None
        }

        *self = other_self;

        Some(result)
    }


    /// Provides an empty register. If there is one available right away, that one
    /// is chosen. When all registers are used, the stack is used to store the data.
    /// Registers containing the values of ignored objects will not be chosen.
    /// Registers may only be chosen when the allow_reg function returns true on them.
    pub fn provide_empty_register(&mut self, ignoring: Vec<Uuid>, allow_reg: &dyn Fn(Register) -> bool) -> (Register, Vec<AssemblyInstruction>) {
        // Try to locate an empty register if possible.
        for register in self.register_map.registers.clone() {
            if register.1.is_none() && allow_reg(register.0.clone()) {
                return (register.0, vec![])
            }
        }


        // Take a random register and move its contents to the stack
        for i in 0..self.register_map.registers.len() {
            let register = self.register_map.registers[i].clone();

            if register.1.is_none() {
                // This register has been skipped, although it is empty, there's no
                // reason for that to change now, so skip it again
                continue;
            }

            if ignoring.contains(&register.1.unwrap()) {
                continue;
            }

            if matches!(register.0.kind, RegisterKind::GeneralPurpose) {
                // Found a general purpose register
                // If it's been stored on the stack already, re-use this position.

                if !allow_reg(register.0.clone()) { continue; }

                let stack_pos = self.register_map.stack.iter().find(|x|*x.0 == register.1.unwrap());

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

    backup_reg_map: Vec<(Register, Option<Uuid>)>,

    /// The index of the scratch register in the [registers map](Self::registers)
    scratch_register: usize,

    /// The stack offset. Must be reset at function start.
    stack_offset: usize,

    /// The contents of the stack and the offset in comparison to the stack pointer.
    stack: HashMap</* object- */Uuid, /*stack offset: */usize>,

    /// The index of the stack pointer register in the [registers map](Self::registers)
    stack_pointer_register: usize,

    /// A map of where all the arguments go in a C-Style call
    /// The registers are given by their indexes in the registers.
    c_style_arg_map: Vec<usize>
}

#[derive(new, Debug, Clone, PartialEq)]
pub struct Register {
    pub name: String,
    pub kind: RegisterKind,
    pub size_bytes: u8,
    pub saving_behaviour: RegisterSavingBehaviour,
    pub options: Vec<RegisterDataType>
}