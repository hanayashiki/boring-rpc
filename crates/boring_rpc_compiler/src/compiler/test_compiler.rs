use boring_rpc_printers::{Printer, TypeScriptPrinter};
use boring_rpc_vfs::mem_fs::MemFs;
use expect_test::expect;

use crate::compiler::Compiler;

use super::CompilerOptions;

fn check(input: &str, expect: expect_test::Expect) {
    let mut compiler = Compiler::<MemFs>::in_mem(
        MemFs::from(&[("main.br", input)]),
        CompilerOptions {
            entry_point: "main.br".into(),
            writers: vec![Box::new(TypeScriptPrinter {})],
        },
    );

    let result = compiler.compile();

    expect.assert_eq( &result.outputs[0].1);
}

#[test]
fn test_simple() {
    check("
        type A = {
            a: string,
            b: string,
        }
        
        type B = {}
        ",
        expect![[r#"
            export interface A {
                a: string, 
                b: string,
            }

            export interface B {}

        "#]],
    );
}
