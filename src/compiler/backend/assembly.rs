use std::fs;
use std::io::{ErrorKind, Write};
use std::rc::Rc;
use crate::compiler::backend::arch::{Architecture, Register};
use crate::compiler::backend::arch::isa::Isa;
use crate::compiler::backend::flattener::{Instruction, JumpComparisonType};
use crate::compiler::parser::function_meta::FunctionStyle;

#[derive(Debug, Clone)]
pub enum AssemblyInstruction {
    /// Copy the contents of one register into the other one.  
    /// The first register is the target, the second one contains the data.
    MoveReg(Register, Register),
    /// Copy the data into the given register.
    MoveImm(Register, i64),
    
    /// Load (2) bytes of data at address at register (1) into register (0)
    Load(Register, Register, u8),
    /// Store (2) bytes of data from register (0) into the address at register (1)
    Store(Register, Register, u8),
    
    /// Adds the contents of the second register to the first register's contents
    AddReg(Register, Register),
    /// Adds the data to the register's contents
    AddImm(Register, i64),
    /// Subtracts the contents of the second register from the first register's contents
    SubReg(Register, Register),
    /// Subtracts the data from the register's contents.
    SubImm(Register, i64),
    /// Multiplies the contents of the second register with the first register's contents.
    MulReg(Register, Register),
    /// Divides the first register with the second one.
    DivReg(Register, Register),

    /// Get data from the stack at a specific offset and store it into a register
    /// without changing the stack address
    StackLoad(Register, u64),

    /// Store data to the stack a given offset without adjusting the stack address pointer.
    StackStore(Register, u64),

    /// Exit (returning Register)
    Exit(Register),

    /// Call a function (don't just jump to it)
    Call(String),
    
    Label(Rc<String>),

    Compare(Register, Register),

    Jump(Rc<String>),
    JumpEqual(Rc<String>),
    JumpNotEqual(Rc<String>),
}


pub fn generate_assembly<T: Isa>(code: Vec<AssemblyInstruction>, arch: Architecture, output_name: String) {
    let arch = Rc::new(arch);

    if let Some(err) = fs::remove_file(output_name.clone()).err() {
        match err.kind() {
            ErrorKind::NotFound => { /* This can be ignored as the file can get created later anyway */ }
            ErrorKind::PermissionDenied => {}
            ErrorKind::IsADirectory => {}
            ErrorKind::DirectoryNotEmpty => {}
            ErrorKind::ReadOnlyFilesystem => {}
            ErrorKind::StorageFull => {}
            ErrorKind::ResourceBusy => {}

            _ => {

            }
        }
    }

    let mut file = fs::File::create(&output_name).unwrap();

    file.write_all(arch.leading_boilerplate.as_bytes()).expect("");

    // Turn the assembly instructions into architecture specific instructions
    let mut arch_instructions: Vec<T> = Vec::with_capacity(code.len()); //code.iter().into::<T>().collect();

    for instruction in code {
        arch_instructions.push(instruction.into());
    }

    for instruction in arch_instructions {
        file.write_all(instruction.to_string().as_bytes()).expect("");
    }

    file.write_all(arch.trailing_boilerplate.as_bytes()).expect("");

    file.flush().expect("");
}

