/// Either a reference to one or a value and information
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



#[derive(Clone, Debug)]
pub struct Trait{
    name: String,
}
