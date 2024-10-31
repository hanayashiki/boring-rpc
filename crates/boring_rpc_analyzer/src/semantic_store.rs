use std::collections::BTreeMap;

use boring_rpc_syn::{SyntaxNode, SyntaxNodeId};

use boring_rpc_parser::parser::Parser;
use boring_rpc_syn::{nodes, AstNode, AstToken};

#[cfg(test)]
mod test_semantic_store;

#[derive(Hash, PartialOrd, Ord, Eq, PartialEq, Debug, Clone)]
pub struct ModuleId(String);

impl ModuleId {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum TypeDeclKind {
    Type,
    Service,
    Scalar,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct TypeDecl {
    pub name: String,
    pub kind: TypeDeclKind,
    pub syntax_node_id: SyntaxNodeId,

    pub fields: Vec<Field>,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Field {
    pub name: String,
    pub syntax_node_id: SyntaxNodeId,
    pub field_type: Option<TypeExpr>,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct TypeExpr {
    pub syntax_node_id: SyntaxNodeId,
    pub node: TypeExprNode,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum TypeExprNode {
    Name(String),
}

#[derive(Debug, Clone)]
pub struct Module {
    pub module_id: ModuleId,
    pub type_decls: Vec<TypeDecl>,
}

#[derive(Debug, Default)]
pub struct SemanticStore {
    modules: BTreeMap<ModuleId, Module>,
    module_nodes: BTreeMap<ModuleId, BTreeMap<SyntaxNodeId, SyntaxNode>>,
}

impl SemanticStore {
    pub fn build_module(&mut self, module_id: ModuleId, ast: &nodes::Module) -> ModuleId {
        let type_decls = ast
            .type_decl_list()
            .map_or(vec![], |x| x.type_decls())
            .iter()
            .map(|type_decl| self.build_type_decl(module_id.clone(), type_decl))
            .collect();

        self.modules.insert(
            module_id.clone(),
            Module {
                module_id: module_id.clone(),
                type_decls,
            },
        );

        module_id
    }

    fn build_type_decl(&mut self, module_id: ModuleId, ast: &nodes::TypeDecl) -> TypeDecl {
        let default_name = "#default_name";
        // TODO: pick a default name
        let name = ast.name().map_or(default_name.into(), |n| {
            n.ident()
                .map_or(default_name.into(), |n| n.syntax().value().to_string())
        });

        TypeDecl {
            name,
            // TODO: other kinds
            kind: TypeDeclKind::Type,
            syntax_node_id: ast.syntax().id(),
            fields: ast.field_list().map_or(vec![], |field| {
                field
                    .fields()
                    .iter()
                    .map(|f| self.build_field(module_id.clone(), f))
                    .collect()
            }),
        }
    }

    fn build_field(&mut self, module_id: ModuleId, ast: &nodes::Field) -> Field {
        let default_name = "#default_name";

        let name = ast.field_name().map_or(default_name.into(), |n| {
            n.ident()
                .map_or(default_name.into(), |n| n.syntax().value().to_string())
        });

        Field {
            name,
            syntax_node_id: ast.syntax().id(),
            field_type: ast
                .field_type()
                .as_ref()
                .map(|e| self.build_type_expr(module_id, e)),
        }
    }

    fn build_type_expr(&mut self, module_id: ModuleId, ast: &nodes::TypeExpr) -> TypeExpr {
        TypeExpr {
            syntax_node_id: ast.syntax().id(),
            node: TypeExprNode::Name(ast.name().map_or("#default_name".into(), |n| {
                n.ident()
                    .map_or("#default_name".into(), |n| n.syntax().value().to_string())
            })),
        }
    }

    pub fn inline_module(text: &str) -> Module {
        let mut p = Parser::of(text);
        let node = p.parse_module();
        let mut store = SemanticStore::default();

        let module = SyntaxNode::root::<nodes::Module>(node).unwrap();

        let module_id = store.build_module(ModuleId::new("inline"), &module);

        store.modules.get(&module_id).unwrap().clone()
    }
}
