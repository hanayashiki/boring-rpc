use std::{collections::BTreeMap, rc::Rc};

use boring_rpc_resolver::{Resolution, ResolutionId, Resolver};
use boring_rpc_syn::{SyntaxNode, SyntaxNodeId};

use boring_rpc_parser::parser::Parser;
use boring_rpc_syn::{nodes, AstNode, AstToken};
use boring_rpc_vfs::Vfs;

#[cfg(test)]
mod test_semantic_store;

#[derive(Hash, PartialOrd, Ord, Eq, PartialEq, Debug, Clone)]
pub struct ModuleId(String);

impl From<ResolutionId> for ModuleId {
    fn from(value: ResolutionId) -> Self {
        ModuleId(value.0)
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct DeclId(pub ModuleId, pub String);

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
    pub fields: Vec<Field>,

    pub syntax_node_id: SyntaxNodeId,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct ImportDecl {
    pub star: bool,
    pub names: Vec<String>,
    pub source: String,

    pub syntax_node_id: SyntaxNodeId,
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
    pub(crate) module_id: ModuleId,
    pub(crate) type_decls: Vec<TypeDecl>,
    pub(crate) import_decls: Vec<ImportDecl>,
}

#[derive(Debug, Default)]
pub struct SemanticStore {
    modules: BTreeMap<ModuleId, Rc<Module>>,
    module_nodes: BTreeMap<ModuleId, BTreeMap<SyntaxNodeId, SyntaxNode>>,
}

impl SemanticStore {
    pub fn build_module(&mut self, module_id: ModuleId, ast: &nodes::Module) -> ModuleId {
        let type_decls = map_statements(ast, |statement| -> Option<TypeDecl> {
            if let Some(type_decl) = statement.type_decl() {
                Some(self.build_type_decl(module_id.clone(), &type_decl))
            } else {
                None
            }
        });

        let import_decls = map_statements(ast, |statement| -> Option<ImportDecl> {
            Some(self.build_import_decl(&statement.import_decl()?))
        });

        self.modules.insert(
            module_id.clone(),
            Rc::new(Module {
                module_id: module_id.clone(),
                type_decls,
                import_decls,
            }),
        );

        module_id
    }

    fn build_type_decl(&mut self, module_id: ModuleId, ast: &nodes::TypeDecl) -> TypeDecl {
        let name = ast
            .name()
            .iter()
            .filter_map(|f| f.ident())
            .map(|s| s.syntax().value().to_string())
            .next()
            .unwrap_or("#default_name".into());

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

    fn build_import_decl(&mut self, ast: &nodes::ImportDecl) -> ImportDecl {
        ImportDecl {
            syntax_node_id: ast.syntax().id(),
            star: ast
                .import_body()
                .iter()
                .map(|s| s.star().is_some())
                .next()
                .unwrap_or(false),
            names: ast
                .import_body()
                .iter()
                .filter_map(|f| f.import_specifier_list())
                .map(|f| f.import_specifiers())
                .map(|s| {
                    s.iter()
                        .filter_map(|s| Some(s.ident()?.syntax().value().to_string()))
                        .collect()
                })
                .next()
                .unwrap_or(vec![]),
            source: ast
                .import_source()
                .iter()
                .filter_map(|s| s.string())
                .map(|s| s.syntax().value().to_string())
                .next()
                .unwrap_or("#default_source".into()),
        }
    }

    pub fn resolve_name<V: Vfs>(
        &mut self,
        module_id: ModuleId,
        name: &String,
        resolver: &Resolver<V>,
    ) -> Option<DeclId> {
        let module = self.modules.get(&module_id)?;

        if let Some(_) = module
            .type_decls
            .iter()
            .find(|type_decl| type_decl.name == *name)
        {
            return Some(DeclId(module_id.clone(), name.clone()));
        }

        if let Some(resolution) = module.import_decls.iter().find_map(|import_decl| {
            if import_decl.star {
                // TODO: resolve star imports
                None
            } else {
                resolver.resolve(&import_decl.source).ok()
            }
        }) {
            match resolution {
                Resolution::Module((resolution_id, module)) => {
                    self.build_module(resolution_id.into(), &module);

                    return Some(DeclId(module_id.clone(), name.clone()));
                }
                // TODO: std
                _ => return None,
            }
        }

        None
    }

    pub fn get_module(&self, module_id: ModuleId) -> Option<Rc<Module>> {
        self.modules.get(&module_id).cloned()
    }

    pub fn inline_module(text: &str) -> Rc<Module> {
        let mut p = Parser::of(text);
        let node = p.parse_module();
        let mut store = SemanticStore::default();

        let module = SyntaxNode::root::<nodes::Module>(node).unwrap();

        let module_id = store.build_module(ModuleId::new("inline"), &module);

        store.get_module(module_id).clone().unwrap()
    }
}

fn map_statements<T, F: FnMut(&nodes::Statement) -> Option<T>>(
    ast: &nodes::Module,
    f: F,
) -> Vec<T> {
    ast.statement_list()
        .map_or(vec![], |x| x.statements())
        .iter()
        .filter_map(f)
        .collect()
}
