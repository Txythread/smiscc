use crate::compiler::object::{Object, ObjectType};
use crate::compiler::tokenizer::Token;
use crate::config::target::*;


/// ### Generates data types via tokens.
/// A struct that makes a data type able to generate
/// instances of itself from code in the form of an
/// [object](Object).
/// This also means that it has to be able to build
/// an [object type](ObjectType) that describes what
/// it can do and is referenced by all its children
/// (the objects) by its uuid.
pub trait Buildable {
    /// ### Create an object with tokens
    ///
    /// Try to create an [object](Object) given some tokens,
    /// look if this data type was explicitly requested by the
    /// user (i.g. `25u8`) or not (i.g. `25`). And generate an
    /// error if it fails regardless of it should've been built
    /// in the first place. The message should only be shown if
    /// `ambiguous` (in the [result](BuildResult) is set to
    /// false.
    ///
    /// **Note:** The originally generated object type needs to
    /// be re-supplied as the uuid is required again. Don't
    /// re-generate it for this purpose, as the uuid might differ.
    fn build(&self, tokens: Vec<Token>, parent_type: ObjectType) -> BuildResult;

    /// ### Create an object type
    ///
    /// Create an [object type](ObjectType), which is necessary
    /// for building instances of the type.
    fn build_type(&self) -> ObjectType;
}


/// The result after trying to generate a [buildable](Buildable) object.
pub struct BuildResult {
    /// ### The Resulting object or an error
    ///
    /// An error is not necessarily negative.
    /// If the result is ambiguous, an error might not
    /// be displayed at all.
    pub result: Result<Object, ObjectBuildingError>,


    /// ### If it is this data type for sure.
    ///
    /// If there is no other way to way of building an object,
    /// it's clear from the code that that's the correct
    /// interpretation, it's unambiguous, so to say, this is
    /// set to false. If building that object from that builder
    /// is possible, but not clearly specified, this is true.
    ///
    /// This might be set to unambiguous although no result
    /// exists when it should be that data type for sure,
    /// but is malformed.
    pub ambiguous: bool,
}


/// Contains information about an error that arose from trying
/// to build an object via the [buildable trait](Buildable)
pub struct ObjectBuildingError {
    /// The name of the object that should've been built.
    pub expected_object: String,

    /// The message displayed as an error
    pub message: String,
}



/// An enum that holds information about any basic integer type.
pub enum IntegerType{
    Unsigned8BitInteger,
    Signed8BitInteger,
    Unsigned16BitInteger,
    Signed16BitInteger,
    Unsigned32BitInteger,
    Signed32BitInteger,
    Address
}


impl IntegerType {
    /// ### The maximum value that the integer is allowed to contain.
    ///
    /// Right now, no checks are performed. This might, however, be changed
    /// in the future.
    ///
    /// For an u32, this would be 2^32 - 1.
    /// For an i32, this would be 2^31 - 1.
    pub fn get_upper_bound(&self) -> u64 {
        match self {
            IntegerType::Unsigned8BitInteger =>     0xff,
            IntegerType::Signed8BitInteger =>       0x7f,
            IntegerType::Unsigned16BitInteger =>    0xff_ff,
            IntegerType::Signed16BitInteger =>      0x7f_ff,
            IntegerType::Unsigned32BitInteger =>    0xff_ff_ff_ff,
            IntegerType::Signed32BitInteger =>      0x7f_ff_ff_ff,

            IntegerType::Address =>                 ADDRESS_INTEGER_TYPE.get_upper_bound(),
        }
    }

    /// ### The minimum value that the integer is allowed to contain.
    ///
    /// Right now, no checks regarding over- or underflows are implemented.
    /// This might, however, be changed in the future.
    ///
    /// For an u32, this would be 0.
    /// For an i32, this would be 2^31.
    ///
    /// **Note:** The number returned here is always positive or zero.
    /// The bound is inverted when checking.
    pub fn get_lower_bound(&self) -> u64 {
        match self {
            IntegerType::Signed8BitInteger =>   0x80,
            IntegerType::Signed16BitInteger =>  0x80_00,
            IntegerType::Signed32BitInteger =>  0x80_00_00_00,

            IntegerType::Address =>             ADDRESS_INTEGER_TYPE.get_lower_bound(),

            _ => /* Unsigned - no negatives */  0x0
        }
    }


    /// ### The name used for errors & warnings.
    ///
    /// This should usually correlate with
    /// [get_code_name](Integer::get_code_name)
    pub fn display_name(&self) -> String {
        // In this language, this is just the same as the code name.
        self.get_code_name()
    }

    /// ### The name that refers to the type in the user's code
    ///
    /// **Note:** This should correlate with
    /// [get_display_name](Integer::get_display_name)
    /// in most cases.
    pub fn get_code_name(&self) -> String {
        match self {
            IntegerType::Unsigned8BitInteger =>     "u8".to_string(),
            IntegerType::Signed8BitInteger =>       "i8".to_string(),
            IntegerType::Unsigned16BitInteger =>    "u16".to_string(),
            IntegerType::Signed16BitInteger =>      "i16".to_string(),
            IntegerType::Unsigned32BitInteger =>    "u32".to_string(),
            IntegerType::Signed32BitInteger =>      "i32".to_string(),

            IntegerType::Address =>                 ADDRESS_INTEGER_TYPE.get_code_name(),
        }
    }


    /// ### Amount of bytes
    ///
    /// The amount of bytes that need to be stored or loaded
    /// when an instance of a datatype is stored.
    ///
    /// #### Examples
    ///
    /// `u8`  | `i8`  -> 1
    /// `u16` | `i16` -> 2
    /// `u32` | `i32` -> 4
    /// `u64` | `i64` -> 8
    pub fn get_memory_size(&self) -> u8 {
        match self {
            IntegerType::Unsigned8BitInteger  | IntegerType::Signed8BitInteger =>   1,
            IntegerType::Unsigned16BitInteger | IntegerType::Signed16BitInteger =>  2,
            IntegerType::Unsigned32BitInteger | IntegerType::Signed32BitInteger =>  4,


            IntegerType::Address => ADDRESS_INTEGER_TYPE.get_memory_size(),
        }
    }
}