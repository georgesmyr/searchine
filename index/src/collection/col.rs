use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use documents::DocumentId;

use crate::collection::CollectionEntry;

/// A struct representing a corpus index, which also serves as cache.
///
/// This struct is used to build an in-memory index for multiple documents.
/// Each document is assigned a unique document ID, and the last time the
/// document was indexed.
#[derive(Default, Serialize, Deserialize)]
pub struct Collection {
    root_dir: PathBuf,
    index: HashMap<PathBuf, CollectionEntry>,
}

impl Collection {
    /// Creates a new `CorpusIndex` from an iterator of paths.
    pub fn from_paths(iter: impl IntoIterator<Item = PathBuf>) -> io::Result<Self> {
        let mut index = Self::default();
        for path in iter {
            index.insert(path)?;
        }
        Ok(index)
    }

    /// Adds a document to the index, and assigns it a unique ID.
    pub fn insert(&mut self, document_path: PathBuf) -> io::Result<()> {
        if !self.index.contains_key(&document_path) {
            let modified = document_path.metadata()?.modified()?;
            let next_id = self.index.len() as u32;
            let entry = CollectionEntry::new(next_id, modified);
            self.index.insert(document_path, entry);
        }
        Ok(())
    }

    /// Returns true if the index contains a document with the specified path.
    /// Otherwise, it returns false.
    pub fn contains_path(&self, document_path: &PathBuf) -> bool {
        self.index.contains_key(document_path)
    }

    /// Returns the document id for a given path. If the path is not found
    /// in the index, `None` is returned.
    ///
    /// # Arguments
    ///
    /// * `document_path` - The path to the document.
    pub fn get_document_id(&self, document_path: &PathBuf) -> Option<DocumentId> {
        Some(self.index.get(document_path)?.document_id())
    }

    /// Returns the last modified time for a given path. If the path is not found
    /// in the index, `None` is returned.
    ///
    /// # Arguments
    ///
    /// * `document_path` - The path to the document.
    ///
    /// # Returns
    ///
    /// An `Option` containing the last modified time if the document exists,
    /// or `None` if it does not.
    pub fn get_last_modified(&self, document_path: &PathBuf) -> Option<SystemTime> {
        Some(self.index.get(document_path)?.modified())
    }

    /// Removes an index entry with the specified document path.
    ///
    /// # Arguments
    ///
    /// * `document_path` - The path to the document.
    ///
    /// # Returns
    ///
    /// An `Option` containing the removed `CollectionEntry` if it exists,
    /// or `None` if it does not.
    pub fn remove(&mut self, document_path: &PathBuf) -> Option<CollectionEntry> {
        self.index.remove(document_path)
    }

    /// Write the document index to a disk.
    pub fn into_file(self, path: impl AsRef<Path>) -> io::Result<()> {
        let path = path.as_ref();
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self)?;
        Ok(())
    }

    /// Load the document index from a disk.
    pub fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = path.as_ref();
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let index = serde_json::from_reader(reader)?;
        Ok(index)
    }
}

impl IntoIterator for Collection {
    type Item = (PathBuf, CollectionEntry);
    type IntoIter = std::collections::hash_map::IntoIter<PathBuf, CollectionEntry>;

    fn into_iter(self) -> Self::IntoIter {
        self.index.into_iter()
    }
}

impl<'a> IntoIterator for &'a Collection {
    type Item = (&'a PathBuf, &'a CollectionEntry);
    type IntoIter = std::collections::hash_map::Iter<'a, PathBuf, CollectionEntry>;

    fn into_iter(self) -> Self::IntoIter {
        self.index.iter()
    }
}
