// == Code Tokenization ===

/// ### Defines how code gets split into tokens and how those tokens are then classified.
///
/// These infos are used in both: [the splitter](crate::compiler::trimmer::split) and
/// [the tokenizer (/classifier)](crate::compiler::tokenization::tokenizer::tokenize).
pub mod tokenization_options {
    use crate::util::math::Base;
    use strum_macros::{AsRefStr, EnumIter, EnumString};
    

    /// ### Characters that split tokens but are not themselves supposed to appear in the result
    ///
    /// For example:
    /// `hello world` => `["hello", "world"]` when " " is an ignored split character, but
    /// `hello world` => ``["hello", " ", "world"]` when " " is an unignored split character.
    ///
    /// **Note**: When the character is in ignored split characters, no empty string will
    /// appear even if the character is passed multiple times
    /// (`hello        world` will still get processed into `["hello", "world"]`).
    ///
    /// This is used in the splitter.
    pub const IGNORED_SPLIT_CHARACTERS: [&str; 2] = [" ", "\t"];


    /// ### Characters that split tokens and are supposed to appear as a separate token
    ///
    /// For example:
    /// `hello world` => `["hello", "world"]` when " " is an **ignored split character**, but
    /// `hello world` => ``["hello", " ", "world"]` when " " is an **unignored split character**.
    ///
    /// **Note**: When the character occurs multiple times in a row, the character will be treated
    /// as a new token every time.
    /// appear even if the character is passed multiple times
    /// (`hello+++world` will still get processed into `["hello", "+", "+", "+", "world"]`).
    ///
    /// This is used in the splitter.
    pub const UNIGNORED_SPLIT_CHARACTERS: [&str; 23] = ["{", "}", "(", ")", "[", "]", "<", ">", "?", "!", ".", ",", "\"", "\'", "=", "+", "-", "*", "/", "#", "\n", ";", ":"];


    /// ### Character that separetes stuff
    /// 
    /// The character that separates things like function arguments, array items, etc.
    pub const SEPARATOR_CHARACTER: &str = ",";


    /// ### Logical Parentheses
    ///
    /// The characters that will be treated like parentheses in the tokenizer.
    ///
    /// **Note:** Don't use the same character here twice as that would lead
    /// to undefined & unpredictable behaviour.
    #[allow(dead_code)]
    pub const LOGICAL_PARENTHESES: [(&str, &str); 4] = [CODE_BLOCK_PARENTHESES, ARITHMETIC_PARENTHESES, ARRAY_PARENTHESES, TEMPLATE_PARENTHESES];


    /// ### Code Block Parentheses
    ///
    /// ... are parentheses defining the start (.0) and end (.1) of a code block.
    /// In most languages, this correlates with "{" and "}".
    #[allow(dead_code)]
    pub const CODE_BLOCK_PARENTHESES: (&str, &str) = ("{", "}");

    /// ### Array Parentheses
    ///
    /// Those parentheses do everything related with arrays. Whether that be defining
    /// one, indexing one, etc.
    /// In most languages, this correlates with "\[" and "\]"
    #[allow(dead_code)]
    pub const ARRAY_PARENTHESES: (&str, &str) = ("[", "]");

    /// ### Arithmetic Parentheses
    ///
    /// Those parentheses are used for **arithmetic** and **everything that doesn't have an
    /// own type of parentheses**.
    /// In most languages, this correlates with "(" and ")".
    pub const ARITHMETIC_PARENTHESES: (&str, &str) = ("(", ")");

    /// ### Template Parentheses
    ///
    /// ... do exactly what their name suggests. Correlates to "<" & ">" in most languages.
    #[allow(dead_code)]
    pub const TEMPLATE_PARENTHESES: (&str, &str) = ("<", ">");

    /// ### String Markers
    ///
    /// The tokens below can start (0) or end (1) a string in the user's code.
    ///
    /// **Note:** Those can be the same.
    pub const STRING_MARKERS: (char, char) = ('\"', '\"');


    /// ### Integer Bases
    ///
    /// Determines which [bases](Base) are known to the
    /// [conversion function](crate::util::math::convert_to_int).
    ///
    /// **Note:** Bases should be defined in implementations of the
    /// "Base" structure. Avoid defining bases here, as it clutters
    /// the file. If a datatype has no necessary prefix (like decimal),
    /// it has to be listed in the end of the list.
    pub const BASES: [Base; 4] = [Base::BINARY, Base::OCTAL, Base::HEXADECIMAL, Base::DECIMAL];


