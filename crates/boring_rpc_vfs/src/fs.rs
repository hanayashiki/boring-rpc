use std::{fs, io::Result, path::Path};

use crate::vfs::Vfs;

pub struct Fs {}

impl Fs {
    pub fn new() -> Self {
        Self {}
    }
}

impl Vfs for Fs {
    fn read<P: AsRef<Path>>(&self, path: P) -> Result<Vec<u8>> {
        fs::read(path)
    }

    fn write<P: AsRef<Path>, C: AsRef<[u8]>>(&self, path: P, contents: C) -> Result<()> {
        fs::write(path, contents)
    }
}