pub fn generate_assembly_instructions(code: Vec<Instruction>, architecture: Architecture) -> Vec<AssemblyInstruction> {
    let mut architecture = architecture;
    let mut instructions: Vec<AssemblyInstruction> = Vec::new();

    let mut function_start_idx: usize = 0;

    architecture.prepare_new_function();


    let mut i = 0;
    for instruction in code.clone() {
        i += 1;
        let _instructions_length = instructions.len();
        println!("instruction: {:?}", instruction);
        match instruction {
            Instruction::Move(obj_a, obj_b) => {
                let mut reg_a = architecture.get_object(obj_a, vec![obj_b]);
                let mut reg_b = architecture.get_object(obj_b, vec![obj_a]);

                instructions.append(reg_a.1.as_mut());
                instructions.append(reg_b.1.as_mut());

                instructions.push(AssemblyInstruction::MoveReg(reg_a.0, reg_b.0));
            }
            Instruction::MoveData(obj_a, data) => {
                let mut reg_a = architecture.get_object(obj_a, vec![]);

                instructions.append(reg_a.1.as_mut());

                instructions.push(AssemblyInstruction::MoveImm(reg_a.0, data));
            }
            Instruction::Add(obj_a, obj_b) => {
                let mut reg_a = architecture.get_object(obj_a, vec![obj_b]);
                let mut reg_b = architecture.get_object(obj_b, vec![obj_a]);

                instructions.append(reg_a.1.as_mut());
                instructions.append(reg_b.1.as_mut());

                instructions.push(AssemblyInstruction::AddReg(reg_a.0, reg_b.0));
            }
            Instruction::Sub(obj_a, obj_b) => {
                let mut reg_a = architecture.get_object(obj_a, vec![obj_b]);
                let mut reg_b = architecture.get_object(obj_b, vec![obj_a]);

                instructions.append(reg_a.1.as_mut());
                instructions.append(reg_b.1.as_mut());

                instructions.push(AssemblyInstruction::SubReg(reg_a.0, reg_b.0));
            }
            Instruction::Mul(obj_a, obj_b) => {
                let mut reg_a = architecture.get_object(obj_a, vec![obj_b]);
                let mut reg_b = architecture.get_object(obj_b, vec![obj_a]);

                instructions.append(reg_a.1.as_mut());
                instructions.append(reg_b.1.as_mut());

                instructions.push(AssemblyInstruction::MulReg(reg_a.0, reg_b.0));
            }
            Instruction::Div(obj_a, obj_b) => {
                let mut reg_a = architecture.get_object(obj_a, vec![obj_b]);
                let mut reg_b = architecture.get_object(obj_b, vec![obj_a]);

                instructions.append(reg_a.1.as_mut());
                instructions.append(reg_b.1.as_mut());

                instructions.push(AssemblyInstruction::DivReg(reg_a.0, reg_b.0));
            }

            Instruction::Mod(_, _) => {}
            Instruction::Load(_, _, _) => {}
            Instruction::Store(_, _, _) => {}
            Instruction::Drop(obj) => {
                architecture.delete_object(obj);
            }
            Instruction::Exit(obj) => {
                let mut reg_a = architecture.get_object(obj, vec![]);

                instructions.append(reg_a.1.as_mut());

                instructions.push(AssemblyInstruction::Exit(reg_a.0));
            }
            Instruction::Call(asm_name, args, _out) => {
                instructions.append(&mut architecture.backup_caller_saved_regs());

                for arg in args.clone().iter().enumerate() {
                    let i = arg.0;
                    let arg = arg.1;

                    let reg = architecture.get_register_for_argument(i, FunctionStyle::C);
                    let mut move_instructions = architecture.move_into_reg(*arg, reg.unwrap(), args.clone());

                    instructions.append(move_instructions.as_mut());
                }

                instructions.push(AssemblyInstruction::Call(asm_name));
                
                for arg in args.clone().iter() {
                    architecture.delete_object(*arg);
                }

            },
            Instruction::Label(asm_name, _global) => {
                instructions.push(AssemblyInstruction::Label(asm_name));
            }
            Instruction::FunctionEnd => {
                let (header, mut trailer) = architecture.end_function();
                instructions.append(&mut trailer);
                instructions.splice(function_start_idx..function_start_idx, header);
            }
            Instruction::ReceiveArgument(arg_name, arg_index) => {
                if let Some(position) = architecture.get_register_for_argument(arg_index as usize, FunctionStyle::C) {
                    architecture.move_into_reg_no_code(arg_name, position);
                } else {
                    todo!("No register for argument found")
                }
            }
            Instruction::JumpConditional(condition, label) => {
                let mut reg_a: Option<Register> = None;
                let mut reg_b: Option<Register> = None;

                if condition.comparison.requires_args() {
                    let mut register_a = architecture.get_object(condition.a.unwrap(), vec![]);
                    let mut register_b = architecture.get_object(condition.b.unwrap(), vec![]);

                    instructions.append(&mut register_a.1.as_mut());
                    instructions.append(&mut register_b.1.as_mut());

                    instructions.push(AssemblyInstruction::Compare(
                        register_a.0.clone(),
                        register_b.0.clone(),
                    ));

                    reg_a = Some(register_a.0);
                    reg_b = Some(register_b.0);
                }

                match condition.comparison {
                    JumpComparisonType::Equal => {
                        instructions.push(AssemblyInstruction::JumpEqual(label));
                    }

                    JumpComparisonType::NotEqual => {
                        instructions.push(AssemblyInstruction::JumpNotEqual(label));
                    }

                    _ => { todo!() }
                }
            }
            Instruction::Jump(label) => {
                instructions.push(AssemblyInstruction::Jump(label))
            }
            Instruction::FunctionStart => {
                println!("FunctionStart at {}", i);
                function_start_idx = instructions.len();
            }
        }
    }

    instructions
}