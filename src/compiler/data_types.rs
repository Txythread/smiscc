/// A trait that any basic integer type in the language should implement.
pub trait Integer{
    /// ### Get the maximum value that the integer is allowed to contain.
    ///
    /// Right now, no checks are performed. This might, however, be changed
    /// in the future.
    ///
    /// For a u32, this would be 2^32 - 1.  
    /// For a i32, this would be 2^31 - 1.
    fn get_upper_bound() -> u64;

    /// ### Get the minimum value that the integer is allowed to contain.
    ///
    /// Right now, no checks regarding over- or underflows are implemented.
    /// This might, however, be changed in the future.
    ///
    /// For a u32, this would be 0.  
    /// For a i32, this would be 2^31.
    ///
    /// **Note:** The number returned here is always positive or zero.
    /// The bound is inverted when checking.
    fn get_lower_bound() -> u64;


    /// ### The name used for errors & warnings.  
    ///
    /// This should usually correlate with
    /// [get_code_name](crate::compiler::data_types::Integer::get_code_name)
    fn get_display_name() -> String;

    /// ### The name that refers to the type in the user's code
    ///
    /// **Note:** This should correlate with 
    /// [get_display_name](crate::compiler::data_types::Integer::get_display_name)
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
