use std::io::{Result, Write};

use boring_rpc_analyzer::type_store;

pub mod rust_printer;
pub mod typescript_printer;

pub use typescript_printer::TypeScriptPrinter;

pub trait Printer {
    fn write(&self, writer: &mut dyn Write, module: &type_store::Module) -> Result<()>;
}
