use std::{
    io::{Result, Write},
    primitive,
};

use crate::Printer;
use boring_rpc_analyzer::type_store::{self, TypeRef};

pub struct RustPrinter {}

impl RustPrinter {
    fn get_header(&self) -> &[&str] {
        return &[
            "use serde::{Serialize, Deserialize};",
        ];
    }

    fn get_derives(&self) -> &str {
        return "#[derive(Debug, Clone, Serialize, Deserialize)]";
    }
}

impl Printer for RustPrinter {
    fn file_name(&self) -> String {
        "schema.rs".to_string()
    }

    fn write(&self, writer: &mut dyn Write, module: &type_store::Module) -> Result<()> {
        for header in self.get_header() {
            write!(writer, "{}\n", header)?;
        }

        writeln!(writer)?;

        for ty in module.types.iter() {
            write!(writer, "{}\n", self.get_derives())?;
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
