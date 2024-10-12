use boring_rpc_syn::GreenNodeOrToken;
use expect_test::expect;

use super::analyze_inline;

fn check(input: &str, expect: expect_test::Expect) {
    let mut p = analyze_inline(input);
    expect.assert_eq(&format!("{:?}", p));
}

#[test]
fn type_simple_module() {
    check(
        "type A = {}",
        expect![[r#"Module { module_id: ModuleId("inline") }"#]],
    );
}
