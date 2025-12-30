use uuid::Uuid;
use crate::compiler::data_types::data_types::Buildable;
use crate::compiler::line_map::{DisplayCodeInfo, DisplayCodeKind, LineMap, NotificationInfo, TokenPosition};
use crate::compiler::tokenization::token::Token;

/// about what can be done with it.
#[derive(Clone, Debug, PartialEq)]
pub struct Object{
    /// The universal unique identifier of the [object type](ObjectType).
    pub type_uuid: Uuid,

    /// The long name of the variable (full path)
    pub name: String,

    /// If the object in question is constant,
    /// it's value should be in here. This might
    /// originate from a simple integer (`5`) in code.
    pub initial_content: Option<i128>,
}

impl Object {
    pub fn new(type_uuid: Uuid, name: String, initial_content: Option<i128>) -> Object {
        Object { name, type_uuid, initial_content }
    }
}

/// ### Generates objects if possible
///
/// This is done using the ["Buildable" trait](Buildable), which
/// works by exposing both the struct implementing the trait and a
/// one-time generated ObjectType. The tokens can usually be left
/// unclassified for simple objects and the line map and the line
/// number are required for producing token positions.
///
/// The resulting token will always contain an object.
pub fn generate_object<T: Buildable + ?Sized>(tokens: &mut Vec<Token>, object_types: Vec<(ObjectType, Box<T>)>, line_map: &mut LineMap, line_number: u32,  first_token_index: u32, last_token_index: u32) -> Option<Object> {
    // Where ambiguous results get stored for later.
    // If there is more than one element in here in the end,
    // that's an error that should be displayed (except when
    // there's an explicit one in there, than that one should
    // be selected).
    let mut successful_ambiguous_results: Vec<Object> = Vec::new();

    // Where explicit results get stored for later.
    // If there is more than one element in here in the end,
    // that's an error that should be displayed in the end.
    // If there are none, then successful_ambiguous_results
    // should be used instead.
    let mut successful_explicit_results: Vec<Object> = Vec::new();

    // Calculate the position of the resulting token
    let result_start_pos = tokens[0].get_position().start;
    let last_token_pos = tokens.last().unwrap().get_position();
    let result_end_pos = last_token_pos.start + last_token_pos.length;
    let result_length = result_end_pos - result_start_pos;
    let _result_position = TokenPosition::new(result_start_pos, result_length);


    // Was geht ab in Rumänien?

    for object_type in object_types.iter().clone() {
        let object_type = object_type;
        let parent_type = object_type.0.clone();
        let build_result = object_type.1.build(tokens.clone(), parent_type);

        if build_result.ambiguous {
            if build_result.result.is_ok() {
                successful_ambiguous_results.push(build_result.result.unwrap());
            }
        } else {
            if build_result.result.is_ok() {
                successful_explicit_results.push(build_result.result.unwrap());
            } else {
                // The datatype was explicitly requested by the code but
                // couldn't be built. Throw an error.
                let error_info = DisplayCodeInfo::new(
                    line_number,
                    first_token_index,
                    last_token_index as i32,
                    vec![
                        format!("**note:** this couldn't be parsed as a {}", build_result.result.clone().err().unwrap().expected_object)
                    ],
                    DisplayCodeKind::InitialError
                );

                let error = NotificationInfo::new(
                    "Failed to Decode Object".to_string(),
                    build_result.result.err().unwrap().message.clone(),
                    vec![error_info],
                );

                line_map.display_error(error);
            }
        }
    }

    // All objects have had a chance to build their versions.
    // Now, return whatever is available.
    if successful_explicit_results.len() > 0 {
        if successful_explicit_results.len() != 1 {
            // There are too many results all claiming to be unambiguous.
            // The datatype was explicitly requested by the code but
            // couldn't be built. Throw an error.
            let error_info = DisplayCodeInfo::new(
                line_number,
                first_token_index,
                last_token_index as i32,
                vec![],
                DisplayCodeKind::InitialError
            );

            let mut message = String::from("There are multiple ways to decode this object: ");

            for result in successful_explicit_results.clone() {
                let type_uuid = result.type_uuid;
                let type_with_uuid = object_types.iter().filter(|&x| x.0.type_uuid == type_uuid).collect::<Vec<&(ObjectType, Box<T>)>>().iter().nth(0).unwrap().0.clone();
                message += type_with_uuid.name.clone().as_str();
                message += "  ";
            }

            let error = NotificationInfo::new(
                "Statement Ambiguous".to_string(),
                message,
                vec![error_info],
            );

            line_map.display_error(error);

        } else {
            //let token = Token::Object(successful_explicit_results[0].clone(), result_position);
            return Some(successful_explicit_results[0].clone());
        }
    }

    // All objects have had a chance to build their versions.
    // Now, return whatever is available.
    if successful_ambiguous_results.len() > 0 {
        if successful_ambiguous_results.len() != 1 {
            // There are too many results all claiming to be unambiguous.
            // The datatype was explicitly requested by the code but
            // couldn't be built. Throw an error.
            let error_info = DisplayCodeInfo::new(
                line_number,
                first_token_index,
                last_token_index as i32,
                vec![],
                DisplayCodeKind::InitialError
            );

            let mut message = String::from("There are multiple ways to decode this object: ");

            for result in successful_ambiguous_results {
                let type_uuid = result.type_uuid;
                let type_with_uuid = object_types.iter().filter(|&x| x.0.type_uuid == type_uuid).collect::<Vec<&(ObjectType, Box<T>)>>().iter().nth(0).unwrap().0.clone();
                message += type_with_uuid.name.clone().as_str();
                message += "  ";
            }

            let error = NotificationInfo::new(
                "Statement Ambiguous".to_string(),
                message,
                vec![error_info],
            );

            line_map.display_error(error);

        } else {
            //let token = Token::Object(successful_ambiguous_results[0].clone(), result_position);
            //return Some(token)
            return Some(successful_ambiguous_results[0].clone())
        }
    }

    None
}


