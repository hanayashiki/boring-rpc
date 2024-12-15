use expect_test::expect;

use crate::semantic_store::SemanticStore;

fn check(input: &str, expect: expect_test::Expect) {
    let m = SemanticStore::inline_module(input);
    expect.assert_eq(&format!("{:#?}", *m));
}

#[test]
fn test_simple() {
    check(
        "type A = {}",
        expect![[r#"
            Module {
                module_id: ModuleId(
                    "inline",
                ),
                type_decls: [
                    TypeDecl {
                        name: "A",
                        kind: Type,
                        fields: [],
                        syntax_node_id: SyntaxNodeId {
                            kind: TypeDecl,
                            offset: 0,
                        },
                    },
                ],
                import_decls: [],
            }"#]],
    );
}

#[test]
fn test_import_decl() {
    check(
        "
            import { A } from '~/a.br'
            import * from '~/star.br'
        ",
        expect![[r#"
            Module {
                module_id: ModuleId(
                    "inline",
                ),
                type_decls: [],
                import_decls: [
                    ImportDecl {
                        star: false,
                        names: [
                            "A",
                        ],
                        source: "~/a.br",
                        syntax_node_id: SyntaxNodeId {
                            kind: ImportDecl,
                            offset: 13,
                        },
                    },
                    ImportDecl {
                        star: true,
                        names: [],
                        source: "~/star.br",
                        syntax_node_id: SyntaxNodeId {
                            kind: ImportDecl,
                            offset: 50,
                        },
                    },
                ],
            }"#]],
    );
}

#[test]
fn test_service_decl() {
    check(
        "
            service A {
                a(): number
            }
        ",
        expect![[r#"
            Module {
                module_id: ModuleId(
                    "inline",
                ),
                type_decls: [
                    TypeDecl {
                        name: "A",
                        kind: Service,
                        fields: [
                            Field {
                                name: "a",
                                syntax_node_id: SyntaxNodeId {
                                    kind: ServiceMethod,
                                    offset: 41,
                                },
                                field_type: Some(
                                    TypeExpr {
                                        syntax_node_id: SyntaxNodeId {
                                            kind: ServiceMethod,
                                            offset: 41,
                                        },
                                        node: TypeExprMethod {
                                            syntax_node_id: SyntaxNodeId {
                                                kind: ServiceMethod,
                                                offset: 41,
                                            },
                                            name: "a",
                                            fields: [],
                                            return_type: Some(
                                                TypeExpr {
                                                    syntax_node_id: SyntaxNodeId {
                                                        kind: TypeExpr,
                                                        offset: 46,
                                                    },
                                                    node: TypeExprName {
                                                        syntax_node_id: SyntaxNodeId {
                                                            kind: TypeExpr,
                                                            offset: 46,
                                                        },
                                                        name: "number",
                                                    },
                                                },
                                            ),
                                        },
                                    },
                                ),
                            },
                        ],
                        syntax_node_id: SyntaxNodeId {
                            kind: ServiceDecl,
                            offset: 13,
                        },
                    },
                ],
                import_decls: [],
            }"#]],
    )
}
