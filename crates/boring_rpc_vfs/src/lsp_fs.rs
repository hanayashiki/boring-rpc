use crate::vfs::Vfs;
use std::io::Result;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub enum LspFileChange {}

#[derive(Default, Debug)]
pub struct LspFs {
    store: HashMap<PathBuf, Vec<u8>>,
}

impl Vfs for LspFs {
    fn read<P: AsRef<Path>>(&self, path: P) -> Result<Vec<u8>> {
        self.store
            .get(path.as_ref())
            .cloned()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "file not found"))
    }
}

impl LspFs {
    pub fn set_file_content<P>(&mut self, path: P, change: LspFileChange)
    where
        P: AsRef<Path>,
    {
        unimplemented!()
    }
}
