use std::{
    collections::{BTreeMap, HashSet},
    marker::PhantomData,
};

use boring_rpc_resolver::Resolver;
use boring_rpc_vfs::Vfs;

use crate::semantic_store::{self, DeclId, SemanticStore, TypeExprNode};

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct TypeId {
    pub module_id: semantic_store::ModuleId,
    pub name: String,
}

impl From<DeclId> for TypeId {
    fn from(DeclId(module_id, name): DeclId) -> Self {
        Self { module_id, name }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum TypeKind {
    Type,
    Service,
    Scalar,
}

impl From<semantic_store::TypeDeclKind> for TypeKind {
    fn from(kind: semantic_store::TypeDeclKind) -> Self {
        match kind {
            semantic_store::TypeDeclKind::Type => TypeKind::Type,
            semantic_store::TypeDeclKind::Service => TypeKind::Service,
            semantic_store::TypeDeclKind::Scalar => TypeKind::Scalar,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Type {
    pub name: String,
    pub kind: TypeKind,
    pub fields: Vec<(String, TypeExpr)>,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Module {
    pub types: Vec<Type>,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum TypeRef {
    PrimitiveType(PrimitiveType),
    TypeId(TypeId),
    Unknown,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum TypeExpr {
    TypeRef(TypeRef),
    Method {
        parameters: Box<[(String, TypeExpr)]>,
        method_return: Box<TypeExpr>,
    },
    Array(Box<TypeExpr>),
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum PrimitiveType {
    Number,
    String,
}

pub struct InferenceContext<'a, V: Vfs> {
    pub sementic_store: &'a mut SemanticStore,
    pub resolver: &'a Resolver<V>,
    pub module_id: semantic_store::ModuleId,
}

#[derive(Debug)]
pub struct TypeStore<V: Vfs> {
    types: BTreeMap<TypeId, Type>,
    vfs: PhantomData<V>,
}

impl<V: Vfs> TypeStore<V> {
    pub fn new() -> Self {
        Self {
            types: BTreeMap::new(),
            vfs: PhantomData,
        }
    }

    pub fn infer_type_decl(
        &mut self,
        type_decl: &semantic_store::TypeDecl,
        ctx: &mut InferenceContext<V>,
    ) -> Type {
        let mut names = HashSet::<&String>::new();

        Type {
            name: type_decl.name.clone(),
            kind: type_decl.kind.clone().into(),
            fields: type_decl
                .fields
                .iter()
                .filter_map(|field| {
                    if names.contains(&field.name) {
                        None
                    } else {
                        names.insert(&field.name);

                        match field.field_type {
                            None => Some((field.name.clone(), TypeExpr::TypeRef(TypeRef::Unknown))),
                            Some(ref type_expr) => {
                                Some((field.name.clone(), self.infer_type_expr(type_expr, ctx)))
                            }
                            _ => unimplemented!(),
                        }
                    }
                })
                .collect(),
        }
    }

    pub fn infer_type_expr(
        &mut self,
        type_expr: &semantic_store::TypeExpr,
        ctx: &mut InferenceContext<V>,
    ) -> TypeExpr {
        match &type_expr.node {
            TypeExprNode::TypeExprName { name, .. } => {
                match ctx
                    .sementic_store
                    .resolve_name(ctx.module_id.clone(), name, ctx.resolver)
                {
                    Some(decl_id) => TypeExpr::TypeRef(TypeRef::TypeId(decl_id.into())),
                    None => Self::name_to_primitive(name)
                        .map(|primitive| TypeExpr::TypeRef(TypeRef::PrimitiveType(primitive)))
                        .unwrap_or(TypeExpr::TypeRef(TypeRef::Unknown)),
                }
            }
            TypeExprNode::TypeExprMethod {
                fields,
                return_type,
                ..
            } => {
                let parameters: Box<[(String, TypeExpr)]> = fields
                    .iter()
                    .map(|field| -> (String, TypeExpr) {
                        (
                            field.name.clone(),
                            self.infer_type_expr(
                                field.field_type.as_ref().expect("to handle missing type"),
                                ctx,
                            ),
                        )
                    })
                    .collect::<Vec<_>>()
                    .into();

                let method_return =
                    Box::new(self.infer_type_expr(
                        return_type.as_ref().expect("to handle missing type"),
                        ctx,
                    ));

                TypeExpr::Method {
                    parameters,
                    method_return,
                }
            }
        }
    }

    fn name_to_primitive(name: &str) -> Option<PrimitiveType> {
        match name {
            "number" => Some(PrimitiveType::Number),
            "string" => Some(PrimitiveType::String),
            _ => None,
        }
    }

    pub fn infer_module(&mut self, ctx: &mut InferenceContext<V>) -> Module {
        let mut names = HashSet::<&String>::new();

        let module = ctx
            .sementic_store
            .get_module(ctx.module_id.clone())
            .expect("Module not found");

        Module {
            types: module
                .type_decls
                .iter()
                .map(|type_decl| {
                    if names.contains(&type_decl.name) {
                        None
                    } else {
                        names.insert(&type_decl.name);

                        Some(self.infer_type_decl(type_decl, ctx))
                    }
                })
                .filter_map(|x| x)
                .collect(),
        }
    }
}
