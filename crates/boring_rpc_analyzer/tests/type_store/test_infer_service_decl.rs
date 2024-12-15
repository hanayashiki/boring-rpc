use expect_test::expect;

use crate::test_utils::check;

#[test]
fn test_simple() {
    check(
        &[("/index.br", "service A {}")],
        expect![[r#"
            Module {
                types: [
                    Type {
                        name: "A",
                        kind: Service,
                        fields: [],
                    },
                ],
            }"#]],
    );
}


#[test]
fn test_simple_method() {
    check(
        &[(
            "/index.br",
            "
                service A {
                    method1(): string,
                }
            ",
        )],
        expect![[r#"
            Module {
                types: [
                    Type {
                        name: "A",
                        kind: Service,
                        fields: [
                            (
                                "method1",
                                Method {
                                    parameters: [],
                                    method_return: TypeRef(
                                        PrimitiveType(
                                            String,
                                        ),
                                    ),
                                },
                            ),
                        ],
                    },
                ],
            }"#]],
    );
}

#[test]
fn test_method_with_parameters() {
    check(
        &[(
            "/index.br",
            "
                service A {
                    method1(a: string, b: number): string,
                }
            ",
        )],
        expect![[r#"
            Module {
                types: [
                    Type {
                        name: "A",
                        kind: Service,
                        fields: [
                            (
                                "method1",
                                Method {
                                    parameters: [
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
                                                    Number,
                                                ),
                                            ),
                                        ),
                                    ],
                                    method_return: TypeRef(
                                        PrimitiveType(
                                            String,
                                        ),
                                    ),
                                },
                            ),
                        ],
                    },
                ],
            }"#]],
    );
}
