use expect_test::expect;

use crate::{semantic_store::SemanticStore, type_store::TypeStore};

fn check(input: &str, expect: expect_test::Expect) {
    let mut module = SemanticStore::inline_module(input);
    let mut ty = TypeStore::default();

    expect.assert_eq(&format!("{:#?}", ty.infer_module(&module)));
}

#[test]
fn type_simple_module() {
    check(
        "
type A = {}
type B = {}
",
        expect![[r#"
            Module {
                types: [
                    Type {
                        name: "A",
                        fields: [],
                    },
                    Type {
                        name: "B",
                        fields: [],
                    },
                ],
            }"#]],
    );
}
