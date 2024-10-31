use boring_rpc_analyzer::{semantic_store::SemanticStore, type_store::TypeStore};
use boring_rpc_vfs::vfs::Vfs;

pub struct CompilerOptions {
    entry_point: String,
}

pub struct Compiler<V: Vfs> {
    vfs: V,
    semantic: SemanticStore,
    type_store: TypeStore,
    options: CompilerOptions,
}

pub struct CompilationResult {
    
}

impl<V: Vfs> Compiler<V>  {
    fn compile(&mut self) {
    }
}
