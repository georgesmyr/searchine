use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct DocumentIndex {
    pub index: HashMap<PathBuf, usize>,
    pub next_id: usize,
}


impl DocumentIndex {
    /// Creates an empty `DocumentIndex`.
    pub fn new() -> Self {
        Self { index: HashMap::new(), next_id: 0 }
    }

    /// Adds a document to the index, and assigns it a unique ID.
    pub fn insert(&mut self, document_path: PathBuf) {
        if !self.index.contains_key(&document_path) {
            self.index.insert(document_path, self.next_id);
            self.next_id += 1;
        }
    }

    /// Returns the document id for a given path.
    pub fn get_document_id(&self, document_path: &PathBuf) -> Option<&usize> {
        self.index.get(document_path)
    }

    /// Write it to a disk.
    pub fn write_to_disk(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        todo!()
    }
}