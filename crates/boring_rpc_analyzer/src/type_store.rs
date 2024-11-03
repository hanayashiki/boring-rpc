use std::{collections::{BTreeMap, HashSet}, slice::Iter};

use crate::semantic_store;

#[cfg(test)]
mod test_infer_type_decl;

#[cfg(test)]
mod test_infer_module;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct TypeId {
    module_id: semantic_store::ModuleId,
    name: String,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Type {
    pub name: String,
    pub fields: Vec<(String, TypeRef)>,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Module {
    types: Vec<Type>,
}

impl Module {
    pub fn iter_types(&self) -> Iter<'_, Type> {
        self.types.iter()
    }
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
    pub fn infer_type_decl(&mut self, type_decl: &semantic_store::TypeDecl) -> Type {
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

    pub fn infer_module(&mut self, module: &semantic_store::Module) -> Module {
        let mut names = HashSet::<&String>::new();

        Module {
            types: module
                .type_decls
                .iter()
                .map(|type_decl| {
                    if names.contains(&type_decl.name) {
                        None
                    } else {
                        names.insert(&type_decl.name);

                        Some(self.infer_type_decl(type_decl))
                    }
                })
                .filter_map(|x| x)
                .collect(),
        }
    }
}
