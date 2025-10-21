pub const ABC: [dyn Integer; 2] = [Signed8BitInteger, Signed16BitInteger];


/// A trait that any basic integer type in the language should implement.
pub trait Integer{
    /// ### Get the maximum value that the integer is allowed to contain.
    ///
    /// Right now, no checks are performed. This might, however, be changed
    /// in the future.
    ///
    /// For an u32, this would be 2^32 - 1.
    /// For an i32, this would be 2^31 - 1.
    fn get_upper_bound() -> u64;

    /// ### Get the minimum value that the integer is allowed to contain.
    ///
    /// Right now, no checks regarding over- or underflows are implemented.
    /// This might, however, be changed in the future.
    ///
    /// For an u32, this would be 0.
    /// For an i32, this would be 2^31.
    ///
    /// **Note:** The number returned here is always positive or zero.
    /// The bound is inverted when checking.
    fn get_lower_bound() -> u64;


    /// ### The name used for errors & warnings.  
    ///
    /// This should usually correlate with
    /// [get_code_name](Integer::get_code_name)
    fn get_display_name() -> String;

    /// ### The name that refers to the type in the user's code
    ///
    /// **Note:** This should correlate with 
    /// [get_display_name](Integer::get_display_name)
    /// in most cases.
    fn get_code_name() -> String;


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
    fn get_memory_length() -> u8;
}


// === Integer Implementations ===
//
// Contains all integer datatypes, though they have to be linked
// as a default datatype in the config file.


pub struct Signed32BitInteger;

impl Integer for Signed32BitInteger {
    fn get_upper_bound() -> u64 { 0x7f_ff_ff_ff}
    fn get_lower_bound() -> u64 { 0x80_00_00_00 }
    fn get_display_name() -> String { String::from("i32") }
    fn get_code_name() -> String { String::from("i32") }
    fn get_memory_length() -> u8 { 4 }
}

pub struct Unsigned32BitInteger;

impl Integer for Unsigned32BitInteger {
    fn get_upper_bound() -> u64 { 0xff_ff_ff_ff }
    fn get_lower_bound() -> u64 { 0x00_00_00_00 }
    fn get_display_name() -> String { String::from("u32") }
    fn get_code_name() -> String { String::from("u32") }
    fn get_memory_length() -> u8 { 4 }
}

pub struct Signed16BitInteger;

impl Integer for Signed16BitInteger {
    fn get_upper_bound() -> u64 { 0x7f_ff }
    fn get_lower_bound() -> u64 { 0x80_00 }
    fn get_display_name() -> String { String::from("i16") }
    fn get_code_name() -> String { String::from("i16") }
    fn get_memory_length() -> u8 { 2 }
}

pub struct Unsigned16BitInteger;

impl Integer for Unsigned16BitInteger {
    fn get_upper_bound() -> u64 { 0xff_ff }
    fn get_lower_bound() -> u64 { 0x00_00 }
    fn get_display_name() -> String { String::from("u16") }
    fn get_code_name() -> String { String::from("u16") }
    fn get_memory_length() -> u8 { 2 }
}

pub struct Signed8BitInteger;

impl Integer for Signed8BitInteger {
    fn get_upper_bound() -> u64 { 0x7f }
    fn get_lower_bound() -> u64 { 0x80 }
    fn get_display_name() -> String { String::from("i8") }
    fn get_code_name() -> String { String::from("i8") }
    fn get_memory_length() -> u8 { 1 }
}

pub struct Unsigned8BitInteger;
impl Integer for Unsigned8BitInteger {
    fn get_upper_bound() -> u64 { 0xff }
    fn get_lower_bound() -> u64 { 0x00 }
    fn get_display_name() -> String { String::from("u8") }
    fn get_code_name() -> String { String::from("u8") }
    fn get_memory_length() -> u8 { 1 }
}