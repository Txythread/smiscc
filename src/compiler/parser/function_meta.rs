use derive_new::new;
use uuid::Uuid;

#[derive(Debug, Clone, new)]
pub struct FunctionMeta {
    pub code_name: String,
    pub assembly_name: String,
    pub assembly_style: FunctionStyle,
    pub return_type_uuid: Option<Uuid>,
    pub arguments: Vec<FunctionArgument>,
}

#[derive(Debug, Clone, new)]
pub struct FunctionArgument {
    pub name: Option<String>,
    pub type_uuid: Uuid,
}

#[derive(Debug, Clone)]
pub enum FunctionStyle {
    C,
    Smisc
}
