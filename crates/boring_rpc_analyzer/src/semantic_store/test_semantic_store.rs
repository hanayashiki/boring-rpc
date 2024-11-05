use expect_test::expect;

use crate::semantic_store::SemanticStore;

fn check(input: &str, expect: expect_test::Expect) {
    let p = SemanticStore::inline_module(input);
    expect.assert_eq(&format!("{:#?}", p));
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
