use std::collections::HashMap;
use std::fs::{File, DirEntry};
use std::io::{self, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone)]
pub struct CorpusIndexEntry {
    pub document_id: usize,
    pub modified: SystemTime,
}

impl CorpusIndexEntry {
    /// Creates a new `CorpusIndexEntry` with specified document ID,
    /// and the last time the document was modified.
    pub fn new(document_id: usize, modified: SystemTime) -> Self {
        Self {
            document_id,
            modified,
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct CorpusIndex {
    pub(crate) index: HashMap<PathBuf, CorpusIndexEntry>,
    next_id: usize,
}

impl Default for CorpusIndex {
    fn default() -> Self {
        Self {
            index: HashMap::new(),
            next_id: 0,
        }
    }
}

impl TryFrom<Vec<DirEntry>> for CorpusIndex {
    type Error = io::Error;
    fn try_from(iter: Vec<DirEntry>) -> io::Result<Self> {
        let mut index = CorpusIndex::default();
        for entry in iter {
            index.insert(entry)?;
        }
        Ok(index)
    }
}

impl CorpusIndex {
    /// Adds a document to the index, and assigns it a unique ID.
    fn insert(&mut self, dir_entry: DirEntry) -> io::Result<()> {
        let document_path = dir_entry.path();
        if !self.index.contains_key(&document_path) {
            let modified = dir_entry.metadata()?.modified()?;
            let entry = CorpusIndexEntry::new(self.next_id, modified);
            self.index.insert(document_path, entry);
            self.next_id += 1;
        }
        Ok(())
    }


    /// Returns the document id for a given path. If the path is not found
    /// in the index, `None` is returned.
    ///
    /// # Arguments
    ///
    /// * `document_path` - The path to the document.
    pub fn get_document_id(&self, document_path: &PathBuf) -> Option<usize> {
        Some(self.index.get(document_path)?.document_id)
    }

    /// Returns the last modified time for a given path. If the path is not found
    /// in the index, `None` is returned.
    ///
    /// # Arguments
    ///
    /// * `document_path` - The path to the document.
    pub fn get_modified(&self, document_path: &PathBuf) -> Option<SystemTime> {
        Some(self.index.get(document_path)?.modified)
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
    type Item = (PathBuf, CorpusIndexEntry);
    type IntoIter = std::collections::hash_map::IntoIter<PathBuf, CorpusIndexEntry>;

    fn into_iter(self) -> Self::IntoIter {
        self.index.into_iter()
    }
}

impl<'a> IntoIterator for &'a CorpusIndex {
    type Item = (&'a PathBuf, &'a CorpusIndexEntry);
    type IntoIter = std::collections::hash_map::Iter<'a, PathBuf, CorpusIndexEntry>;

    fn into_iter(self) -> Self::IntoIter {
        self.index.iter()
    }
}
