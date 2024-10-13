use std::{collections::BTreeMap, default};

use boring_rpc_syn::TextRange;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Default)]
pub struct ModuleId(String);

impl ModuleId {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Default, Clone)]
pub enum TypeDeclKind {
    #[default]
    Type,
    Service,
}

#[derive(Hash, Eq, PartialEq, Debug, Default, Clone)]
pub struct TypeDecl {
    pub name: String,
    pub kind: TypeDeclKind,

    pub range: TextRange,
}

#[derive(Debug, Default)]
pub struct Module {
    pub module_id: ModuleId,
    pub type_decls: BTreeMap<String, TypeDecl>,
}

#[derive(Debug, Default)]
pub struct Store {
    modules: BTreeMap<ModuleId, Module>,
}
