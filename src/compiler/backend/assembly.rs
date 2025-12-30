use std::fs;
use std::io::Write;
use std::ops::Deref;
use std::rc::Rc;
use crate::compiler::backend::arch::{Architecture, Register};
use crate::compiler::backend::flattener::{Instruction, InstructionMeta};
use crate::compiler::parser::function_meta::FunctionStyle;

#[derive(Debug)]
pub enum AssemblyInstruction {
    /// Copy the contents of one register into the other one.  
    /// The first register is the target, the second one contains the data.
    MoveReg(Register, Register),
    /// Copy the data into the given register.
    MoveImm(Register, i64),
    
    /// Load (2) bytes of data at address at register (1) into register (0)
    Load(Register, Register, Register),
    /// Store (2) bytes of data from register (0) into the address at register (1)
    Store(Register, Register, Register),
    
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
}

impl AssemblyInstruction {
    pub fn into_asm(&self) -> String {
        todo!()
    }

    pub fn get_instruction_meta(&self) -> InstructionMeta {
        match self {
            AssemblyInstruction::MoveReg(_, _) => InstructionMeta::MoveReg,
            AssemblyInstruction::MoveImm(_, _) => InstructionMeta::MoveImm,
            AssemblyInstruction::Load(_, _, _) => InstructionMeta::Load,
            AssemblyInstruction::Store(_, _, _) => InstructionMeta::Store,
            AssemblyInstruction::StackStore(_, _) => InstructionMeta::StackStore,
            AssemblyInstruction::AddReg(_, _) => InstructionMeta::AddReg,
            AssemblyInstruction::SubReg(_, _) => InstructionMeta::SubReg,
            AssemblyInstruction::StackLoad(_, _) => InstructionMeta::StackLoad,
            AssemblyInstruction::AddImm(_, _) | &AssemblyInstruction::SubImm(_, _) => todo!(),
            AssemblyInstruction::MulReg(_, _) => InstructionMeta::MulReg,
            AssemblyInstruction::DivReg(_, _) => InstructionMeta::DivReg,
            AssemblyInstruction::Exit(_) => InstructionMeta::Exit,
            AssemblyInstruction::Call(_) => InstructionMeta::Call,
        }
    }

    pub fn get_arguments(&self) -> Vec<(/* key */String, /* value: */String)> {
        match self {
            AssemblyInstruction::StackStore(a, b) => {
                vec![
                    (
                        String::from("$a"),
                        a.name.clone()
                    ),
                    (
                        String::from("$b"),
                        format!("{}", *b)
                    )
                ]
            }

            AssemblyInstruction::StackLoad(a, b) => {
                vec![
                    (
                        String::from("$a"),
                        a.name.clone()
                    ),
                    (
                        String::from("$b"),
                        format!("{}", *b)
                    )
                ]
            }

            AssemblyInstruction::AddImm(a, b) => {
                vec![
                    (
                        String::from("$a"),
                        a.name.clone()
                    ),
                    (
                        String::from("$b"),
                        format!("{}", *b)
                    )
                ]
            }

            AssemblyInstruction::SubImm(a, b) => {
                vec![
                    (
                        String::from("$a"),
                        a.name.clone()
                    ),
                    (
                        String::from("$b"),
                        format!("{}", *b)
                    )
                ]
            }

            AssemblyInstruction::MoveReg(a, b) => {
                vec![
                    (
                        String::from("$a"),
                        a.name.clone()
                    ),
                    (
                        String::from("$b"),
                        b.name.clone()
                    )
                ]
            }

            AssemblyInstruction::MoveImm(a, b) => {
                vec![
                    (
                        String::from("$a"),
                        a.name.clone()
                    ),
                    (
                        String::from("$b"),
                        format!("{}", *b)
                    )
                ]
            },
            AssemblyInstruction::Load(a, b, c) => {
                vec![
                    (
                        String::from("$a"),
                        a.name.clone()
                    ),
                    (
                        String::from("$b"),
                        b.name.clone()
                    ),
                    (
                        String::from("$c"),
                        c.name.clone()
                    )

                ]
            }
            AssemblyInstruction::Store(a, b, c) => {
                vec![
                    (
                        String::from("$a"),
                        a.name.clone()
                    ),
                    (
                        String::from("$b"),
                        b.name.clone()
                    ),
                    (
                        String::from("$c"),
                        c.name.clone()
                    )
                ]
            }
            AssemblyInstruction::AddReg(a, b) => {
                vec![
                    (
                        String::from("$a"),
                        a.name.clone()
                    ),
                    (
                        String::from("$b"),
                        b.name.clone()
                    )
                ]
            }
            AssemblyInstruction::SubReg(a, b) => {
                vec![
                    (
                        String::from("$a"),
                        a.name.clone()
                    ),
                    (
                        String::from("$b"),
                        b.name.clone()
                    )
                ]
            },

            AssemblyInstruction::MulReg(a, b) => {
                vec![
                    (
                        String::from("$a"),
                        a.name.clone()
                    ),
                    (
                        String::from("$b"),
                        b.name.clone()
                    )
                ]
            }


            AssemblyInstruction::DivReg(a, b) => {
                vec![
                    (
                        String::from("$a"),
                        a.name.clone()
                    ),
                    (
                        String::from("$b"),
                        b.name.clone()
                    )
                ]
            }

            AssemblyInstruction::Exit(a) => {
                vec![
                    (
                        String::from("$a"),
                        a.name.clone()
                    )
                ]
            }
            AssemblyInstruction::Call(a) => {
                vec![
                    (
                        String::from("$a"),
                        a.clone()
                    )
                ]
            }
        }
    }

