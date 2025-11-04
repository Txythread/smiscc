// == Code Tokenization ===

/// ### Defines how code gets split into tokens and how those tokens are then classified.
///
/// These infos are used in both: [the splitter](crate::compiler::splitter::split) and
/// [the tokenizer (/classifier)](crate::compiler::tokenizer::tokenize).
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
    pub const UNIGNORED_SPLIT_CHARACTERS: [&str; 20] = ["{", "}", "(", ")", "[", "]", "<", ">", "?", "!", ".", ",", "\"", "\'", "=", "+", "-", "*", "/", "#"];


    /// ### Characters that prevent split characters from creating new tokens until their counterpart is met.
    ///
    /// This can be, for example, used for strings and characters.
    /// *So far, this is the only use coming to my mind, but more use cases might appear in the future.*
    ///
    /// **For Example:**
    /// `hello "world, what's up"` => `["hello", "\"", "world, what's up", "\""]` *when this list
    /// **does** include " as a such character*
    /// `hello "world, what's up"` => `["hello", "\"", "world", ",", "what", "\'", "s", "up", "\""]`
    /// **when this list **does not** include " as a such character*
    ///
    /// The list contains tuples (start_character, end_character).
    /// When .0 is found, the mode described above should be entered, when .1 is found,
    /// it should exit it when it is in that mode.
    ///
    /// This is used in the splitter
    pub const ESCAPE_PREVENTING_CHARACTERS: [(char, char); 2] = [STRING_MARKERS, ('\'', '\'')];


    /// ### Characters that will trigger a new logical line
    ///
    /// **Note**: Do not add newlines here. Those get handled separately and will always cause a new logical line.
    /// If you wish to change that behaviour, please take a look at [splitter.rs](crate::compiler::splitter::split) a
    /// nd figure that out yourself.
    ///
    /// **Note**: Characters listed here won't be part of the splitter's result
    /// in either of the lines (except if defined otherwise by other config constants,
    /// though you should not add the same character to the ignored splitting characters.).
    pub const NEW_LOGICAL_LINE_CHARACTERS: [&str; 3] = [";", "{", "}"];


    /// ### Keyword creating an unmodifiable "variable"
    ///
    /// The word below marks an **unmodifiable** "variable"/"constant"
    /// **within a function** in the language.
    /// This is equivalent to Rust's `let` without `mut` or Swift's `let`
    /// statement.
    ///
    /// The keyword for creating a modifiable variable in the language
    /// is defined [here](MODIFIABLE_OBJECT_DECLARATION_KEYWORD).
    pub const UNMODIFIABLE_OBJECT_DECLARATION_KEYWORD: &str = "let";


    /// ### Keyword creating a modifiable variable
    ///
    /// The word below marks an **modifiable** variable **within the code**
    /// This is equivalent to Rust's `let mut` or Swift's `let` statement.
    ///
    /// **Note:** Only pass things that will get parsed as a single token
    /// here. So no `let mut` statements are allowed here with normal parsing.
    pub const MODIFIABLE_OBJECT_DECLARATION_KEYWORD: &str = "var";

    /// ### Logical Parentheses
    ///
    /// The characters that will be treated like parentheses in the tokenizer.
    ///
    /// **Note:** Don't use the same character here twice as that would lead
    /// to undefined & unpredictable behaviour.
    pub const LOGICAL_PARENTHESES: [(char, char); 4] = [('{', '}'), ('(', ')'), ('[', ']'), ('<', '>')];

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
    /// [splitter](crate::compiler::splitter::split) as a single string,
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
    }

    /// ### Names of the Boolean States
    ///
    /// Those are usually "true" & "false" or "yes" & "no".
    /// The first state maps to a physical "1", the second to a "0".
    ///
    /// **Note to future self:** Add "maybe" state like in
    /// [DreamBerd/Gulf of Mexico](https://github.com/TodePond/GulfOfMexico).
    pub const BOOL_STATE_NAMES: (&str, &str) = ("true", "false");


    ////////////////////////////////////////////////
    ///////////////     Operators    ///////////////
    ////////////////////////////////////////////////
    //
    // Note that operators are defined in
    // crate::util::operator under src/util/operator.rs.
    // There, the symbol for addition, subtraction,
    // multiplication, ..., can be changed.
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
    use crate::compiler::data_types::IntegerType;

    /// ### The integer type used for addresses
    ///
    /// This should usually be an unsigned number with the maximal
    /// amount of bits the architecture allows.
    ///
    /// **Note:** Don't use the address type here, as this will
    /// lead to an infinite recursion.
    pub const ADDRESS_INTEGER_TYPE: IntegerType = IntegerType::Unsigned32BitInteger;
}