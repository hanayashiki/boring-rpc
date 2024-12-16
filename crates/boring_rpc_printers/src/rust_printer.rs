use boring_rpc_common::to_upper_camel_case;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::io::{Result, Write};
use syn::{parse_quote, spanned::Spanned};

use crate::Printer;
use boring_rpc_analyzer::type_store::{self, Type, TypeExpr, TypeId, TypeKind, TypeRef};

mod rust_printer_options;

pub struct RustPrinter {}

fn get_request_name(ty: &Type, method_name: &str) -> String {
    return format!(
        "{}{}Request",
        to_upper_camel_case(&ty.name),
        to_upper_camel_case(method_name)
    );
}

impl RustPrinter {
    fn quote_type_expr(&self, type_expr: &type_store::TypeExpr) -> TokenStream {
        match type_expr {
            TypeExpr::TypeRef(type_ref) => match type_ref {
                TypeRef::PrimitiveType(primitive_type) => match primitive_type {
                    type_store::PrimitiveType::Number => quote! { f64 },
                    type_store::PrimitiveType::String => quote! { String },
                },
                TypeRef::TypeId(TypeId { name, .. }) => {
                    let name = format_ident!("{}", name);
                    quote! { #name }
                }
                TypeRef::Unknown => panic!("Unexpected unknown type"),
            },
            _ => todo!(),
        }
    }

    fn quote_type_as_type(&self, ty: &Type) -> TokenStream {
        let name = format_ident!("{}", ty.name);
        let fields = ty.fields.iter().map(|(field_name, field_type)| {
            let field_name = format_ident!("{}", field_name);

            let field_type = self.quote_type_expr(&field_type);

            quote! {
                pub #field_name: #field_type
            }
        });

        quote! {
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct #name {
                #(#fields,)*
            }
        }
    }

    fn quote_type_as_service(&self, ty: &Type) -> TokenStream {
        let service_name = format_ident!("{}", ty.name);

        let methods = ty.fields.iter().map(|(method_name, method_type)| {
            let method_name_ident = format_ident!("{}", method_name);
            let method_path = format!("/{}", method_name.replace("_", "-").to_lowercase());

            let request_type = format_ident!("{}", get_request_name(ty, method_name));

            let response_type = match method_type {
                TypeExpr::Method { method_return, .. } => self.quote_type_expr(method_return),
                _ => panic!("expected method type"),
            };

            quote! {
                pub fn #method_name_ident<H, T>(self, handler: H) -> Self
                where
                    H: BoringRPCHandler<T, #request_type, #response_type, S>,
                {
                    Self {
                        inner: self.inner.handle(#method_path, handler),
                    }
                }
            }
        });

        quote! {
            pub struct #service_name<S = ()> {
                inner: BoringRPC<S>,
            }

            impl<S> #service_name<S>
            where
                S: Clone + Send + Sync + 'static,
            {
                pub fn new() -> Self {
                    Self {
                        inner: BoringRPC::new(),
                    }
                }

                #(#methods)*
            }

            impl<S> Into<Router<S>> for #service_name<S>
            where
                S: Clone + Send + Sync + 'static,
            {
                fn into(self) -> Router<S> {
                    self.inner.into_router()
                }
            }

        }
    }

    fn quote_type(&self, ty: &Type) -> TokenStream {
        match ty.kind {
            TypeKind::Type => self.quote_type_as_type(ty),
            TypeKind::Service => self.quote_type_as_service(ty),
            _ => unimplemented!(),
        }
    }

    fn quote_request_types(&self, module: &type_store::Module) -> TokenStream {
        let request_types = module
            .types
            .iter()
            .filter_map(|t| match t.kind {
                TypeKind::Service => Some(t.fields.iter().map(|(method_name, method_type)| {
                    if let TypeExpr::Method { parameters, .. } = method_type {
                        let request_type = format_ident!("{}", get_request_name(t, method_name));

                        let fields = parameters.iter().map(|(field_name, field_type)| {
                            let field_name = format_ident!("{}", field_name);

                            let field_type = self.quote_type_expr(&field_type);

                            quote! {
                                pub #field_name: #field_type
                            }
                        });

                        quote! {
                            #[derive(Debug, Clone, Serialize, Deserialize)]
                            pub struct #request_type {
                                #(#fields,)*
                            }
                        }
                    } else {
                        panic!("expected method type");
                    }
                })),
                _ => None,
            })
            .flatten();

        quote! {
            #(#request_types)*
        }
    }

    fn quote_module(&self, module: &type_store::Module) -> TokenStream {
        let request_types = self.quote_request_types(module);
        let types = module.types.iter().map(|t| self.quote_type(t));

        quote! {
            use axum::Router;
            use boring_rpc_axum::{BoringRPC, BoringRPCHandler};
            use serde::{Serialize, Deserialize};

            #request_types

            #(#types)*
        }
    }
}

impl Printer for RustPrinter {
    fn file_name(&self) -> String {
        "schema.rs".to_string()
    }

    fn write(&self, writer: &mut dyn Write, module: &type_store::Module) -> Result<()> {
        let syntax_tree = syn::parse2(self.quote_module(module));
        match syntax_tree {
            Ok(_) => {
                write!(writer, "{}", prettyplease::unparse(&syntax_tree.unwrap()))?;
            }
            Err(_) => {
                // Use rustfmt to give me a nice syntax error message
                use std::process::{Command, Stdio};

                let generated_code = self.quote_module(module).to_string();

                let mut rustfmt = Command::new("rustfmt")
                    .args(["--color=always"])
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .unwrap();

                let mut stdin = rustfmt.stdin.take().unwrap();
                std::thread::spawn(move || {
                    stdin
                        .write_all(generated_code.as_bytes())
                        .expect("Failed to write to stdin");
                });

                let output = rustfmt.wait_with_output().unwrap();

                panic!(
                    "Unexpected generated token stream: \n{}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }

        Ok(())
    }
}
