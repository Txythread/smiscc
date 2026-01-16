use std::rc::Rc;
use uuid::Uuid;
use derive_new::new;
use crate::compiler::data_types::object::ObjectType;
use crate::compiler::line_map::LineMap;
use crate::compiler::parser::function_meta::FunctionArgument;
use crate::compiler::tokenization::token::Token;

/// Gets a datatype from the list of types and returns its uuid.
pub fn parse_datatype(tokens: Rc<Vec<Token>>, cursor: &mut usize, types: Rc<Vec<ObjectType>>, _: &mut LineMap) -> Uuid {
    if let Token::Identifier(type_name, _) = tokens[*cursor].clone() {
        *cursor += 1;

        return types.iter().find(|&x|x.name==type_name).unwrap().type_uuid
    }

    todo!("Expected datatype")
}

/// Generates a parameter with external and (if applicable) internal name
pub fn parse_parameter_descriptor(tokens: Rc<Vec<Token>>, cursor: &mut usize, types: Rc<Vec<ObjectType>>, line_map: &mut LineMap, parse_internal: bool) -> ParameterDescriptor {
    let name: Option<String>;
    let mut internal_name: Option<String> = None;
    let datatype: Option<Uuid>;

    if let Token::Identifier(name_, _) = tokens[*cursor].clone() {
        *cursor += 1;
        name = Some(name_);

        if parse_internal {
            internal_name = name.clone();
        }
    } else {
        todo!("Expected parameter name")
    }

    let cursor_backup = *cursor;
    *cursor += 1;
    match tokens[cursor_backup].clone() {
        Token::Colon(_) => {
            datatype = Some(parse_datatype(tokens.clone(), cursor, types.clone(), line_map));
        }

        Token::Identifier(name_, _) => {
            internal_name = Some(name_);


            if !matches!(tokens[*cursor].clone(), Token::Colon(_)) {
                todo!("Expected ':' in parameter descriptor")
            }

            *cursor += 1;

            datatype = Some(parse_datatype(tokens.clone(), cursor, types.clone(), line_map));
        }

        _ => todo!("Not expected in parameter descriptor"),
    }

    ParameterDescriptor::new(name, internal_name, datatype.unwrap())

}


#[derive(Clone, Debug, new)]
pub struct ParameterDescriptor {
    /// The external name or name of the parameter
    /// or none if it was omitted.
    pub name: Option<String>,

    /// The internal name of the parameter
    pub internal_name: Option<String>,

    pub datatype: Uuid,
}

impl ParameterDescriptor {
    pub fn generate_function_argument(&self) -> FunctionArgument {
        FunctionArgument::new(self.name.clone(), self.datatype, Uuid::new_v4())
    }
}