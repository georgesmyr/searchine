pub mod blocks;
pub mod docs;

pub use blocks::DocumentBlocks;
pub use docs::Document;

use std::path::Path;
use std::fs::DirEntry;
use std::io;


pub fn read_dir(path: impl AsRef<Path>) -> io::Result<Vec<DirEntry>> {
    let dir = std::fs::read_dir(path.as_ref())?;
    Ok(Vec::new())
}


fn filter_dir_entry(entry: DirEntry) -> Option<DirEntry> {
    let path = entry.path();
    None
}