    pub fn make_string(&self, arch: Rc<Architecture>) -> String {
        let mut meta = arch.instructions.get(&self.get_instruction_meta()).unwrap().clone();
        let params = self.get_arguments();

        for param in params {
            meta = meta.replace(param.0.as_str(), param.1.as_str());
        }

        meta = meta.replace("$sp", arch.get_stack_pointer().name.as_str());
        meta = meta.replace("$scratch", arch.get_scratch_register().name.as_str());

        meta
    }
}

pub fn generate_assembly(code: Vec<AssemblyInstruction>, arch: Architecture, output_name: String) {
    let arch = Rc::new(arch);

    fs::remove_file(output_name.clone()).unwrap();

    let mut file = fs::File::create(&output_name).unwrap();

    file.write(arch.leading_boilerplate.as_bytes()).expect("");

    for instruction in code {
        file.write(instruction.make_string(arch.clone()).as_bytes()).expect("");
    }
    
    file.write(arch.trailing_boilerplate.as_bytes()).expect("");

    file.flush().expect("");
}

pub fn generate_assembly_instructions(code: Vec<Instruction>, architecture: Architecture) -> Vec<AssemblyInstruction> {
    let mut architecture = architecture;
    let mut instructions: Vec<AssemblyInstruction> = Vec::new();


    for instruction in code.clone() {
        let mut instructions_length = instructions.len();
        match instruction {
            Instruction::Move(objA, objB) => {
                let mut regA = architecture.get_object(objA, vec![objB]);
                let mut regB = architecture.get_object(objB, vec![objA]);

                instructions.append(regA.1.as_mut());
                instructions.append(regB.1.as_mut());

                instructions.push(AssemblyInstruction::MoveReg(regA.0, regB.0));
            }
            Instruction::MoveData(objA, data) => {
                let mut regA = architecture.get_object(objA, vec![]);

                instructions.append(regA.1.as_mut());

                instructions.push(AssemblyInstruction::MoveImm(regA.0, data));
            }
            Instruction::Add(objA, objB) => {
                let mut regA = architecture.get_object(objA, vec![objB]);
                let mut regB = architecture.get_object(objB, vec![objA]);

                instructions.append(regA.1.as_mut());
                instructions.append(regB.1.as_mut());

                instructions.push(AssemblyInstruction::AddReg(regA.0, regB.0));
            }
            Instruction::Sub(objA, objB) => {
                let mut regA = architecture.get_object(objA, vec![objB]);
                let mut regB = architecture.get_object(objB, vec![objA]);

                instructions.append(regA.1.as_mut());
                instructions.append(regB.1.as_mut());

                instructions.push(AssemblyInstruction::SubReg(regA.0, regB.0));
            }
            Instruction::Mul(objA, objB) => {
                let mut regA = architecture.get_object(objA, vec![objB]);
                let mut regB = architecture.get_object(objB, vec![objA]);

                instructions.append(regA.1.as_mut());
                instructions.append(regB.1.as_mut());

                instructions.push(AssemblyInstruction::MulReg(regA.0, regB.0));
            }
            Instruction::Div(objA, objB) => {
                let mut regA = architecture.get_object(objA, vec![objB]);
                let mut regB = architecture.get_object(objB, vec![objA]);

                instructions.append(regA.1.as_mut());
                instructions.append(regB.1.as_mut());

                instructions.push(AssemblyInstruction::DivReg(regA.0, regB.0));
            }

            Instruction::Mod(_, _) => {}
            Instruction::Load(_, _, _) => {}
            Instruction::Store(_, _, _) => {}
            Instruction::Drop(obj) => {
                architecture.delete_object(obj);
            }
            Instruction::Exit(obj) => {
                let mut regA = architecture.get_object(obj, vec![]);

                instructions.append(regA.1.as_mut());

                instructions.push(AssemblyInstruction::Exit(regA.0));
            }
            Instruction::Call(asm_name, args, out) => {
                instructions.append(&mut architecture.backup_caller_saved_regs());
                
                for arg in args.clone().iter().enumerate() {
                    let i = arg.0;
                    let arg = arg.1;

                    let reg = architecture.get_register_for_argument(i, FunctionStyle::C);
                    let mut move_instructions = architecture.move_into_reg(arg.clone(), reg.unwrap(), args.clone());

                    instructions.append(move_instructions.as_mut());
                }

                instructions.push(AssemblyInstruction::Call(asm_name));
                
                for arg in args.clone().iter() {
                    architecture.delete_object(*arg);
                }

            }
        }
    }

    instructions
}
