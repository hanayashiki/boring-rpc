use std::io::{Result, Write};

use crate::Printer;
use boring_rpc_analyzer::type_store::{self, TypeRef};

pub struct TypeScriptPrinter {}

impl Printer for TypeScriptPrinter {
    fn write(&self, writer: &mut dyn Write, module: &type_store::Module) -> Result<()> {
        for ty in module.types.iter() {
            write!(writer, "export interface {} {{", ty.name)?;
            if ty.fields.len() > 0 {
                write!(
                    writer,
                    "\n{},\n",
                    ty.fields
                        .iter()
                        .map(|(name, ty)| {
                            format!(
                                "    {}: {}",
                                name,
                                match ty {
                                    type_store::TypeExpr::TypeRef(TypeRef::PrimitiveType(
                                        primitive,
                                    )) => match primitive {
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
            }
            write!(writer, "}}\n\n")?;
        }

        Ok(())
    }
}
