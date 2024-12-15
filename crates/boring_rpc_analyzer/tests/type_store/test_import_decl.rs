use expect_test::expect;

use crate::test_utils::check;

#[test]
fn test_simple_import() {
    check(
        &[
            (
                "/index.br",
                "
                    import { B } from '~/imported.br'
                    type A = { b: B }
                ",
            ),
            ("/imported.br", "type B = {}"),
        ],
        expect![[r#"
            Module {
                types: [
                    Type {
                        name: "A",
                        kind: Type,
                        fields: [
                            (
                                "b",
                                TypeRef(
                                    TypeId(
                                        TypeId {
                                            module_id: ModuleId(
                                                "/index.br",
                                            ),
                                            name: "B",
                                        },
                                    ),
                                ),
                            ),
                        ],
                    },
                ],
            }"#]],
    );
}
