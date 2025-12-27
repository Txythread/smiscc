use derive_new::new;

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