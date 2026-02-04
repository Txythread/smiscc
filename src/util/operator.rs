use anyhow::anyhow;
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

    #[strum(to_string = "!=")]
    NotEquals,

    #[strum(to_string = "<")]
    LessThan,

    #[strum(to_string = "<=")]
    LessThanOrEqual,

    #[strum(to_string = ">")]
    GreaterThan,

    #[strum(to_string = ">=")]
    GreaterThanOrEqual,
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
            Operation::Equals => 0,
            Operation::NotEquals => 0,
            Operation::LessThan => 0,
            Operation::LessThanOrEqual => 0,
            Operation::GreaterThan => 0,
            Operation::GreaterThanOrEqual => 0,
        }
    }

    /// Gets the identity if possible (impossible on non-commutative operations).
    /// The identity is the element that can be applied to a number over an operator
    /// that will not change the number.
    #[inline]
    pub fn get_identity(&self) -> Result<i64, anyhow::Error> {
        if !self.is_commutative() { return Err(anyhow!("Identity only exists for commutative operations (in this program rn)")) }
        
        match self {
            Operation::Addition => Ok(0),
            Operation::Multiplication => Ok(1),

            _ => Err(anyhow!("Identity not specified although the operation is commutative on operation {:?}", self))
        }
    }
    
    /// Gets whether the operation is commutative, meaning the operands can be
    /// used in any order (x + y == y + x). or not (x / y != y / x).
    #[inline]
    pub fn is_commutative(&self) -> bool {
        use Operation as OP;
        matches!(self, OP::Addition | OP::Equals | OP::Multiplication | OP::NotEquals)
    }
}