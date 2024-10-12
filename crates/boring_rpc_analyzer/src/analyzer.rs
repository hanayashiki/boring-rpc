use boring_rpc_parser::parser::Parser;
use boring_rpc_syn::{nodes, SyntaxNode};

use crate::store::{Module, ModuleId, Store};

#[cfg(test)]
mod test_analyzer;

pub fn analyze_module(store: &Store, module_id: ModuleId, ast: &nodes::Module) -> Module {
    Module { module_id }
}

pub fn analyze_inline(text: &str) -> Module {
    let mut p = Parser::of(text);
    let node = p.parse_module();
    let store = Store::default();

    let module = SyntaxNode::root::<nodes::Module>(&node).unwrap();

    analyze_module(&store, ModuleId::new("inline"), &module)
}
