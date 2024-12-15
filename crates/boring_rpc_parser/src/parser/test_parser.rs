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

#[test]
fn import_decl_star() {
    check(
        "import * from 'std'",
        expect![[r#"
            Module
                StatementList
                    Statement
                        ImportDecl
                            ImportKeyword "import"
                            Whitespace " "
                            ImportBody
                                Star "*"
                            Whitespace " "
                            FromKeyword "from"
                            Whitespace " "
                            ImportSource
                                String "std"
        "#]],
    )
}

#[test]
fn import_decl_list() {
    check(
        "import { A, B } from 'std'",
        expect![[r#"
            Module
                StatementList
                    Statement
                        ImportDecl
                            ImportKeyword "import"
                            Whitespace " "
                            ImportBody
                                LCurly "{"
                                Whitespace " "
                                ImportSpecifierList
                                    ImportSpecifier
                                        Ident "A"
                                    Comma ","
                                    Whitespace " "
                                    ImportSpecifier
                                        Ident "B"
                                    Whitespace " "
                                RCurly "}"
                            Whitespace " "
                            FromKeyword "from"
                            Whitespace " "
                            ImportSource
                                String "std"
        "#]],
    )
}

#[test]
fn service_decl() {
    check(
        "service A {}",
        expect![[r#"
            Module
                StatementList
                    Statement
                        ServiceDecl
                            ServiceKeyword "service"
                            Whitespace " "
                            Name
                                Ident "A"
                            Whitespace " "
                            LCurly "{"
                            RCurly "}"
        "#]],
    );

    check(
        "service A { rpc(): string }",
        expect![[r#"
            Module
                StatementList
                    Statement
                        ServiceDecl
                            ServiceKeyword "service"
                            Whitespace " "
                            Name
                                Ident "A"
                            Whitespace " "
                            LCurly "{"
                            Whitespace " "
                            ServiceMethodList
                                ServiceMethod
                                    Name
                                        Ident "rpc"
                                    LParenthesis "("
                                    RParenthesis ")"
                                    Colon ":"
                                    Whitespace " "
                                    TypeExpr
                                        Name
                                            Ident "string"
                                Whitespace " "
                            RCurly "}"
        "#]],
    );

    check(
        "service A { rpc(p1: P, p2: P): R }",
        expect![[r#"
            Module
                StatementList
                    Statement
                        ServiceDecl
                            ServiceKeyword "service"
                            Whitespace " "
                            Name
                                Ident "A"
                            Whitespace " "
                            LCurly "{"
                            Whitespace " "
                            ServiceMethodList
                                ServiceMethod
                                    Name
                                        Ident "rpc"
                                    LParenthesis "("
                                    FieldList
                                        Field
                                            Name
                                                Ident "p1"
                                            Colon ":"
                                            Whitespace " "
                                            TypeExpr
                                                Name
                                                    Ident "P"
                                        Comma ","
                                        Whitespace " "
                                        Field
                                            Name
                                                Ident "p2"
                                            Colon ":"
                                            Whitespace " "
                                            TypeExpr
                                                Name
                                                    Ident "P"
                                    RParenthesis ")"
                                    Colon ":"
                                    Whitespace " "
                                    TypeExpr
                                        Name
                                            Ident "R"
                                Whitespace " "
                            RCurly "}"
        "#]]
    )

}
