use std::io::{Result, Write};

use boring_rpc_analyzer::type_store;
use crate::Printer;

pub struct TypeScriptPrinter {}

impl Printer for TypeScriptPrinter {
    fn write(&self, writer: &mut dyn Write, module: &type_store::Module) -> Result<()> {
        for ty in module.iter_types() {
            write!(writer, "export interface {} {{\n", ty.name)?;
            write!(
                writer,
                "{},\n",
                ty.fields
                    .iter()
                    .map(|(name, ty)| {
                        format!(
                            "    {}: {}",
                            name,
                            match ty {
                                type_store::TypeRef::PrimitiveType(primitive) => match primitive {
                                    type_store::PrimitiveType::Number => "number".to_string(),
                                    type_store::PrimitiveType::String => "string".to_string(),
                                },
                                _ => todo!(),
                            }
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(", \n")
            )?;
            write!(writer, "}}\n\n")?;
        }

        Ok(())
    }
}