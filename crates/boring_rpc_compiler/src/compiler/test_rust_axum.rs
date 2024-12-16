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

#[test]
fn test_service() {
    check(
        "
            service Example {
                method1(): string,
                method2(a: string, b: number): string,
            }
        ",
        expect![[r#"
            use axum::Router;
            use boring_rpc_axum::{BoringRPC, BoringRPCHandler};
            use serde::{Serialize, Deserialize};
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct ExampleMethod1Request {}
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct ExampleMethod2Request {
                pub a: String,
                pub b: f64,
            }
            pub struct Example<S = ()> {
                inner: BoringRPC<S>,
            }
            impl<S> Example<S>
            where
                S: Clone + Send + Sync + 'static,
            {
                pub fn new() -> Self {
                    Self { inner: BoringRPC::new() }
                }
                pub fn method1<H, T>(&self, handler: H) -> Self
                where
                    H: BoringRPCHandler<T, Method1Request, Method1Response, S>,
                {
                    Self {
                        inner: self.inner.handle("/method1", handler),
                    }
                }
                pub fn method2<H, T>(&self, handler: H) -> Self
                where
                    H: BoringRPCHandler<T, Method1Request, Method1Response, S>,
                {
                    Self {
                        inner: self.inner.handle("/method2", handler),
                    }
                }
            }
        "#]],
    );
}