    /// Holds all keywords that, when split by the
    /// [splitter](crate::compiler::trimmer::split) as a single string,
    /// should be classified separately.
    /// #[derive(EnumString, AsRefStr, Debug)]
    ///
    /// **Note:** This is **not** a place for values such as booleans, numbers, etc.
    #[derive(AsRefStr, Clone, Debug, EnumString, EnumIter, PartialEq)]
    pub enum Keyword {
        /// ### Creates an Unmodifiable "Variable"
        ///
        /// This is similar to Swift's "let"-statement & Rust's "let"
        /// without mod. The action allocates data on the heap and can
        /// only be performed inside a function.
        #[strum(serialize = "let")]
        Let,

        /// ### Creates a Modifiable Variable
        ///
        /// This is similar to Swift's "var"-statement & Rust's "let mut".
        /// The action allocates data on the heap and can only be performed
        /// inside a function.
        #[strum(serialize = "var")]
        Var,
        
        /// ### Exit the Current Process
        /// 
        /// This is similar to the "exit"-statements in scripting languages,
        /// such as most shell languages. This action requires a value.
        #[strum(serialize = "exit")]
        Exit,

        /// ### Define a New Function
        ///
        /// This is similar to the "func", "fn" or "function" statements in most
        /// languages.
        #[strum(serialize = "func")]
        Function,


        /// ### Modify Assembly Output
        ///
        /// This is similar to Rust's `extern` to the extent that it allows for
        /// an ABI to be chosen, but it also expects the assembly name.
        ///
        /// For example: `extern "C" "_start"`
        #[strum(serialize = "extern")]
        Extern,

        /// ### Control Execution Flow
        ///
        /// This is just a normal fucking if statement as you might see it in any
        /// other language.
        ///
        /// *Technical Info:*
        /// Any code within the if-clause is basically a part of the if-block, as
        /// in its stack and its value are part of said block.
        /// Lines within the header of the statement must produce boolean outputs
        /// and are connected using either **`&&`**, **`||`** or **`^^`**.
        /// The last two of those have the same priority but the '&&' has a higher
        /// one. When there are two items of the same priority, those can are processed
        /// from left to right.
        #[strum(serialize = "if")]
        If,
    }


    /// ### Assignment Operator ("=")
    ///
    /// This is equivalent to a single "=" in basically every programming language.
    pub const ASSIGNMENT_OPERATION: &str = "=";

    /// ### Names of the Boolean States
    ///
    /// Those are usually "true" & "false" or "yes" & "no".
    /// The first state maps to a physical "1", the second to a "0".
    ///
    /// **Note to future self:** Add "maybe" state like in
    /// [DreamBerd/Gulf of Mexico](https://github.com/TodePond/GulfOfMexico).
    pub const BOOL_STATE_NAMES: (&str, &str) = ("true", "false");


    ////////////////////////////////////////////////////////
    ///////////////////     Operators    ///////////////////
    ////////////////////////////////////////////////////////
    //                                                    //
    // Note that operators are defined in                 //
    // crate::util::operator under src/util/operator.rs.  //
    // There, the symbol for addition, subtraction,       //
    // multiplication, ..., can be changed.               //
    //                                                    //
    ////////////////////////////////////////////////////////
}


pub mod misc {
    /// ### Characters That Will Be Ignored When Reading Integers
    ///
    /// For readability purposes, the user might want to insert underscores
    /// or other characters when writing integers.
    ///
    /// All characters in this array will be ignored when trying to build
    /// an integer via [this function](crate::util::math::convert_to_int)
    pub const INTEGER_CONVERSION_IGNORED_CHARACTERS: [char; 1] = ['_'];
}


/// ### Options in the language that are likely to vary by target
///
/// This includes options such as which integer numbers are allowed,
/// which ones are standard, etc.
pub mod target {
    use crate::compiler::data_types::integer::IntegerType;

    /// ### The integer type used for addresses
    ///
    /// This should usually be an unsigned number with the maximal
    /// amount of bits the architecture allows.
    ///
    /// **Note:** Don't use the address type here, as this will
    /// lead to an infinite recursion.
    pub const ADDRESS_INTEGER_TYPE: IntegerType = IntegerType::Unsigned32BitInteger;
}