use std::{f32::consts::E, io, path::PathBuf, rc::Rc};

use thiserror::Error;

use boring_rpc_parser::parser::Parser;
use boring_rpc_syn::{nodes, AstNode, SyntaxNode};
use boring_rpc_vfs::Vfs;

pub struct Resolver<V: Vfs> {
    vfs: Rc<V>,
    root: PathBuf,
}

#[derive(Debug, Error)]
pub enum ModuleError {
    #[error("io error")]
    IOError(#[from] io::Error),
    #[error("from utf8 error")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolutionId(pub String);

#[derive(Debug)]
pub enum Resolution {
    Module((ResolutionId, nodes::Module)),
    Std,
}

impl<V: Vfs> Resolver<V> {
    pub fn new(vfs: Rc<V>, root: PathBuf) -> Self {
        Resolver { vfs, root }
    }

    pub fn resolve(&self, source: &str) -> Result<Resolution, ModuleError> {
        if source == "std" {
            Ok(Resolution::Std)
        } else {
            assert!(source.starts_with("~/"));
            let abs_path = self.root.join(&source[2..]);
            let resolution_id = ResolutionId(abs_path.to_string_lossy().to_string());
            let content = self.vfs.read(abs_path)?;

            match String::from_utf8(content) {
                Ok(s) => {
                    let green_node = Parser::of(&s).parse_module();

                    Ok(Resolution::Module((
                        resolution_id,
                        SyntaxNode::root::<nodes::Module>(green_node)
                            .expect("parsing source should get nodes::Module"),
                    )))
                }
                Err(err) => Err(err.into()),
            }
        }
    }
}
