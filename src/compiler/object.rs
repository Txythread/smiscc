

#[derive(Clone, Debug)]
pub struct Object{
    object_uuid: u32,
    name: String,
    traits: Vec<Trait>
    
}



#[derive(Clone, Debug)]
pub struct Trait{
    name: String,
}
