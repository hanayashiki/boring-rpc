use expect_test::expect;

use crate::test_utils::check;

#[test]
fn test_simple_type() {
    check(
        &[("/index.br", "type A = {}")],
        expect![[r#"
            Module {
                types: [
                    Type {
                        name: "A",
                        kind: Type,
                        fields: [],
                    },
                ],
            }"#]],
    );
}

#[test]
fn test_string_fields() {
    check(
        &[("/index.br", "type A = { a: string, b: string }")],
        expect![[r#"
            Module {
                types: [
                    Type {
                        name: "A",
                        kind: Type,
                        fields: [
                            (
                                "a",
                                TypeRef(
                                    PrimitiveType(
                                        String,
                                    ),
                                ),
                            ),
                            (
                                "b",
                                TypeRef(
                                    PrimitiveType(
                                        String,
                                    ),
                                ),
                            ),
                        ],
                    },
                ],
            }"#]],
    );
}
