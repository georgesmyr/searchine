use std::collections::HashMap;
use std::fs;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use anyhow::Context;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::doc::freq::DocumentFrequencyIndex;
use crate::doc::term::DocumentTermsCounter;
use crate::inverted::Index;
use crate::postings::*;

/// An in-memory inverted index. The inverted index is a HashMap with
/// the token as the key and a postings list as the value.
#[derive(Default, Debug, Serialize, Deserialize)]
struct FrequencyInvertedIndex {
    inner: HashMap<u32, FrequencyPostingsList>,
}

impl FrequencyInvertedIndex {
    /// Inserts a document index into the in-memory frequency inverted
    /// index.
    ///
    /// For each token in the document index, the method inserts the
    /// token into the inverted index. If the token is already in the
    /// index, the posting is inserted into the postings list.
    fn index(&mut self, doc_index: DocumentFrequencyIndex) {
        let doc_id = doc_index.doc_id();

        for (token, token_freq) in doc_index {
            let posting = FrequencyPosting::new(doc_id, token_freq);
            if let Some(postings_list) = self.inner.get_mut(&token) {
                postings_list.add(posting);
            } else {
                let mut postings_list = FrequencyPostingsList::new();
                postings_list.add(posting);
                self.inner.insert(token, postings_list);
            }
        }
    }
}

/// Frequency indexing model.
///
/// It stores the inverted frequency index, and a structure
/// that stores the number of terms in each document in the
/// index.
#[derive(Default, Debug, Deserialize, Serialize)]
pub struct FrequencyIndex {
    inverted_index: FrequencyInvertedIndex,
    doc_terms_counter: DocumentTermsCounter,
}

impl FrequencyIndex {
    /// Creates a new, empty frequency index.
    pub fn new() -> Self {
        Self::default()
    }

    /// Indexes a document index with frequency postings.
    pub fn index(&mut self, doc_index: DocumentFrequencyIndex) {
        self.doc_terms_counter
            .insert_doc_terms(doc_index.doc_id(), doc_index.n_terms());
        self.inverted_index.index(doc_index);
    }

    /// Writes inverted index with frequency postings to file.
    pub fn write_to_file(self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let path = path.as_ref();
        let file = fs::File::create(path).context(format!(
            "Failed to create index file at: {}",
            path.display()
        ))?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self).context("Failed to write index to writer.")
    }

    /// Loads inverted index with frequency postings from file.
    pub fn from_file(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = path.as_ref();
        let file =
            fs::File::open(path).context(format!("Failed to open file at: {}", path.display()))?;
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).context(format!(
            "Failed to read index from file: {}",
            path.display()
        ))
    }
}

impl Index for FrequencyIndex {
    fn n_docs(&self) -> u32 {
        self.doc_terms_counter.n_docs()
    }

    fn doc_ids_containing(&self, term_id: u32) -> Vec<u32> {
        let res = self.inverted_index.inner.get(&term_id);
        match res {
            Some(postings_list) => postings_list.doc_ids(),
            _ => Vec::new(),
        }
    }

    fn n_docs_containing(&self, term_id: u32) -> u32 {
        self.inverted_index
            .inner
            .get(&term_id)
            .map_or(0, |p_lst| p_lst.len() as u32)
    }

    fn n_terms(&self, doc_id: u32) -> u32 {
        self.doc_terms_counter.get_doc_terms(doc_id)
    }

    fn term_frequency(&self, doc_id: u32, term_id: u32) -> u32 {
        let x = self.inverted_index.inner.get(&term_id).unwrap();
        let y = x.get(doc_id).unwrap();
        y.frequency()
    }
}

#[cfg(test)]
mod tests {
    use crate::doc::freq::DocumentFrequencyIndex;

    use super::*;

    #[test]
    fn test_frequency_indexing() {
        let tokens_1 = vec![1, 2, 3, 1, 4];
        let mut doc_index_1 = DocumentFrequencyIndex::new(0);
        doc_index_1.index_tokens(tokens_1);

        let tokens_2 = vec![1, 2, 5];
        let mut doc_index_2 = DocumentFrequencyIndex::new(1);
        doc_index_2.index_tokens(tokens_2);

        let mut index = FrequencyIndex::new();
        index.index(doc_index_1);
        index.index(doc_index_2);

        assert_eq!(index.n_docs(), 2);
        assert_eq!(index.n_docs_containing(1), 2);
        assert_eq!(index.n_terms(0), 5);
        assert_eq!(index.n_terms(1), 3);
        assert_eq!(index.term_frequency(0, 1), 2);
        assert_eq!(index.term_frequency(1, 1), 1);
    }
}
