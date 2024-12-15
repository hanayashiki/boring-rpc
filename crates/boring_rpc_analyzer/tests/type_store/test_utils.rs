use std::rc::Rc;

use boring_rpc_resolver::Resolver;
use boring_rpc_vfs::MemFs;

use boring_rpc_analyzer::{
    semantic_store::SemanticStore,
    type_store::{InferenceContext, TypeStore},
};

/// Check a list of files (using the first one as entry point), get the infered types
pub fn check(files: &[(&str, &str)], expect: expect_test::Expect) {
    let mut semantic_store = SemanticStore::default();
    let mut type_store = TypeStore::new();
    let vfs = Rc::new(MemFs::from(files));
    let resolver = Resolver::new(vfs.clone(), "/".into());

    if let boring_rpc_resolver::Resolution::Module((resolution_id, module)) =
        resolver.resolve(&format!("~/{}", files[0].0)).unwrap()
    {
        let module_id = semantic_store.build_module(resolution_id.into(), &module);

        let mut ctx = InferenceContext {
            resolver: &resolver,
            sementic_store: &mut semantic_store,
            module_id,
        };

        expect.assert_eq(&format!("{:#?}", type_store.infer_module(&mut ctx)));
    } else {
        panic!("Expected module");
    }
}
