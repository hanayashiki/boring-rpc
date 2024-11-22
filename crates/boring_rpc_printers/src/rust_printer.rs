use std::{
    io::{Result, Write},
    primitive,
};

use crate::Printer;
use boring_rpc_analyzer::type_store::{self, TypeRef};

pub struct RustPrinter {}

impl Printer for RustPrinter {
    fn write(&self, writer: &mut dyn Write, module: &type_store::Module) -> Result<()> {
        for ty in module.types.iter() {
            write!(writer, "pub struct {} {{", ty.name)?;
            if ty.fields.len() > 0 {
                write!(
                    writer,
                    "\n{},\n",
                    ty.fields
                        .iter()
                        .map(|(name, ty)| {
                            format!(
                                "    pub {}: {}",
                                name,
                                match ty {
                                    type_store::TypeExpr::TypeRef(TypeRef::PrimitiveType(
                                        primitive,
                                    )) => match primitive {
                                        type_store::PrimitiveType::Number => "f64".to_string(),
                                        type_store::PrimitiveType::String => "String".to_string(),
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
