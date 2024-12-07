use std::io::{Result, Write};

use boring_rpc_analyzer::type_store;

pub mod rust_printer;
pub mod typescript_printer;

pub use rust_printer::RustPrinter;
pub use typescript_printer::TypeScriptPrinter;

pub trait Printer {
    fn file_name(&self) -> String;

    fn write(&self, writer: &mut dyn Write, module: &type_store::Module) -> Result<()>;
}
