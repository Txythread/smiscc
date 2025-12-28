use uuid::Uuid;

pub struct FunctionMeta {
    pub code_name: String,
    pub assembly_name: String,
    pub assembly_style: FunctionStyle,
    pub return_type_uuid: Uuid,
    pub arguments: Vec<FunctionArgument>,
}

pub struct FunctionArgument {
    pub name: Option<String>,
    pub type_uuid: Uuid,
}

pub enum FunctionStyle {
    C,
    Smisc
}
