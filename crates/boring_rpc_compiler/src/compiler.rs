use std::path::PathBuf;

use boring_rpc_analyzer::{
    semantic_store::{ModuleId, SemanticStore},
    type_store::TypeStore,
};
use boring_rpc_parser::parser::Parser;
use boring_rpc_syn::{nodes, SyntaxNode};
use boring_rpc_vfs::{mem_fs::MemFs, vfs::Vfs};

use boring_rpc_printers::Printer;

#[cfg(test)]
mod test_compiler;

pub struct CompilerOptions {
    entry_point: PathBuf,
    writers: Vec<Box<dyn Printer>>,
}

pub struct Compiler<V: Vfs> {
    vfs: V,
    semantic_store: SemanticStore,
    type_store: TypeStore,
    options: CompilerOptions,
}

#[derive(Debug)]
pub struct CompilationResult {
    pub outputs: Vec<(String, String)>,
}

impl<V: Vfs> Compiler<V> {
    pub fn in_mem(vfs: MemFs, options: CompilerOptions) -> Compiler<MemFs> {
        Compiler::<MemFs> {
            vfs,
            semantic_store: SemanticStore::default(),
            type_store: TypeStore::default(),
            options,
        }
    }

    fn compile(&mut self) -> CompilationResult {
        let entry_point_content: String =
            String::from_utf8(self.vfs.read(&self.options.entry_point).unwrap()).unwrap();

        let mut parser = Parser::of(&entry_point_content);

        let node = parser.parse_module();
        let module_syntax_node = SyntaxNode::root::<nodes::Module>(node).unwrap();

        let module_id = self
            .semantic_store
            .build_module(ModuleId::new("inline"), &module_syntax_node);

        let module = self.semantic_store.get_module(module_id.clone()).unwrap();

        let module_type = self.type_store.infer_module(module);

        let mut result = CompilationResult {
            outputs: Vec::new(),
        };

        for writer in &self.options.writers {
            let mut output: Vec<u8> = Vec::new();

            writer.write(&mut output, &module_type).unwrap();

            result
                .outputs
                .push(("output.ts".into(), String::from_utf8(output).unwrap()));
        }

        result
    }
}
