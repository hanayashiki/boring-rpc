use std::{f32::consts::E, io, path::PathBuf, rc::Rc};

use boring_rpc_parser::parser::Parser;
use boring_rpc_syn::{nodes, AstNode, SyntaxNode};
use boring_rpc_vfs::vfs::Vfs;

pub struct Resolver<V: Vfs> {
    vfs: Rc<V>,
    root: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleError {
    pub kind: ModuleErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleErrorKind {
    FromUtf8Error,
}

#[derive(Debug)]
pub enum Resolution {
    Module(Result<nodes::Module, ModuleError>),
    Std,
}

impl<V: Vfs> Resolver<V> {
    pub fn new(vfs: Rc<V>, root: PathBuf) -> Self {
        Resolver { vfs, root }
    }

    pub fn resolve(&self, source: &str) -> Result<Resolution, io::Error> {
        if source == "std" {
            Ok(Resolution::Std)
        } else {
            assert!(source.starts_with("~/"));
            let relative_path = &source[2..];
            let content = self.vfs.read(relative_path)?;

            match String::from_utf8(content) {
                Ok(s) => {
                    let green_node = Parser::of(&s).parse_module();

                    Ok(Resolution::Module(Ok(SyntaxNode::root::<nodes::Module>(
                        green_node,
                    )
                    .expect("parsing source should get nodes::Module"))))
                }
                Err(_) => Ok(Resolution::Module(Err(ModuleError {
                    kind: ModuleErrorKind::FromUtf8Error,
                }))),
            }
        }
    }
}
