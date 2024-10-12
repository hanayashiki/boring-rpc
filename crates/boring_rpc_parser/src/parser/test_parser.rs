use boring_rpc_syn::GreenNodeOrToken;
use expect_test::expect;

use crate::parser::Parser;

fn check(input: &str, expect: expect_test::Expect) {
    let mut p = Parser::of(input);
    let node = p.parse_module();

    assert!(p.errors.is_empty());
    expect.assert_eq(&format!("{}", GreenNodeOrToken::Node(node)));
}

fn check_error(input: &str, expect: expect_test::Expect) {
    let mut p = Parser::of(input);
    let node = p.parse_module();

    expect.assert_eq(&format!("{:#?}", p.errors));
}

#[test]
fn type_decl_simple() {
    check(
        "type A = {}",
        expect![[r#"
            Module
                TypeDecl
                    TypeKeyword "type"
                    Ident "A"
                    Whitespace " "
                    Equal "="
                    Whitespace " "
                    LCurly "{"
                    FieldList
                    RCurly "}"
        "#]],
    );
}

#[test]
fn type_decl_keyword_as_ident() {
    check(
        "type type = {}",
        expect![[r#"
            Module
                TypeDecl
                    TypeKeyword "type"
                    Ident "type"
                    Whitespace " "
                    Equal "="
                    Whitespace " "
                    LCurly "{"
                    FieldList
                    RCurly "}"
        "#]],
    );
}

#[test]
fn type_decl_list() {
    check(
        "
            type A = {}
            type B = {}
        ",
        expect![[r#"
            Module
                TypeDecl
                    TypeKeyword "type"
                    Ident "A"
                    Whitespace " "
                    Equal "="
                    Whitespace " "
                    LCurly "{"
                    FieldList
                    RCurly "}"
                TypeDecl
                    TypeKeyword "type"
                    Ident "B"
                    Whitespace " "
                    Equal "="
                    Whitespace " "
                    LCurly "{"
                    FieldList
                    RCurly "}"
        "#]],
    );
}

#[test]
fn type_decl_error() {
    check_error(
        "type A = {",
        expect![[r#"
            [
                SyntaxError(
                    "Expect '}', got 'EOF'",
                    10..10,
                ),
            ]"#]],
    );
}
