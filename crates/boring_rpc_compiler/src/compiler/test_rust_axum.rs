use boring_rpc_printers::{Printer, RustPrinter, TypeScriptPrinter};
use boring_rpc_vfs::MemFs;
use expect_test::expect;

use crate::compiler::Compiler;

use super::CompilerOptions;

fn check(input: &str, expect: expect_test::Expect) {
    let mut compiler = Compiler::<MemFs>::new_in_mem(
        MemFs::from(&[("/main.br", input)]),
        CompilerOptions {
            entry_point: "/main.br".into(),
            writers: vec![Box::new(RustPrinter {})],
            out_dir: "/".into(),
        },
    );

    let result = compiler.compile();

    expect.assert_eq(&result.outputs[0].1);
}

#[test]
fn test_simple() {
    check(
        "
        type A = {}
        ",
        expect![[r#"
            use serde::{Serialize, Deserialize};
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct A {}
        "#]],
    );
}

#[test]
fn test_primitive_fields() {
    check(
        "
        type A = {
            field_number: number,
            field_string: string,
        }

        type B = {
            a: A
        }
        ",
        expect![[r#"
            use serde::{Serialize, Deserialize};
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct A {
                pub field_number: f64,
                pub field_string: String,
            }
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct B {
                pub a: A,
            }
        "#]],
    );
}
