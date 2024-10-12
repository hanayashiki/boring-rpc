use std::collections::BTreeMap;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct ModuleId(String);

impl ModuleId {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}

#[derive(Debug)]
pub struct Module {
    pub(crate) module_id: ModuleId,
}

#[derive(Debug, Default)]
pub struct Store {
    modules: BTreeMap<ModuleId, Module>,
}
