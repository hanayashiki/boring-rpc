use boring_rpc_syn::syn::GreenNodeOrToken;
use expect_test::expect;

use crate::parser::Parser;

fn check(input: &str, expect: expect_test::Expect) {
    let mut p = Parser::of(input);
    let node = p.parse_type_decl();

    assert!(p.errors.is_empty());
    expect.assert_eq(&format!("{}", GreenNodeOrToken::Node(node)));
}

fn check_error(input: &str, expect: expect_test::Expect) {
    let mut p = Parser::of(input);
    let node = p.parse_type_decl();

    expect.assert_eq(&format!("{:#?}", p.errors));
}


#[test]
fn type_decl_simple() {
    check("type A = {}", expect![[r#"
        TypeDecl
            TypeKeyword type
            Ident A
            Equal =
            LCurly {
            FieldList
            RCurly }
    "#]]);

    check("type type = {}", expect![[r#"
        TypeDecl
            TypeKeyword type
            Ident type
            Equal =
            LCurly {
            FieldList
            RCurly }
    "#]]);
}

#[test]
fn type_decl_error() {
    check_error("type A = {", expect![[r#"
        [
            SyntaxError(
                "Expect '}', got 'EOF'",
                10..10,
            ),
        ]"#]]);
}