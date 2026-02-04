use derive_new::new;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumIter};
use uuid::Uuid;
use crate::compiler::context::Context;

#[derive(Debug, Clone, new)]
pub struct FunctionMeta {
    pub code_name: String,
    pub assembly_name: String,
    pub assembly_style: FunctionStyle,
    pub return_type_uuid: Option<Uuid>,
    pub arguments: Vec<FunctionArgument>,
}


#[derive(Debug, Clone, new, Serialize, Deserialize)]
pub struct FunctionMetaJson {
    pub code_name: String,
    pub assembly_name: String,
    pub assembly_style: String,
    pub return_type_name: String,
    pub arguments: Vec<FunctionArgumentJson>,
}

impl FunctionMetaJson {
    pub fn to_function_meta(&self, context: &Context) -> FunctionMeta {
        let style = match self.assembly_style.as_str() {
            "C" => FunctionStyle::C,
            "SMISC" => FunctionStyle::Smisc,
            _ => panic!("Unknown assembly style: {}", self.assembly_style.as_str()),
        };
        
        let arguments = self.arguments.iter().map(|x|x.to_function_argument(context)).collect();
        
        let mut return_type_uuid = None;
        
        if !self.return_type_name.is_empty() {
            for type_ in context.datatypes.iter() {
                if type_.1.name == self.return_type_name {
                    return_type_uuid = Some(type_.0.clone());
                    break;
                }
            }
            
            if return_type_uuid.is_none() {
                panic!("Type not found: {}", self.return_type_name);
            }
        }
        
        FunctionMeta::new(self.code_name.clone(), self.assembly_name.clone(), style, return_type_uuid, arguments)
    }
}

#[derive(Debug, Clone, new)]
pub struct FunctionArgument {
    pub name: Option<String>,
    pub own_uuid: Uuid,
    pub type_uuid: Uuid,
}

#[derive(Debug, Clone, new, Serialize, Deserialize)]
pub struct FunctionArgumentJson {
    pub name: String,
    pub type_name: String,
}

impl FunctionArgumentJson {
    pub fn to_function_argument(&self, context: &Context) -> FunctionArgument {
        let type_uuid = context.datatypes.iter().find(|x|x.1.name == self.type_name).unwrap().0;

        let name = if self.name == "_" { None } else { Some(self.name.clone()) };

        FunctionArgument::new(name, Uuid::new_v4(), *type_uuid)
    }
}

#[derive(Debug, Clone, EnumIter, AsRefStr)]
pub enum FunctionStyle {
    #[strum(serialize = "C")]
    C,
    #[allow(dead_code)]
    #[strum(serialize = "SMISC")]
    Smisc
}
