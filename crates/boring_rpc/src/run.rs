use anyhow::Result;
use boring_rpc_compiler::{Compiler, CompilerOptions};
use boring_rpc_vfs::{Fs, Vfs};
use clap::Parser;
use std::{ffi::OsString, rc::Rc};

use crate::args::Args;

pub fn try_run<I, T>(args: I) -> Result<()>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let args = Args::try_parse_from(args)?;

    let Args { entry_point, out_dir } = args;

    // TODO: resolve options
    let options = CompilerOptions {
        entry_point: std::path::PathBuf::from(entry_point),
        out_dir: out_dir.into(),
        writers: vec![
            // Box::new(boring_rpc_printers::TypeScriptPrinter {}),
            Box::new(boring_rpc_printers::RustPrinter {}),
        ],
    };

    let fs = Rc::new(Fs::new());
    let mut compiler = Compiler::<Fs>::new(fs.clone(), options);

    let result = compiler.compile();

    for (name, content) in result.outputs {
        fs.write(&name, &content)?;
    }

    Ok(())
}
