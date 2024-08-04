use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CorpusIndex {
    pub(crate) index: HashMap<PathBuf, usize>,
    next_id: usize,
}

impl CorpusIndex {
    /// Creates an empty `DocumentIndex`.
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
            next_id: 0,
        }
    }

    /// Creates a new `DocumentIndex` from a list of paths.
    pub fn from_paths(entries: Vec<PathBuf>) -> Self {
        let mut index = Self::new();
        for entry in entries {
            index.insert(entry.clone());
        }
        index
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

    /// Write the document index to a disk.
    pub fn write_to_disk(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let path = path.as_ref();
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self)?;
        Ok(())
    }

    /// Load the document index from a disk.
    pub fn read_from_disk(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = path.as_ref();
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let index: CorpusIndex = serde_json::from_reader(reader)?;
        Ok(index)
    }
}

impl IntoIterator for CorpusIndex {
    type Item = (PathBuf, usize);
    type IntoIter = std::collections::hash_map::IntoIter<PathBuf, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.index.into_iter()
    }
}

impl<'a> IntoIterator for &'a CorpusIndex {
    type Item = (&'a PathBuf, &'a usize);
    type IntoIter = std::collections::hash_map::Iter<'a, PathBuf, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.index.iter()
    }
}
