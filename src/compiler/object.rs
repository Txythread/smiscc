use uuid::Uuid;

/// about what can be done with it.
#[derive(Clone, Debug)]
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


/// Contains relevant information for syntax checking an object
#[derive(Clone)]
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
}



/// Something that might be applied to an [object](Object) to give
/// it some properties (like arithmetic, etc.)
#[derive(Clone, Debug)]
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

    /// The object is a reference to an object of the uuid (reference:uuid)
    pub const REFERENCE_TYPE: &str = "reference:";

    /// The object has a size and can be moved to memory etc.
    ///
    /// **Note:** Size in bytes.
    pub const SIZED: &str = "sized:";
}