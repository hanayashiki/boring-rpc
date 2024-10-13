use boring_rpc_parser::parser::Parser;
use boring_rpc_syn::{nodes, AstNode, AstToken, SyntaxNode};

use crate::store::{Module, ModuleId, Store, TypeDecl};

#[cfg(test)]
mod test_analyzer;

pub fn analyze_module(store: &Store, module_id: ModuleId, ast: &nodes::Module) -> Module {
    println!("{:?}", ast.type_decl_list().unwrap().type_decls()[0].name());
    let m = Module {
        module_id: module_id,
        type_decls: ast
            .type_decl_list()
            .map_or(vec![], |x| x.type_decls())
            .iter()
            .filter_map(|type_decl| -> Option<(String, TypeDecl)> {
                let name = type_decl.name()?.ident()?.syntax().value().to_string();

                Some((
                    name.clone(),
                    TypeDecl {
                        name,

                        range: type_decl.syntax().range(),
                        ..Default::default()
                    },
                ))
            })
            .collect(),
    };

    m
}

pub fn analyze_inline(text: &str) -> Module {
    let mut p = Parser::of(text);
    let node = p.parse_module();
    let store = Store::default();

    let module = SyntaxNode::root::<nodes::Module>(&node).unwrap();

    analyze_module(&store, ModuleId::new("inline"), &module)
}
