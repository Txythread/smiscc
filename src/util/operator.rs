use strum_macros::{EnumString, EnumIter};
use strum::IntoEnumIterator;

// This file contains methods & definitions for arithmetic operators.

/// ### Arithmetic Operator
///
/// Any operator (+, -, /, *, ...) that is not responsible for setting a
/// value to another one
///
/// The operator can be used for both: Basic operations (e.g. adding two integers)
/// and complex ones (e.g. adding two strings)
#[derive(EnumString, EnumIter)]
pub enum Operator {
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

}