/// Contains relevant information for syntax checking an object
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectType{
    /// The name of the object. For example "u32" or "ObjectType".
    pub name: String,

    /// What the object can do.
    /// For example: addition.
    pub traits: Vec<Trait>,

    pub type_uuid: Uuid,
}


impl ObjectType {
    pub fn new(name: String, type_uuid: Uuid) -> ObjectType {
        ObjectType { name, traits: vec![], type_uuid }
    }

    pub fn add_trait(&mut self, trait_: &str) {
        self.traits.push(Trait { name: trait_.to_string().clone() });
    }

    /// Add a trait with at least one argument.
    /// The name of the trait should end with a colon.
    pub fn add_complex_trait(&mut self, trait_: &str, trait_args: Vec<String>){
        self.traits.push(Trait { name: trait_.to_string().clone() + trait_args.join(":").as_str()});
    }

    /// Whether this ObjectType has a trait with the given name
    /// and it's arguments if any.
    pub fn get_trait(&self, trait_: &str) -> Option<Vec<String>> {
        for test_trait in self.traits.iter() {
            if let Some(arguments) = test_trait.name.strip_prefix(trait_) {
                return Some(arguments.split(':').map(|x| x.to_string()).collect::<Vec<String>>())
            }
        }

        None
    }

    /// Uses [get_trait](Self::get_trait) to check whether the given trait is present
    pub fn has_trait(&self, trait_: &str) -> bool {
        self.get_trait(trait_).is_some()
    }
}



/// Something that might be applied to an [object](Object) to give
/// it some properties (like arithmetic, etc.)
#[derive(Clone, Debug, PartialEq)]
pub struct Trait{
    name: String,
}

impl Trait{
    /// Means that the object the trait belongs to can be used for
    /// arithmetic operations (+, -, *, /, etc.) (regardless of being a reference or a
    /// value type)
    pub const ARITHMETIC_COMPATIBLE: &str = "arithmetic";

    /// Means that the object the trait belongs to is a direct value, not a reference.
    pub const VALUE_TYPE: &str = "direct_value";

    /// Means that the type can be interpreted as containing a boolean value.
    ///
    /// Bool Comparison:
    /// `X==0` => `false`
    /// `X!=0 && X%2==1` => `true`
    pub const BOOLEAN_COMPATIBLE: &str = "boolean";

    /// The object is a reference to an object of the uuid (reference:uuid)
    pub const REFERENCE_TYPE: &str = "reference:";

    /// The object has a size and can be moved to memory etc.
    ///
    /// **Note:** Size in bytes.
    pub const SIZED: &str = "sized:";

    /// Marks the type as being some kind of integer.
    /// **Note:** This alone won't make the type accept arithmetic operations.
    pub const INTEGER: &str = "integer";
}