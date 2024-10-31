use std::collections::{BTreeMap, HashSet};

use crate::semantic_store::{ModuleId, TypeDecl};

#[cfg(test)]
mod test_type_store;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct TypeId {
    module_id: ModuleId,
    name: String,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Type {
    name: String,
    fields: Vec<(String, TypeRef)>,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum TypeRef {
    PrimitiveType(PrimitiveType),
    TypeId(TypeId),
    Unknown,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum PrimitiveType {
    Number,
    String,
}

#[derive(Debug, Default)]
pub struct TypeStore {
    types: BTreeMap<TypeId, Type>,
}

impl TypeStore {
    pub fn infer_type_decl(&mut self, type_decl: &TypeDecl) -> Type {
        let mut names = HashSet::<&String>::new();

        Type {
            name: type_decl.name.clone(),
            fields: type_decl
                .fields
                .iter()
                .filter_map(|field| {
                    if names.contains(&field.name) {
                        None
                    } else {
                        names.insert(&field.name);

                        match field.field_type {
                            None => Some((field.name.clone(), TypeRef::Unknown)),
                            Some(ref type_expr) => Some((
                                field.name.clone(),
                                TypeRef::PrimitiveType(PrimitiveType::String),
                            )),
                            _ => unimplemented!(),
                        }
                    }
                })
                .collect(),
        }
    }
}
