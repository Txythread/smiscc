use strum_macros::{EnumString, EnumIter, AsRefStr};

// This file contains methods & definitions for arithmetic operators.

/// ### Arithmetic Operator
///
/// Any operator (+, -, /, *, ...) that is not responsible for setting a
/// value to another one
///
/// The operator can be used for both: Basic operations (e.g. adding two integers)
/// and complex ones (e.g. adding two strings)
#[derive(EnumString, EnumIter, Clone, Debug, PartialEq, AsRefStr)]
pub enum Operation {
    /// Either add numbers if basic arithmetic is supported by the variable or use
    /// the software-defined overwrite for that.
    #[strum(to_string = "+")]
    Addition,

    #[strum(to_string = "-")]
    Subtraction,

    #[strum(to_string = "*")]
    Multiplication,

    #[strum(to_string = "/")]
    Division,

    #[strum(to_string = "%")]
    Modulo,

    /// Equal (==) in comparisons only
    #[strum(to_string = "==")]
    Equals,
}


impl Operation {
    /// Whether the resulting value is of boolean type.
    /// If not, it should be the type of the object this is performed on.
    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Equals)
    }

    /// A value determining the importance of the operation (like PEMDAS).
    /// A higher value indicates higher significance.
    pub fn get_operation_order(&self) -> u8 {
        match self {
            Operation::Addition => 1,
            Operation::Subtraction => 1,
            Operation::Multiplication => 2,
            Operation::Division => 2,
            Operation::Modulo => 2,
            Operation::Equals => 3,
        }
    }
    
    /// Gets whether the operation is commutative, meaning the operands can be
    /// used in any order (x + y == y + x). or not (x / y != y / x).
    pub fn is_commutative(&self) -> bool {
        match self {
            Operation::Addition | Operation::Multiplication | Operation::Equals => true,
            Operation::Subtraction | Operation::Division | Operation::Modulo => false,
        }
    }
}