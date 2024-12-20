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
            writers: vec![Box::new(TypeScriptPrinter {}), Box::new(RustPrinter {})],
            out_dir: "/".into(),
        },
    );

    let result = compiler.compile();

    expect.assert_eq(&format!(
        "// typescript\n{}// rust\n{}",
        &result.outputs[0].1, &result.outputs[1].1
    ));
}

#[test]
fn test_simple() {
    check(
        "
        type A = {
            a: string,
            b: string,
        }
        
        type B = {}
        ",
        expect![[r#"
            // typescript
            export interface A {
                a: string, 
                b: string,
            }

            export interface B {}

            // rust
            pub struct A {
                pub a: String, 
                pub b: String,
            }

            pub struct B {}

        "#]],
    );
}
