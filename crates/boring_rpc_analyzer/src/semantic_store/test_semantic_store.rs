use expect_test::expect;

use crate::semantic_store::SemanticStore;

fn check(input: &str, expect: expect_test::Expect) {
    let mut p = SemanticStore::inline_module(input);
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
                        syntax_node_id: SyntaxNodeId {
                            kind: TypeDecl,
                            offset: 0,
                        },
                        fields: [],
                    },
                ],
            }"#]],
    );
}
