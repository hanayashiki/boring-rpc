use std::io::Result;
use std::path::Path;

pub trait Vfs {
    fn read<P: AsRef<Path>>(&self, path: P) -> Result<Vec<u8>>;
}
