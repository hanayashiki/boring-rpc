use std::{path::PathBuf, rc::Rc};

use boring_rpc_analyzer::{
    semantic_store::{ModuleId, SemanticStore},
    type_store::{InferenceContext, TypeStore},
};
use boring_rpc_parser::parser::Parser;
use boring_rpc_resolver::Resolver;
use boring_rpc_syn::{nodes, SyntaxNode};
use boring_rpc_vfs::{MemFs, Vfs};

use boring_rpc_printers::Printer;

#[cfg(test)]
mod test_compiler;
#[cfg(test)]
mod test_rust_axum;

#[derive(Default)]
pub struct CompilerOptions {
    pub entry_point: PathBuf,
    pub out_dir: PathBuf,
    pub writers: Vec<Box<dyn Printer>>,
}

pub struct Compiler<V: Vfs> {
    vfs: Rc<V>,
    resolver: Resolver<V>,
    semantic_store: SemanticStore,
    type_store: TypeStore<V>,
    options: CompilerOptions,
}

#[derive(Debug)]
pub struct CompilationResult {
    pub outputs: Vec<(String, String)>,
}

impl<V: Vfs> Compiler<V> {
    pub fn new_in_mem(vfs: MemFs, options: CompilerOptions) -> Compiler<MemFs> {
        let vfs = Rc::new(vfs);

        let root = options.entry_point.parent().unwrap();

        Compiler::<MemFs> {
            vfs: vfs.clone(),
            resolver: Resolver::new(vfs.clone(), root.into()),
            semantic_store: SemanticStore::default(),
            type_store: TypeStore::new(),
            options,
        }
    }

    pub fn new(vfs: Rc<V>, options: CompilerOptions) -> Compiler<V> {
        let root = options.entry_point.parent().unwrap();

        Compiler::<V> {
            vfs: vfs.clone(),
            resolver: Resolver::new(vfs.clone(), root.into()),
            semantic_store: SemanticStore::default(),
            type_store: TypeStore::new(),
            options,
        }
    }

    pub fn compile(&mut self) -> CompilationResult {
        let entry_point_content =
            String::from_utf8(self.vfs.read(&self.options.entry_point).unwrap()).unwrap();

        let mut parser = Parser::of(&entry_point_content);

        let node = parser.parse_module();
        let module_syntax_node = SyntaxNode::root::<nodes::Module>(node).unwrap();

        let module_id = self
            .semantic_store
            .build_module(ModuleId::new("inline"), &module_syntax_node);

        let module_type = self.type_store.infer_module(&mut InferenceContext {
            sementic_store: &mut self.semantic_store,
            resolver: &self.resolver,
            module_id,
        });

        let mut result = CompilationResult {
            outputs: Vec::new(),
        };

        for writer in &self.options.writers {
            let mut output: Vec<u8> = Vec::new();

            writer.write(&mut output, &module_type).unwrap();

            result.outputs.push((
                self.options.out_dir.join(writer.file_name()).to_str().unwrap().into(),
                String::from_utf8(output).unwrap(),
            ));
        }

        result
    }
}
