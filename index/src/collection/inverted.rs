use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::Context;

use documents::DocumentId;

use crate::collection::Collection;

/// A structure that maps document IDs to their path.
pub struct InvertedCollection {
    inner: HashMap<DocumentId, PathBuf>,
}

impl InvertedCollection {
    /// Creates an `InvertedCollection` from a `Collection` file.
    pub fn from_file(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let collection =
            Collection::from_file(path).context("Failed to load collection from file.")?;
        let inv = collection
            .into_iter()
            .map(|(path, entry)| (entry.document_id(), path.clone()))
            .collect::<HashMap<DocumentId, PathBuf>>();

        Ok(InvertedCollection { inner: inv })
    }

    /// Returns the path of the document with specified document ID.
    pub fn get_path(&self, doc_id: DocumentId) -> Option<&PathBuf> {
        self.inner.get(&doc_id)
    }
}
