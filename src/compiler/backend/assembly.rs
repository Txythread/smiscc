pub enum AssemblyInstruction {
    /// Copy the contents of one register into the other one.  
    /// The first register is the target, the second one contains the data.
    MoveReg(u8, u8),
    /// Copy the data into the given register.
    MoveImm(u8, i64),
    
    /// Load (2) bytes of data at address at register (1) into register (0)
    Load(u8, u8, u8),
    /// Store (2) bytes of data from register (0) into the address at register (1)
    Store(u8, u8, u8),
    
    /// Adds the contents of the second register to the first register's contents
    AddReg(u8, u8),
    /// Adds the data to the register's contents
    AddImm(u8, i64),
}

impl AssemblyInstruction {
    pub fn into_asm(&self) -> String {
        todo!()
    }
}