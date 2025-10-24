/// about what can be done with it.
#[derive(Clone, Debug)]
pub struct Object{
    /// The universal unique identifier of the [object type](ObjectType).
    type_uuid: u32,

    /// The long name of the variable (full path)
    name: String,

    /// If the object in question is constant,
    /// it's value should be in here. This might
    /// originate from a simple integer (`5`) in code.
    constant_content: Option<u64>,
}


/// Contains relevant information for syntax checking an object
pub struct ObjectType{
    /// The name of the object. For example "u32" or "ObjectType".
    name: String,

    /// What the object can do.
    /// For example: addition.
    traits: Vec<Trait>,

    type_uuid: u32,
}



/// Something that might be applied to an [object](Object) to give
/// it some properties (like arithmatic, etc.)
#[derive(Clone, Debug)]
pub struct Trait{
    name: String,
}

impl Trait{
    /// Means that the object the trait belongs to can be used for
    /// arithmatic operations (+, -, *, /, etc.) (regardless of being a reference or a
    /// value type)
    pub const ARITHMATIC_COMPATIBLE = Trait { "arithmatic".to_string() };

    /// Means that the object the trait belongs to is a direct value, not a reference.
    pub const VALUE_TYPE = Trait { "direct_value".to_string() };

    /// The object is an reference to an object of the uuid (reference:uuid)
    pub const REFERENCE_TYPE = Trait { "reference:".to_string() };
}
