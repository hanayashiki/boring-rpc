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
    p.parse_module();

    expect.assert_eq(&format!("{:#?}", p.errors));
}

#[test]
fn type_decl_simple() {
    check(
        "type A = {}",
        expect![[r#"
            Module
                StatementList
                    Statement
                        TypeDecl
                            TypeKeyword "type"
                            Whitespace " "
                            Name
                                Ident "A"
                            Whitespace " "
                            Equal "="
                            Whitespace " "
                            LCurly "{"
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
                StatementList
                    Statement
                        TypeDecl
                            TypeKeyword "type"
                            Whitespace " "
                            Name
                                Ident "type"
                            Whitespace " "
                            Equal "="
                            Whitespace " "
                            LCurly "{"
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
                Whitespace "\n            "
                StatementList
                    Statement
                        TypeDecl
                            TypeKeyword "type"
                            Whitespace " "
                            Name
                                Ident "A"
                            Whitespace " "
                            Equal "="
                            Whitespace " "
                            LCurly "{"
                            RCurly "}"
                    Whitespace "\n            "
                    Statement
                        TypeDecl
                            TypeKeyword "type"
                            Whitespace " "
                            Name
                                Ident "B"
                            Whitespace " "
                            Equal "="
                            Whitespace " "
                            LCurly "{"
                            RCurly "}"
                    Whitespace "\n        "
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

#[test]
fn type_decl_fields() {
    check(
        "
            type A = {
                a: string,
                b: number,
            }",
        expect![[r#"
            Module
                Whitespace "\n            "
                StatementList
                    Statement
                        TypeDecl
                            TypeKeyword "type"
                            Whitespace " "
                            Name
                                Ident "A"
                            Whitespace " "
                            Equal "="
                            Whitespace " "
                            LCurly "{"
                            Whitespace "\n                "
                            FieldList
                                Field
                                    Name
                                        Ident "a"
                                    Colon ":"
                                    Whitespace " "
                                    TypeExpr
                                        Name
                                            Ident "string"
                                Comma ","
                                Whitespace "\n                "
                                Field
                                    Name
                                        Ident "b"
                                    Colon ":"
                                    Whitespace " "
                                    TypeExpr
                                        Name
                                            Ident "number"
                                Comma ","
                                Whitespace "\n            "
                            RCurly "}"
        "#]],
    );

    check(
        "type NoTrailingComma = {
            a: string,
            b: number
        }",
        expect![[r#"
            Module
                StatementList
                    Statement
                        TypeDecl
                            TypeKeyword "type"
                            Whitespace " "
                            Name
                                Ident "NoTrailingComma"
                            Whitespace " "
                            Equal "="
                            Whitespace " "
                            LCurly "{"
                            Whitespace "\n            "
                            FieldList
                                Field
                                    Name
                                        Ident "a"
                                    Colon ":"
                                    Whitespace " "
                                    TypeExpr
                                        Name
                                            Ident "string"
                                Comma ","
                                Whitespace "\n            "
                                Field
                                    Name
                                        Ident "b"
                                    Colon ":"
                                    Whitespace " "
                                    TypeExpr
                                        Name
                                            Ident "number"
                                Whitespace "\n        "
                            RCurly "}"
        "#]],
    )
}
