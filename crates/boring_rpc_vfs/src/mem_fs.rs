use crate::vfs::Vfs;
use std::cell::RefCell;
use std::io::Result;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub struct MemFs {
    store: RefCell<HashMap<PathBuf, Vec<u8>>>,
}

impl Vfs for MemFs {
    fn read<P: AsRef<Path>>(&self, path: P) -> Result<Vec<u8>> {
        self.store
            .borrow()
            .get(path.as_ref())
            .cloned()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "file not found"))
    }

    fn write<P: AsRef<Path>, C: AsRef<[u8]>>(&self, path: P, contents: C) -> Result<()> {
        self.store
            .borrow_mut()
            .insert(path.as_ref().to_path_buf(), contents.as_ref().to_vec());
        Ok(())
    }
}

impl MemFs {
    pub fn from(files: &[(&str, &str)]) -> Self {
        let store = files
            .iter()
            .map(|(path, content)| (PathBuf::from(path), content.as_bytes().to_vec()))
            .collect();

        Self {
            store: RefCell::new(store),
        }
    }
}
