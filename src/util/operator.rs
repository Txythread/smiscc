use strum_macros::{EnumString, EnumIter, AsRefStr};
use strum::IntoEnumIterator;

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

    #[strum(to_string = "==")]
    Equals,
}


impl Operation {
    /// Whether the resulting value is of boolean type.
    /// If not, it should be the type of the object this is performed on.
    pub fn is_boolean(&self) -> bool {
        match self {
            Operation::Equals => true,
            _ => false,
        }
    }
}