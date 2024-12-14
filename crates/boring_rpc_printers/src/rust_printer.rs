use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::io::{Result, Write};
use syn::parse_quote;

use crate::Printer;
use boring_rpc_analyzer::type_store::{self, TypeExpr, TypeId, TypeRef};

mod rust_printer_options;

pub struct RustPrinter {}

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
}

impl Printer for RustPrinter {
    fn file_name(&self) -> String {
        "schema.rs".to_string()
    }

    fn write(&self, writer: &mut dyn Write, module: &type_store::Module) -> Result<()> {
        let types = module.types.iter().map(|t| {
            let name = format_ident!("{}", t.name);
            let fields = t.fields.iter().map(|(field_name, field_type)| {
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
        });

        let syntax_tree = parse_quote! {
            use serde::{Serialize, Deserialize};

            #( #types )*
        };

        write!(writer, "{}", prettyplease::unparse(&syntax_tree))?;

        Ok(())
    }
}
