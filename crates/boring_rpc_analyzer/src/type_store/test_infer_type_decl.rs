use expect_test::expect;

use crate::{semantic_store::SemanticStore, type_store::TypeStore};

fn check(input: &str, expect: expect_test::Expect) {
    let mut module = SemanticStore::inline_module(input);
    let mut ty = TypeStore::default();

    expect.assert_eq(&format!(
        "{:#?}",
        module.type_decls
            .iter()
            .map(|type_decl| { ty.infer_type_decl(type_decl) })
            .collect::<Vec<_>>()
    ));
}

#[test]
fn type_simple_type() {
    check("type A = {}", expect![[r#"
        [
            Type {
                name: "A",
                fields: [],
            },
        ]"#]]);
}

#[test]
fn test_string_fields() {
    check(
        "type A = { a: string, b: string }",
        expect![[r#"
            [
                Type {
                    name: "A",
                    fields: [
                        (
                            "a",
                            PrimitiveType(
                                String,
                            ),
                        ),
                        (
                            "b",
                            PrimitiveType(
                                String,
                            ),
                        ),
                    ],
                },
            ]"#]],
    );
}
